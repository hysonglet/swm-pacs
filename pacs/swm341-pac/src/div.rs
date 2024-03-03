#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - SR register"]
    pub sr: SR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - DIVIDEND register"]
    pub dividend: DIVIDEND,
    #[doc = "0x14 - DIVISOR register"]
    pub divisor: DIVISOR,
    #[doc = "0x18 - QUO register"]
    pub quo: QUO,
    #[doc = "0x1c - REMAIN register"]
    pub remain: REMAIN,
    #[doc = "0x20 - RADICAND register"]
    pub radicand: RADICAND,
    #[doc = "0x24 - ROOT register"]
    pub root: ROOT,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "DIVIDEND (rw) register accessor: an alias for `Reg<DIVIDEND_SPEC>`"]
pub type DIVIDEND = crate::Reg<dividend::DIVIDEND_SPEC>;
#[doc = "DIVIDEND register"]
pub mod dividend;
#[doc = "DIVISOR (rw) register accessor: an alias for `Reg<DIVISOR_SPEC>`"]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "DIVISOR register"]
pub mod divisor;
#[doc = "QUO (rw) register accessor: an alias for `Reg<QUO_SPEC>`"]
pub type QUO = crate::Reg<quo::QUO_SPEC>;
#[doc = "QUO register"]
pub mod quo;
#[doc = "REMAIN (rw) register accessor: an alias for `Reg<REMAIN_SPEC>`"]
pub type REMAIN = crate::Reg<remain::REMAIN_SPEC>;
#[doc = "REMAIN register"]
pub mod remain;
#[doc = "RADICAND (rw) register accessor: an alias for `Reg<RADICAND_SPEC>`"]
pub type RADICAND = crate::Reg<radicand::RADICAND_SPEC>;
#[doc = "RADICAND register"]
pub mod radicand;
#[doc = "ROOT (rw) register accessor: an alias for `Reg<ROOT_SPEC>`"]
pub type ROOT = crate::Reg<root::ROOT_SPEC>;
#[doc = "ROOT register"]
pub mod root;
