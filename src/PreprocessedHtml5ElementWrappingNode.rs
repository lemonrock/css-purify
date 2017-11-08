// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// This struct should only be used on a HTML5 DOM that has been pre-processed to remove Comments, Processing Instructions, elements with namespaces and empty text nodes.
#[derive(Clone)]
pub struct PreprocessedHtml5ElementWrappingNode
{
	node: Rc<Node>,
}

impl Debug for PreprocessedHtml5ElementWrappingNode
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		match self.node.data
		{
			Document => write!(f, "Document()"),
			
			Doctype { ref name, ref public_id, ref system_id } => write!(f, "Doctype({:?}, {:?}, {:?})", name, public_id, system_id),
			
			Text { ref contents } => write!(f, "Text({:?})", contents),
			
			Comment { ref contents } => write!(f, "Comment({:?})", contents),
			
			NodeData::Element { ref name, ref attrs, .. } => write!(f, "Element({:?}, {:?})", name, attrs),
			
			ProcessingInstruction { ref target, ref contents } => write!(f, "ProcessingInstruction({:?}, {:?})", target, contents),
		}
	}
}

impl QualNameExt for PreprocessedHtml5ElementWrappingNode
{
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.is_only_local(local_name),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.is_only_local_of(local_names),
			
			_ => false,
		}
	}
}

impl Element for PreprocessedHtml5ElementWrappingNode
{
	type Impl = OurSelectorImpl;
	
	/// Converts self into an opaque representation.
	#[inline(always)]
	fn opaque(&self) -> OpaqueElement
	{
		OpaqueElement::new(self.node.as_ref())
	}
	
	#[inline(always)]
	fn parent_element(&self) -> Option<Self>
	{
		let pointer = self.node.parent.as_ptr();
		unsafe
		{
			match *pointer
			{
				None => None,
				Some(ref weak_parent_node) => weak_parent_node.upgrade().map(|node| Self { node })
			}
		}
	}
	
	/// Skips non-element nodes
	#[inline(always)]
	fn first_child_element(&self) -> Option<Self>
	{
		self.iterate_element_children(false)
	}
	
	/// Skips non-element nodes
	#[inline(always)]
	fn last_child_element(&self) -> Option<Self>
	{
		self.iterate_element_children(true)
	}
	
	/// Skips non-element nodes
	#[inline(always)]
	fn prev_sibling_element(&self) -> Option<Self>
	{
		self.iterate_element_siblings(false)
	}
	
	/// Skips non-element nodes
	#[inline(always)]
	fn next_sibling_element(&self) -> Option<Self>
	{
		self.iterate_element_siblings(true)
	}
	
	#[inline(always)]
	fn is_html_element_in_html_document(&self) -> bool
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.is_only_local(&local_name!("html")),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn get_local_name(&self) -> &<Self::Impl as SelectorImpl>::BorrowedLocalName
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.local.deref(),
			
			_ => "",
		}
	}
	
	#[inline(always)]
	fn get_namespace(&self) -> &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.ns.deref(),
			
			_ => "",
		}
	}
	
	#[inline(always)]
	fn attr_matches(&self, ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>, local_name: &<Self::Impl as SelectorImpl>::LocalName, operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>)
		-> bool
	{
		use self::NamespaceConstraint::*;
		
		match self.node.data
		{
			NodeData::Element { ref attrs, .. } =>
			{
				for attribute in attrs.borrow().iter()
				{
					if attribute.name.local.deref() == local_name.deref()
					{
						match *ns
						{
							Any => return operation.eval_str(attribute.value.deref()),
							Specific(&NamespaceUrl(ref atom)) => if atom.deref() == attribute.name.ns.deref()
							{
								return operation.eval_str(attribute.value.deref());
							},
						}
					}
				}
				false
			},
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn match_non_ts_pseudo_class<F: FnMut(&Self, ElementSelectorFlags)>(&self, pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass, _context: &mut LocalMatchingContext<Self::Impl>, _relevant_link: &RelevantLinkStatus, _flags_setter: &mut F) -> bool
	{
		use ::css::domain::selectors::NonTreeStructuralPseudoClass::*;
		
		match *pc
		{
			any_link(_) | link | visited => self.is_link(),
			
			_ => true,
		}
	}
	
	#[inline(always)]
	fn match_pseudo_element(&self, _pe: &<Self::Impl as SelectorImpl>::PseudoElement, _context: &mut MatchingContext) -> bool
	{
		true
	}
	
	#[inline(always)]
	fn is_link(&self) -> bool
	{
		match self.node.data
		{
			NodeData::Element { ref name, ref attrs, .. } =>
			{
				if name.is_only_local_of(&[local_name!("a"), local_name!("area"), local_name!("link")])
				{
					Self::_use_attribute_value(&local_name!("href"), |_| true, false, attrs)
				}
				else
				{
					false
				}
			},
			
			_ => false
		}
	}
	
	#[inline(always)]
	fn has_id(&self, id: &<Self::Impl as SelectorImpl>::Identifier, case_sensitivity: CaseSensitivity) -> bool
	{
		if id.is_empty()
		{
			return false;
		}
		
		self.use_attribute_value(&local_name!("id"), |id_attribute_value| Self::case_sensitive_equality(case_sensitivity, id_attribute_value, id.deref()), false)
	}
	
	#[inline(always)]
	fn has_class(&self, name: &<Self::Impl as SelectorImpl>::ClassName, case_sensitivity: CaseSensitivity) -> bool
	{
		if name.is_empty()
		{
			return false;
		}
		
		self.use_attribute_value(&local_name!("class"), |class_attribute_value| class_attribute_value.split(SELECTOR_WHITESPACE).any(|class| Self::case_sensitive_equality(case_sensitivity, &**name, class)), false)
	}
	
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.node.children.borrow().is_empty()
	}
	
	#[inline(always)]
	fn is_root(&self) -> bool
	{
		if self.is_only_local(&local_name!("html"))
		{
			if let Some(parent) = self.parent_element()
			{
				match parent.node.data
				{
					Document => return true,
					_ => return false,
				}
			}
		}
		false
	}
}

