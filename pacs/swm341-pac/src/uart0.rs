#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DATA register"]
    pub data: DATA,
    #[doc = "0x04 - CTRL register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - BAUD register"]
    pub baud: BAUD,
    #[doc = "0x0c - FIFO register"]
    pub fifo: FIFO,
    #[doc = "0x10 - LINCR register"]
    pub lincr: LINCR,
    _reserved_5_ctscr: [u8; 0x04],
    #[doc = "0x18 - CFG register"]
    pub cfg: CFG,
    #[doc = "0x1c - TOCR register"]
    pub tocr: TOCR,
}
impl RegisterBlock {
    #[doc = "0x14 - RTSCR register"]
    #[inline(always)]
    pub fn rtscr(&self) -> &RTSCR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const RTSCR) }
    }
    #[doc = "0x14 - CTSCR register"]
    #[inline(always)]
    pub fn ctscr(&self) -> &CTSCR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CTSCR) }
    }
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CTRL register"]
pub mod ctrl;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "BAUD register"]
pub mod baud;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO register"]
pub mod fifo;
#[doc = "LINCR (rw) register accessor: an alias for `Reg<LINCR_SPEC>`"]
pub type LINCR = crate::Reg<lincr::LINCR_SPEC>;
#[doc = "LINCR register"]
pub mod lincr;
#[doc = "CTSCR (rw) register accessor: an alias for `Reg<CTSCR_SPEC>`"]
pub type CTSCR = crate::Reg<ctscr::CTSCR_SPEC>;
#[doc = "CTSCR register"]
pub mod ctscr;
#[doc = "RTSCR (rw) register accessor: an alias for `Reg<RTSCR_SPEC>`"]
pub type RTSCR = crate::Reg<rtscr::RTSCR_SPEC>;
#[doc = "RTSCR register"]
pub mod rtscr;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CFG register"]
pub mod cfg;
#[doc = "TOCR (rw) register accessor: an alias for `Reg<TOCR_SPEC>`"]
pub type TOCR = crate::Reg<tocr::TOCR_SPEC>;
#[doc = "TOCR register"]
pub mod tocr;
