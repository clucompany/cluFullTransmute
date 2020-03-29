

use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


#[test]
fn full_transmute_correct_struct() {
	use cluFullTransmute::mem::MaybeTransmute;
	
	use core::mem::ManuallyDrop;
	use std::string::String;
	
	#[repr(C)]
	#[derive(Hash)]
	struct UsizeStrData {
		u: usize,
		str: ManuallyDrop<String>, // !!!UNK DATA!!!
	}
	
	impl UsizeStrData {
		fn usize_hash(&self) -> u64 {
			let mut hasher = DefaultHasher::new();
			Hash::hash(self, &mut hasher);
			hasher.finish()
		}
	}
	
	static mut CHECK_DROP: bool = false;
	impl Drop for UsizeStrData {
		fn drop(&mut self) {
			unsafe {
				ManuallyDrop::drop(&mut self.str);
				
				CHECK_DROP = true;
			}
		}
	}
	
		
	struct A {
		#[allow(dead_code)]
		data: MaybeTransmute<usize, UsizeStrData>,
	}
	
	impl A {
		fn data(self) -> MaybeTransmute<usize, UsizeStrData> {
			let mut new_self = ManuallyDrop::new(self);
			unsafe { 
				ManuallyDrop::drop(&mut new_self);
				cluFullTransmute::mem::full_transmute(new_self)
			}
			
			// Mini hak, execute the destructor of the current structure 
			// but at the same time pull the value out of it. 
			//
			// We are sure that your instructor does not use this value, so we can.
			// We need it for the test!
		}
		
		fn into(self) -> UsizeStrData {
			let mut shadow = unsafe { self.data().into() }; //ShadowData
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
		UsizeStrData {
			u: 10,
			str: ManuallyDrop::new("test".into()),
		}.usize_hash()
	);
	drop(shadow_data);
	
	
	assert_eq!(unsafe { CHECK_DROP },  true);
	assert_eq!(unsafe { CHECK_DROP2 }, true);
	// We check the work of the destructor, 
	//
	// if the destructor does not work, 
	// then everything is bad.
}