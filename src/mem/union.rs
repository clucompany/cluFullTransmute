
use core::ops::DerefMut;
use core::ops::Deref;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;


/// Delayed transformation wrapper.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TransmuteData<T, To> {
	data: T,
	_p: PhantomData<To>,
}

impl<T, To> TransmuteData<T, To> {
	#[inline]
	pub const fn new(t: T) -> Self {
		Self {
			data: t,
			_p: PhantomData,
		}
	}
	
	#[inline(always)]
	pub const fn as_data(&self) -> &T {
		&self.data
	}
	
	pub const fn data(self) -> T {
		let new_self = ManuallyDrop::new(self);
		unsafe{ crate::mem::full_transmute(new_self) }
	}
	
	/// To transform data.
	pub const unsafe fn into(self) -> To {
		let new_self = ManuallyDrop::new(self);
		crate::mem::full_transmute(new_self)
	}
}

impl<T, To> From<T> for TransmuteData<T, To> {
	#[inline(always)]
	fn from(t: T) -> Self {
		Self::new(t)
	}
}

impl<T, To> Deref for TransmuteData<T, To> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<T, To> DerefMut for TransmuteData<T, To> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}



#[allow(unions_with_drop_fields)]
#[allow(dead_code)]
#[repr(C)]
pub (crate) union RawUnionTransmute<T, To> {
	data: T,
	to_data: To,
}

impl<T, To> RawUnionTransmute<T, To> {
	#[inline]
	pub const unsafe fn auto_transmute(t: T) -> To {
		Self {
			data: t
		}.to_data
	}
}


/// To transform data.
#[inline(always)]
pub const unsafe fn full_transmute<T, To>(t: T) -> To {
	RawUnionTransmute::auto_transmute(t)
}
