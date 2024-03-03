#[doc = r"Register block"]
#[repr(C)]
pub struct L0 {
    #[doc = "0x00 - MAR register"]
    pub mar: MAR,
    #[doc = "0x04 - OR register"]
    pub or: OR,
    #[doc = "0x08 - PFCCR register"]
    pub pfccr: PFCCR,
    #[doc = "0x0c - COLOR register"]
    pub color: COLOR,
}
#[doc = "MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "MAR register"]
pub mod mar;
#[doc = "OR (rw) register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "OR register"]
pub mod or;
#[doc = "PFCCR (rw) register accessor: an alias for `Reg<PFCCR_SPEC>`"]
pub type PFCCR = crate::Reg<pfccr::PFCCR_SPEC>;
#[doc = "PFCCR register"]
pub mod pfccr;
#[doc = "COLOR (rw) register accessor: an alias for `Reg<COLOR_SPEC>`"]
pub type COLOR = crate::Reg<color::COLOR_SPEC>;
#[doc = "COLOR register"]
pub mod color;
