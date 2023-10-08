//! Intel VMX (Virtual Machine Extensions) support wrappers.
//! All references are to the Intel SDM Vol. 3C unless otherwise specified.

use kernel::prelude::*;
use crate::x86::*;

enum VmxError {
    Ia32FeatureControlLocked,
}

/// Enable and Enter VMX operation.
/// Reference: SDM Vol. 3C 23.7
pub fn enable_vmx() -> Result {
    Cr4::write(Cr4Flags::VMXE);
    let fctl = Msr::new(IA32_FEATURE_CONTROL).read();
    if fctl & 0x1 == 0 {
        return Err(VmxError::Ia32FeatureControlLocked);
    }
    Ok(())
}
