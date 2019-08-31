//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 1819
/*!

A more complete and advanced version of data transmutation.

# Opportunities
1. Reduction of any A to any B, without checking the dimensionality of the data.
2. The ability to use transmute with const functions.
3. Possibility of delayed transmutation.
4. The library uses #!\[no_std\]

# A warning!

1. This library only works in a nightly compiler, we expect stabilization features.
2. You really need to understand what you are doing.


# Use

1. Easy

```rust
use cluFullTransmute::mem::full_transmute;

fn main() {
	let a: bool = unsafe{ full_transmute(1u8) };
	assert_eq!(a, true);
	
	let b: bool = unsafe{ full_transmute(0u8) };
	assert_eq!(b, false);
	
	// Why does this work?
	//
	// Is bool one bit?
	// No, bool is not one bit, but u8.
	//
	assert_eq!(std::mem::size_of::<bool>(), 1);
}
```

2. GenericType

```rust
use cluFullTransmute::mem::full_transmute;

#[allow(dead_code)]
struct A<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

#[allow(dead_code)]
struct B<T>(T);

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

fn main() {
	let data = A(9999usize); //ignore drop!
	
	let b: B<usize> = unsafe{ full_transmute(data) };
	assert_eq!(b.0, 9999);
	
	b.my_fn();
}
```

*/

#![feature(untagged_unions)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![allow(non_snake_case)]
#![no_std]

/// Methods for converting data in RAM.
pub mod mem {
	mod union;
	pub use self::union::*;
}



#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	extern crate alloc;
	
	#[test]
	fn full_transmute_correct() {
		use core::hash::{Hash, Hasher};
		
		#[allow(deprecated)]
		use core::hash::SipHasher;
		//Why SipHasher, not DefaultHasher?
		//
		//DefaultHasher is only in std, we only need core.
		
		#[derive(Hash, Debug)]
		struct A(usize, usize, bool);
		
		static mut CHECK_DROP1: bool = false;
		impl Drop for A {
			#[inline]
			fn drop(&mut self) {
				unsafe { 
					CHECK_DROP1 = true;
				}
				// CHECK_DROP1 must be false, 
				//
				// if the destructor is executed,
				// then everything is bad.
			}
		}
		
		#[derive(Hash, Debug)]
		struct B(usize, usize, bool);
		
		static mut CHECK_DROP2: bool = false;
		impl Drop for B {
			fn drop(&mut self) {
				unsafe { 
					CHECK_DROP2 = true;
				}
			}
		}
		
		// Why not use `#[repr (C)]`?
		//
		// I assume that the Rust compiler optimizes 
		// two structures with the same attachments 
		// the same way, and in case of an error I get a test failure.
		
		const ONE_DATA: usize = usize::max_value();
		const TWO_DATA: usize = usize::min_value();
		const THREE_DATA: bool = true;
		
		let a = A(ONE_DATA, TWO_DATA, THREE_DATA);
		let a_hash = {
			#[allow(deprecated)]
			let mut hasher = SipHasher::new();
			a.hash(&mut hasher);
			hasher.finish()
		};
		
		let b: B = unsafe { crate::mem::full_transmute(a) };
		let b_hash = {
			#[allow(deprecated)]
			let mut hasher = SipHasher::new();
			b.hash(&mut hasher);
			hasher.finish()
		};
		
		assert_eq!(b.0, ONE_DATA);
		assert_eq!(b.1, TWO_DATA);
		assert_eq!(b.2, THREE_DATA);
		
		assert_eq!(a_hash, b_hash);
		
		drop(b);
		assert_eq!(unsafe{ CHECK_DROP1 }, false);
		assert_eq!(unsafe{ CHECK_DROP2 }, true);
		// We check the work of the destructor, 
		//
		// if the destructor does not work, 
		// then everything is bad.
	}
	
	
	#[test]
	fn full_transmute_correct_struct() {
		use crate::mem::TransmuteData;
		use core::hash::{Hash, Hasher};
		use core::mem::ManuallyDrop;
		use alloc::string::String;
		
		#[allow(deprecated)]
		use core::hash::SipHasher;
		//Why SipHasher, not DefaultHasher?
		//
		//DefaultHasher is only in std, we only need core.
		
		#[repr(C)]
		#[derive(Hash)]
		struct ShadowData {
			u: usize,
			str: ManuallyDrop<String>, // !!!UNK DATA!!!
		}
		
		impl ShadowData {
			fn usize_hash(&self) -> u64 {
				#[allow(deprecated)]
				let mut hasher = SipHasher::new();
				Hash::hash(self, &mut hasher);
				hasher.finish()
			}
		}
		
		static mut CHECK_DROP: bool = false;
		impl Drop for ShadowData {
			fn drop(&mut self) {
				unsafe {
					ManuallyDrop::drop(&mut self.str);
					
					CHECK_DROP = true;
				}
			}
		}
		
		struct A {
			#[allow(dead_code)]
			data: TransmuteData<usize, ShadowData>,
		}
		
		impl A {
			fn data(self) -> TransmuteData<usize, ShadowData> {
				let mut new_self = ManuallyDrop::new(self);
				unsafe { 
					ManuallyDrop::drop(&mut new_self);
					crate::mem::full_transmute(new_self)
				}
				
				// Mini hak, execute the destructor of the current structure 
				// but at the same time pull the value out of it. 
				//
				// We are sure that your instructor does not use this value, so we can.
				// We need it for the test!
			}
			
			fn into(self) -> ShadowData {				
				let mut shadow = unsafe{ self.data().into() }; //ShadowData
				shadow.str = ManuallyDrop::new("test".into());
					
				shadow
			}
		}
		
		static mut CHECK_DROP2: bool = false;
		impl Drop for A {
			fn drop(&mut self) {
				unsafe {
					CHECK_DROP2 = true;
				}
			}
		}
		
		let data = A {
			data: 10.into(),
		};
		let shadow_data = data.into();
		
		
		assert_eq!(
			shadow_data.usize_hash(),
			ShadowData {
				u: 10,
				str: ManuallyDrop::new("test".into()),
			}.usize_hash()
		);
		drop(shadow_data);
		
		
		assert_eq!(unsafe{ CHECK_DROP },  true);
		assert_eq!(unsafe{ CHECK_DROP2 }, true);
		// We check the work of the destructor, 
		//
		// if the destructor does not work, 
		// then everything is bad.
	}
}