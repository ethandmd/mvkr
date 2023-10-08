//! Assembly wrappers for x86_64 vmx ops.
//! All references are to the Intel SDM unless otherwise specified.
/// Most code copied from osdev x86_64 crate because strapping in a
/// 3P crate didn't seem feasible and bindgen was giving me flak.

use kernel::error::Result;
use core::arch::asm;

/// Consts for x86_64.
pub const IA32_FEATURE_CONTROL: u32 = 0x3A; // Vol 3C 24.1
pub const IA32_VMX_BASIC: u32 = 0x480; // Vol 3C A.1

/// Control Register 4.
pub struct Cr4;

impl Cr4 {
    pub fn read() -> u64 { 
        let value: u64;
        // SAFETY: Read from CR4.
        unsafe {
            asm!("mov {}, cr4", out(reg) value)
        }
        value
    }

    pub fn write(flags: u64) {
        // Sanity check that flags are in range:
        let flags = flags & 0x3FF_FFFF;
        let old = Self::read();
        let res = old & !0x3FF_FFFF;
        let new = res | flags;

        // SAFETY: You could overwrite protected bits.
        unsafe { Self::overwrite(new) }
    }

    pub fn overwrite(flags: u64) {
        // SAFETY: You could overwrite protected bits.
        unsafe {
            asm!("mov cr4, {}", in(reg) flags as u64)
        }
    }
}

/// CR4 Bitflags
// Unnecessary? #[repr(u64)]
pub enum Cr4Flags {
    VME = 1 << 0,
    PVI = 1 << 1,
    TSD = 1 << 2,
    DE = 1 << 3,
    PSE = 1 << 4,
    PAE = 1 << 5,
    MCE = 1 << 6,
    PGE = 1 << 7,
    PCE = 1 << 8,
    OSFXSR = 1 << 9,
    OSXMMEXCPT = 1 << 10,
    VMXE = 1 << 13, // Virtual Machine Extensions enable
    SMXE = 1 << 14, // Safer Mode Extensions Enable
    FSGSBASE = 1 << 16,
    PCIDE = 1 << 17,
    OSXSAVE = 1 << 18,
    SMEP = 1 << 20,
    SMAP = 1 << 21,
    PKE = 1 << 22,
    CET = 1 << 23,
    PKS = 1 << 24,
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
    pub fn read(&self) -> u64 {
        let (high, low): (u32, u32);
        // SAFETY: Read from MSR. Caller ensures no side effects.
        unsafe {
            asm!("rdmsr", in("ecx") self.0, out("eax") low, out("edx") high)
        }
        ((high as u64) << 32) | (low as u64)
    }

    /// Write the MSR.
    #[inline]
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

/// Enter VMX operation.
pub fn vmxon(pa: u64) -> Result {
    Ok(())
}
