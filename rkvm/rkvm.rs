//! Experimental port of KVM to Rust: rkvm.

use kernel::prelude::*;
use kernel::{
    error,
    sync::Arc,
    miscdev,
    ForeignOwnable,
    file::{File, IoctlCommand, IoctlHandler, Operations},
};

module! {
    type: Rkvm,
    name: "rkvm",
    author: "ethan",
    description: "Experimental port of KVM to Rust",
    license: "GPL",
}

const RKVM_API_VERSION: u32 = 99;

const RKVM_GET_API_VERSION: u32 = 0;
const RKVM_CREATE_VM: u32 = 1;

struct ApiHandler;

impl IoctlHandler for ApiHandler {
    type Target<'a> = Self;

    fn pure(_this: Self::Target<'_>, _file: &File, cmd: u32, _arg: usize) -> Result<i32> {
        match cmd {
            RKVM_GET_API_VERSION => Ok(RKVM_API_VERSION as i32),
            RKVM_CREATE_VM => Err(error::code::ENOSYS),
            _ => Err(error::code::EINVAL),
        }

    }
}

struct Rkvm {
    _dev: Pin<Box<miscdev::Registration<Self>>>,
}

struct FopsData;

#[vtable]
impl Operations for Rkvm {
    type OpenData = Arc<FopsData>;
    type Data = Arc<FopsData>;
    fn open(data: &Self::OpenData, _file: &File) -> Result<Self::Data> {
        Ok(data.clone())
    }
    
    fn ioctl(
        _data: <Self::Data as ForeignOwnable>::Borrowed<'_>,
        file: &File,
        cmd: &mut IoctlCommand,
    ) -> Result<i32> {
        let api = ApiHandler;
        cmd.dispatch::<ApiHandler>(api, file)
    }
}

impl kernel::Module for Rkvm {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rkvm init");
        let state = Arc::try_new(FopsData)?;
        Ok(Self {
            _dev: miscdev::Registration::new_pinned(fmt!("rkvm"), state)?,
        })
    }
}

impl Drop for Rkvm {
    fn drop(&mut self) {
        pr_info!("rkvm exit");
    }
}
