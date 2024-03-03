#[doc = r"Register block"]
#[repr(C)]
pub struct FRAME {
    #[doc = "0x00 - INFO register"]
    pub info: INFO,
    #[doc = "0x04..0x34 - DATA register"]
    pub data: [DATA; 12],
}
#[doc = "INFO (rw) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "INFO register"]
pub mod info;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
