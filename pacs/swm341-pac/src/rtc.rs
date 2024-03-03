#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MINSEC register"]
    pub minsec: MINSEC,
    #[doc = "0x04 - DATHUR register"]
    pub dathur: DATHUR,
    #[doc = "0x08 - MONDAY register"]
    pub monday: MONDAY,
    #[doc = "0x0c - YEAR register"]
    pub year: YEAR,
    #[doc = "0x10 - MINSECAL register"]
    pub minsecal: MINSECAL,
    #[doc = "0x14 - DAYHURAL register"]
    pub dayhural: DAYHURAL,
    #[doc = "0x18 - LOAD register"]
    pub load: LOAD,
    #[doc = "0x1c - IE register"]
    pub ie: IE,
    #[doc = "0x20 - IF register"]
    pub if_: IF,
    #[doc = "0x24 - EN register"]
    pub en: EN,
    #[doc = "0x28 - CFGABLE register"]
    pub cfgable: CFGABLE,
    #[doc = "0x2c - TRIM register"]
    pub trim: TRIM,
    #[doc = "0x30 - TRIMM register"]
    pub trimm: TRIMM,
}
#[doc = "MINSEC (rw) register accessor: an alias for `Reg<MINSEC_SPEC>`"]
pub type MINSEC = crate::Reg<minsec::MINSEC_SPEC>;
#[doc = "MINSEC register"]
pub mod minsec;
#[doc = "DATHUR (rw) register accessor: an alias for `Reg<DATHUR_SPEC>`"]
pub type DATHUR = crate::Reg<dathur::DATHUR_SPEC>;
#[doc = "DATHUR register"]
pub mod dathur;
#[doc = "MONDAY (rw) register accessor: an alias for `Reg<MONDAY_SPEC>`"]
pub type MONDAY = crate::Reg<monday::MONDAY_SPEC>;
#[doc = "MONDAY register"]
pub mod monday;
#[doc = "YEAR (rw) register accessor: an alias for `Reg<YEAR_SPEC>`"]
pub type YEAR = crate::Reg<year::YEAR_SPEC>;
#[doc = "YEAR register"]
pub mod year;
#[doc = "MINSECAL (rw) register accessor: an alias for `Reg<MINSECAL_SPEC>`"]
pub type MINSECAL = crate::Reg<minsecal::MINSECAL_SPEC>;
#[doc = "MINSECAL register"]
pub mod minsecal;
#[doc = "DAYHURAL (rw) register accessor: an alias for `Reg<DAYHURAL_SPEC>`"]
pub type DAYHURAL = crate::Reg<dayhural::DAYHURAL_SPEC>;
#[doc = "DAYHURAL register"]
pub mod dayhural;
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "LOAD register"]
pub mod load;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "EN register"]
pub mod en;
#[doc = "CFGABLE (rw) register accessor: an alias for `Reg<CFGABLE_SPEC>`"]
pub type CFGABLE = crate::Reg<cfgable::CFGABLE_SPEC>;
#[doc = "CFGABLE register"]
pub mod cfgable;
#[doc = "TRIM (rw) register accessor: an alias for `Reg<TRIM_SPEC>`"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "TRIM register"]
pub mod trim;
#[doc = "TRIMM (rw) register accessor: an alias for `Reg<TRIMM_SPEC>`"]
pub type TRIMM = crate::Reg<trimm::TRIMM_SPEC>;
#[doc = "TRIMM register"]
pub mod trimm;
