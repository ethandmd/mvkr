//! Assembly wrappers for x86_64 vmx ops.
//! All references are to the Intel SDM unless otherwise specified.
/// Most code copied from osdev x86_64 crate because strapping in a
/// 3P crate didn't seem feasible and bindgen was giving me flak.

use kernel::error::{code, Result};
use core::arch::asm;

/// MSR IA32_FEATURE_CONTROL
pub const IA32_FEATURE_CONTROL: u32 = 0x3A; // Vol 3C 24.1
/// MSR IA32_VMX_BASIC
pub const IA32_VMX_BASIC: u32 = 0x480; // Vol 3C A.1

/// Control Register 4.
pub struct Cr4;

impl Cr4 {
    /// Read CR4.
    pub fn read() -> u64 { 
        let value: u64;
        // SAFETY: Read from CR4.
        unsafe {
            asm!("mov {}, cr4", out(reg) value)
        }
        value
    }

    /// Write CR4.
    pub fn write(flags: u64) {
        // Sanity check that flags are in range:
        let flags = flags & 0x3FF_FFFF;
        let old = Self::read();
        let res = old & !0x3FF_FFFF;
        let new = res | flags;

        // SAFETY: You could overwrite protected bits.
        Self::overwrite(new)
    }

    /// Overwrite CR4.
    pub fn overwrite(flags: u64) {
        // SAFETY: You could overwrite protected bits.
        unsafe {
            asm!("mov cr4, {}", in(reg) flags as u64)
        }
    }
}

/// CR4 Bitflags
pub enum Cr4Flags {
    /// TODO
    VME = 1 << 0,
    /// TODO
    PVI = 1 << 1,
    /// TODO
    TSD = 1 << 2,
    /// TODO
    DE = 1 << 3,
    /// TODO
    PSE = 1 << 4,
    /// TODO
    PAE = 1 << 5,
    /// TODO
    MCE = 1 << 6,
    /// TODO
    PGE = 1 << 7,
    /// TODO
    PCE = 1 << 8,
    /// TODO
    OSFXSR = 1 << 9,
    /// TODO
    OSXMMEXCPT = 1 << 10,
    /// VMX enable
    VMXE = 1 << 13,
    /// Safer mode extensions enable
    SMXE = 1 << 14,
    /// TODO
    FSGSBASE = 1 << 16,
    /// TODO
    PCIDE = 1 << 17,
    /// TODO
    OSXSAVE = 1 << 18,
    /// TODO
    SMEP = 1 << 20,
    /// TODO
    SMAP = 1 << 21,
    /// TODO
    PKE = 1 << 22,
    /// TODO
    CET = 1 << 23,
    /// TODO
    PKS = 1 << 24,
    /// TODO
    UINTR = 1 << 25,
}

/// Model Specific Register
pub struct Msr(u32);

impl Msr {
    /// Create a new MSR with the given register number.
    pub fn new(reg: u32) -> Self {
        Self(reg)
    }

    /// Read the MSR.
    #[inline(always)]
    pub fn read(&self) -> u64 {
        let (high, low): (u32, u32);
        // SAFETY: Read from MSR. Caller ensures no side effects.
        unsafe {
            asm!("rdmsr", in("ecx") self.0, out("eax") low, out("edx") high)
        }
        ((high as u64) << 32) | (low as u64)
    }

    /// Write the MSR.
    #[inline(always)]
    pub unsafe fn write(&mut self, value: u64) {
        let low = value as u32;
        let high = (value >> 32) as u32;

        unsafe {
            asm!(
                "wrmsr",
                in("ecx") self.0,
                in("eax") low, in("edx") high,
                options(nostack, preserves_flags),
            );
        }
    }
}

/// Read VMX success/failure.
/// Reference Vol 3C 31.2: Conventions.
#[inline(always)]
pub fn vmx_result() -> Result {
    let rflags: u64;

    // SAFETY: Read low 8 bits from RFLAGS into ah.
    // Can't use pushfq; popq to get entire 64b reg.
    unsafe {
        //asm!("lahf {}", out(reg) rflags)
        asm!("pushfq; popq {}", out(reg) rflags, options(att_syntax))
    }
    // Check CF (1 << 0), PF, AF, ZF (1 << 6), SF, and OF.
    // VMsuccess := All bits cleared.
    // VMfailInvalid: CF = 1.
    // VMfailValid(ErrorNumber): ZF = 1.
    if rflags & 0x1F == 0 {
        Ok(())
    } else if rflags & 0x1 == 1 {
        Err(code::EINVAL)
    } else {
        Err(code::EINVAL)
    }
}

/// Enter VMX operation.
pub fn vmxon(pa: u64) -> Result {
    // SAFETY: Caller ensures no side effects. Requires CPL 0.
    unsafe { asm!("vmxon [{}]", in(reg) pa); }
    vmx_result()
}
