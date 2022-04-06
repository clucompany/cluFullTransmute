
use crate::mem::contract::DataTransmutContract;

/// Data Transformation Contract.
#[deprecated(since="1.0.6", note="please use `cluFullTransmute::mem::contract::DataTransmutContract` instead")]
pub type MaybeTransmute<T, To> = DataTransmutContract<T, To>;


// !!TODO!!
// It is possible to violate the terms of the API, because. by default, 
// the default function is marked safe.
//
// This trait needs to be recognized as deprecated.
#[allow(deprecated)]
impl<T, To> Default for MaybeTransmute<T, To> where T: Default {
	fn default() -> Self {
		let value: T = Default::default();
		
		unsafe {
			Self::force_new(value)
		}
	}
}
