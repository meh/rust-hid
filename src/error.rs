use std::ffi::CStr;
use libc::c_int;
use sys::*;
use std::{error, fmt};

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::String(ref s) => write!(f, "{}", s),
			_ => write!(f, "{}", error::Error::description(self))
		}
    }
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Initialized => "already initialized",
			Error::NotFound => "device not found",
			Error::General => "general error",
			Error::Write => "write error",
			Error::Read => "read error",
			Error::String(_) => "other error",
		}
	}

}
