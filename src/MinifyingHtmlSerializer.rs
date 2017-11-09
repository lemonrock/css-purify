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

impl<W: Write> Serializer for MinifyingHtmlSerializer<W>
{
	fn start_elem<'a, AttrIter: Iterator<Item=AttrRef<'a>>>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
	{
		self.write_all(b"<")?;
		self.write_all_qualified_name(&name)?;
		for (attribute_name, attribute_value) in attrs
		{
			// Write space before attribute
			
			self.write_all(b" ")?;
			
			
			// Write attribute name
			
			// Special exemption to write xmlns:xmlns as xmlns
			if attribute_name.ns == ns!(xmlns) && attribute_name.local == local_name!("xmlns")
			{
				self.write_all_str(attribute_name.local.deref())?;
			}
			else
			{
				self.write_all_qualified_name(attribute_name)?;
			}
			
			
			// Write attribute value (with '=') only if not-an-empty attribute
			
			if !attribute_value.is_empty()
			{
				// From HTML 5 specification at https://www.w3.org/TR/html5/syntax.html#attributes-0
				// Unquoted form: must not contain any literal space characters, any U+0022 QUOTATION MARK characters ("), U+0027 APOSTROPHE characters ('), "=" (U+003D) characters, "<" (U+003C) characters, ">" (U+003E) characters, or U+0060 GRAVE ACCENT characters (`)
				// The space characters, for the purposes of this specification, are U+0020 SPACE, "tab" (U+0009), "LF" (U+000A), "FF" (U+000C), and "CR" (U+000D).
				
				let mut can_write_unquoted = true;
				let mut contains_double_quotes = false;
				let mut contains_single_quotes = false;
				for character in attribute_value.chars()
				{
					match character
					{
						'\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' | '\u{003C}' | '\u{003D}' | '\u{003E}' | '\u{0060}' =>
						{
							can_write_unquoted = false;
						}
						'\u{0022}' =>
						{
							can_write_unquoted = false;
							contains_double_quotes = true;
						}
						'\u{0027}' =>
						{
							can_write_unquoted = false;
							contains_single_quotes = true;
						}
						_ => (),
					}
				}
				
				self.write_all(b"=")?;
				
				
				// In theory, we don't always have to escape ampersand (`&`). In practice, because of "An ambiguous ampersand is a U+0026 AMPERSAND character (&) that is followed by one or more alphanumeric ASCII characters, followed by a ";" (U+003B) character, where these characters do not match any of the names given in the named character references section" in the HTML 5 specification, we do; it would be rare for an unescaped ampersand to be unambiguous.
				
				if can_write_unquoted
				{
					self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
				}
				// Write as ='attribute_value' and escape single quotes `'` in attribute_value if `contains_single_quotes`
				else if contains_double_quotes
				{
					self.write_single_quote()?;
					if contains_single_quotes
					{
						self.write_attribute_value_escaping_ampersand_and_single_quote(attribute_value)?;
					}
					// There are no single quotes
					else
					{
						self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
					}
					self.write_single_quote()?;
				}
				// Write as ="attribute_value"; since we've previously evaluated contains_double_quotes as false, there can be no double quotes in attribute_value
				else if contains_single_quotes
				{
					self.write_double_quote()?;
					self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
					self.write_double_quote()?;
				}
				// does not contain double or single quotes; prefer the single quoted form
				else
				{
					self.write_single_quote()?;
					self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
					self.write_single_quote()?;
				}
			}
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
	
	/*
	An html element's start tag may be omitted if the first thing inside the html element is not a comment.

An html element's end tag may be omitted if the html element is not immediately followed by a comment.

A head element's start tag may be omitted if the element is empty, or if the first thing inside the head element is an element.

A head element's end tag may be omitted if the head element is not immediately followed by a space character or a comment.

A body element's start tag may be omitted if the element is empty, or if the first thing inside the body element is not a space character or a comment, except if the first thing inside the body element is a meta, link, script, style, or template element.

A body element's end tag may be omitted if the body element is not immediately followed by a comment.

An li element's end tag may be omitted if the li element is immediately followed by another li element or if there is no more content in the parent element.

A dt element's end tag may be omitted if the dt element is immediately followed by another dt element or a dd element.

A dd element's end tag may be omitted if the dd element is immediately followed by another dd element or a dt element, or if there is no more content in the parent element.

A p element's end tag may be omitted if the p element is immediately followed by an address, article, aside, blockquote, div, dl, fieldset, footer, form, h1, h2, h3, h4, h5, h6, header, hgroup, hr, main, nav, ol, p, pre, section, table, or ul, element, or if there is no more content in the parent element and the parent element is not an a element.

An rb element's end tag may be omitted if the rb element is immediately followed by an rb, rt, rtc or rp element, or if there is no more content in the parent element.

An rt element's end tag may be omitted if the rt element is immediately followed by an rb, rt, rtc, or rp element, or if there is no more content in the parent element.

An rtc element's end tag may be omitted if the rtc element is immediately followed by an rb, rtc or rp element, or if there is no more content in the parent element.

An rp element's end tag may be omitted if the rp element is immediately followed by an rb, rt, rtc or rp element, or if there is no more content in the parent element.

An optgroup element's end tag may be omitted if the optgroup element is immediately followed by another optgroup element, or if there is no more content in the parent element.

An option element's end tag may be omitted if the option element is immediately followed by another option element, or if it is immediately followed by an optgroup element, or if there is no more content in the parent element.

A colgroup element's start tag may be omitted if the first thing inside the colgroup element is a col element, and if the element is not immediately preceded by another colgroup element whose end tag has been omitted. (It can't be omitted if the element is empty.)

A colgroup element's end tag may be omitted if the colgroup element is not immediately followed by a space character or a comment.

A thead element's end tag may be omitted if the thead element is immediately followed by a tbody or tfoot element.

A tbody element's start tag may be omitted if the first thing inside the tbody element is a tr element, and if the element is not immediately preceded by a tbody, thead, or tfoot element whose end tag has been omitted. (It can't be omitted if the element is empty.)

A tbody element's end tag may be omitted if the tbody element is immediately followed by a tbody or tfoot element, or if there is no more content in the parent element.

A tfoot element's end tag may be omitted if the tfoot element is immediately followed by a tbody element, or if there is no more content in the parent element.

A tr element's end tag may be omitted if the tr element is immediately followed by another tr element, or if there is no more content in the parent element.

A td element's end tag may be omitted if the td element is immediately followed by a td or th element, or if there is no more content in the parent element.

A th element's end tag may be omitted if the th element is immediately followed by a td or th element, or if there is no more content in the parent element.

However, a start tag must never be omitted if it has any attributes.
	*/
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
			self.write_text_escaped(text)
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
	fn write_text_escaped(&mut self, text: &str) -> io::Result<()>
	{
		for character in text.chars()
		{
			match character
			{
				'&' => self.write_ampersand_escape()?,
				
				'<' => self.write_all(b"&lt;")?,
				
				'>' => self.write_all(b"&gt;")?,
				
				_ => self.write_char(character)?,
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn write_attribute_value_escaping_ampersand_and_single_quote(&mut self, attribute_value: &str) -> io::Result<()>
	{
		for character in attribute_value.chars()
		{
			match character
			{
				'&' => self.write_ampersand_escape()?,
				
				'\u{0027}' => self.write_apostrophe_escape()?,
				
				_ => self.write_char(character)?,
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn write_attribute_value_escaping_only_ampersand(&mut self, attribute_value: &str) -> io::Result<()>
	{
		for character in attribute_value.chars()
		{
			match character
			{
				'&' => self.write_ampersand_escape()?,
				
				_ => self.write_char(character)?,
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn write_single_quote(&mut self) -> io::Result<()>
	{
		self.write_all(b"'")
	}
	
	#[inline(always)]
	fn write_double_quote(&mut self) -> io::Result<()>
	{
		self.write_all(b"\"")
	}
	
	#[inline(always)]
	fn write_ampersand_escape(&mut self) -> io::Result<()>
	{
		self.write_all(b"&amp;")
	}
	
	#[inline(always)]
	fn write_apostrophe_escape(&mut self) -> io::Result<()>
	{
		// Strictly speaking `&apos;` is more descriptive but `&#39;` is shorter
		self.write_all(b"&#39;")
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
			self.write_all_str(&prefix.deref().to_ascii_lowercase())?;
			self.write_all(b":")?;
		}
		self.write_all_str(&name.local.deref().to_ascii_lowercase())
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
