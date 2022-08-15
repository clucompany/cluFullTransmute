
//! Reinterprets the bits of a value of one type as another type. 
//! A more secure version than raw_transmute because it additionally checks the data sizes.

use crate::raw_transmute::unchecked_transmute;

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely constant, in case of a size mismatch, a panic pops up.
pub const unsafe fn transmute_or_panic<D, To>(in_data: D) -> To {
	let size_d = core::mem::size_of::<D>();
	let size_to = core::mem::size_of::<To>();
	
	if size_d != size_to {
		panic!(
			concat!(
				"Error using `transmute_or_panic`, size of type `",
				stringify!(size_d),
				"` is not equal to size of type `",
				stringify!(size_to),
				"`."
			)
		);
	}
	
	unchecked_transmute(in_data)
}

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely constant, in case of a size mismatch, a panic pops up.
#[inline(always)]
pub const unsafe fn inline_transmute_or_panic<D, To>(in_data: D) -> To {
	let size_d = core::mem::size_of::<D>();
	let size_to = core::mem::size_of::<To>();
	
	if size_d != size_to {
		panic!(
			concat!(
				"Error using `transmute_or_panic`, size of type `",
				stringify!(size_d),
				"` is not equal to size of type `",
				stringify!(size_to),
				"`."
			)
		);
	}
	
	unchecked_transmute(in_data)
}

/// Reinterprets the bits of a value of one type as another type.
/// The function is completely constant, if the size does not match, an error pops up.
pub const unsafe fn transmute_or_errresult<D, To>(in_data: D) -> Result<To, (&'static str, D)> {
	let size_d = core::mem::size_of::<D>();
	let size_to = core::mem::size_of::<To>();
	
	if size_d != size_to {
		return Err((
			concat!(
				"Error using `transmute_or_errresult`, size of type `",
				stringify!(size_d),
				"` is not equal to size of type `",
				stringify!(size_to),
				"`."
			), in_data
		));
	}
	
	Ok(unchecked_transmute(in_data))
}

/// Reinterprets the bits of a value of one type as another type.
/// The function is completely constant, if the size does not match, an error pops up.
#[inline(always)]
pub const unsafe fn inline_transmute_or_errresult<D, To>(in_data: D) -> Result<To, (&'static str, D)> {
	let size_d = core::mem::size_of::<D>();
	let size_to = core::mem::size_of::<To>();
	
	if size_d != size_to {
		return Err((
			concat!(
				"Error using `transmute_or_errresult`, size of type `",
				stringify!(size_d),
				"` is not equal to size of type `",
				stringify!(size_to),
				"`."
			), in_data
		));
	}
	
	Ok(unchecked_transmute(in_data))
}