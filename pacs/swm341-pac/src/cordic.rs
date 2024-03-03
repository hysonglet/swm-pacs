#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMD register"]
    pub cmd: CMD,
    #[doc = "0x04 - INPUT register"]
    pub input: INPUT,
    #[doc = "0x08 - COS register"]
    pub cos: COS,
    #[doc = "0x0c - SIN register"]
    pub sin: SIN,
    #[doc = "0x10 - ARCTAN register"]
    pub arctan: ARCTAN,
    #[doc = "0x14 - IF register"]
    pub if_: IF,
    #[doc = "0x18 - IE register"]
    pub ie: IE,
    #[doc = "0x1c - TANH register"]
    pub tanh: TANH,
}
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "CMD register"]
pub mod cmd;
#[doc = "INPUT (rw) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "INPUT register"]
pub mod input;
#[doc = "COS (rw) register accessor: an alias for `Reg<COS_SPEC>`"]
pub type COS = crate::Reg<cos::COS_SPEC>;
#[doc = "COS register"]
pub mod cos;
#[doc = "SIN (rw) register accessor: an alias for `Reg<SIN_SPEC>`"]
pub type SIN = crate::Reg<sin::SIN_SPEC>;
#[doc = "SIN register"]
pub mod sin;
#[doc = "ARCTAN (rw) register accessor: an alias for `Reg<ARCTAN_SPEC>`"]
pub type ARCTAN = crate::Reg<arctan::ARCTAN_SPEC>;
#[doc = "ARCTAN register"]
pub mod arctan;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "TANH (rw) register accessor: an alias for `Reg<TANH_SPEC>`"]
pub type TANH = crate::Reg<tanh::TANH_SPEC>;
#[doc = "TANH register"]
pub mod tanh;
