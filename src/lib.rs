extern crate hidapi_sys as sys;
extern crate libc;

mod error;
pub use error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

mod manager;
pub use manager::{Manager, init};

mod devices;
pub use devices::Devices;

mod device;
pub use device::Device;

mod handle;
pub use handle::Handle;
