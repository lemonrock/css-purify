// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// An extension trait for HasCssRules
pub trait HasCssRulesExt: HasCssRules
{
	/// Removes unused CSS rules.
	///
	/// Returns `true` if the CssRules still contain at least one rule.
	fn remove_unused_css_rules(&mut self, rc_dom: &RcDom) -> bool;
}

impl<H: HasCssRules> HasCssRulesExt for H
{
	fn remove_unused_css_rules(&mut self, rc_dom: &RcDom) -> bool
	{
		let css_rules = self.css_rules_vec_mut();
		
		css_rules.retain_mut(|css_rule|
		{
			match *css_rule
			{
				CssRule::Style(ref mut style_rule) =>
				{
					style_rule.selectors.remove_unmatched_selectors(rc_dom);
					!style_rule.selectors.0.is_empty()
				}
				
				CssRule::Document(ref mut document_at_rule) => document_at_rule.remove_unused_css_rules(rc_dom),
				
				CssRule::Media(ref mut media_at_rule) => media_at_rule.remove_unused_css_rules(rc_dom),
				
				CssRule::Supports(ref mut supports_at_rule) => supports_at_rule.remove_unused_css_rules(rc_dom),
				
				_ => true,
			}
		});
		
		!css_rules.is_empty()
	}
}
