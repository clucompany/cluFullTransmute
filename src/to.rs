//! A handy trait for converting any `T` to the desired `To` without directly calling
//! crate functions.

use crate::TransmuteErr;

pub trait ToTransmute
where
	Self: Sized,
{
	/// A constant function reinterprets the bits of a value of one type as another type.
	///
	/// # Safety
	///
	/// If the sizes do not match, a panic arises.
	#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
	#[cfg(any(test, feature = "support_size_check_transmute"))]
	unsafe fn transmute_or_panic<To>(self) -> To;

	/// A constant function reinterprets the bits of a value of one type as another type.
	///
	/// # Safety
	///
	/// If the size does not match, an error occurs.
	#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
	#[cfg(any(test, feature = "support_size_check_transmute"))]
	unsafe fn transmute_or_errresult<To>(self) -> Result<To, TransmuteErr<Self>>;

	/// Reinterprets the bits of a value of one type as another type.
	/// The function is completely const, data dimensions are not checked.
	///
	/// # Safety
	/// No protections.
	unsafe fn unchecked_transmute<To>(self) -> To;
	/// Reinterprets the bits of a value of one type as another type.
	/// The function is completely const, data dimensions are not checked.
	///
	/// # Safety
	/// No protections.
	#[cfg_attr(docsrs, doc(cfg(feature = "transmute-inline")))]
	#[cfg(any(test, feature = "transmute-inline"))]
	unsafe fn inline_unchecked_transmute<To>(self) -> To;
}

impl<T> ToTransmute for T
where
	T: Sized,
{
	/// A constant function reinterprets the bits of a value of one type as another type.
	///
	/// # Safety
	///
	/// If the sizes do not match, a panic arises.
	#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
	#[cfg(any(test, feature = "support_size_check_transmute"))]
	unsafe fn transmute_or_panic<To>(self) -> To {
		unsafe { crate::transmute_or_panic(self) }
	}

	/// A constant function reinterprets the bits of a value of one type as another type.
	///
	/// # Safety
	///
	/// If the size does not match, an error occurs.
	#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
	#[cfg(any(test, feature = "support_size_check_transmute"))]
	unsafe fn transmute_or_errresult<To>(self) -> Result<To, TransmuteErr<Self>> {
		unsafe { crate::transmute_or_errresult(self) }
	}

	/// Reinterprets the bits of a value of one type as another type.
	/// The function is completely const, data dimensions are not checked.
	///
	/// # Safety
	/// No protections.
	unsafe fn unchecked_transmute<To>(self) -> To {
		unsafe { crate::raw::unchecked_transmute(self) }
	}

	/// Reinterprets the bits of a value of one type as another type.
	/// The function is completely const, data dimensions are not checked.
	///
	/// # Safety
	/// No protections.
	#[inline(always)]
	#[cfg_attr(docsrs, doc(cfg(feature = "transmute-inline")))]
	#[cfg(any(test, feature = "transmute-inline"))]
	unsafe fn inline_unchecked_transmute<To>(self) -> To {
		unsafe { crate::raw::inline_unchecked_transmute(self) }
	}
}
