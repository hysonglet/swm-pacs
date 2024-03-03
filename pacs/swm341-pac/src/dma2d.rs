#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IF register"]
    pub if_: IF,
    #[doc = "0x04 - IE register"]
    pub ie: IE,
    #[doc = "0x08 - CR register"]
    pub cr: CR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10..0x20 - register cluster"]
    pub l0: L0,
    #[doc = "0x20..0x30 - register cluster"]
    pub l1: L1,
    #[doc = "0x30..0x40 - register cluster"]
    pub l2: L2,
    #[doc = "0x40 - NLR register"]
    pub nlr: NLR,
}
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "register cluster"]
pub use l0::L0;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod l0;
#[doc = "register cluster"]
pub use l1::L1;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod l1;
#[doc = "register cluster"]
pub use l2::L2;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod l2;
#[doc = "NLR (rw) register accessor: an alias for `Reg<NLR_SPEC>`"]
pub type NLR = crate::Reg<nlr::NLR_SPEC>;
#[doc = "NLR register"]
pub mod nlr;
