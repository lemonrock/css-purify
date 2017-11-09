// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// Extensions to DeduplicatedSelectors.
pub trait DeduplicatedSelectorsExt
{
	/// Removes all selectors that don't match.
	/// As a result, the associated StyleRule for these selectors may no longer be necessary if there are no matching selectors at all.
	#[inline(always)]
	fn remove_unmatched_selectors(&mut self, rc_dom: &RcDom);
}

impl DeduplicatedSelectorsExt for DeduplicatedSelectors
{
	#[inline(always)]
	fn remove_unmatched_selectors(&mut self, rc_dom: &RcDom)
	{
		self.0.retain(|selector|
		{
			let has_matches = rc_dom.find_all_matching_nodes(selector, &mut |_| true);
			has_matches
		});
	}
}
