extern crate hidapi_sys as sys;
extern crate libc;

mod error;
pub use error::{Error, Result};

mod manager;
pub use manager::{init, Manager};

mod devices;
pub use devices::Devices;

mod device;
pub use device::Device;

pub mod handle;
pub use handle::Handle;
