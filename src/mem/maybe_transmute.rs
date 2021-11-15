
use core::marker::PhantomData;
use core::fmt::Debug;
use core::ops::DerefMut;
use core::ops::Deref;
use core::mem::ManuallyDrop;

/// Delayed transformation wrapper.
#[derive(/*Copy, */Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[allow(dead_code)]
pub struct MaybeTransmute<T, To> {
	data: ManuallyDrop<T>,
	_pp: PhantomData<To>,
}

impl<T, To> MaybeTransmute<T, To> {
	#[inline]
	pub const unsafe fn new(data: T) -> Self {
		Self {
			data: ManuallyDrop::new(data),
			_pp: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn as_data<'a>(&'a self) -> &'a T {
		&self.data
	}
	
	#[inline(always)]
	pub fn as_mut_data<'a>(&'a mut self) -> &'a mut T {
		&mut self.data
	}
	
	#[deprecated(since="1.0.5", note="please use `ignore_into` instead")]
	#[inline]
	pub const fn data(self) -> T {
		self.ignore_into()
	}
	
	#[inline]
	pub const fn ignore_into(self) -> T {
		let new_self = ManuallyDrop::new(self);
		
		let value: ManuallyDrop<T> = unsafe {
			crate::mem::full_transmute(new_self)
		};
		
		ManuallyDrop::into_inner(value)
	}
	
	#[inline]
	pub const fn into(self) -> To {
		let new_self = ManuallyDrop::new(self);
		
		#[allow(unused_unsafe)]
		let value: ManuallyDrop<To> = unsafe {
			crate::mem::full_transmute(new_self)
		};
		
		ManuallyDrop::into_inner(value)
	}
}

impl<T, To> Deref for MaybeTransmute<T, To> {
	type Target = T;
	
	#[inline(always)]
	fn deref<'a>(&'a self) -> &'a Self::Target {
		self.as_data()
	}
}

impl<T, To> DerefMut for MaybeTransmute<T, To> {
	#[inline(always)]
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
		self.as_mut_data()
	}
}

impl<T, To> Drop for MaybeTransmute<T, To> {
	#[inline(always)]
	fn drop(&mut self) {
		unsafe {
			ManuallyDrop::drop(&mut self.data);
		}
	}
}
