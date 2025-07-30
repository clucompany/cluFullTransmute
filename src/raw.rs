//! Reinterprets the bits of a value of one type as another type without checking.

use core::mem::ManuallyDrop;

union TransmutData<In, Out> {
	r#in: ManuallyDrop<In>,
	out: ManuallyDrop<Out>,
}

/// Reinterprets the bits of a value of one type as another type.
/// The function is completely const, data dimensions are not checked.
///
/// # Safety
/// No protections.
#[track_caller]
#[cfg_attr(
	all(feature = "transmute-inline", not(feature = "transmute-inline-always")),
	inline
)]
#[cfg_attr(feature = "transmute-inline-always", inline(always))]
pub const unsafe fn transmute_unchecked<T, To>(in_data: T) -> To {
	// Add transmutation checks regardless of the selected function,
	// only works when `debug_assert` is active
	#[cfg(all(feature = "require_debug_assert_transmute", debug_assertions))]
	{
		use crate::err::TransmuteErrKind;

		let size_d = size_of::<T>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let errkind = TransmuteErrKind::size_mismatch(size_d, size_to);

			errkind.unwrap();
		}
	}
	let wait_transmute_data = TransmutData {
		r#in: ManuallyDrop::new(in_data),
	};
	ManuallyDrop::into_inner(unsafe { wait_transmute_data.out })
}
