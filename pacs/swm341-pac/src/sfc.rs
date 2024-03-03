#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFG register"]
    pub cfg: CFG,
    #[doc = "0x04 - TIM register"]
    pub tim: TIM,
    #[doc = "0x08 - SR register"]
    pub sr: SR,
    #[doc = "0x0c - IF register"]
    pub if_: IF,
    #[doc = "0x10 - IE register"]
    pub ie: IE,
    #[doc = "0x14 - GO register"]
    pub go: GO,
    #[doc = "0x18 - ADDR register"]
    pub addr: ADDR,
    #[doc = "0x1c - DATA register"]
    pub data: DATA,
    #[doc = "0x20 - CMDAHB register"]
    pub cmdahb: CMDAHB,
    #[doc = "0x24 - CMD register"]
    pub cmd: CMD,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CFG register"]
pub mod cfg;
#[doc = "TIM (rw) register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "TIM register"]
pub mod tim;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "GO (rw) register accessor: an alias for `Reg<GO_SPEC>`"]
pub type GO = crate::Reg<go::GO_SPEC>;
#[doc = "GO register"]
pub mod go;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "ADDR register"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
#[doc = "CMDAHB (rw) register accessor: an alias for `Reg<CMDAHB_SPEC>`"]
pub type CMDAHB = crate::Reg<cmdahb::CMDAHB_SPEC>;
#[doc = "CMDAHB register"]
pub mod cmdahb;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "CMD register"]
pub mod cmd;
