#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RSTVAL register"]
    pub rstval: RSTVAL,
    #[doc = "0x04 - INTVAL register"]
    pub intval: INTVAL,
    #[doc = "0x08 - CR register"]
    pub cr: CR,
    #[doc = "0x0c - IF register"]
    pub if_: IF,
    #[doc = "0x10 - FEED register"]
    pub feed: FEED,
}
#[doc = "RSTVAL (rw) register accessor: an alias for `Reg<RSTVAL_SPEC>`"]
pub type RSTVAL = crate::Reg<rstval::RSTVAL_SPEC>;
#[doc = "RSTVAL register"]
pub mod rstval;
#[doc = "INTVAL (rw) register accessor: an alias for `Reg<INTVAL_SPEC>`"]
pub type INTVAL = crate::Reg<intval::INTVAL_SPEC>;
#[doc = "INTVAL register"]
pub mod intval;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "FEED (rw) register accessor: an alias for `Reg<FEED_SPEC>`"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "FEED register"]
pub mod feed;
