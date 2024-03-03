#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LOAD register"]
    pub load: LOAD,
    #[doc = "0x04 - VALUE register"]
    pub value: VALUE,
    #[doc = "0x08 - CR register"]
    pub cr: CR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - IE register"]
    pub ie: IE,
    #[doc = "0x14 - IF register"]
    pub if_: IF,
    #[doc = "0x18 - HALT register"]
    pub halt: HALT,
    #[doc = "0x1c - OCCR register"]
    pub occr: OCCR,
    #[doc = "0x20 - OCMAT register"]
    pub ocmat: OCMAT,
    #[doc = "0x24 - RESERVED2 register"]
    pub reserved2: RESERVED2,
    #[doc = "0x28 - ICLOW register"]
    pub iclow: ICLOW,
    #[doc = "0x2c - ICHIGH register"]
    pub ichigh: ICHIGH,
    #[doc = "0x30 - PREDIV register"]
    pub prediv: PREDIV,
}
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "LOAD register"]
pub mod load;
#[doc = "VALUE (r) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "VALUE register"]
pub mod value;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "HALT (rw) register accessor: an alias for `Reg<HALT_SPEC>`"]
pub type HALT = crate::Reg<halt::HALT_SPEC>;
#[doc = "HALT register"]
pub mod halt;
#[doc = "OCCR (rw) register accessor: an alias for `Reg<OCCR_SPEC>`"]
pub type OCCR = crate::Reg<occr::OCCR_SPEC>;
#[doc = "OCCR register"]
pub mod occr;
#[doc = "OCMAT (rw) register accessor: an alias for `Reg<OCMAT_SPEC>`"]
pub type OCMAT = crate::Reg<ocmat::OCMAT_SPEC>;
#[doc = "OCMAT register"]
pub mod ocmat;
#[doc = "RESERVED2 (rw) register accessor: an alias for `Reg<RESERVED2_SPEC>`"]
pub type RESERVED2 = crate::Reg<reserved2::RESERVED2_SPEC>;
#[doc = "RESERVED2 register"]
pub mod reserved2;
#[doc = "ICLOW (rw) register accessor: an alias for `Reg<ICLOW_SPEC>`"]
pub type ICLOW = crate::Reg<iclow::ICLOW_SPEC>;
#[doc = "ICLOW register"]
pub mod iclow;
#[doc = "ICHIGH (rw) register accessor: an alias for `Reg<ICHIGH_SPEC>`"]
pub type ICHIGH = crate::Reg<ichigh::ICHIGH_SPEC>;
#[doc = "ICHIGH register"]
pub mod ichigh;
#[doc = "PREDIV (rw) register accessor: an alias for `Reg<PREDIV_SPEC>`"]
pub type PREDIV = crate::Reg<prediv::PREDIV_SPEC>;
#[doc = "PREDIV register"]
pub mod prediv;
