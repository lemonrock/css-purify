// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// Extensions to DeduplicatedSelectors.
pub trait DeduplicatedSelectorsExt
{
	/// Removes all selectors that don't match in the set of `html_document_and_nodes`.
	/// As a result, the associated StyleRule for these selectors may no longer be necessary if there are no matching selectors at all.
	#[inline(always)]
	fn remove_unmatched_selectors<HtmlDocumentOrNode: Selectable>(&mut self, html_document_and_nodes: &[&HtmlDocumentOrNode]);
}

impl DeduplicatedSelectorsExt for DeduplicatedSelectors
{
	#[inline(always)]
	fn remove_unmatched_selectors<HtmlDocumentOrNode: Selectable>(&mut self, html_document_and_nodes: &[&HtmlDocumentOrNode])
	{
		#[inline(always)]
		fn retain<HtmlDocumentOrNode: Selectable>(selector: &OurSelector, html_document_and_nodes: &[&HtmlDocumentOrNode]) -> bool
		{
			const HAS_MATCHES_SO_RETAIN: bool = true;
			
			for selectable in html_document_and_nodes.iter()
			{
				let has_matches = selectable.find_all_matching_child_nodes_depth_first_including_this_one(selector, &mut |_| HAS_MATCHES_SO_RETAIN);
				if has_matches
				{
					return HAS_MATCHES_SO_RETAIN;
				}
			}
			false
		}
		
		self.0.retain(|selector| retain(selector, html_document_and_nodes));
	}
}
