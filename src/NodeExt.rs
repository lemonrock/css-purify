// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// This trait adds additional methods to a a HTML DOM node.
pub trait NodeExt
{
	/// Validated a HTML DOM node, removes any child comments and processing instructions.
	fn validate_children_and_remove_comments_and_processing_instructions(&self, context: &Path) -> Result<(), PurifyError>;
	
	/// Serializes an instance of an HTML node to file.
	#[inline(always)]
	fn to_file_path<P: AsRef<Path>>(&self, html_node_file_path: P) -> Result<(), PurifyError>;
	
	/// Serializes an instance of an HTML node to a vector of bytes.
	#[inline(always)]
	fn to_bytes(&self) -> io::Result<Vec<u8>>;
	
	/// Serializes an instance of an HTML node to a writer.
	#[inline(always)]
	fn serialize_to_writer<W: Write>(&self, writer: W) -> io::Result<()>;
	
	/// Returns whether this element matches this selector.
	/// Use only after calling validate_children_and_remove_comments_and_processing_instructions()
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool;
	
	/// Recursively find element nodes that match this selector.
	/// Return true from MatchUser to abort early.
	/// The result of this function is true if MatchUser aborted early, or false otherwise.
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(PreprocessedHtml5ElementWrappingNode) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool;
}

impl NodeExt for Rc<Node>
{
	fn validate_children_and_remove_comments_and_processing_instructions(&self, context: &Path) -> Result<(), PurifyError>
	{
		let mut children = self.children.borrow_mut();
		
		if self.can_have_children() && !children.is_empty()
		{
			return Err(PurifyError::InvalidFile(context.to_path_buf(), "This node contains children when it should not.".to_owned()));
		}
		
		let mut processed_children = Vec::with_capacity(children.len());
		
		let mut previous_was_text_node = false;
		let mut last_added_node_was_text_node = false;
		for child_node in children.iter()
		{
			match child_node.data
			{
				Comment { .. } | ProcessingInstruction { .. } =>
				{
					previous_was_text_node = false;
				},
				
				Text { ref contents } =>
				{
					if !child_node.children.borrow().is_empty()
					{
						return Err(PurifyError::InvalidFile(context.to_path_buf(), "Text nodes must not have children".to_owned()));
					}
					
					if previous_was_text_node
					{
						return Err(PurifyError::InvalidFile(context.to_path_buf(), "Text nodes can not have a previous sibling which is also a text node".to_owned()));
					}
					
					// Merge with a previous text node; this occurs because we remove comments and processing instructions
					if last_added_node_was_text_node
					{
						let previous_text_node: Rc<Node> = processed_children.pop().unwrap();
						match previous_text_node.data
						{
							Text { contents: ref previous_node_contents } =>
							{
								let merged_node = Node
								{
									parent: Cell::new(Some(Rc::downgrade(self))),
									children: RefCell::new(Vec::new()),
									data: Text
									{
										contents:
										{
											let previous_contents = previous_node_contents.borrow();
											let contents = contents.borrow();
											let mut merged_contents: Tendril<UTF8, NonAtomic> = Tendril::with_capacity(previous_contents.len32() + contents.len32());
											merged_contents.push_tendril(&previous_contents);
											merged_contents.push_tendril(&contents);
											RefCell::new(merged_contents)
										}
									}
								};
								processed_children.push(Rc::new(merged_node));
							}
							_ => unreachable!("Previously added a text node"),
						}
					}
					else
					{
						processed_children.push(child_node.clone());
					}
					previous_was_text_node = true;
					last_added_node_was_text_node = true;
				}
				
				Document | Doctype { .. } =>
				{
					return Err(PurifyError::InvalidFile(context.to_path_buf(), "Document and DOCTYPE nodes are not valid children".to_owned()));
				}
				
				NodeData::Element { .. } =>
				{
					child_node.validate_children_and_remove_comments_and_processing_instructions(context)?;
					processed_children.push(child_node.clone());
					previous_was_text_node = false;
					last_added_node_was_text_node = false;
				}
			}
		}
		
		*children = processed_children;
		
		Ok(())
	}
	
	/// Saves an instance of an HTML DOM.
	#[inline(always)]
	fn to_file_path<P: AsRef<Path>>(&self, html_node_file_path: P) -> Result<(), PurifyError>
	{
		use ::std::fs::File;
		
		let path = html_node_file_path.as_ref();
		
		let file = File::create(path).context(path)?;
		
		self.serialize_to_writer(file).context(path)?;
		
		Ok(())
	}
	
	#[inline(always)]
	fn to_bytes(&self) -> io::Result<Vec<u8>>
	{
		let mut bytes = Vec::new();
		
		self.serialize_to_writer(&mut bytes)?;
		
		Ok(bytes)
	}
	
	#[inline(always)]
	fn serialize_to_writer<W: Write>(&self, writer: W) -> io::Result<()>
	{
		MinifyingHtmlSerializer::serialize_node(writer, self)
	}
	
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool
	{
		match self.data
		{
			NodeData::Element { .. } => matches(selector, &PreprocessedHtml5ElementWrappingNode
			{
				node: self.clone(),
			}),
			
			_ => false,
		}
	}
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(PreprocessedHtml5ElementWrappingNode) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool
	{
		if self.matches(selector)
		{
			let should_finish = match_user(PreprocessedHtml5ElementWrappingNode
			{
				node: self.clone(),
			});
			if should_finish
			{
				return true;
			}
		}
		
		for child in self.children.borrow().iter()
		{
			if child.find_all_matching_child_nodes_depth_first_including_this_one(selector, match_user)
			{
				return true;
			}
		}
		false
	}
}
