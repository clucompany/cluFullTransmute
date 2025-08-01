//! Error structure and error type with a detailed description of the cause.

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;

/// Error structure and error type with a detailed description of the cause.
///
/// (Note that the `stderr` build flag includes std and
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
	/// Mismatch in input/output type sizes (e.g. `size_of::<A>() != size_of::<B>()`)
	SizeMismatch { atype: usize, btype: usize },
}

impl TransmuteErrKind {
	/// An error occurred while comparing the sizes of input and output types
	/// (sizeA is not equal to sizeB).
	#[inline]
	pub const fn size_mismatch(atype: usize, btype: usize) -> Self {
		Self::SizeMismatch { atype, btype }
	}

	/// Whether the current cause of the error is related to the inequality
	/// of data dimensions at the input and output.
	#[inline]
	pub const fn is_size_mismatch(&self) -> bool {
		matches!(self, Self::SizeMismatch { .. })
	}

	/// Creates a formatted error description in const mode.
	#[inline]
	pub const fn as_description(&self) -> DescriptionOut {
		m_as_description(*self)
	}

	/// Initialize thread panic.
	#[track_caller]
	pub const fn unwrap(self) -> ! {
		let description = self.as_description();

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
		let description = self.as_description();

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
	pub const fn size_mismatch(sizea: usize, sizeb: usize, data: T) -> Self {
		Self::new(TransmuteErrKind::size_mismatch(sizea, sizeb), data)
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

	/// Returns the reason for receiving the error.
	#[inline]
	pub fn kind(&self) -> TransmuteErrKind {
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

#[cfg_attr(docsrs, doc(cfg(feature = "stderr")))]
#[cfg(feature = "stderr")]
mod stderr {
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
				TransmuteErrKind::SizeMismatch { .. } => {
					"TransmuteErrKind::SizeMismatch(atype != bsize)"
				}
			}
		}
	}
}

#[allow(unused_imports)]
#[cfg(feature = "stderr")]
pub use stderr::*;

#[cfg_attr(docsrs, doc(cfg(feature = "error_details")))]
#[cfg(feature = "error_details")]
mod error_details {
	use crate::err::TransmuteErrKind;
	use cluConstData::buf::ConstStrBuf;
	use cluConstData::buf::size::ConstByteBufSize;

	pub type DescriptionOut = ConstStrBuf<{ CAPACITY }>;

	const CAPACITY: usize = DESCRIPTION_S0.len() // str
				+ usize::MAX_DECIMAL_LEN // usize
				+ DESCRIPTION_S1.len() // str
				+ usize::MAX_DECIMAL_LEN // usize
				+ DESCRIPTION_S2.len(); // str
	const DESCRIPTION_S0: &str = "Invalid transmute: attempted to reinterpret type A (";
	const DESCRIPTION_S1: &str = " bytes) as incompatible type B (";
	const DESCRIPTION_S2: &str = " bytes). Sizes must match exactly.";
	/// Creates a formatted error description in const mode.
	pub(crate) const fn as_description(kind: TransmuteErrKind) -> DescriptionOut {
		let (a_size, b_size) = match kind {
			TransmuteErrKind::SizeMismatch { atype, btype } => (atype, btype),
		};

		let mut buf = ConstStrBuf::new();
		//
		// format!(
		//	{DESCRIPTION_S0} {a_size} {DESCRIPTION_S1} {b_size} {DESCRIPTION_S2}
		//)
		buf.push_str(DESCRIPTION_S0);
		buf.push_usize(a_size);
		buf.push_str(DESCRIPTION_S1);
		buf.push_usize(b_size);
		buf.push_str(DESCRIPTION_S2);

		buf
	}
}

#[cfg_attr(docsrs, doc(cfg(not(feature = "error_details"))))]
#[cfg(not(feature = "error_details"))]
mod error_details {
	use core::ops::Deref;

	use crate::err::TransmuteErrKind;

	pub type DescriptionOut = &'static Str;

	/// The most common `&str`, but in a separate type with the `as_str()`
	/// function to solve the following problem:
	///
	/// use of unstable library feature `str_as_str`
	/// see issue #130366 <https://github.com/rust-lang/rust/issues/130366> for more information
	#[derive(Debug)]
	#[repr(transparent)]
	pub struct Str(str);

	impl Str {
		#[inline]
		pub const fn new(a: &str) -> &Str {
			unsafe { &*(a as *const _ as *const Str) }
		}

		#[inline]
		pub const fn as_str(&self) -> &str {
			unsafe { &*(self as *const _ as *const str) }
		}
	}

	impl Deref for Str {
		type Target = str;

		#[inline]
		fn deref(&self) -> &Self::Target {
			self.as_str()
		}
	}

	/// Creates a formatted error description in const mode.
	pub(crate) const fn as_description(kind: TransmuteErrKind) -> DescriptionOut {
		match kind {
			TransmuteErrKind::SizeMismatch { .. } => {
				Str::new("TransmuteErrKind::SizeMismatch(asize != bsize)")
			}
		}
	}
}

pub use error_details::DescriptionOut;
pub(crate) use error_details::as_description as m_as_description;
