#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - SR register"]
    pub sr: SR,
    #[doc = "0x08 - TR register"]
    pub tr: TR,
    #[doc = "0x0c - RXDATA register"]
    pub rxdata: RXDATA,
    #[doc = "0x10 - TXDATA register"]
    pub txdata: TXDATA,
    #[doc = "0x14 - IF register"]
    pub if_: IF,
    #[doc = "0x18 - IE register"]
    pub ie: IE,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - MCR register"]
    pub mcr: MCR,
    #[doc = "0x24 - CLK register"]
    pub clk: CLK,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - SCR register"]
    pub scr: SCR,
    #[doc = "0x34 - SADDR register"]
    pub saddr: SADDR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "TR register"]
pub mod tr;
#[doc = "RXDATA (rw) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "RXDATA register"]
pub mod rxdata;
#[doc = "TXDATA (rw) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "TXDATA register"]
pub mod txdata;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "MCR register"]
pub mod mcr;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "CLK register"]
pub mod clk;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "SCR register"]
pub mod scr;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "SADDR register"]
pub mod saddr;
