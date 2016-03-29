use std::ffi::CStr;
use libc::c_int;
use sys::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
	Initialized,
	NotFound,
	General,
	Write,
	Read,
	String(String),
}

impl From<c_int> for Error {
	fn from(value: c_int) -> Error {
		match value {
			_ => Error::General
		}
	}
}

impl From<*mut hid_device> for Error {
	fn from(value: *mut hid_device) -> Error {
		unsafe {
			Error::String(CStr::from_ptr(hid_error(value) as *const _).to_str().unwrap().to_owned())
		}
	}
}
