//! Data Transformation Contract.

use crate::TransmuteErr;
use crate::TransmuteErrKind;
use crate::mem::unchecked_transmute;
use core::cmp::Ordering;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::hash::Hash;
use core::hash::Hasher;
use core::marker::PhantomData;
use core::mem::size_of;
use core::ops::Deref;
use core::ops::DerefMut;

/// A contract for converting or reading data of related types.
/// Creating such a contract is not safe because only the creator of
/// the contract can guarantee that the converted type will match.
#[repr(transparent)]
pub struct Contract<T, To> {
	data: T,

	_pp: PhantomData<To>,
}

impl<T, To> Clone for Contract<T, To>
where
	T: Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		let new_data = Clone::clone(&self.data);

		unsafe { Self::new_unchecked(new_data) }
	}
}

impl<T, To> Debug for Contract<T, To>
where
	T: Debug,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
		Debug::fmt(&self.data as &T, f)
	}
}

impl<T, To> PartialEq for Contract<T, To>
where
	T: PartialEq,
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self.data, other)
	}

	// I allow it because we are not making our own implementation,
	// but only redirecting trait functions to original ones.
	#[allow(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool {
		PartialEq::ne(&self.data as &T, other)
	}
}

impl<T, To> PartialOrd for Contract<T, To>
where
	T: PartialOrd,
{
	#[inline(always)]
	fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
		PartialOrd::partial_cmp(&self.data as &T, o)
	}

	#[inline(always)]
	fn lt(&self, other: &Self) -> bool {
		PartialOrd::lt(&self.data as &T, other)
	}

	#[inline(always)]
	fn le(&self, other: &Self) -> bool {
		PartialOrd::le(&self.data as &T, other)
	}

	#[inline(always)]
	fn gt(&self, other: &Self) -> bool {
		PartialOrd::gt(&self.data as &T, other)
	}

	#[inline(always)]
	fn ge(&self, other: &Self) -> bool {
		PartialOrd::ge(&self.data as &T, other)
	}
}

impl<T, To> Eq for Contract<T, To>
where
	T: Eq,
{
	#[inline(always)]
	fn assert_receiver_is_total_eq(&self) {
		Eq::assert_receiver_is_total_eq(&self.data as &T)
	}
}

impl<T, To> Ord for Contract<T, To>
where
	T: Ord,
{
	#[inline(always)]
	fn cmp(&self, c: &Self) -> core::cmp::Ordering {
		Ord::cmp(&self.data as &T, c)
	}
}

impl<T, To> Hash for Contract<T, To>
where
	T: Hash,
{
	#[inline(always)]
	fn hash<H>(&self, h: &mut H)
	where
		H: Hasher,
	{
		Hash::hash(&self.data as &T, h)
	}
}

impl<T, To> Contract<T, To> {
	/// Create a contract without checks.
	///
	/// # Safety
	///
	/// This function does not check that the provided data is valid for this contract.
	/// It is up to the caller to ensure that the data meets the requirements of the contract.
	#[inline]
	pub const unsafe fn new_unchecked(data: T) -> Self {
		Self {
			data,
			_pp: PhantomData,
		}
	}

	// Create a contract, but only check the data sizes.
	///
	/// # Safety
	///
	/// This function only checks that the provided data has the correct size for this contract.
	/// It does not check that the data is valid for this contract.
	/// It is up to the caller to ensure that the data meets the requirements of the contract.
	#[cfg_attr(
		all(feature = "transmute-inline", not(feature = "transmute-inline-always")),
		inline
	)]
	#[cfg_attr(feature = "transmute-inline-always", inline(always))]
	pub const unsafe fn new_checksize(in_data: T) -> Result<Self, TransmuteErr<T>> {
		{
			// Data dimension check
			let size_d = size_of::<T>();
			let size_to = size_of::<To>();

			if size_d != size_to {
				let err = TransmuteErr::new_invalid_sizecheck(size_d, size_to, in_data);

				return Err(err);
			}
		}

		let sself = unsafe { Self::new_unchecked(in_data) };
		Ok(sself)
	}

	/// Create a contract, but check the data sizes.
	/// In case of a dimension mismatch, throw a panic.
	///
	/// # Safety
	///
	/// This function only checks that the provided data has the correct size for this contract.
	/// It does not check that the data is valid for this contract.
	/// It is up to the caller to ensure that the data meets the requirements of the contract.
	#[track_caller]
	#[cfg_attr(
		all(feature = "transmute-inline", not(feature = "transmute-inline-always")),
		inline
	)]
	#[cfg_attr(feature = "transmute-inline-always", inline(always))]
	pub const unsafe fn new_checksize_or_panic(data: T) -> Self {
		{
			// #1: Data dimension check
			let size_d = size_of::<T>();
			let size_to = size_of::<To>();

			if size_d != size_to {
				let errkind = TransmuteErrKind::new_invalid_sizecheck(size_d, size_to);

				errkind.unwrap();
			}
		}

		unsafe { Self::new_unchecked(data) }
	}

	/// Get a link to the data.
	#[inline]
	pub const fn as_data(&self) -> &T {
		&self.data
	}

	/// Get a link to the mutable data.
	#[inline]
	pub const fn as_mut_data(&mut self) -> &mut T {
		&mut self.data
	}

	/// Getting a pseudo-pointer to the converted value without substitution.
	#[inline]
	pub const fn as_datato<'a>(&'a self) -> &'a To {
		let data: &'a T = self.as_data();

		unsafe {
			let new_data_ptr: &'a To = unchecked_transmute(data);

			new_data_ptr
		}
	}

	/// Getting a mutable pseudo-pointer to the converted value without substitution.
	#[inline]
	pub const fn as_mut_datato<'a>(&'a mut self) -> &'a mut To {
		let data: &'a mut T = self.as_mut_data();

		unsafe {
			let new_data_ptr: &'a mut To = unchecked_transmute(data);

			new_data_ptr
		}
	}

	/// Ignoring the contract, the requirement to return the data back.
	#[inline]
	pub const fn ignore_into(self) -> T {
		// To implement permanent movement, follow these steps:
		let sself: Self = self;
		let data: T = unsafe { unchecked_transmute(sself) };

		// This is allowed because we have repr transparent.

		data
	}

	/// Execute the contract and return a value with the new data type.
	#[inline]
	#[track_caller]
	pub const fn into(self) -> To {
		let data: T = self.ignore_into();

		unsafe {
			let result: To = unchecked_transmute(data);
			result
		}
	}
}

impl<T, To> Deref for Contract<T, To> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_data()
	}
}

impl<T, To> DerefMut for Contract<T, To> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_data()
	}
}
