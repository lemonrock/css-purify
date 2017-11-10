// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of css-purify, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
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
//! ```
//! extern crate css_purify;
//! use ::css_purify::RcDomExt;
//! use ::css_purify::css::Stylesheet;
//!
//! let document = RcDomExt::from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type("/path/to/document.html");
//!	let stylesheet = Stylesheet::from_file_path("/path/to/stylesheet.css");
//! stylesheet.remove_unused_css_rules(&document);
//!
//! // (Optionally) Save stylesheet
//! stylesheet.to_file_path("/path/to/stylesheet.css");
//!
//! // (Optionally) Save document
//! document.XXXXX
//!
//! // (Optionally) Inject CSS into document, eg for use in self-contained Google AMP pages.
//! XXXXX
//!


pub extern crate css;
#[macro_use] extern crate html5ever;
#[macro_use] extern crate quick_error;


use ::css::domain::atRules::namespace::NamespaceUrl;
use ::css::domain::CssRule;
use ::css::domain::HasCssRules;
use ::css::domain::selectors::DeduplicatedSelectors;
use ::css::domain::selectors::matches;
use ::css::domain::selectors::OurSelector;
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
use ::html5ever::serialize::AttrRef;
use ::html5ever::serialize::Serialize;
use ::html5ever::serialize::Serializer;
use ::quick_error::ResultExt;
use ::std::ascii::AsciiExt;
use ::std::cell::Cell;
use ::std::cell::RefCell;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::io;
use ::std::io::Write;
use ::std::mem::uninitialized;
use ::std::ops::Deref;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::rc::Rc;


include!("DeduplicatedSelectorsExt.rs");
include!("FindHtmlElementsMatchingCssSelector.rs");
include!("HasCssRulesExt.rs");
include!("MinifyingHtmlSerializer.rs");
include!("MinifyingHtmlSerializerStackItem.rs");
include!("NodeExt.rs");
include!("QualNameExt.rs");
include!("PreprocessedHtml5ElementWrappingNode.rs");
include!("UltraMinifyingHtmlSerializer.rs");
include!("PurifyError.rs");
include!("RcDomExt.rs");
include!("VecExt.rs");
