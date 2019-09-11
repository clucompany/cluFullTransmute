
use core::fmt::Debug;
use core::ops::DerefMut;
use core::ops::Deref;
use core::mem::ManuallyDrop;

/// Delayed transformation wrapper.
//#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub union MaybeTransmute<T, To> {
	data: ManuallyDrop<T>,
	to_data: ManuallyDrop<To>,
}

impl<T, To> MaybeTransmute<T, To> {
	#[inline]
	pub const fn new(data: T) -> Self {
		Self {
			data: ManuallyDrop::new(data)
		}
	}
	
	pub fn as_data<'a>(&'a self) -> &'a T {
		unsafe { &self.data }
	}
	
	#[inline]
	pub const fn data(self) -> T {
		let new_self = ManuallyDrop::new(self);
		unsafe { ManuallyDrop::into_inner(crate::mem::full_transmute(new_self)) }
	}
	
	#[inline]
	pub const unsafe fn into(self) -> To {
		let new_self = ManuallyDrop::new(self);
		ManuallyDrop::into_inner(crate::mem::full_transmute(new_self))
	}
}

impl<T, To> From<T> for MaybeTransmute<T, To> {
	#[inline(always)]
	fn from(t: T) -> Self {
		Self::new(t)
	}
}

impl<T, To> Deref for MaybeTransmute<T, To> {
	type Target = T;
	
	#[inline(always)]
	fn deref<'a>(&'a self) -> &'a Self::Target {
		unsafe { &self.data }
	}
}


impl<T, To> DerefMut for MaybeTransmute<T, To> {
	#[inline(always)]
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
		unsafe { &mut self.data }
	}
}

////#[derive(Debug, Clone, PartialEq)]
impl<T, To> Debug for MaybeTransmute<T, To> where T: Debug {
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		T::fmt(self.deref(), f)
	}
}

impl<T, To> Clone for MaybeTransmute<T, To> where T: Clone {
	#[inline(always)]
	fn clone(&self) -> Self {
		MaybeTransmute::new(T::clone(self.deref()))
	}
}

impl<T, To, Rhs> PartialEq<Rhs> for MaybeTransmute<T, To> where T: PartialEq<Rhs> {
	#[inline(always)]
	fn eq(&self, other: &Rhs) -> bool {
		T::eq(self.deref(), other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Rhs) -> bool {
		T::ne(self.deref(), other)
	}
}
//

impl<T, To> Drop for MaybeTransmute<T, To> {
	#[inline]
	fn drop(&mut self) {
		unsafe { ManuallyDrop::drop(&mut self.data) }
	}
}




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
