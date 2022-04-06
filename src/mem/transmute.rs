
use core::mem::ManuallyDrop;

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely const, data dimensions are not checked.
pub const unsafe fn force_transmute<D, To>(in_data: D) -> To {
	union UnsafeTransmute<D, To> {
		data: ManuallyDrop<D>,
		to_data: ManuallyDrop<To>,
		
		#[allow(dead_code)]
		_shadow_null: (), // It's necessary?
	}
	
	let wait_transmute_data = UnsafeTransmute {
		data: ManuallyDrop::new(in_data)
	};
	
	ManuallyDrop::into_inner(wait_transmute_data.to_data)
}

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely constant, in case of a size mismatch, a panic pops up.
pub const unsafe fn check_sizedata_transmute<D, To>(in_data: D) -> To {
	if core::mem::size_of::<D>() != core::mem::size_of::<To>() {
		panic!(
			concat!(
				"Error using `check_sizedata_transmute`, size of type `",
				stringify!(T),
				"` is not equal to size of type `",
				stringify!(D),
				" ."
			)
		);
	}
	
	force_transmute(in_data)
}

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely constant, in case of a size mismatch, a panic pops up.
#[inline(always)]
pub const unsafe fn inline_check_sizedata_transmute<D, To>(in_data: D) -> To {
	if core::mem::size_of::<D>() != core::mem::size_of::<To>() {
		panic!(
			concat!(
				"Error using `inline_check_sizedata_transmute`, size of type `",
				stringify!(T),
				"` is not equal to size of type `",
				stringify!(D),
				" ."
			)
		);
	}
	
	inline_force_transmute(in_data)
}

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely const, data dimensions are not checked.
#[inline(always)]
pub const unsafe fn inline_force_transmute<D, To>(in_data: D) -> To {
	union UnsafeTransmute<D, To> {
		data: ManuallyDrop<D>,
		to_data: ManuallyDrop<To>,
		
		#[allow(dead_code)]
		_shadow_null: (), // It's necessary?
	}
	
	let wait_transmute_data = UnsafeTransmute {
		data: ManuallyDrop::new(in_data)
	};
	
	ManuallyDrop::into_inner(wait_transmute_data.to_data)
}

