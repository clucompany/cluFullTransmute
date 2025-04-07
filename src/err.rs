//! Error structure and error type with a detailed description of the cause.

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

	const DESCRIPTION_S0: &str = "Error using `transmute`, size of type A=";
	const DESCRIPTION_S1: &str = " is not equal to size of type B=";
	const DESCRIPTION_S2: &str = ".";
	/// Creates a formatted error description in const mode.
	const fn description(
		&self,
	) -> ConcatWriter<
		{
			Self::DESCRIPTION_S0.len() // str
				+ ConcatWriter::USIZE_MAX_LEN // usize
				+ Self::DESCRIPTION_S1.len() // str
				+ ConcatWriter::USIZE_MAX_LEN // usize
				+ Self::DESCRIPTION_S2.len() // str
		},
	> {
		let (a_size, b_size) = match self {
			TransmuteErrKind::InvalidSizeCheck(a, b) => (*a, *b),
		};

		let mut concat = ConcatWriter::<
			{
				Self::DESCRIPTION_S0.len() // str
					+ ConcatWriter::USIZE_MAX_LEN // usize
					+ Self::DESCRIPTION_S1.len() // str
					+ ConcatWriter::USIZE_MAX_LEN // usize
					+ Self::DESCRIPTION_S2.len() // str
			},
		>::zeroed();
		//
		// similar: format!(
		//	{DESCRIPTION_S0} {a_size} {DESCRIPTION_S1} {b_size} {DESCRIPTION_S2}
		//)
		concat.push_str(Self::DESCRIPTION_S0);
		concat.push_usize(a_size);
		concat.push_str(Self::DESCRIPTION_S1);
		concat.push_usize(b_size);
		concat.push_str(Self::DESCRIPTION_S2);

		concat
	}

	/// Initialize thread panic.
	#[track_caller]
	pub const fn unwrap(self) -> ! {
		let description = self.description();

		/// Cold Panic It is assumed that panic is not the main purpose of this library.
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

use crate::err::compact_concat::ConcatWriter;

// TODO!: needs to be taken out of this crate.
/// Compact concatenation in constant mode.
///
/// **Important:** This module is intended for internal use and should ideally
/// be moved to a separate crate.
mod compact_concat {
	/// A concatenator that can perform a small set of concatenations in const mode.
	/// Note that you must initially set the concatenation buffer to the largest possible size
	///
	/// (for numbers, just use ConcatWriter::USIZE_MAX_LEN)
	pub struct ConcatWriter<const BUFF_SIZE: usize> {
		/// Raw buffer for writing data, it is important that all data written is UTF-8 compliant.
		arr: [u8; BUFF_SIZE],
		/// The amount of data actually written
		len: usize,
	}

	impl ConcatWriter<0> {
		/// The number 0, always visible to the user.
		pub const VI_ZEROED: u8 = b'0';

		/// An array filled with visible zeros is used as the initial buffer.
		pub const USIZE_VIZEROED: [u8; Self::USIZE_MAX_LEN] =
			[Self::VI_ZEROED; Self::USIZE_MAX_LEN];

		/// Defines the maximum possible length of the string representation of usize.
		pub const USIZE_MAX_LEN: usize = match size_of::<usize>() {
			a if a == size_of::<u128>() => b"340282366920938463463374607431768211455".len(),
			a if a == size_of::<u64>() => b"18446744073709551615".len(),
			a if a == size_of::<u32>() => b"4294967295".len(),
			a if a == size_of::<u16>() => b"65535".len(),
			a if a == size_of::<u8>() => b"255".len(),

			_ => b"340282366920938463463374607431768211455".len(),
		};
	}

	impl<const BUFF_SIZE: usize> ConcatWriter<BUFF_SIZE> {
		/// Initialize an empty buffer, presumably filled with zeros, but with a length of 0.
		#[inline]
		pub const fn zeroed() -> Self {
			Self {
				arr: unsafe { core::mem::zeroed() },
				len: 0,
			}
		}

		/// Cloning a buffer with the same data.
		#[allow(dead_code)]
		pub const fn clone(&self) -> Self {
			Self {
				arr: self.arr,
				len: self.len,
			}
		}

		/// Clears the concatenator buffer, effectively setting the length to 0.
		#[allow(dead_code)]
		#[inline]
		pub const fn clear(&mut self) {
			self.len = 0;
		}

		/// Pointer to the array itself
		#[inline]
		pub const fn as_ptr(&self) -> *const u8 {
			self.arr.as_ptr()
		}

		/// Number of bytes written
		#[inline]
		pub const fn len(&self) -> usize {
			self.len
		}

		/// What is the maximum number of bytes that can be written to this array initially?
		///
		/// (Note that the number of bytes written is not taken into account in any way.)
		#[inline]
		pub const fn max_available_len(&self) -> usize {
			self.arr.len()
		}

		/// How many more bytes can be written?
		#[allow(dead_code)]
		#[inline]
		pub const fn available(&self) -> usize {
			self.max_available_len() - self.len()
		}

		/// Returns how many more bytes can be written to the buffer
		#[inline]
		pub const fn as_slice(&self) -> &[u8] {
			unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
		}

		/// Represent buffer as string
		#[inline]
		pub const fn as_str(&self) -> &str {
			unsafe { core::str::from_utf8_unchecked(self.as_slice()) }
		}

		/// Write a string to the buffer.
		pub const fn push_str(&mut self, a: &str) -> usize {
			self.push_arr(a.as_bytes())
		}

		/// Add an array to the buffer, note that this function is internal since there is no guarantee whether the array is UTF-8 compliant or not.
		///
		/// The function may fail if the array length is not sufficient.
		const fn push_arr(&mut self, a: &[u8]) -> usize {
			let a_len = a.len();
			if (self.len() + a_len) > self.max_available_len() {
				panic!("The buffer is too small to write these values.");
			}

			let max = a_len;
			let mut i = 0;
			while i < max {
				self.arr[self.len + i] = a[i];

				i += 1;
			}

			self.len += a_len;

			a_len
		}

		/// Add a number to an array.
		pub const fn push_usize(&mut self, mut a: usize) -> usize {
			let mut u_buff = ConcatWriter::USIZE_VIZEROED;
			let mut u_len;

			match a {
				0 => {
					// result[USIZE_TOARRSIZE - 1] = VI_ZEROED;
					u_len = 1;
				}
				_ => {
					u_len = 0;

					let mut i = ConcatWriter::USIZE_MAX_LEN;
					loop {
						u_buff[i - 1] = ((a % 10) as u8) + ConcatWriter::VI_ZEROED;
						a /= 10;
						u_len += 1;

						if a == 0 {
							break;
						}
						i -= 1;
					}
				}
			}

			let ptr = u_buff.as_ptr();
			let ptr = unsafe { ptr.add(u_buff.len() - u_len) };
			let slice = unsafe { core::slice::from_raw_parts(ptr, u_len) };

			self.push_arr(slice)
		}
	}

	#[cfg(test)]
	#[test]
	fn __test_compact_concat_strusize_const() {
		{
			let mut u_writer = ConcatWriter::<{ ConcatWriter::USIZE_MAX_LEN }>::zeroed();
			let u_sizeof = size_of::<usize>();

			if size_of::<u128>() <= u_sizeof {
				// machine u64 mode
				u_writer.push_usize(u128::MAX as _);
				assert_eq!(u_writer.as_str(), "340282366920938463463374607431768211455");

				if size_of::<u128>() == u_sizeof {
					assert_eq!(u_writer.available(), 0);
				}
				u_writer.clear();
			}
			if size_of::<u64>() <= u_sizeof {
				// machine u64 mode
				u_writer.push_usize(u64::MAX as _);
				assert_eq!(u_writer.as_str(), "18446744073709551615");
				if size_of::<u64>() == u_sizeof {
					assert_eq!(u_writer.available(), 0);
				}
				u_writer.clear();
			}
			if size_of::<u32>() <= u_sizeof {
				// machine u32 mode
				u_writer.push_usize(u32::MAX as _);
				assert_eq!(u_writer.as_str(), "4294967295");
				if size_of::<u32>() == u_sizeof {
					assert_eq!(u_writer.available(), 0);
				}
				u_writer.clear();
			}
			if size_of::<u16>() <= u_sizeof {
				// machine u16 mode
				u_writer.push_usize(u16::MAX as _);
				assert_eq!(u_writer.as_str(), "65535");
				if size_of::<u16>() == u_sizeof {
					assert_eq!(u_writer.available(), 0);
				}
				u_writer.clear();
			}
			if size_of::<u8>() <= u_sizeof {
				// machine u16 mode
				u_writer.push_usize(u8::MAX as _);
				assert_eq!(u_writer.as_str(), "255");
				if size_of::<u8>() == u_sizeof {
					assert_eq!(u_writer.available(), 0);
				}
				u_writer.clear();
			}

			// zero
			u_writer.push_usize(0 as _);
			assert_eq!(u_writer.as_str(), "0");
			u_writer.clear();
			assert_eq!(u_writer.as_str(), "");
		}
		{
			// format_test
			//
			// format!(
			//	Error using `transmute`, size of type A={} is not equal to size of type B={}.
			// )
			let a_size = 1024;
			let b_size = 1025;

			const S0: &str = "Error using `transmute`, size of type A=";
			const S1: &str = " is not equal to size of type B=";
			const S2: &str = ".";

			let mut concat = ConcatWriter::<
				{
					S0.len()
						+ ConcatWriter::USIZE_MAX_LEN
						+ S1.len() + ConcatWriter::USIZE_MAX_LEN
						+ S2.len()
				},
			>::zeroed();
			concat.push_str(S0);
			concat.push_usize(a_size);
			concat.push_str(S1);
			concat.push_usize(b_size);
			concat.push_str(S2);

			assert_eq!(
				concat.as_str(),
				"Error using `transmute`, size of type A=1024 is not equal to size of type B=1025.",
			);
		}
	}
}
