
use core::mem::ManuallyDrop;

/// To transform data.
pub const unsafe fn full_transmute<T, To>(t: T) -> To {
	union UnsafeTransmute<T, To> {
		data: ManuallyDrop<T>,
		to_data: ManuallyDrop<To>,
	}
	
	let to = UnsafeTransmute {
		data: ManuallyDrop::new(t)
	}.to_data;
	
	ManuallyDrop::into_inner(to)
}

