// note to self, you can't use union here to store both result and error code because C++ side doesn't
// support non-POD types as union fields

use std::{ffi::c_void, marker::PhantomData, mem::MaybeUninit};

use crate::templ::receive_string;
use crate::types::Unit;
use crate::Error;

#[repr(C)]
pub struct Result<S, O = S> {
	pub error_code: i32,
	pub error_msg: *mut c_void,
	pub result: MaybeUninit<S>,
	_p: PhantomData<O>,
}

impl<S: Into<O>, O> Result<S, O> {
	#[inline]
	pub fn into_result(self) -> crate::Result<O> {
		if self.error_msg.is_null() {
			Ok(unsafe { self.result.assume_init() }.into())
		} else {
			let error_msg = if self.error_msg.is_null() {
				"Unable to receive error message".to_string()
			} else {
				unsafe { receive_string(self.error_msg.cast::<String>()) }
			};
			Err(Error::new(self.error_code, error_msg))
		}
	}
}

pub type ResultVoid = Result<Unit, ()>;
