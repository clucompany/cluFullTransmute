//Copyright 2022-2024 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022-2024
/*!

A more complete and extended version of data type conversion without constraint checks.

## !!! ATTENTION !!!

1. When converting types without checking the size of the data, you really need to understand what you are doing.
2. You must understand the specifics of the platform you are using.

## Example:

```rust
use cluFullTransmute::transmute_or_panic;
use core::fmt::Display;

/*
	For example, let's write some code with a Drop trait that panics when dropped and
	holds some data. We then transmute this data to another similar struct and check
	that we have effectively overridden the Drop trait and have a different struct
	with some data.

	We can also remove the Drop trait altogether or do any number of other things.
*/

/// Struct to panic when dropped
#[derive(Debug)]
#[repr(transparent)]
struct PanicWhenDrop<T>(T);

impl<T> Drop for PanicWhenDrop<T> {
	fn drop(&mut self) {
		panic!("panic, discovered `drop(PanicWhenDrop);`");
	}
}

/// Struct to print value when dropped
#[derive(Debug)]
#[repr(transparent)]
struct PrintlnWhenDrop<T: Display>(T)
where
	T: Display;

impl<T> Drop for PrintlnWhenDrop<T>
where
	T: Display,
{
	fn drop(&mut self) {
		println!("println: {}", self.0);
	}
}

fn main() {
	let a: PanicWhenDrop<u16> = PanicWhenDrop(1024);
	println!("in a: {:?}", a);

	let b: PrintlnWhenDrop<u16> = unsafe { transmute_or_panic(a as PanicWhenDrop<u16>) };
	println!("out b: {:?}", b);

	drop(b); // <--- drop, PrintlnWhenDrop!
}
```

## Library Features

1. Casting any type A to any type B with generic data without and with data dimension checking.
2. Ability to use transmutation in constant functions in very old versions of rust..
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library.

*/

#![allow(non_snake_case)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::match_like_matches_macro)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "support_stderr"), no_std)]

/// Basic functions for dealing with memory.
///
/// (An optional module for ensuring compatibility with the standard library, which is turned on and off with the `compatible_stdapi` build flag.)
#[cfg_attr(docsrs, doc(cfg(feature = "compatible_stdapi")))]
#[cfg(any(test, feature = "compatible_stdapi"))]
pub mod mem {
	/// Reinterprets the bits of a value of one type as another type.
	/// The function is completely constant, in case of a size mismatch, a panic pops up.
	pub use crate::transmute_or_panic as transmute;

	#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
	#[cfg(any(test, feature = "inline"))]
	pub use crate::raw::inline_unchecked_transmute;
	pub use crate::raw::unchecked_transmute;
}

#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub mod err;
pub mod raw;

#[cfg_attr(docsrs, doc(cfg(feature = "to")))]
#[cfg(any(test, feature = "to"))]
pub mod to;

#[cfg_attr(docsrs, doc(cfg(feature = "contract")))]
#[cfg(any(test, feature = "contract"))]
pub mod contract;

#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
use crate::err::TransmuteErr;
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
use crate::err::TransmuteErrKind;
#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
#[cfg(any(test, feature = "inline"))]
use crate::raw::inline_unchecked_transmute;
pub use crate::raw::unchecked_transmute;
use core::mem::size_of;

/// A constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the sizes do not match, a panic arises.
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub const unsafe fn transmute_or_panic<D, To>(in_data: D) -> To {
	{
		// #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let errkind = TransmuteErrKind::new_invalid_sizecheck(size_d, size_to);

			errkind.unwrap();
		}
	}

	unchecked_transmute(in_data)
}

/// A inline constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the sizes do not match, a panic arises.
#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
#[cfg(any(test, feature = "inline"))]
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
#[inline(always)]
pub const unsafe fn inline_transmute_or_panic<D, To>(in_data: D) -> To {
	{
		// #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let errkind = TransmuteErrKind::new_invalid_sizecheck(size_d, size_to);

			errkind.unwrap();
		}
	}

	inline_unchecked_transmute(in_data)
}

/// A constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the size does not match, an error occurs.
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub const unsafe fn transmute_or_errresult<D, To>(in_data: D) -> Result<To, TransmuteErr<D>> {
	{
		// #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let err = TransmuteErr::new_invalid_sizecheck(size_d, size_to, in_data);

			return Err(err);
		}
	}

	Ok(unchecked_transmute(in_data))
}

/// A inline constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the size does not match, an error occurs.
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
#[cfg(any(test, feature = "inline"))]
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub const unsafe fn inline_transmute_or_errresult<D, To>(
	in_data: D,
) -> Result<To, TransmuteErr<D>> {
	{
		// #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let err = TransmuteErr::new_invalid_sizecheck(size_d, size_to, in_data);

			return Err(err);
		}
	}

	Ok(inline_unchecked_transmute(in_data))
}
