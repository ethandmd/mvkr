//! Intel VMX (Virtual Machine Extensions) support wrappers.
//! All references are to the Intel SDM Vol. 3C updated 09/23 unless otherwise specified.

use kernel::prelude::*;
use kernel::{
    error,
    bindings,
};
use crate::x86::*;

/// Reference SDM Vol. 3C 25.11.5
// TODO: Handle ORDER generic.
pub struct VmxonRegion {
    // TODO: Better way to handle send + sync for struct page *.
    va: u64,
    pa: u64,
}

impl VmxonRegion {
    /// Create a new VmxonRegion by allocating a 4k page aligned region and writing
    /// the vmcs revision ID to the first 4 bytes.
    pub fn new() -> Result<Self> {
        let vmx_basic = Msr::new(IA32_VMX_BASIC).read();
        // Check bits 30:0 for revision ID and set bit 31 as zero.
        let revision_id = vmx_basic & 0x7FFF_FFFF;
        // Check bits 44:32 of IA32_VMX_BASIC for the nr bytes to alloc for vmxon region.
        let vmxon_region_size = (vmx_basic >> 32) & 0x1FFF;
        if vmxon_region_size != 0x1000 {
            return Err(error::code::EINVAL);
        }
        // SAFETY: FFI Call on valid flags. Returns struct page *.
        let page = unsafe { bindings::alloc_pages(bindings::GFP_KERNEL | bindings::__GFP_ZERO | bindings::___GFP_DMA, 0) };
        if page.is_null() {
            return Err(error::code::ENOMEM);
        }
        // TODO: Want low mem pages.
        // SAFETY: FFI Call on valid pointer.
        let region_pa = unsafe { bindings::slow_virt_to_phys(page as *mut _) };
        // Check that the region is page aligned.
        if region_pa != 0x1000 {
            return Err(error::code::EINVAL);
        }
        // Revision ID value is valid for write. Set bit 31 to 0.
        let revision_id = revision_id & 0x7FFF_FFFF;
        
        // SAFETY: Pointer is valid, src is valid.
        // Write 31-bit rev-id to bits 30:0 of the first 4 bytes of region. Bit 31 cleared.
        unsafe { (page as *mut u32).write(revision_id as u32); }

        Ok(Self { va: (page as *mut u8) as u64, pa: region_pa })
    }

    /// Get physical address of vmxon region.
    pub fn get_phys_addr(&self) -> u64 {
        self.pa
    }

    /// Read vmxon region as a struct page *.
    pub fn page(&self) -> *const bindings::page {
        self.va as *const bindings::page
    }
}

/// Free allocated vmxon region.
impl Drop for VmxonRegion {
    fn drop(&mut self) {
        // SAFETY: FFI Call on valid pointer.
        unsafe { bindings::free_pages(self.va, 0) };
    }
}

/// Enable and Enter VMX operation.
/// Reference: SDM Vol. 3C 23.7
pub fn enable_vmx(vmxon_region_pa: u64) -> Result {
    Cr4::write(Cr4Flags::VMXE as u64);
    let fctl = Msr::new(IA32_FEATURE_CONTROL).read();
    if fctl & 0x1 == 0 {
        // TODO: Revamp error reporting.
        return Err(error::code::EINVAL);
    }
    vmxon(vmxon_region_pa)
}
