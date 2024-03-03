#[doc = r"Register block"]
#[repr(C)]
pub struct INEP15 {
    #[doc = "0x00 - TXCR register"]
    pub txcr: TXCR,
    #[doc = "0x04 - TXSR register"]
    pub txsr: TXSR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - TXTRSZ register"]
    pub txtrsz: TXTRSZ,
}
#[doc = "TXCR (rw) register accessor: an alias for `Reg<TXCR_SPEC>`"]
pub type TXCR = crate::Reg<txcr::TXCR_SPEC>;
#[doc = "TXCR register"]
pub mod txcr;
#[doc = "TXSR (rw) register accessor: an alias for `Reg<TXSR_SPEC>`"]
pub type TXSR = crate::Reg<txsr::TXSR_SPEC>;
#[doc = "TXSR register"]
pub mod txsr;
#[doc = "TXTRSZ (rw) register accessor: an alias for `Reg<TXTRSZ_SPEC>`"]
pub type TXTRSZ = crate::Reg<txtrsz::TXTRSZ_SPEC>;
#[doc = "TXTRSZ register"]
pub mod txtrsz;
