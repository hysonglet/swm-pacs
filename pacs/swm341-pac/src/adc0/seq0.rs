#[doc = r"Register block"]
#[repr(C)]
pub struct SEQ0 {
    #[doc = "0x00 - SR register"]
    pub sr: SR,
    #[doc = "0x04 - DR register"]
    pub dr: DR,
    #[doc = "0x08 - CMP register"]
    pub cmp: CMP,
}
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "DR register"]
pub mod dr;
#[doc = "CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "CMP register"]
pub mod cmp;
