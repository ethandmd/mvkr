//! Intel VMX (Virtual Machine Extensions) support wrappers.
//! All references are to the Intel SDM Vol. 3C updated 09/23 unless otherwise specified.

use kernel::prelude::*;
use kernel::{
    bindings,
    error,
};
use crate::x86::*;

/// Reference SDM Vol. 3C 25.11.5
pub struct VmxonRegion {
    revision_id: u32,
    region: u64,
}

impl VmxonRegion {
    /// Create a new VmxonRegion by identifying the revision ID and allocating
    /// a page aligned (i.e. bottom 12 bits are clear) memory region and retrieving
    /// its physical address.
    pub fn new() -> Self {
        let revision_id = (Msr::new(IA32_VMX_BASIC).read() & 0xFFFF_FFFF) as u32;
        // Safety: Allocating a page aligned region, ffi call.
        let region = unsafe { bindings::__vmalloc(0x1000, bindings::GFP_KERNEL) } as u64;
        Self { revision_id, region }
    }
}

/// Enable and Enter VMX operation.
/// Reference: SDM Vol. 3C 23.7
pub fn enable_vmx(vmxon_region: Box<VmxonRegion>) -> Result {
    Cr4::write(Cr4Flags::VMXE as u64);
    let fctl = Msr::new(IA32_FEATURE_CONTROL).read();
    if fctl & 0x1 == 0 {
        // TODO: Revamp error reporting.
        return Err(error::code::EINVAL);
    }
    vmxon(vmxon_region.region);
    Ok(())
}