impl PreprocessedHtml5ElementWrappingNode
{
	/// If this is an element node, use an attribute's value to produce a result, if the attribute exists, otherwise return a default value
	#[inline(always)]
	pub fn use_attribute_value<R, AttributeValueUser: Fn(&str) -> R>(&self, attribute_name: &LocalName, attribute_value_user: AttributeValueUser, default: R) -> R
	{
		match self.node.data
		{
			NodeData::Element { ref attrs, .. } => Self::_use_attribute_value(attribute_name, attribute_value_user, default, attrs),
			
			_ => default,
		}
	}
	
	/// If this is a text node, use the text's value to produce a result otherwise return a default value
	#[inline(always)]
	pub fn use_text_value<R, TextValueUser: Fn(&str) -> R>(&self, text_value_user: TextValueUser, default: R) -> R
	{
		match self.node.data
		{
			Text { ref contents } => text_value_user(contents.borrow().deref()),
			
			_ => default,
		}
	}
	
	/// Is this an element node, eg <a href="hello"> ?
	#[inline(always)]
	pub fn is_element_node(&self) -> bool
	{
		match self.node.data
		{
			NodeData::Element { .. } => true,
			
			_ => false,
		}
	}
	
	/// Is this a text node, eg BLAH in <a>BLAH</a> ?
	#[inline(always)]
	pub fn is_text_node(&self) -> bool
	{
		match self.node.data
		{
			NodeData::Text { .. } => true,
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn _use_attribute_value<R, AttributeValueUser: Fn(&str) -> R>(attribute_name: &LocalName, attribute_value_user: AttributeValueUser, default: R, attrs: &RefCell<Vec<Attribute>>) -> R
	{
		for attribute in attrs.borrow().iter()
		{
			if attribute.name.is_only_local(attribute_name)
			{
				return attribute_value_user(attribute.value.deref());
			}
		}
		default
	}
	
	#[inline(always)]
	fn iterate_element_children(&self, reverse: bool) -> Option<Self>
	{
		#[inline(always)]
		fn iterate<'a, I: Iterator<Item=&'a std::rc::Rc<Node>>>(mut children_iterator: I) -> Option<PreprocessedHtml5ElementWrappingNode>
		{
			let mut child_node;
			while
			{
				child_node = children_iterator.next();
				child_node.is_some()
			}
			{
				let possible = child_node.unwrap();
				match possible.data
				{
					NodeData::Element { .. } => return Some(PreprocessedHtml5ElementWrappingNode
					{
						node: possible.clone(),
					}),
					
					_ => (),
				}
			}
			None
		}
		
		let borrowed = self.node.children.borrow();
		let iterator = borrowed.iter();
		if reverse
		{
			iterate(iterator.rev())
		}
		else
		{
			iterate(iterator)
		}
		
	}
	
	#[inline(always)]
	fn iterate_element_siblings(&self, reverse: bool) -> Option<Self>
	{
		#[inline(always)]
		fn iterate<'a, I: Iterator<Item=&'a std::rc::Rc<Node>>>(this: &PreprocessedHtml5ElementWrappingNode, sibling_iterator: I) -> Option<PreprocessedHtml5ElementWrappingNode>
		{
			let mut previous_sibling = None;
			for current_sibling in sibling_iterator
			{
				if Rc::ptr_eq(&this.node, &current_sibling)
				{
					return previous_sibling;
				}
				previous_sibling = Some(PreprocessedHtml5ElementWrappingNode
				{
					node: current_sibling.clone(),
				});
			}
			unreachable!();
		}
		
		if let Some(parent) = self.parent_element()
		{
			let borrowed = parent.node.children.borrow();
			let iterator = borrowed.iter();
			if reverse
			{
				iterate(self, iterator.rev())
			}
			else
			{
				iterate(self, iterator)
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn case_sensitive_equality(case_sensitivity: CaseSensitivity, left: &str, right: &str) -> bool
	{
		case_sensitivity.eq(left.as_bytes(), right.as_bytes())
	}
}
