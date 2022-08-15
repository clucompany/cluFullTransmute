
//! Reinterprets the bits of a value of one type as another type. 

use core::marker::PhantomData;
use core::mem::ManuallyDrop;

pub use unchecked_transmute as transmute;

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely const, data dimensions are not checked.
pub const unsafe fn unchecked_transmute<D, To>(in_data: D) -> To {
	union UnsafeTransmute<D, To> {
		data: ManuallyDrop<D>,
		to_data: ManuallyDrop<To>,
		
		#[allow(dead_code)]
		_shadow_null: PhantomData<*const ()>, // It's necessary?
	}
	
	let wait_transmute_data = UnsafeTransmute {
		data: ManuallyDrop::new(in_data)
	};
	
	ManuallyDrop::into_inner(wait_transmute_data.to_data)
}

/// Reinterprets the bits of a value of one type as another type. 
/// The function is completely const, data dimensions are not checked.
#[inline(always)]
pub const unsafe fn inline_unchecked_transmute<D, To>(in_data: D) -> To {
	union UnsafeTransmute<D, To> {
		data: ManuallyDrop<D>,
		to_data: ManuallyDrop<To>,
		
		#[allow(dead_code)]
		_shadow_null: PhantomData<*const ()>, // It's necessary?
	}
	
	let wait_transmute_data = UnsafeTransmute {
		data: ManuallyDrop::new(in_data)
	};
	
	ManuallyDrop::into_inner(wait_transmute_data.to_data)
}

