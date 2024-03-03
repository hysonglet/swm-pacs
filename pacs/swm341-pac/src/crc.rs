#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - DATAIN register"]
    pub datain: DATAIN,
    #[doc = "0x08 - INIVAL register"]
    pub inival: INIVAL,
    #[doc = "0x0c - RESULT register"]
    pub result: RESULT,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "DATAIN (w) register accessor: an alias for `Reg<DATAIN_SPEC>`"]
pub type DATAIN = crate::Reg<datain::DATAIN_SPEC>;
#[doc = "DATAIN register"]
pub mod datain;
#[doc = "INIVAL (rw) register accessor: an alias for `Reg<INIVAL_SPEC>`"]
pub type INIVAL = crate::Reg<inival::INIVAL_SPEC>;
#[doc = "INIVAL register"]
pub mod inival;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "RESULT register"]
pub mod result;
