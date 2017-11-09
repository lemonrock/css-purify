// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


trait VecExt<T>
{
	fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, f: F);
}

impl<T> VecExt<T> for Vec<T>
{
	fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, mut f: F)
	{
		let length = self.len();
		let mut delete_count = 0;
		{
			let vector = &mut **self;
			
			for index in 0 .. length
			{
				if !f(&mut vector[index])
				{
					delete_count += 1;
				}
				else if delete_count > 0
				{
					vector.swap(index - delete_count, index);
				}
			}
		}
		if delete_count > 0
		{
			self.truncate(length - delete_count);
		}
	}
}
