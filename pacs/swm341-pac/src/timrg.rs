#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HALLIE register"]
    pub hallie: HALLIE,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - HALLIF register"]
    pub hallif: HALLIF,
    #[doc = "0x0c - HALLEN register"]
    pub hallen: HALLEN,
    #[doc = "0x10 - HALL0V register"]
    pub hall0v: HALL0V,
    #[doc = "0x14 - HALL3V register"]
    pub hall3v: HALL3V,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - HALLSR register"]
    pub hallsr: HALLSR,
    _reserved6: [u8; 0x20],
    #[doc = "0x40 - EN register"]
    pub en: EN,
}
#[doc = "HALLIE (rw) register accessor: an alias for `Reg<HALLIE_SPEC>`"]
pub type HALLIE = crate::Reg<hallie::HALLIE_SPEC>;
#[doc = "HALLIE register"]
pub mod hallie;
#[doc = "HALLIF (rw) register accessor: an alias for `Reg<HALLIF_SPEC>`"]
pub type HALLIF = crate::Reg<hallif::HALLIF_SPEC>;
#[doc = "HALLIF register"]
pub mod hallif;
#[doc = "HALLEN (rw) register accessor: an alias for `Reg<HALLEN_SPEC>`"]
pub type HALLEN = crate::Reg<hallen::HALLEN_SPEC>;
#[doc = "HALLEN register"]
pub mod hallen;
#[doc = "HALL0V (rw) register accessor: an alias for `Reg<HALL0V_SPEC>`"]
pub type HALL0V = crate::Reg<hall0v::HALL0V_SPEC>;
#[doc = "HALL0V register"]
pub mod hall0v;
#[doc = "HALL3V (rw) register accessor: an alias for `Reg<HALL3V_SPEC>`"]
pub type HALL3V = crate::Reg<hall3v::HALL3V_SPEC>;
#[doc = "HALL3V register"]
pub mod hall3v;
#[doc = "HALLSR (rw) register accessor: an alias for `Reg<HALLSR_SPEC>`"]
pub type HALLSR = crate::Reg<hallsr::HALLSR_SPEC>;
#[doc = "HALLSR register"]
pub mod hallsr;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "EN register"]
pub mod en;
