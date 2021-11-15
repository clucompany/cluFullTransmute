
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


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
	
	const ONE_DATA: usize = usize::max_value();
	const TWO_DATA: usize = usize::min_value();
	const THREE_DATA: bool = true;
	
	let a = A(ONE_DATA, TWO_DATA, THREE_DATA);
	let a_hash = {
		let mut hasher = DefaultHasher::new();
		a.hash(&mut hasher);
		hasher.finish()
	};
	
	let b: B = unsafe {
		cluFullTransmute::mem::full_transmute(a)
	};
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
	assert_eq!(unsafe { CHECK_DROP1 }, false);
	assert_eq!(unsafe { CHECK_DROP2 }, true);
	// We check the work of the destructor, 
	//
	// if the destructor does not work, 
	// then everything is bad.
}

