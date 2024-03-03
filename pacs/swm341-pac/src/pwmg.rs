#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - START register"]
    pub start: START,
    #[doc = "0x04 - SWBRK register"]
    pub swbrk: SWBRK,
    #[doc = "0x08 - RESET register"]
    pub reset: RESET,
    _reserved_3_restart: [u8; 0x04],
    #[doc = "0x10 - PULSE register"]
    pub pulse: PULSE,
    #[doc = "0x14 - FILTER register"]
    pub filter: FILTER,
    #[doc = "0x18 - BRKPOL register"]
    pub brkpol: BRKPOL,
    #[doc = "0x1c - BRKIE register"]
    pub brkie: BRKIE,
    _reserved_8_brkif: [u8; 0x04],
    #[doc = "0x24 - EVSR register"]
    pub evsr: EVSR,
}
impl RegisterBlock {
    #[doc = "0x0c - RESTART register"]
    #[inline(always)]
    pub fn restart(&self) -> &RESTART {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const RESTART) }
    }
    #[doc = "0x0c - RELOADEN register"]
    #[inline(always)]
    pub fn reloaden(&self) -> &RELOADEN {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const RELOADEN) }
    }
    #[doc = "0x20 - BRKSR register"]
    #[inline(always)]
    pub fn brksr(&self) -> &BRKSR {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const BRKSR) }
    }
    #[doc = "0x20 - BRKIF register"]
    #[inline(always)]
    pub fn brkif(&self) -> &BRKIF {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const BRKIF) }
    }
}
#[doc = "START (rw) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "START register"]
pub mod start;
#[doc = "SWBRK (rw) register accessor: an alias for `Reg<SWBRK_SPEC>`"]
pub type SWBRK = crate::Reg<swbrk::SWBRK_SPEC>;
#[doc = "SWBRK register"]
pub mod swbrk;
#[doc = "RESET (rw) register accessor: an alias for `Reg<RESET_SPEC>`"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "RESET register"]
pub mod reset;
#[doc = "RELOADEN (rw) register accessor: an alias for `Reg<RELOADEN_SPEC>`"]
pub type RELOADEN = crate::Reg<reloaden::RELOADEN_SPEC>;
#[doc = "RELOADEN register"]
pub mod reloaden;
#[doc = "RESTART (rw) register accessor: an alias for `Reg<RESTART_SPEC>`"]
pub type RESTART = crate::Reg<restart::RESTART_SPEC>;
#[doc = "RESTART register"]
pub mod restart;
#[doc = "PULSE (rw) register accessor: an alias for `Reg<PULSE_SPEC>`"]
pub type PULSE = crate::Reg<pulse::PULSE_SPEC>;
#[doc = "PULSE register"]
pub mod pulse;
#[doc = "FILTER (rw) register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "FILTER register"]
pub mod filter;
#[doc = "BRKPOL (rw) register accessor: an alias for `Reg<BRKPOL_SPEC>`"]
pub type BRKPOL = crate::Reg<brkpol::BRKPOL_SPEC>;
#[doc = "BRKPOL register"]
pub mod brkpol;
#[doc = "BRKIE (rw) register accessor: an alias for `Reg<BRKIE_SPEC>`"]
pub type BRKIE = crate::Reg<brkie::BRKIE_SPEC>;
#[doc = "BRKIE register"]
pub mod brkie;
#[doc = "BRKIF (rw) register accessor: an alias for `Reg<BRKIF_SPEC>`"]
pub type BRKIF = crate::Reg<brkif::BRKIF_SPEC>;
#[doc = "BRKIF register"]
pub mod brkif;
#[doc = "BRKSR (rw) register accessor: an alias for `Reg<BRKSR_SPEC>`"]
pub type BRKSR = crate::Reg<brksr::BRKSR_SPEC>;
#[doc = "BRKSR register"]
pub mod brksr;
#[doc = "EVSR (rw) register accessor: an alias for `Reg<EVSR_SPEC>`"]
pub type EVSR = crate::Reg<evsr::EVSR_SPEC>;
#[doc = "EVSR register"]
pub mod evsr;
