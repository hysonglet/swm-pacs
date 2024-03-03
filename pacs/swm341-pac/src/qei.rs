#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - POSCNT register"]
    pub poscnt: POSCNT,
    #[doc = "0x08 - MAXCNT register"]
    pub maxcnt: MAXCNT,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - IE register"]
    pub ie: IE,
    #[doc = "0x24 - IM register"]
    pub im: IM,
    #[doc = "0x28 - IC register"]
    pub ic: IC,
    #[doc = "0x2c - IF register"]
    pub if_: IF,
    #[doc = "0x30 - IFOV register"]
    pub ifov: IFOV,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "POSCNT (rw) register accessor: an alias for `Reg<POSCNT_SPEC>`"]
pub type POSCNT = crate::Reg<poscnt::POSCNT_SPEC>;
#[doc = "POSCNT register"]
pub mod poscnt;
#[doc = "MAXCNT (rw) register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "MAXCNT register"]
pub mod maxcnt;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IM (rw) register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "IM register"]
pub mod im;
#[doc = "IC (w) register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "IC register"]
pub mod ic;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IFOV (rw) register accessor: an alias for `Reg<IFOV_SPEC>`"]
pub type IFOV = crate::Reg<ifov::IFOV_SPEC>;
#[doc = "IFOV register"]
pub mod ifov;
