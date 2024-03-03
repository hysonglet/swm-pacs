#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTRL register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DATA register"]
    pub data: DATA,
    #[doc = "0x08 - STAT register"]
    pub stat: STAT,
    #[doc = "0x0c - IE register"]
    pub ie: IE,
    #[doc = "0x10 - IF register"]
    pub if_: IF,
    #[doc = "0x14 - I2SCR register"]
    pub i2scr: I2SCR,
    #[doc = "0x18 - I2SPR register"]
    pub i2spr: I2SPR,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CTRL register"]
pub mod ctrl;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "STAT register"]
pub mod stat;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "I2SCR (rw) register accessor: an alias for `Reg<I2SCR_SPEC>`"]
pub type I2SCR = crate::Reg<i2scr::I2SCR_SPEC>;
#[doc = "I2SCR register"]
pub mod i2scr;
#[doc = "I2SPR (rw) register accessor: an alias for `Reg<I2SPR_SPEC>`"]
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
#[doc = "I2SPR register"]
pub mod i2spr;
