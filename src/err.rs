//! Error structure and error type with a detailed description of the cause.

use cluConstData::buf::size::ConstByteBufSize;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;

/// Error structure and error type with a detailed description of the cause.
///
/// (Note that the `support_stderr` build flag includes std and
/// implements std::error::Error for the given error.)
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
	InvalidSizeCheck(usize, usize),
}

impl TransmuteErrKind {
	/// An error occurred while comparing the sizes of input and output types
	/// (sizeA is not equal to sizeB).
	#[inline]
	pub const fn new_invalid_sizecheck(anum: usize, bnum: usize) -> Self {
		Self::InvalidSizeCheck(anum, bnum)
	}

	/// Whether the current cause of the error is related to the inequality
	/// of data dimensions at the input and output.
	#[inline]
	pub const fn is_invalid_sizecheck(&self) -> bool {
		matches!(self, Self::InvalidSizeCheck(..))
	}

	const CAPACITY: usize = Self::DESCRIPTION_S0.len() // str
				+ usize::MAX_DECIMAL_LEN // usize
				+ Self::DESCRIPTION_S1.len() // str
				+ usize::MAX_DECIMAL_LEN // usize
				+ Self::DESCRIPTION_S2.len(); // str
	const DESCRIPTION_S0: &str = "Error using `transmute`, size of type A=";
	const DESCRIPTION_S1: &str = " is not equal to size of type B=";
	const DESCRIPTION_S2: &str = ".";
	/// Creates a formatted error description in const mode.
	const fn description(&self) -> ConstStrBuf<{ Self::CAPACITY }> {
		let (a_size, b_size) = match self {
			TransmuteErrKind::InvalidSizeCheck(a, b) => (*a, *b),
		};

		let mut buf = ConstStrBuf::new();
		//
		// format!(
		//	{DESCRIPTION_S0} {a_size} {DESCRIPTION_S1} {b_size} {DESCRIPTION_S2}
		//)
		buf.push_str(Self::DESCRIPTION_S0);
		buf.push_usize(a_size);
		buf.push_str(Self::DESCRIPTION_S1);
		buf.push_usize(b_size);
		buf.push_str(Self::DESCRIPTION_S2);

		buf
	}

	/// Initialize thread panic.
	#[track_caller]
	pub const fn unwrap(self) -> ! {
		let description = self.description();

		/// Cold Panic 
		/// 
		/// It is assumed that panic is not the main purpose of this library.
		#[cold]
		#[track_caller]
		const fn __cold_panic(dstr: &str) -> ! {
			panic!("{}", dstr);
		}

		__cold_panic(description.as_str());
	}
}

impl<T> Display for TransmuteErr<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
		let description = self.description();

		Display::fmt(description.as_str(), f)
	}
}

impl<T> TransmuteErr<T> {
	/// Create a new error with a reason.
	#[inline]
	pub const fn new(kind: TransmuteErrKind, data: T) -> Self {
		Self { data, kind }
	}

	/// Quickly create a bug with a predefined reason for output and input type unequal size.
	#[inline]
	pub const fn new_invalid_sizecheck(sizea: usize, sizeb: usize, data: T) -> Self {
		Self::new(TransmuteErrKind::new_invalid_sizecheck(sizea, sizeb), data)
	}

	/// Always panics in const mode, this feature will be added in the future.
	#[inline]
	#[track_caller]
	pub const fn unwrap(self) -> ! {
		self.kind.unwrap()
	}

	/// Returns the data involved in creating the transmutation.
	#[inline]
	pub fn into_data(self) -> T {
		self.data
	}

	/// Returns only the reason why the error was received.
	#[inline]
	pub fn into_kind(self) -> TransmuteErrKind {
		self.kind
	}

	/// Get a link to the data.
	#[inline]
	pub const fn as_data(&self) -> &T {
		&self.data
	}

	/// Get a mutable reference to the data.
	#[inline]
	pub fn as_mut_data(&mut self) -> &mut T {
		&mut self.data
	}
}

impl<T> Deref for TransmuteErr<T> {
	type Target = TransmuteErrKind;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.kind
	}
}

#[cfg(feature = "support_stderr")]
mod _support_stderr {
	use crate::err::TransmuteErr;
	use crate::err::TransmuteErrKind;
	use core::fmt::Debug;
	use std::error::Error;

	impl<T> Error for TransmuteErr<T>
	where
		T: Debug,
	{
		#[allow(deprecated)]
		#[inline]
		fn description(&self) -> &str {
			match self.kind {
				TransmuteErrKind::InvalidSizeCheck(..) => {
					"TransmuteErr::InvalidSizeData(asize != bsize)"
				}
			}
		}
	}
}

#[allow(unused_imports)]
#[cfg(feature = "support_stderr")]
pub use _support_stderr::*;
use cluConstData::buf::ConstStrBuf;
