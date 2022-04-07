
use crate::mem::transmute::inline_force_transmute;
use crate::force_transmute;

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely const, data dimensions are not checked.
#[doc(hidden)]
#[deprecated(since="1.0.6", note="please use `crate::transmute::force_transmute` instead")]
pub const unsafe fn full_transmute<D, To>(in_data: D) -> To {
	force_transmute(in_data)
}

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely const, data dimensions are not checked.
#[doc(hidden)]
#[deprecated(since="1.0.6", note="please use `crate::mem::inline_force_transmute` instead")]
#[inline(always)]
pub const unsafe fn inline_full_transmute<D, To>(in_data: D) -> To {
	inline_force_transmute(in_data)
}
