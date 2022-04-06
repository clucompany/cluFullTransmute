//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022
/*!

A more complete and extended version of data type conversion without constraint checks.

# Library Features

1. Casting any type A to any type B without checking the dimension of the data.
2. Ability to use transmutation in constant functions.
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library
5. Extended support for rust versions, this library was originally designed to support permanent transmutation in circumstances where this cannot be done.

# !!! ATTENTION !!!

1. When converting types without checking the size of the data, you really need to understand what you are doing.


# Use

### 1. GenericType

```rust
/*
	This example is notable because by the Rust standard it does not allow 
	converting common types A to B, since it cannot check their sizes, 
	this example solves this.
	
	Additionally, as an example, we manipulate the Drop::drop function.
*/

use cluFullTransmute::mem::force_transmute;

struct A<T>(T);
struct B<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

impl<T> Drop for B<T> {
	fn drop(&mut self) {
		
	}
}

fn main() {
	let data = A(9999usize); // We expect panic at the death of A.
	
	let b: B<usize> = unsafe { force_transmute(data) }; // type A no longer exists, it is now type B.
	
	assert_eq!(b.0, 9999usize); // Checking the value
	b.my_fn();
	
	drop(b);
	// That's it, no panic, type B.
}
```

### 2. DataTransmutContract

```rust
/*
	For example, we will sign a contract to convert a String to a Vec<u8>, 
	although this may not be exactly the case.
	
	Contracts are needed to create more secure APIs using transmutation in 
	situations where it can't be proven.
*/

use cluFullTransmute::mem::contract::DataTransmutContract;

/// 
struct MyData {
	data: DataTransmutContract<String, Vec<u8>>,
}

impl MyData {
	#[inline]
	pub fn new<I: Into<String>>(t: I) -> Self {
		Self::__new(t.into())
	}
	
	#[inline]
	const fn __new(data: String) -> Self {
		let data = unsafe {
			// DataTransmutContract::force_new
			// 
			
			// The `checksize_new_or_panic` function can only guarantee equality of data 
			// dimensions, creating a contract is always unsafe, since the transmutation 
			// of such data types can only be proven orally. But after signing the 
			// transmutation contract, all functions for working with the transmuted are 
			// not marked as unsafe.
			//
			DataTransmutContract::checksize_new_or_panic(data)
		};
		Self {
			data,
		}	
	}
	
	#[inline]
	pub fn as_string(&mut self) -> &mut String {
		&mut self.data
	}
	
	#[inline]
	pub fn into(self) -> Vec<u8> {
		self.data.into()
	}
}


fn main() {
	// String
	let mut data = MyData::new("Test");
	assert_eq!(data.as_string().as_bytes(), b"Test");
	assert_eq!(data.as_string(), "Test");
	//
	
	let vec = data.into(); // String -> Vec<u8>
	assert_eq!(vec, b"Test");
}
```

*/

#![allow(non_snake_case)]

#![no_std]

/// Basic functions for dealing with memory. 
/// 
/// (To facilitate compatibility with the standard library, a similar file hierarchy was made.)
pub mod mem {
	mod full_transmute;
	pub use self::full_transmute::*;
	
	mod maybe_transmute;
	pub use self::maybe_transmute::*;
	
	/// Reinterprets the bits of a value of one type as another type. 
	pub mod transmute;
	
	/// Data Transformation Contract.
	pub mod contract;
	
	/// Reinterprets the bits of a value of one type as another type. 
	/// The function is completely const, data dimensions are not checked.
	pub use transmute::force_transmute as transmute;
	
	/// Reinterprets the bits of a value of one type as another type. 
	/// The function is completely const, data dimensions are not checked.
	pub use transmute::force_transmute;
	
	/// Reinterprets the bits of a value of one type as another type. 
	/// The function is completely const, data dimensions are not checked.
	pub use transmute::inline_force_transmute as inline_force_transmute;
}

/// TODO, Probably already outdated.
#[allow(deprecated)]
#[deprecated(since="1.0.6", note="please use `force_transmute` instead")]
#[doc(hidden)]
pub use self::mem::full_transmute;
