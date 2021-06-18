
use core::mem::ManuallyDrop;

/// Data transformation.
pub const unsafe fn full_transmute<T, To>(t: T) -> To {
	union UnsafeTransmute<T, To> {
		data: ManuallyDrop<T>,
		to_data: ManuallyDrop<To>,
	}
	
	let to = UnsafeTransmute {
		data: ManuallyDrop::new(t)
	};
	
	ManuallyDrop::into_inner(to.to_data)
}

