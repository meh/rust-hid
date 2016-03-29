use std::time::Duration;

use libc::c_int;
use sys::*;
use Result as Res;
use Error;

pub struct Handle {
	ptr: *mut hid_device,
}

impl Handle {
	#[doc(hidden)]
	pub unsafe fn new(ptr: *mut hid_device) -> Self {
		Handle {
			ptr: ptr,
		}
	}

	pub unsafe fn as_ptr(&self) -> *const hid_device {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut hid_device {
		self.ptr
	}
}

impl Handle {
	pub fn blocking(&mut self, value: bool) -> Res<()> {
		unsafe {
			match hid_set_nonblocking(self.as_mut_ptr(), if value { 1 } else { 0 }) {
				0 =>
					Ok(()),

				_ =>
					Err(Error::General)
			}
		}
	}

	pub fn data(&mut self) -> Data {
		unsafe {
			Data::new(self)
		}
	}

	pub fn feature(&mut self) -> Feature {
		unsafe {
			Feature::new(self)
		}
	}
}

pub struct Data<'a> {
	handle: &'a mut Handle,
}

impl<'a> Data<'a> {
	#[doc(hidden)]
	pub unsafe fn new(handle: &mut Handle) -> Data {
		Data { handle: handle }
	}

	pub fn write_direct<T: AsRef<[u8]>>(&mut self, data: T) -> Res<usize> {
		let data = data.as_ref();

		unsafe {
			match hid_write(self.handle.as_mut_ptr(), data.as_ptr(), data.len()) {
				-1 =>
					Err(Error::Write),

				length =>
					Ok(length as usize)
			}
		}
	}

	pub fn write_to<T: AsRef<[u8]>>(&mut self, id: u8, data: T) -> Res<usize> {
		let     data   = data.as_ref();
		let mut buffer = Vec::with_capacity(data.len() + 1);

		buffer.push(id);
		buffer.extend(data);

		self.write(&buffer).map(|v| v - 1)
	}

	pub fn write<T: AsRef<[u8]>>(&mut self, data: T) -> Res<usize> {
		self.write_to(0, data)
	}

	pub fn read_direct<T: AsMut<[u8]>>(&mut self, mut data: T, timeout: Duration) -> Res<Option<usize>> {
		let data   = data.as_mut();
		let result = if timeout.as_secs() == 0 && timeout.subsec_nanos() == 0 {
			unsafe {
				hid_read(self.handle.as_mut_ptr(), data.as_mut_ptr(), data.len())
			}
		}
		else {
			unsafe {
				hid_read_timeout(self.handle.as_mut_ptr(), data.as_mut_ptr(), data.len(),
					timeout.as_secs() as c_int * 1_000 + timeout.subsec_nanos() as c_int * 1_000)
			}
		};

		match result {
			-1 =>
				Err(Error::Read),

			0 =>
				Ok(None),

			v =>
				Ok(Some(v as usize))
		}
	}

	pub fn read<T: AsMut<[u8]>>(&mut self, data: T, timeout: Duration) -> Res<Option<usize>> {
		self.read_from(data, timeout).map(|r| r.map(|(_, l)| l))
	}

	pub fn read_from<T: AsMut<[u8]>>(&mut self, mut data: T, timeout: Duration) -> Res<Option<(u8, usize)>> {
		let mut data   = data.as_mut();
		let mut buffer = Vec::with_capacity(data.len() + 1);

		match try!(self.read(&mut buffer, timeout)) {
			None => {
				Ok(None)
			}

			Some(length) => {
				data.clone_from_slice(&buffer[1..length]);

				Ok(Some((data[0], length - 1)))
			}
		}
	}
}

pub struct Feature<'a> {
	handle: &'a mut Handle,
}

impl<'a> Feature<'a> {
	#[doc(hidden)]
	pub unsafe fn new(handle: &mut Handle) -> Feature {
		Feature { handle: handle }
	}

	pub fn send_direct<T: AsRef<[u8]>>(&mut self, data: T) -> Res<usize> {
		let data = data.as_ref();

		unsafe {
			match hid_send_feature_report(self.handle.as_mut_ptr(), data.as_ptr(), data.len()) {
				-1 =>
					Err(Error::Write),

				length =>
					Ok(length as usize)
			}
		}
	}

	pub fn send_to<T: AsRef<[u8]>>(&mut self, id: u8, data: T) -> Res<usize> {
		let     data   = data.as_ref();
		let mut buffer = Vec::with_capacity(data.len() + 1);

		buffer.push(id);
		buffer.extend(data);

		self.send(&buffer).map(|v| v - 1)
	}

	pub fn send<T: AsRef<[u8]>>(&mut self, data: T) -> Res<usize> {
		self.send_to(0, data)
	}

	pub fn get_direct<T: AsMut<[u8]>>(&mut self, mut data: T) -> Res<Option<usize>> {
		let data = data.as_mut();

		unsafe {
			match hid_get_feature_report(self.handle.as_mut_ptr(), data.as_mut_ptr(), data.len()) {
				-1 =>
					Err(Error::Read),

				0 =>
					Ok(None),

				v =>
					Ok(Some(v as usize))
			}
		}
	}

	pub fn get_from<T: AsMut<[u8]>>(&mut self, id: u8, mut data: T) -> Res<Option<usize>> {
		let     data   = data.as_mut();
		let mut buffer = vec![0u8; data.len() + 1];

		buffer[0] = id;
		self.get_direct(&mut buffer).map(|l| l.map(|v| v - 1))
	}

	pub fn get<T: AsMut<[u8]>>(&mut self, data: T) -> Res<Option<usize>> {
		self.get_from(0, data)
	}
}


impl Drop for Handle {
	fn drop(&mut self) {
		unsafe {
			hid_close(self.as_mut_ptr());
		}
	}
}
