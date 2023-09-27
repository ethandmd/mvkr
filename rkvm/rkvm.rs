//! Experimental port of KVM to Rust: rkvm.

use kernel::prelude::*;
use kernel::{
    error,
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

#[vtable]
impl Operations for Rkvm {
    //type OpenData = ();
    //type Data = ();
    fn open(data: &Self::OpenData, _file: &File) -> Result<Self::Data> {
        pr_info!("rkvm open");
        Ok(*data)
    }
    
    fn ioctl(
        _data: <Self::Data as ForeignOwnable>::Borrowed<'_>,
        _file: &File,
        _cmd: &mut IoctlCommand,
    ) -> Result<i32> {
        pr_info!("rkvm ioctl");
        Ok(0)
    }
}

impl kernel::Module for Rkvm {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rkvm init");
        //let dev = miscdev::Registration::new_pinned(fmt!("rkvm"), ());
        if let Ok(dev) = miscdev::Registration::new_pinned(fmt!("rkvm"), ()) {
            Ok(Rkvm { _dev: dev })
        } else {
            pr_err!("rkvm init failed");
            Err(error::code::ENODEV)
        }
    }
}

impl Drop for Rkvm {
    fn drop(&mut self) {
        pr_info!("rkvm exit");
    }
}
