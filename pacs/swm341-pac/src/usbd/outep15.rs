#[doc = r"Register block"]
#[repr(C)]
pub struct OUTEP15 {
    #[doc = "0x00 - RXCR register"]
    pub rxcr: RXCR,
}
#[doc = "RXCR (rw) register accessor: an alias for `Reg<RXCR_SPEC>`"]
pub type RXCR = crate::Reg<rxcr::RXCR_SPEC>;
#[doc = "RXCR register"]
pub mod rxcr;
