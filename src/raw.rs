//! Reinterprets the bits of a value of one type as another type without checking.

use core::marker::PhantomData;
use core::mem::ManuallyDrop;

/// Reinterprets the bits of a value of one type as another type.
/// The function is completely const, data dimensions are not checked.
pub use unchecked_transmute as transmute;

/// Reinterprets the bits of a value of one type as another type.
/// The function is completely const, data dimensions are not checked.
///
/// # Safety
/// No protections.
pub const unsafe fn unchecked_transmute<T, To>(in_data: T) -> To {
	union __TransmutData<T, To> {
		indata: ManuallyDrop<T>,
		todata: ManuallyDrop<To>,

		#[allow(dead_code)]
		_shadow_null: PhantomData<*const ()>, // It's necessary?
	}

	let wait_transmute_data = __TransmutData {
		indata: ManuallyDrop::new(in_data),
	};

	ManuallyDrop::into_inner(unsafe { wait_transmute_data.todata })
}

/// Reinterprets the bits of a value of one type as another type.
/// The function is completely const, data dimensions are not checked.
///
/// # Safety
/// No protections.
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
#[cfg(any(test, feature = "inline"))]
pub const unsafe fn inline_unchecked_transmute<T, To>(in_data: T) -> To {
	union __TransmutData<T, To> {
		indata: ManuallyDrop<T>,
		todata: ManuallyDrop<To>,

		#[allow(dead_code)]
		_shadow_null: PhantomData<*const ()>, // It's necessary?
	}

	let wait_transmute_data = __TransmutData {
		indata: ManuallyDrop::new(in_data),
	};

	ManuallyDrop::into_inner(unsafe { wait_transmute_data.todata })
}
