// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of css-purify, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


#![warn(missing_docs)]


//! # css-purify
//!
//! This library provides a way to identify and remove unused CSS.


extern crate css;
#[macro_use] extern crate html5ever;
#[macro_use] extern crate quick_error;


use ::css::domain::selectors::OurSelectorImpl;
use ::css::selectors::Element;
use ::css::selectors::OpaqueElement;
use ::css::selectors::SelectorImpl;
use ::css::selectors::attr::AttrSelectorOperation;
use ::css::selectors::attr::CaseSensitivity;
use ::css::selectors::attr::NamespaceConstraint;
use ::css::selectors::attr::SELECTOR_WHITESPACE;
use ::css::selectors::matching::ElementSelectorFlags;
use ::css::selectors::matching::LocalMatchingContext;
use ::css::selectors::matching::MatchingContext;
use ::css::selectors::matching::RelevantLinkStatus;
use ::html5ever::Attribute;
use ::html5ever::LocalName;
use ::html5ever::interface::QualName;
use ::html5ever::tendril::NonAtomic;
use ::html5ever::tendril::Tendril;
use ::html5ever::tendril::fmt::UTF8;
use ::html5ever::rcdom::Node;
use ::html5ever::rcdom::NodeData;
use ::html5ever::rcdom::NodeData::*;
use ::html5ever::rcdom::RcDom;
use ::std::ascii::AsciiExt;
use ::std::cell::Cell;
use ::std::cell::RefCell;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::ops::Deref;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::rc::Rc;


include!("NodeExt.rs");
include!("QualNameExt.rs");
include!("PreprocessedHtml5ElementWrappingNode.rs");
include!("PurifyError.rs");
include!("RcDomExt.rs");




//	pub fn operateOnStyleRules<H: HasCssRules, UseStyleRule: FnMut(&mut StyleRule)>(hasCssRules: &mut H, mut useStyleRule: UseStyleRule)
//	{
//		use ::css::domain::CssRule::*;
//
//		for cssRule in hasCssRules.css_rules_vec_mut().iter_mut()
//		{
//			match *cssRule
//			{
//				Style(ref mut styleRule) => useStyleRule(styleRule),
//
//				Media(ref mut media) => Self::operateOnStyleRules(media, useStyleRule),
//
//				Supports(ref mut supports) => Self::operateOnStyleRules(supports, useStyleRule),
//
//				Document(ref mut document) => Self::operateOnStyleRules(document, useStyleRule),
//
//				_ =>
//				{
//				}
//			}
//		}
//	}
//
//	pub fn x(filePath: &Path, stylesheet: &mut Stylesheet) -> Result<(), CordialError>
//	{
//		let dom = filePath.fileContentsAsHtmlDom()?;
//
//		dom.verify(filePath)?;
//
//		dom.recursivelyStripNodesOfCommentsAndProcessingInstructionAndCreateSaneDocType(filePath)?;
//
//		Self::operateOnStyleRules(stylesheet, |mut styleRule|
//		{
//			// do something with an &mut StyleRule...
//		});
//
//		/*
//			CSS
//				- strip all namespaced selectors (HTML5 is not XHTML)
//		*/
//
//		// TODO: Sort element attributes; check id has single value; sort class attribute
//
//		// TODO: Remove extra spaces in class attributes, img src, and others...
//
//		// TODO: Custom serialization to eliminate unnecessary " and ' in attributes
//
//		// TODO: convert all node names, attributes to lower case (can be done at serialization time)
//
//		filePath.createFileWithHtmlDom(&dom.document).context(filePath)?;
//		Ok(())
//	}
