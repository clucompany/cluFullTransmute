
//! Error structure and error type with a detailed description of the cause.

use core::fmt::Debug;
use core::ops::Deref;

/// Error structure and error type with a detailed description of the cause.
/// 
/// (Note that the `support_stderr` build flag includes std and 
/// implements std::error::Error and Display for the given error.)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransmuteErr<T> {
	/// The data involved in creating the transmutation.
	data: T,
	
	/// Reason for the error.
	kind: TransmuteErrKind,
}

/// Reason for getting the error.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransmuteErrKind {
	/// An error occurred while comparing the sizes of input and output types 
	/// (sizeA is not equal to sizeB).
	InvalidSizeCheck(usize, usize)
}

impl TransmuteErrKind {
	/// An error occurred while comparing the sizes of input and output types 
	/// (sizeA is not equal to sizeB).
	#[inline(always)]
	pub const fn new_invalid_sizecheck(anum: usize, bnum: usize) -> Self {
		Self::InvalidSizeCheck(anum, bnum)
	}
	
	/// Whether the current cause of the error is related to the inequality 
	/// of data dimensions at the input and output.
	#[inline(always)]
	pub const fn is_invalid_sizecheck(&self) -> bool {
		match self {
			Self::InvalidSizeCheck(..) => true,
			
			#[allow(unreachable_patterns)]
			_ => false,
		}
	}
	
	/// Always panics in const mode, this feature will be added in the future.
	#[inline]
	pub const fn unwrap(self) -> ! {
		// TODO! This function will be added in the future to fully support const.
		let str = match self {
			TransmuteErrKind::InvalidSizeCheck(_sizea, _sizeb) => concat!(
				"Error using `transmute`, size of type `",
					stringify!(sizea), // <--
				"` is not equal to size of type `",
					stringify!(sizeb), // <--
				"`."
			),
		};
		
		panic!("{}", str)
	}
}

impl<T> TransmuteErr<T> {
	/// Create a new error with a reason.
	#[inline]
	pub const fn new(kind: TransmuteErrKind, data: T) -> Self {
		Self {
			data,
			kind,
		}
	}
	
	/// Quickly create a bug with a predefined reason for output and input type unequal size.
	#[inline]
	pub const fn new_invalid_sizecheck(sizea: usize, sizeb: usize, data: T) -> Self {
		Self::new(
			TransmuteErrKind::new_invalid_sizecheck(sizea, sizeb), 
			data
		)
	}
	
	/// Always panics in const mode, this feature will be added in the future.
	#[inline]
	pub const fn unwrap(self) -> ! {
		// TODO! This function will be added in the future to fully support const.
		self.kind.unwrap()
	}
	
	/// Returns the data involved in creating the transmutation.
	#[inline(always)]
	pub /*const*/ fn into_data(self) -> T {
		self.data
	}
	
	/// Returns only the reason why the error was received.
	#[inline(always)]
	pub /*const*/ fn into_kind(self) -> TransmuteErrKind {
		self.kind
	}
	
	/// Get a link to the data.
	#[inline(always)]
	pub const fn as_data(&self) -> &T {
		&self.data
	}
	
	/// Get a mutable reference to the data.
	#[inline(always)]
	pub /*const*/ fn as_mut_data(&mut self) -> &mut T {
		&mut self.data
	}
}

impl<T> Deref for TransmuteErr<T> {
	type Target = TransmuteErrKind;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.kind
	}
}

#[cfg(feature = "support_stderr")]
mod _support_stderr {
	use core::fmt::Debug;
	use crate::err::TransmuteErr;
	use crate::err::TransmuteErrKind;
	
	use core::fmt::Display;
	use core::fmt::Formatter;
	use std::format;
	use std::error::Error;
	
	impl<T> Display for TransmuteErr<T> {
		#[inline]
		fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
			match self.kind {
				TransmuteErrKind::InvalidSizeCheck(asize, bsize) => Display::fmt(&format!("InvalidSizeCheck, {} != {}", asize, bsize), f),
			}
		}
	}

	impl<T> Error for TransmuteErr<T> where T: Debug {
		#[allow(deprecated)]
		#[inline]
		fn description(&self) -> &str {
			match self.kind {
				TransmuteErrKind::InvalidSizeCheck(..) => "TransmuteErr::InvalidSizeData(asize != bsize)"
			}
		}
	}
}

#[allow(unused_imports)]
#[cfg(feature = "support_stderr")]
pub use _support_stderr::*;
