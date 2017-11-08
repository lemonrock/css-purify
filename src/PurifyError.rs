// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


quick_error!
{
	/// Represents errors that can happen within css-purify
	#[derive(Debug)]
	pub enum PurifyError
	{
		/// A file, when processed, is invalid according to a reason.
		InvalidFile(path: PathBuf, reason: String)
		{
			description(&reason)
			display("The file {:?} can not be used because: {}", path, reason)
		}
	}
}
