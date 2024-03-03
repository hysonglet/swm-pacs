#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IF register"]
    pub if_: IF,
    #[doc = "0x04 - IE register"]
    pub ie: IE,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - START register"]
    pub start: START,
    #[doc = "0x10 - RESERVED2 register"]
    pub reserved2: RESERVED2,
    #[doc = "0x14 - CR register"]
    pub cr: CR,
    #[doc = "0x18 - CRH register"]
    pub crh: CRH,
    #[doc = "0x1c - CRV register"]
    pub crv: CRV,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - BGC register"]
    pub bgc: BGC,
    _reserved8: [u8; 0x18],
    #[doc = "0x40..0x58 - register cluster"]
    pub l0: L0,
    _reserved9: [u8; 0x28],
    #[doc = "0x80..0x98 - register cluster"]
    pub l1: L1,
    _reserved10: [u8; 0x68],
    #[doc = "0x100 - MPUCR register"]
    pub mpucr: MPUCR,
    #[doc = "0x104 - MPUIR register"]
    pub mpuir: MPUIR,
    #[doc = "0x108 - MPUDR register"]
    pub mpudr: MPUDR,
    #[doc = "0x10c - MPUAR register"]
    pub mpuar: MPUAR,
    #[doc = "0x110 - MPULEN register"]
    pub mpulen: MPULEN,
}
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "START (rw) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "START register"]
pub mod start;
#[doc = "RESERVED2 (rw) register accessor: an alias for `Reg<RESERVED2_SPEC>`"]
pub type RESERVED2 = crate::Reg<reserved2::RESERVED2_SPEC>;
#[doc = "RESERVED2 register"]
pub mod reserved2;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "CRH (rw) register accessor: an alias for `Reg<CRH_SPEC>`"]
pub type CRH = crate::Reg<crh::CRH_SPEC>;
#[doc = "CRH register"]
pub mod crh;
#[doc = "CRV (rw) register accessor: an alias for `Reg<CRV_SPEC>`"]
pub type CRV = crate::Reg<crv::CRV_SPEC>;
#[doc = "CRV register"]
pub mod crv;
#[doc = "BGC (rw) register accessor: an alias for `Reg<BGC_SPEC>`"]
pub type BGC = crate::Reg<bgc::BGC_SPEC>;
#[doc = "BGC register"]
pub mod bgc;
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
#[doc = "MPUCR (rw) register accessor: an alias for `Reg<MPUCR_SPEC>`"]
pub type MPUCR = crate::Reg<mpucr::MPUCR_SPEC>;
#[doc = "MPUCR register"]
pub mod mpucr;
#[doc = "MPUIR (rw) register accessor: an alias for `Reg<MPUIR_SPEC>`"]
pub type MPUIR = crate::Reg<mpuir::MPUIR_SPEC>;
#[doc = "MPUIR register"]
pub mod mpuir;
#[doc = "MPUDR (rw) register accessor: an alias for `Reg<MPUDR_SPEC>`"]
pub type MPUDR = crate::Reg<mpudr::MPUDR_SPEC>;
#[doc = "MPUDR register"]
pub mod mpudr;
#[doc = "MPUAR (rw) register accessor: an alias for `Reg<MPUAR_SPEC>`"]
pub type MPUAR = crate::Reg<mpuar::MPUAR_SPEC>;
#[doc = "MPUAR register"]
pub mod mpuar;
#[doc = "MPULEN (rw) register accessor: an alias for `Reg<MPULEN_SPEC>`"]
pub type MPULEN = crate::Reg<mpulen::MPULEN_SPEC>;
#[doc = "MPULEN register"]
pub mod mpulen;
