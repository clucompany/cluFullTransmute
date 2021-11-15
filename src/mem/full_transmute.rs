
use core::mem::ManuallyDrop;

/// Data transformation.
pub const unsafe fn full_transmute<D, To>(in_data: D) -> To {
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

/// Data transformation.
#[inline(always)]
pub const unsafe fn inline_full_transmute<D, To>(in_data: D) -> To {
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
