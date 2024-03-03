#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DATA register"]
    pub data: DATA,
    #[doc = "0x04 - ADDR register"]
    pub addr: ADDR,
    #[doc = "0x08 - ERASE register"]
    pub erase: ERASE,
    #[doc = "0x0c - CACHE register"]
    pub cache: CACHE,
    #[doc = "0x10 - CFG0 register"]
    pub cfg0: CFG0,
    #[doc = "0x14 - CFG1 register"]
    pub cfg1: CFG1,
    #[doc = "0x18 - CFG2 register"]
    pub cfg2: CFG2,
    #[doc = "0x1c - CFG3 register"]
    pub cfg3: CFG3,
    #[doc = "0x20 - CFG4 register"]
    pub cfg4: CFG4,
    #[doc = "0x24 - STAT register"]
    pub stat: STAT,
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "ADDR register"]
pub mod addr;
#[doc = "ERASE (rw) register accessor: an alias for `Reg<ERASE_SPEC>`"]
pub type ERASE = crate::Reg<erase::ERASE_SPEC>;
#[doc = "ERASE register"]
pub mod erase;
#[doc = "CACHE (rw) register accessor: an alias for `Reg<CACHE_SPEC>`"]
pub type CACHE = crate::Reg<cache::CACHE_SPEC>;
#[doc = "CACHE register"]
pub mod cache;
#[doc = "CFG0 (rw) register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "CFG0 register"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "CFG1 register"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "CFG2 register"]
pub mod cfg2;
#[doc = "CFG3 (rw) register accessor: an alias for `Reg<CFG3_SPEC>`"]
pub type CFG3 = crate::Reg<cfg3::CFG3_SPEC>;
#[doc = "CFG3 register"]
pub mod cfg3;
#[doc = "CFG4 (rw) register accessor: an alias for `Reg<CFG4_SPEC>`"]
pub type CFG4 = crate::Reg<cfg4::CFG4_SPEC>;
#[doc = "CFG4 register"]
pub mod cfg4;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "STAT register"]
pub mod stat;
