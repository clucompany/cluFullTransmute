//! Data Transformation TransmuteContract.

use crate::mem::transmute_unchecked;
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
pub struct TransmuteContract<IN, OUT> {
	data: IN,

	_pp: PhantomData<OUT>,
}

impl<IN, OUT> Clone for TransmuteContract<IN, OUT>
where
	IN: Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		let new_data = Clone::clone(&self.data);

		unsafe { Self::new_unchecked(new_data) }
	}
}

impl<IN, OUT> Debug for TransmuteContract<IN, OUT>
where
	IN: Debug,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
		Debug::fmt(&self.data as &IN, f)
	}
}

impl<IN, OUT> PartialEq for TransmuteContract<IN, OUT>
where
	IN: PartialEq,
{
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self.data, other)
	}

	// I allow it because we are not making our own implementation,
	// but only redirecting trait functions to original ones.
	#[allow(clippy::partialeq_ne_impl)]
	#[inline]
	fn ne(&self, other: &Self) -> bool {
		PartialEq::ne(&self.data as &IN, other)
	}
}

impl<IN, OUT> PartialOrd for TransmuteContract<IN, OUT>
where
	IN: PartialOrd,
{
	#[inline]
	fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
		PartialOrd::partial_cmp(&self.data as &IN, o)
	}

	#[inline]
	fn lt(&self, other: &Self) -> bool {
		PartialOrd::lt(&self.data as &IN, other)
	}

	#[inline]
	fn le(&self, other: &Self) -> bool {
		PartialOrd::le(&self.data as &IN, other)
	}

	#[inline]
	fn gt(&self, other: &Self) -> bool {
		PartialOrd::gt(&self.data as &IN, other)
	}

	#[inline]
	fn ge(&self, other: &Self) -> bool {
		PartialOrd::ge(&self.data as &IN, other)
	}
}

impl<IN, OUT> Eq for TransmuteContract<IN, OUT>
where
	IN: Eq,
{
	#[inline]
	fn assert_receiver_is_total_eq(&self) {
		Eq::assert_receiver_is_total_eq(&self.data as &IN)
	}
}

impl<IN, OUT> Ord for TransmuteContract<IN, OUT>
where
	IN: Ord,
{
	#[inline]
	fn cmp(&self, c: &Self) -> core::cmp::Ordering {
		Ord::cmp(&self.data as &IN, c)
	}
}

impl<IN, OUT> Hash for TransmuteContract<IN, OUT>
where
	IN: Hash,
{
	#[inline]
	fn hash<H>(&self, h: &mut H)
	where
		H: Hasher,
	{
		Hash::hash(&self.data as &IN, h)
	}
}

impl<IN, OUT> TransmuteContract<IN, OUT> {
	/// Checking contract sizes at compile time
	const TYPE_SIZE_MATCH_ASSERT: () = [()][
			// If you read this in the error logs, then you have violated one of the terms of the
			// agreement: full match of the dimensions of the input and output types.
			(size_of::<IN>() != size_of::<OUT>()) as usize
		];

	/// Create a contract without checks.
	///
	/// # Safety
	///
	/// This function does not check that the provided data is valid for this contract.
	/// It is up to the caller to ensure that the data meets the requirements of the contract.
	#[inline]
	pub const unsafe fn new_unchecked(data: IN) -> Self {
		// clippy doesn't understand what we want to do,
		// and we want to make the const check mandatory, otherwise the compiler may skip it
		#[allow(clippy::let_unit_value)]
		let _constant_checking_of_input_and_output_type_dimensions = Self::TYPE_SIZE_MATCH_ASSERT;

		Self {
			data,
			_pp: PhantomData,
		}
	}

	/// Get a link to the data.
	#[inline]
	pub const fn as_in(&self) -> &IN {
		&self.data
	}

	/// Get a link to the mutable data.
	#[inline]
	pub const fn as_mut_in(&mut self) -> &mut IN {
		&mut self.data
	}

	/// Getting a pseudo-pointer to the converted value without substitution.
	#[inline]
	#[track_caller]
	pub const fn as_out<'a>(&'a self) -> &'a OUT {
		let data: &'a IN = self.as_in();

		unsafe {
			let new_data_ptr: &'a OUT = transmute_unchecked(data);

			new_data_ptr
		}
	}

	/// Getting a mutable pseudo-pointer to the converted value without substitution.
	#[inline]
	#[track_caller]
	pub const fn as_mut_out<'a>(&'a mut self) -> &'a mut OUT {
		let data: &'a mut IN = self.as_mut_in();

		unsafe {
			let new_data_ptr: &'a mut OUT = transmute_unchecked(data);

			new_data_ptr
		}
	}

	/// Ignoring the contract, the requirement to return the data back.
	#[inline]
	#[track_caller]
	pub const fn release_indata(self) -> IN {
		// To implement permanent movement, follow these steps:
		let sself: Self = self;
		let data: IN = unsafe { transmute_unchecked(sself) };

		// This is allowed because we have repr transparent.

		data
	}

	/// Execute the contract and return a value with the new data type.
	#[inline]
	#[track_caller]
	pub const fn into(self) -> OUT {
		let data: IN = self.release_indata();

		unsafe {
			let result: OUT = transmute_unchecked(data);
			result
		}
	}
}

impl<IN, OUT> Deref for TransmuteContract<IN, OUT> {
	type Target = IN;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.as_in()
	}
}

impl<IN, OUT> DerefMut for TransmuteContract<IN, OUT> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_in()
	}
}
