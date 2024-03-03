#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM register"]
    pub tim: TIM,
    #[doc = "0x04 - CFG register"]
    pub cfg: CFG,
    #[doc = "0x08 - T64 register"]
    pub t64: T64,
    #[doc = "0x0c - CR register"]
    pub cr: CR,
    #[doc = "0x10 - SR register"]
    pub sr: SR,
}
#[doc = "TIM (rw) register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "TIM register"]
pub mod tim;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CFG register"]
pub mod cfg;
#[doc = "T64 (rw) register accessor: an alias for `Reg<T64_SPEC>`"]
pub type T64 = crate::Reg<t64::T64_SPEC>;
#[doc = "T64 register"]
pub mod t64;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
