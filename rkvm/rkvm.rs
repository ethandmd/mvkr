//! Experimental port of KVM to Rust: rkvm.

use kernel::prelude::*;
use kernel::{
    error,
    sync::Arc,
    miscdev,
    ForeignOwnable,
    file::{self, File, flags},
};
use core::ffi::c_void;

module! {
    type: Rkvm,
    name: "rkvm",
    author: "ethan",
    description: "Experimental port of KVM to Rust",
    license: "GPL",
}

/// Module constants.
const RKVM_API_VERSION: u32 = 99;

/// Ioctl API constants.
const RKVM_GET_API_VERSION: u32 = 0;
const RKVM_CREATE_VM: u32 = 1;

// struct Vm(UnsafeCell<bindings::kvm>);
// impl Vm {
//    fn new() -> Self {}
// }
// impl file::Operations for Vm {}

struct Vm {
    secret: i32,
}

impl Vm {
    fn create() -> Result<i32> {
        let fd = file::FileDescriptorReservation::new(flags::O_CLOEXEC)?;
        let fd_clone = fd.reserved_fd();
        let this = Arc::try_new(Vm { secret: 42 })?;
        file::AnonInode::<Self>::register(fd, fmt!("rkvm-vm"), this.into_foreign() as *mut c_void, flags::O_RDWR)?;
        Ok(fd_clone as i32)
    }
}

#[vtable]
impl file::Operations for Vm {
    type OpenData = Arc<Self>;
    type Data = Arc<Self>;
    fn open(context: &Self::OpenData, _file: &File) -> Result<Self::Data> {
        Ok(context.clone())
    }

    fn ioctl(
        data: <Self::Data as ForeignOwnable>::Borrowed<'_>,
        _file: &File,
        _cmd: &mut file::IoctlCommand,
    ) -> Result<i32> {
        Ok(data.secret)
    }
}

/// Top-level module ioctl-based API handler.
struct RkvmApiHandler;

impl file::IoctlHandler for RkvmApiHandler {
    type Target<'a> = Self;
    
    fn pure(_this: Self::Target<'_>, _file: &File, cmd: u32, _arg: usize) -> Result<i32> {
        match cmd {
            RKVM_GET_API_VERSION => Ok(RKVM_API_VERSION as i32),
            RKVM_CREATE_VM => Vm::create(),
            _ => Err(error::code::EINVAL),
        }

    }
}

/// Rkvm module struct which owns the miscdev registration.
struct Rkvm {
    _dev: Pin<Box<miscdev::Registration<Self>>>,
}

#[vtable]
impl file::Operations for Rkvm {
    type OpenData = ();
    type Data = ();
    fn open(_context: &Self::OpenData, _file: &File) -> Result<Self::Data> {
        Ok(())
    }
    
    fn ioctl(
        _data: <Self::Data as ForeignOwnable>::Borrowed<'_>,
        file: &File,
        cmd: &mut file::IoctlCommand,
    ) -> Result<i32> {
        let api = RkvmApiHandler;
        cmd.dispatch::<RkvmApiHandler>(api, file)
    }
}

impl kernel::Module for Rkvm {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rkvm init");
        Ok(Self {
            _dev: miscdev::Registration::new_pinned(fmt!("rkvm"), ())?,
        })
    }
}

impl Drop for Rkvm {
    fn drop(&mut self) {
        pr_info!("rkvm exit");
    }
}
