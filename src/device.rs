use std::marker::PhantomData;
use std::path::Path;
use std::ffi::CStr;

use sys::*;
use Result as Res;
use {Error, Handle};

/// The HID device.
pub struct Device<'a> {
	ptr: *const hid_device_info,

	_marker: PhantomData<&'a ()>,
}

impl<'a> Device<'a> {
	#[doc(hidden)]
	pub unsafe fn new<'b>(ptr: *const hid_device_info) -> Device<'b> {
		Device {
			ptr: ptr,

			_marker: PhantomData,
		}
	}

	/// The path representation.
	pub fn path(&self) -> &Path {
		unsafe {
			Path::new(CStr::from_ptr((*self.ptr).path).to_str().unwrap())
		}
	}

	/// The vendor ID.
	pub fn vendor_id(&self) -> u16 {
		unsafe {
			(*self.ptr).vendor_id
		}
	}

	/// The product ID.
	pub fn product_id(&self) -> u16 {
		unsafe {
			(*self.ptr).product_id
		}
	}

	/// The serial number.
	pub fn serial_number(&self) -> &str {
		unsafe {
			CStr::from_ptr((*self.ptr).serial_number as *const _).to_str().unwrap()
		}
	}

	/// The release number.
	pub fn release_number(&self) -> u16 {
		unsafe {
			(*self.ptr).release_number
		}
	}

	/// The usage page.
	pub fn usage_page(&self) -> u16 {
		unsafe {
			(*self.ptr).usage_page
		}
	}

	/// The usage number.
	pub fn usage(&self) -> u16 {
		unsafe {
			(*self.ptr).usage
		}
	}

	/// The interface number.
	pub fn interface_number(&self) -> isize {
		unsafe {
			(*self.ptr).interface_number as isize
		}
	}

	/// Opens the device to use it.
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
