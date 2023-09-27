//! Experimental port of KVM to Rust: rkvm.

use kernel::prelude::*;
use kernel::{
    sync::Arc,
    miscdev,
    ForeignOwnable,
    file::{File, IoctlCommand, Operations},
};

module! {
    type: Rkvm,
    name: "rkvm",
    author: "ethan",
    description: "Experimental port of KVM to Rust",
    license: "GPL",
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
        _file: &File,
        cmd: &mut IoctlCommand,
    ) -> Result<i32> {
        pr_info!("rkvm ioctl: {}", cmd.raw().0);
        Ok(0)
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
