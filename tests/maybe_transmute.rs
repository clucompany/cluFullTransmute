

use core::mem::MaybeUninit;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[test]
fn test_maybe_transmute_correct_struct() {
	use cluFullTransmute::mem::MaybeTransmute;
	use std::string::String;
	
	#[repr(C)]
	struct ShadowData {
		count: MaybeUninit<usize>, // Always init
		
		is_init_str: MaybeUninit<bool>,
		data: MaybeUninit<String>,
	}
	
	impl ShadowData {
		fn usize_hash(&self) -> u64 {
			let mut hasher = DefaultHasher::new();
			unsafe {
				Hash::hash(&*self.count.as_ptr() as &usize, &mut hasher);
			}
			
			let is_init_str: bool = unsafe {
				*self.is_init_str.as_ptr()
			};
			Hash::hash(&is_init_str as &bool, &mut hasher);
			
			if is_init_str {
				unsafe {
					Hash::hash(&*self.data.as_ptr() as &String, &mut hasher);
				}
			}
			
			hasher.finish()
		}
	}
	
	impl Drop for ShadowData {
		fn drop(&mut self) {
			let is_init_str: bool = unsafe {
				*self.is_init_str.as_ptr()
			};
			if is_init_str {
				unsafe {
					std::ptr::drop_in_place(self.data.as_mut_ptr());
				}
			}
			
			unsafe {
				std::ptr::drop_in_place(self.is_init_str.as_mut_ptr());
				std::ptr::drop_in_place(self.count.as_mut_ptr());
			}
			
		}
	}
	
	let maybe = unsafe {
		MaybeTransmute::<_, ShadowData>::new(10)
	};
	let shadow_data: ShadowData = {
		let mut a = maybe.into();
		a.is_init_str = MaybeUninit::new(false);
		unsafe {
			let data: usize = *a.count.as_ptr();
			assert_eq!(data, 10);
		}
		
		a.data = MaybeUninit::new("test".into());
		a.is_init_str = MaybeUninit::new(true);
		
		a
	};
	
	let check_data2 = ShadowData {
		count: MaybeUninit::new(10),
		
		is_init_str: MaybeUninit::new(true),
		data: MaybeUninit::new("test".into()),
	};
	
	assert_eq!(shadow_data.usize_hash(), check_data2.usize_hash());
}