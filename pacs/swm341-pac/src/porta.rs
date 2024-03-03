#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FUNC0 register"]
    pub func0: FUNC0,
    #[doc = "0x04 - FUNC1 register"]
    pub func1: FUNC1,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - PULLU register"]
    pub pullu: PULLU,
    _reserved3: [u8; 0xfc],
    #[doc = "0x200 - PULLD register"]
    pub pulld: PULLD,
    _reserved4: [u8; 0xfc],
    #[doc = "0x300 - INEN register"]
    pub inen: INEN,
    _reserved5: [u8; 0xfc],
    #[doc = "0x400 - OPEND register"]
    pub opend: OPEND,
    _reserved6: [u8; 0xfc],
    #[doc = "0x500 - DRVST register"]
    pub drvst: DRVST,
}
#[doc = "FUNC0 (rw) register accessor: an alias for `Reg<FUNC0_SPEC>`"]
pub type FUNC0 = crate::Reg<func0::FUNC0_SPEC>;
#[doc = "FUNC0 register"]
pub mod func0;
#[doc = "FUNC1 (rw) register accessor: an alias for `Reg<FUNC1_SPEC>`"]
pub type FUNC1 = crate::Reg<func1::FUNC1_SPEC>;
#[doc = "FUNC1 register"]
pub mod func1;
#[doc = "PULLU (rw) register accessor: an alias for `Reg<PULLU_SPEC>`"]
pub type PULLU = crate::Reg<pullu::PULLU_SPEC>;
#[doc = "PULLU register"]
pub mod pullu;
#[doc = "PULLD (rw) register accessor: an alias for `Reg<PULLD_SPEC>`"]
pub type PULLD = crate::Reg<pulld::PULLD_SPEC>;
#[doc = "PULLD register"]
pub mod pulld;
#[doc = "INEN (rw) register accessor: an alias for `Reg<INEN_SPEC>`"]
pub type INEN = crate::Reg<inen::INEN_SPEC>;
#[doc = "INEN register"]
pub mod inen;
#[doc = "OPEND (rw) register accessor: an alias for `Reg<OPEND_SPEC>`"]
pub type OPEND = crate::Reg<opend::OPEND_SPEC>;
#[doc = "OPEND register"]
pub mod opend;
#[doc = "DRVST (rw) register accessor: an alias for `Reg<DRVST_SPEC>`"]
pub type DRVST = crate::Reg<drvst::DRVST_SPEC>;
#[doc = "DRVST register"]
pub mod drvst;
