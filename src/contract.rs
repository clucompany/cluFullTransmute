
//! Data Transformation Contract.

use crate::raw_transmute::inline_unchecked_transmute;
use core::hash::Hash;
use core::marker::PhantomData;
use core::fmt::Debug;
use core::ops::DerefMut;
use core::ops::Deref;

/// A contract for converting or reading data of related types. 
/// Creating such a contract is not safe because only the creator of 
/// the contract can guarantee that the converted type will match.
#[repr(transparent)]
pub struct Contract<T, To> {
	data: T,
	_pp: PhantomData<To>,
}

impl<T, To> Clone for Contract<T, To> where T: Clone {
	#[inline(always)]
	fn clone(&self) -> Self {
		let new_data = Clone::clone(&self.data);
		
		unsafe {
			Self::force_new(new_data)
		}
	}
}

impl<T, To> Debug for Contract<T, To> where T: Debug {
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Debug::fmt(&self.data, f)
	}
}

impl<T, To> PartialEq for Contract<T, To> where T: PartialEq {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self.data, other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool {
		PartialEq::ne(&self.data, other)
	}
}

impl<T, To> PartialOrd for Contract<T, To> where T: PartialOrd {
	#[inline(always)]
	fn partial_cmp(&self, o: &Self) -> core::option::Option<core::cmp::Ordering> {
		PartialOrd::partial_cmp(&self.data, o)
	}
	
	#[inline(always)]
	fn lt(&self, other: &Self) -> bool {
		PartialOrd::lt(&self.data, other)
	}
	
	#[inline(always)]
	fn le(&self, other: &Self) -> bool {
		PartialOrd::le(&self.data, other)
	}
	
	#[inline(always)]
	fn gt(&self, other: &Self) -> bool {
		PartialOrd::gt(&self.data, other)
	}
	
	#[inline(always)]
	fn ge(&self, other: &Self) -> bool {
		PartialOrd::ge(&self.data, other)
	}
}

impl<T, To> Eq for Contract<T, To> where T: Eq {
	#[inline(always)]
	fn assert_receiver_is_total_eq(&self) {
		Eq::assert_receiver_is_total_eq(&self.data)
	}
}

impl<T, To> Ord for Contract<T, To> where T: Ord {
	#[inline(always)]
	fn cmp(&self, c: &Self) -> core::cmp::Ordering {
		Ord::cmp(&self.data, c)
	}
}

impl<T, To> Hash for Contract<T, To> where T: Hash {
	#[inline(always)]
	fn hash<H>(&self, h: &mut H) where H: core::hash::Hasher {
		Hash::hash(&self.data, h)
	}
}

/// Possible mistakes Contract
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractErr<T> {
	InvalidSizeData(T, usize, usize)
}

impl<T, To> Contract<T, To> {
	/// Create a contract without checks.
	#[inline]
	pub const unsafe fn force_new(data: T) -> Self {
		Self {
			data,
			_pp: PhantomData,
		}
	}
	
	/// Create a contract, but only check the data sizes.
	#[inline]
	pub const unsafe fn checksize_new(data: T) -> Result<Self, ContractErr<T>> {
		if core::mem::size_of::<T>() != core::mem::size_of::<To>() {
			return Err(ContractErr::InvalidSizeData(
				data,
				core::mem::size_of::<T>(),
				core::mem::size_of::<To>()
			));
		}
		
		let sself = Self::force_new(data);
		Ok(sself)
	}
	
	/// Create a contract, but just check the data sizes. 
	/// In case of a dimension mismatch, throw a panic.
	#[inline]
	pub const unsafe fn checksize_new_or_panic(data: T) -> Self {
		if core::mem::size_of::<T>() != core::mem::size_of::<To>() {
			panic!(
				concat!(
					"Error using `checksize_new_or_panic`, size of type `",
					stringify!(T),
					"` is not equal to size of type `",
					stringify!(D),
					"`."
				)
			);
		}
		
		Self::force_new(data)
	}
	
	/// Get a link to the data.
	#[inline(always)]
	pub const fn as_data<'a>(&'a self) -> &'a T {
		&self.data
	}
	
	/// Getting a pseudo-pointer to the converted value without substitution.
	#[inline]
	pub const fn as_datato<'a>(&'a self) -> &'a To {
		let data_ptr: &'a T = self.as_data();
		
		unsafe {
			let new_data_ptr: &'a To = inline_unchecked_transmute(data_ptr);
			new_data_ptr
		}
	}
	
	/// Getting a mutable pseudo-pointer to the converted value without substitution.
	#[inline]
	pub fn as_mut_datato<'a>(&'a mut self) -> &'a mut To {
		let data_ptr: &'a mut T = self.as_mut_data();
		
		unsafe {
			let new_data_ptr: &'a mut To = inline_unchecked_transmute(data_ptr);
			new_data_ptr
		}
	}
	
	/// Get a link to the mutable data.
	#[inline(always)]
	pub fn as_mut_data<'a>(&'a mut self) -> &'a mut T {
		&mut self.data
	}
	
	/// Ignoring the contract, the requirement to return the data back.
	#[inline]
	pub const fn ignore_into(self) -> T {
		// To implement permanent movement, follow these steps:
		let sself: Self = self;
		let data: T = unsafe {
			inline_unchecked_transmute(sself)
		};
		
		// This is allowed because we have repr transparent.
		
		data
	}
	
	/// Execute the contract and return a value with the new data type.
	#[inline]
	pub const fn into(self) -> To {
		let data: T = self.ignore_into();
		
		unsafe {
			let result: To = inline_unchecked_transmute(data);
			result
		}
	}
}

impl<T, To> Deref for Contract<T, To> {
	type Target = T;
	
	#[inline(always)]
	fn deref<'a>(&'a self) -> &'a Self::Target {
		self.as_data()
	}
}

impl<T, To> DerefMut for Contract<T, To> {
	#[inline(always)]
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
		self.as_mut_data()
	}
}
