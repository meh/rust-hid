use std::marker::PhantomData;
use std::path::Path;
use std::ffi::CStr;

use sys::*;
use Result as Res;
use {Error, Handle};

pub struct Device<'a> {
	ptr: *const hid_device_info,

	_marker: PhantomData<&'a ()>,
}

impl<'a> Device<'a> {
	pub unsafe fn new<'b>(ptr: *const hid_device_info) -> Device<'b> {
		Device {
			ptr: ptr,

			_marker: PhantomData,
		}
	}

	pub fn path(&self) -> &Path {
		unsafe {
			Path::new(CStr::from_ptr((*self.ptr).path).to_str().unwrap())
		}
	}

	pub fn vendor_id(&self) -> u16 {
		unsafe {
			(*self.ptr).vendor_id
		}
	}

	pub fn product_id(&self) -> u16 {
		unsafe {
			(*self.ptr).product_id
		}
	}

	pub fn serial_number(&self) -> &str {
		unsafe {
			CStr::from_ptr((*self.ptr).serial_number as *const _).to_str().unwrap()
		}
	}

	pub fn release_number(&self) -> u16 {
		unsafe {
			(*self.ptr).release_number
		}
	}

	pub fn usage_page(&self) -> u16 {
		unsafe {
			(*self.ptr).usage_page
		}
	}

	pub fn usage(&self) -> u16 {
		unsafe {
			(*self.ptr).usage
		}
	}

	pub fn interface_number(&self) -> isize {
		unsafe {
			(*self.ptr).interface_number as isize
		}
	}

	pub fn open(&self) -> Res<Handle> {
		unsafe {
			let handle = hid_open((*self.ptr).vendor_id, (*self.ptr).product_id, (*self.ptr).serial_number);

			if handle.is_null() {
				return Err(Error::NotFound);
			}

			Ok(Handle::new(handle))
		}
	}
}
