// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of css-purify, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


#![warn(missing_docs)]


//! # css-purify
//!
//! This library provides a way to identify and remove unused CSS from stylesheets.
//! It does this by looking for selectors that do not match a provided HTML5 document.
//! Pseudo-elements and psuedo-classes are assumed to always match except for `:empty`, `:root`, `:any-link`, `:link` and `:visited`.
//!
//!
//! ## Getting Started
//!
//!
//! ### Purifying a CSS stylesheet
//!
//! ```
//! extern crate css_purify;
//! use ::css_purify::*;
//! use ::css_purify::html5ever_ext::*;
//! use ::css_purify::html5ever_ext::css::*;
//!
//! let document = RcDom::from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type("/path/to/document.html");
//!	let stylesheet = Stylesheet::from_file_path("/path/to/stylesheet.css");
//!
//! // If a CSS rule is unused in all documents (or nodes), then remove it.
//! stylesheet.remove_unused_css_rules(&[&document]);
//!
//! // (Optionally) Save stylesheet
//! stylesheet.to_file_path("/path/to/stylesheet.css");
//!
//! // (Optionally) Inject CSS into document, eg for use in self-contained Google AMP pages.
//! let mut first_style_node = None;
//! rc_dom.find_all_matching_child_nodes_depth_first_including_this_one(&parse_css_selector("head > style[amp]").unwrap(), &mut |node|
//! {
//! 	first_style_node = Some(node.clone());
//! 	true
//! });
//! if let Some(ref first_style_node) = first_style_node
//! {
//! 	first_style_node.append_text(&mut rc_dom, &stylesheet.to_css_string());
//! }
//!```
//!


pub extern crate html5ever_ext;


use ::html5ever_ext::Selectable;
use ::html5ever_ext::css::domain::CssRule;
use ::html5ever_ext::css::domain::HasCssRules;
use ::html5ever_ext::css::domain::selectors::DeduplicatedSelectors;
use ::html5ever_ext::css::domain::selectors::OurSelector;


include!("DeduplicatedSelectorsExt.rs");
include!("HasCssRulesExt.rs");
include!("VecExt.rs");
