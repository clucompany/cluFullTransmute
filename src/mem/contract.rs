
use core::hash::Hash;
use core::marker::PhantomData;
use core::fmt::Debug;
use core::ops::DerefMut;
use core::ops::Deref;

/// Data Transformation Contract.
//pub type DataTransmutContract<T, To> = DataTransmutContract<T, To>;

/// Data Transformation Contract.
//#[derive(/*Copy, *)]
#[repr(transparent)]
pub struct DataTransmutContract<T, To> {
	data: T,
	_pp: PhantomData<To>,
}

impl<T, To> Clone for DataTransmutContract<T, To> where T: Clone {
	#[inline(always)]
	fn clone(&self) -> Self {
		let new_data = Clone::clone(&self.data);
		
		unsafe {
			Self::force_new(new_data)
		}
	}
}

impl<T, To> Debug for DataTransmutContract<T, To> where T: Debug {
	#[inline(always)]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		Debug::fmt(&self.data, f)
	}
}

impl<T, To> PartialEq for DataTransmutContract<T, To> where T: PartialEq {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self.data, other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool {
		PartialEq::ne(&self.data, other)
	}
}

impl<T, To> PartialOrd for DataTransmutContract<T, To> where T: PartialOrd {
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

impl<T, To> Eq for DataTransmutContract<T, To> where T: Eq {
	#[inline(always)]
	fn assert_receiver_is_total_eq(&self) {
		Eq::assert_receiver_is_total_eq(&self.data)
	}
}

impl<T, To> Ord for DataTransmutContract<T, To> where T: Ord {
	#[inline(always)]
	fn cmp(&self, c: &Self) -> core::cmp::Ordering {
		Ord::cmp(&self.data, c)
	}
}

impl<T, To> Hash for DataTransmutContract<T, To> where T: Hash {
	#[inline(always)]
	fn hash<H>(&self, h: &mut H) where H: core::hash::Hasher {
		Hash::hash(&self.data, h)
	}
}

/// Possible mistakes DataTransmutContract
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataTransmutContractErr {
	InvalidSizeData(usize, usize)
}

impl<T, To> DataTransmutContract<T, To> {
	#[deprecated(since="1.0.6", note="please use `DataTransmutContract::force_new` instead")]
	#[inline]
	pub const unsafe fn new(data: T) -> Self {
		Self::force_new(data)
	}
	
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
	pub /*const*/ unsafe fn checksize_new(data: T) -> Result<Self, DataTransmutContractErr> {
		if core::mem::size_of::<T>() != core::mem::size_of::<To>() {
			return Err(DataTransmutContractErr::InvalidSizeData(
				core::mem::size_of::<T>(),
				core::mem::size_of::<To>()
			));
		}
		
		Ok(Self::force_new(data))
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
					" ."
				)
			);
		}
		
		Self::force_new(data)
	}
	
	/// Get a link to the data.
	#[inline(always)]
	pub fn as_data<'a>(&'a self) -> &'a T {
		&self.data
	}
	
	/// Fast data type change to what the pointer points to
	#[inline]
	pub fn as_datato<'a>(&'a self) -> &'a To {
		let data_ptr: &'a T = self.as_data();
		
		unsafe {
			let new_data_ptr: &'a To = crate::mem::inline_force_transmute(data_ptr);
			new_data_ptr
		}
	}
	
	/// Get a link to the mutable data.
	#[inline(always)]
	pub fn as_mut_data<'a>(&'a mut self) -> &'a mut T {
		&mut self.data
	}
	
	#[doc(hidden)]
	#[deprecated(since="1.0.5", note="please use `ignore_into` instead")]
	#[inline]
	pub const fn data(self) -> T {
		self.ignore_into()
	}
	
	/// Ignoring the contract, the requirement to return the data back.
	#[inline]
	pub const fn ignore_into(self) -> T {
		// To implement permanent movement, follow these steps:
		let sself: Self = self;
		let data: T = unsafe {
			crate::mem::inline_force_transmute(sself)
		};
		
		// This is allowed because we have repr transparent.
		
		data
	}
	
	/// Execute the contract and return a value with the new data type.
	#[inline]
	pub const fn into(self) -> To {
		let data: T = self.ignore_into();
		
		unsafe {
			let result: To = crate::mem::inline_force_transmute(data);
			result
		}
	}
}

impl<T, To> Deref for DataTransmutContract<T, To> {
	type Target = T;
	
	#[inline(always)]
	fn deref<'a>(&'a self) -> &'a Self::Target {
		self.as_data()
	}
}

impl<T, To> DerefMut for DataTransmutContract<T, To> {
	#[inline(always)]
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
		self.as_mut_data()
	}
}
