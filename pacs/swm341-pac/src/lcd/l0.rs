#[doc = r"Register block"]
#[repr(C)]
pub struct L0 {
    #[doc = "0x00 - LCR register"]
    pub lcr: LCR,
    #[doc = "0x04 - WHP register"]
    pub whp: WHP,
    #[doc = "0x08 - WVP register"]
    pub wvp: WVP,
    #[doc = "0x0c - ADDR register"]
    pub addr: ADDR,
    #[doc = "0x10 - LLEN register"]
    pub llen: LLEN,
    #[doc = "0x14 - CK register"]
    pub ck: CK,
}
#[doc = "LCR (rw) register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "LCR register"]
pub mod lcr;
#[doc = "WHP (rw) register accessor: an alias for `Reg<WHP_SPEC>`"]
pub type WHP = crate::Reg<whp::WHP_SPEC>;
#[doc = "WHP register"]
pub mod whp;
#[doc = "WVP (rw) register accessor: an alias for `Reg<WVP_SPEC>`"]
pub type WVP = crate::Reg<wvp::WVP_SPEC>;
#[doc = "WVP register"]
pub mod wvp;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "ADDR register"]
pub mod addr;
#[doc = "LLEN (rw) register accessor: an alias for `Reg<LLEN_SPEC>`"]
pub type LLEN = crate::Reg<llen::LLEN_SPEC>;
#[doc = "LLEN register"]
pub mod llen;
#[doc = "CK (rw) register accessor: an alias for `Reg<CK_SPEC>`"]
pub type CK = crate::Reg<ck::CK_SPEC>;
#[doc = "CK register"]
pub mod ck;
