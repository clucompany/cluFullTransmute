//Copyright 2021 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 2021
/*!

A more complete and advanced version of data transmutation without restrictions.

# Opportunities
1. Casting any type `A` to any type `B` without checking the dimensionality of the data.
2. The ability to use transmutation in constant functions.
3. Possibility of delayed transmutation.
4. Possibility of work in #!\[no_std\]

# Attention!

1. This library only works in a nightly compiler.
2. You really need to understand what you are doing.


# Use

### 1. GenericType

```rust
use cluFullTransmute::mem::full_transmute;

struct A<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

struct B<T>(T);

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

fn main() {
	let data = A(9999usize); //ignore drop!
	
	let b: B<usize> = unsafe { full_transmute(data) };
	assert_eq!(b.0, 9999);
	
	b.my_fn();
}
```

### 2. Easy

```rust
use cluFullTransmute::mem::full_transmute;

fn main() {
	let a: bool = unsafe { full_transmute(1u8) };
	assert_eq!(a, true);
	
	let b: bool = unsafe { full_transmute(0u8) };
	assert_eq!(b, false);
	
	// Why does this work?
	//
	// Is bool one bit?
	// No, bool is not one bit, but u8.
	//
	assert_eq!(std::mem::size_of::<bool>(), 1);
}
```

### 3. MaybeTransmute

```rust
use cluFullTransmute::mem::MaybeTransmute;

struct MyData {
	data: MaybeTransmute<String, Vec<u8>>,
}

impl MyData {
	#[inline]
	pub fn new<I: Into<String>>(t: I) -> Self {
		Self::__new(t.into())
	}
	
	#[inline]
	const fn __new(data: String) -> Self {
		let data = unsafe {
			MaybeTransmute::new(data)
		};
		
		Self {
			data
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
	let mut data = MyData::new("Test");
	assert_eq!(data.as_string().as_bytes(), b"Test");
	assert_eq!(data.as_string(), "Test");
	
	
	let vec = data.into();
	assert_eq!(vec, b"Test");
	
}
```

*/


//#![feature(untagged_unions)]
#![feature(const_fn_union)]

#![allow(non_snake_case)]

#![no_std]

/// Methods for converting data in RAM.
pub mod mem {
	mod full_transmute;
	pub use self::full_transmute::*;
	
	mod maybe_transmute;
	pub use self::maybe_transmute::*;
}

pub use self::mem::full_transmute;
