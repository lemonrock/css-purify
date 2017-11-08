// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// Additional methods to work with QualName
pub trait QualNameExt
{
	/// Is this qualified name this local-only name (no prefix, no namespace)
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool;
	
	/// Is this qualified name on these local-only names (no prefix, no namespace)
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool;
}

impl QualNameExt for QualName
{
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		if self.prefix.is_none() && self.ns.is_empty()
		{
			self.local == *local_name
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		if self.prefix.is_none() && self.ns.is_empty()
		{
			for local_name in local_names.iter()
			{
				if self.local == *local_name
				{
					return true;
				}
			}
			false
		}
		else
		{
			false
		}
	}
}
