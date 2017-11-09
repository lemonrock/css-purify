// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// A serializer that, unlike that in the html5ever crate (which is private and used via `::html5ever::serialize::serialize()`), tries hard to minify HTML by adjusting how attributes are written.
/// This serializer does not know how to omit end elements.
/// This serializer does not know about namespaces; namespaces are just ignored (although prefixes are written).
#[derive(Debug, Clone)]
pub struct MinifyingHtmlSerializer<W: Write>
{
	writer: W,
	stack: Vec<MinifyingHtmlSerializerStackItem>,
}

impl<W: Write> MinifyingHtmlSerializer<W>
{
	/// Convenience method to serialize a single HTML DOM node.
	#[inline(always)]
	pub fn serialize_node(writer: W, node: &Rc<Node>) -> io::Result<()>
	{
		use ::html5ever::serialize::TraversalScope::IncludeNode;
		
		let mut serializer = Self::new(writer);
		node.serialize(&mut serializer, IncludeNode)
	}
	
	/// Convenience method to serialize an iterator of HTML DOM nodes.
	#[inline(always)]
	pub fn serialize_nodes<Nodes: Iterator<Item=Rc<Node>>>(writer: W, nodes: Nodes) -> io::Result<()>
	{
		use ::html5ever::serialize::TraversalScope::IncludeNode;
		
		let mut serializer = Self::new(writer);
		for node in nodes
		{
			node.serialize(&mut serializer, IncludeNode)?;
		}
		Ok(())
	}
	
	/// Construct a new serializer of HTML DOM nodes.
	#[inline(always)]
	pub fn new(writer: W) -> Self
	{
		Self
		{
			writer,
			stack: vec!
			[
				MinifyingHtmlSerializerStackItem
				{
					text_content_should_be_escaped: false,
				}
			],
		}
	}
	
	#[inline(always)]
	fn parent(&mut self) -> &mut MinifyingHtmlSerializerStackItem
	{
		self.stack.last_mut().unwrap()
	}
	
	#[inline(always)]
	fn write_escaped(&mut self, text: &str) -> io::Result<()>
	{
		for character in text.chars()
		{
			match character
			{
				'&' => self.write_ampersand()?,
				
				'<' => self.write_all(b"&lt;")?,
				
				'>' => self.write_all(b"&gt;")?,
				
				_ => self.write_char(character)?,
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn write_escaped_in_attribute(&mut self, attribute_value: &str) -> io::Result<()>
	{
		for character in attribute_value.chars()
		{
			match character
			{
				'&' => self.write_ampersand()?,
				
				// Strictly speaking &quot; is more descriptive but &#34; is shorter
				'"' => self.write_all(b"&#34;")?,
				
				_ => self.write_char(character)?,
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn write_ampersand(&mut self) -> io::Result<()>
	{
		self.write_all(b"&amp;")
	}
	
	#[inline(always)]
	fn write_char(&mut self, character: char) -> io::Result<()>
	{
		let mut buffer: [u8; 4] = unsafe { uninitialized() };
		character.encode_utf8(&mut buffer);
		
		self.write_all(&buffer[0 .. character.len_utf8()])
	}
	
	#[inline(always)]
	fn write_all_qualified_name(&mut self, name: &QualName) -> io::Result<()>
	{
		if let Some(ref prefix) = name.prefix
		{
			self.write_all_str(prefix.deref())?;
			self.write_all(b":")?;
		}
		self.write_all_str(name.local.deref())
	}
	
	#[inline(always)]
	fn write_all_str(&mut self, str: &str) -> io::Result<()>
	{
		self.write_all(str.as_bytes())
	}
	
	#[inline(always)]
	fn write_all(&mut self, bytes: &[u8]) -> io::Result<()>
	{
		self.writer.write_all(bytes)
	}
}

impl<W: Write> Serializer for MinifyingHtmlSerializer<W>
{
	fn start_elem<'a, AttrIter: Iterator<Item=AttrRef<'a>>>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
	{
		self.write_all(b"<")?;
		self.write_all_qualified_name(&name)?;
		for (attribute_name, attribute_value) in attrs
		{
			self.write_all(b" ")?;
			// Special exemption to write xmlns:xmlns as xmlns
			if attribute_name.ns == ns!(xmlns) && attribute_name.local == local_name!("xmlns")
			{
				self.write_all_str(attribute_name.local.deref())?;
			}
			else
			{
				self.write_all_qualified_name(attribute_name)?;
			}
			self.write_all(b"=\"")?;
			self.write_escaped_in_attribute(attribute_value)?;
			self.write_all(b"\"")?;
		}
		self.write_all(b">")?;
		
		self.stack.push
		(
			MinifyingHtmlSerializerStackItem
			{
				// Our escape logic does not intercept </style> or </script> sequences embedded within the text, which will cause a parsing bug in any Web browser.
				// Such a sequence is likely to be rare except in code that writes raw HTML, or in incompletely-validated user input.
				text_content_should_be_escaped: name.text_content_should_be_escaped(),
			}
		);
		
		Ok(())
	}
	
	#[inline(always)]
	fn end_elem(&mut self, name: QualName) -> io::Result<()>
	{
		self.stack.pop();
		
		if name.can_have_children()
		{
			self.write_all(b"</")?;
			self.write_all_qualified_name(&name)?;
			self.write_all(b">")?;
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn write_text(&mut self, text: &str) -> io::Result<()>
	{
		if self.parent().text_content_should_be_escaped
		{
			self.write_escaped(text)
		}
		else
		{
			self.write_all_str(text)
		}
	}
	
	#[inline(always)]
	fn write_comment(&mut self, text: &str) -> io::Result<()>
	{
		self.write_all(b"<!--")?;
		self.write_all_str(text)?;
		self.write_all(b"-->")
	}
	
	#[inline(always)]
	fn write_doctype(&mut self, name: &str) -> io::Result<()>
	{
		self.write_all(b"<!DOCTYPE ")?;
		self.write_all_str(name)?;
		self.write_all(b">")
	}
	
	#[inline(always)]
	fn write_processing_instruction(&mut self, target: &str, data: &str) -> io::Result<()>
	{
		self.write_all(b"<?")?;
		self.write_all_str(target)?;
		self.write_all(b" ")?;
		self.write_all_str(data)?;
		self.write_all(b">")
	}
}
