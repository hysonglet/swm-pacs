#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - SR register"]
    pub sr: SR,
    #[doc = "0x08 - SWTRG register"]
    pub swtrg: SWTRG,
    #[doc = "0x0c - DHR register"]
    pub dhr: DHR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "SWTRG (w) register accessor: an alias for `Reg<SWTRG_SPEC>`"]
pub type SWTRG = crate::Reg<swtrg::SWTRG_SPEC>;
#[doc = "SWTRG register"]
pub mod swtrg;
#[doc = "DHR (rw) register accessor: an alias for `Reg<DHR_SPEC>`"]
pub type DHR = crate::Reg<dhr::DHR_SPEC>;
#[doc = "DHR register"]
pub mod dhr;
