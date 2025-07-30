use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use cluFullTransmute::transmute_unchecked;

#[test]
fn full_transmute_correct() {
	//A -> B
	//A: Drop, B: !Drop

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

	const ONE_DATA: usize = usize::MAX;
	const TWO_DATA: usize = usize::MIN;
	const THREE_DATA: bool = true;

	let a = A(ONE_DATA, TWO_DATA, THREE_DATA);
	let a_hash = {
		let mut hasher = DefaultHasher::new();
		a.hash(&mut hasher);
		hasher.finish()
	};

	let b: B = unsafe { transmute_unchecked(a) };
	let b_hash = {
		let mut hasher = DefaultHasher::new();
		b.hash(&mut hasher);
		hasher.finish()
	};

	assert_eq!(b.0, ONE_DATA);
	assert_eq!(b.1, TWO_DATA);
	assert_eq!(b.2, THREE_DATA);

	assert_eq!(a_hash, b_hash);

	drop(b);
	assert!(!unsafe { CHECK_DROP1 });
	assert!(unsafe { CHECK_DROP2 });
	// We check the work of the destructor,
	//
	// if the destructor does not work,
	// then everything is bad.
}
