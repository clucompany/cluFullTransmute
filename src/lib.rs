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

# Library Features

1. Casting any type A to any type B with generic data without and with data dimension checking.
2. Ability to use transmutation in constant functions in very old versions of rust..
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library.

# !!! ATTENTION !!!

1. When converting types without checking the size of the data, you really need to understand what you are doing.
2. You must understand the specifics of the platform you are using.


# Use

### 1. Generic

```rust
use core::fmt::Display;
use cluFullTransmute::transmute_or_panic;

/// Implementation of a simple transmutation with a generic parameter inside.

#[derive(Debug)]
#[repr(transparent)]
struct A<T> {
	#[allow(dead_code)]
	data: T
}

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Invalid beh");
	}
}

#[derive(Debug)]
#[repr(transparent)]
struct B<T> where T: Display {
	data: T,
}

impl<T> Drop for B<T> where T: Display {
	fn drop(&mut self) {
		println!("{}", self.data);
	}
}

fn main() {
	let a: A<u16> = A { // original and panic when falling
		data: 1024
	};
	println!("in: {:?}", a);
	
	let b: B<u16> = unsafe { transmute_or_panic(a) };
	println!("out: {:?}", b);
	
	drop(b); // <--- println!
}
```

### 2. Contract

```rust
use cluFullTransmute::contract::Contract;

/*
	For example, we will sign a contract to convert a String to a Vec<u8>, 
	although this may not be exactly the case.
	
	Contracts are needed to create more secure APIs using transmutation in 
	situations where it can't be proven.
*/

/// 
struct MyData {
	data: Contract<&'static str, &'static [u8]>,
}

impl MyData {
	#[inline]
	const fn new(data: &'static str) -> Self {
		let data = unsafe {
			// Contract::new_checksize_or_panic
			// 
			
			// The `new_checksize_or_panic` function can only guarantee equality of data 
			// dimensions, creating a contract is always unsafe, since the transmutation 
			// of such data types can only be proven orally. But after signing the 
			// transmutation contract, all functions for working with the transmuted are 
			// not marked as unsafe.
			//
			Contract::new_checksize_or_panic(data)
		};
		Self {
			data,
		}	
	}
	
	#[inline]
	pub fn as_data(&self) -> &'static str {
		&self.data
	}
	
	#[inline]
	pub fn as_sliceu8(&self) -> &'static [u8] {
		self.data.as_datato()
	}
	
	#[inline]
	pub fn into(self) -> &'static [u8] {
		self.data.into()
	}
}


fn main() {
	const C_DATA: &'static str = "Test";
	
	// &'static str
	let data = MyData::new(C_DATA);
	assert_eq!(data.as_data(), C_DATA); // const_readtype: &'static str
	assert_eq!(data.as_sliceu8(), C_DATA.as_bytes()); //const_readtype &'static [u8]
	//
	
	// &'static u8
	let vec = data.into(); // const_transmute: &'static str -> &'static [u8]
	assert_eq!(vec, C_DATA.as_bytes());
}
```

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
#[cfg( any(test, feature = "compatible_stdapi") )]
pub mod mem {
	/// Reinterprets the bits of a value of one type as another type. 
	/// The function is completely constant, in case of a size mismatch, a panic pops up.
	pub use crate::transmute_or_panic as transmute;
	
	pub use crate::raw::unchecked_transmute;
	#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
	#[cfg( any(test, feature = "inline") )]
	pub use crate::raw::inline_unchecked_transmute;
}

pub mod raw;
pub mod err;

#[cfg_attr(docsrs, doc(cfg(feature = "to")))]
#[cfg( any(test, feature = "to") )]
pub mod to;

#[cfg_attr(docsrs, doc(cfg(feature = "contract")))]
#[cfg( any(test, feature = "contract") )]
pub mod contract;

use crate::err::TransmuteErrKind;
use crate::err::TransmuteErr;
use core::mem::size_of;
#[cfg_attr(docsrs, doc(cfg(feature = "inline")))]
#[cfg( any(test, feature = "inline") )]
use crate::raw::inline_unchecked_transmute;
pub use crate::raw::unchecked_transmute;

/// A constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the sizes do not match, a panic arises.
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg( any(test, feature = "support_size_check_transmute") )]
pub const unsafe fn transmute_or_panic<D, To>(in_data: D) -> To {
	{ // #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();
		
		if size_d != size_to {
			let errkind = TransmuteErrKind::new_invalid_sizecheck(
				size_d, 
				size_to
			);
			
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
#[cfg( any(test, feature = "inline") )]
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg( any(test, feature = "support_size_check_transmute") )]
#[inline(always)]
pub const unsafe fn inline_transmute_or_panic<D, To>(in_data: D) -> To {
	{ // #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();
		
		if size_d != size_to {
			let errkind = TransmuteErrKind::new_invalid_sizecheck(
				size_d, 
				size_to
			);
			
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
#[cfg( any(test, feature = "support_size_check_transmute") )]
pub const unsafe fn transmute_or_errresult<D, To>(
	in_data: D
) -> Result<To, TransmuteErr<D>> {
	{ // #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();
		
		if size_d != size_to {
			let err = TransmuteErr::new_invalid_sizecheck(size_d, size_to,
				in_data,
			);
			
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
#[cfg( any(test, feature = "inline") )]
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg( any(test, feature = "support_size_check_transmute") )]
pub const unsafe fn inline_transmute_or_errresult<D, To>(
	in_data: D
) -> Result<To, TransmuteErr<D>> {
	{ // #1: Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();
		
		if size_d != size_to {
			let err = TransmuteErr::new_invalid_sizecheck(size_d, size_to,
				in_data,
			);
			
			return Err(err);
		}
	}
	
	Ok(inline_unchecked_transmute(in_data))
}

