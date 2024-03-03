#[doc = r"Register block"]
#[repr(C)]
pub struct CH3 {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - AM register"]
    pub am: AM,
    #[doc = "0x08 - DST register"]
    pub dst: DST,
    #[doc = "0x0c - DSTSGADDR1 register"]
    pub dstsgaddr1: DSTSGADDR1,
    #[doc = "0x10 - DSTSGADDR2 register"]
    pub dstsgaddr2: DSTSGADDR2,
    #[doc = "0x14 - DSTSGADDR3 register"]
    pub dstsgaddr3: DSTSGADDR3,
    #[doc = "0x18 - MUX register"]
    pub mux: MUX,
    #[doc = "0x1c - SRC register"]
    pub src: SRC,
    #[doc = "0x20 - SRCSGADDR1 register"]
    pub srcsgaddr1: SRCSGADDR1,
    #[doc = "0x24 - SRCSGADDR2 register"]
    pub srcsgaddr2: SRCSGADDR2,
    #[doc = "0x28 - SRCSGADDR3 register"]
    pub srcsgaddr3: SRCSGADDR3,
    #[doc = "0x2c - DSTSR register"]
    pub dstsr: DSTSR,
    #[doc = "0x30 - SRCSR register"]
    pub srcsr: SRCSR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "AM (rw) register accessor: an alias for `Reg<AM_SPEC>`"]
pub type AM = crate::Reg<am::AM_SPEC>;
#[doc = "AM register"]
pub mod am;
#[doc = "DST (rw) register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "DST register"]
pub mod dst;
#[doc = "DSTSGADDR1 (rw) register accessor: an alias for `Reg<DSTSGADDR1_SPEC>`"]
pub type DSTSGADDR1 = crate::Reg<dstsgaddr1::DSTSGADDR1_SPEC>;
#[doc = "DSTSGADDR1 register"]
pub mod dstsgaddr1;
#[doc = "DSTSGADDR2 (rw) register accessor: an alias for `Reg<DSTSGADDR2_SPEC>`"]
pub type DSTSGADDR2 = crate::Reg<dstsgaddr2::DSTSGADDR2_SPEC>;
#[doc = "DSTSGADDR2 register"]
pub mod dstsgaddr2;
#[doc = "DSTSGADDR3 (rw) register accessor: an alias for `Reg<DSTSGADDR3_SPEC>`"]
pub type DSTSGADDR3 = crate::Reg<dstsgaddr3::DSTSGADDR3_SPEC>;
#[doc = "DSTSGADDR3 register"]
pub mod dstsgaddr3;
#[doc = "MUX (rw) register accessor: an alias for `Reg<MUX_SPEC>`"]
pub type MUX = crate::Reg<mux::MUX_SPEC>;
#[doc = "MUX register"]
pub mod mux;
#[doc = "SRC (rw) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "SRC register"]
pub mod src;
#[doc = "SRCSGADDR1 (rw) register accessor: an alias for `Reg<SRCSGADDR1_SPEC>`"]
pub type SRCSGADDR1 = crate::Reg<srcsgaddr1::SRCSGADDR1_SPEC>;
#[doc = "SRCSGADDR1 register"]
pub mod srcsgaddr1;
#[doc = "SRCSGADDR2 (rw) register accessor: an alias for `Reg<SRCSGADDR2_SPEC>`"]
pub type SRCSGADDR2 = crate::Reg<srcsgaddr2::SRCSGADDR2_SPEC>;
#[doc = "SRCSGADDR2 register"]
pub mod srcsgaddr2;
#[doc = "SRCSGADDR3 (rw) register accessor: an alias for `Reg<SRCSGADDR3_SPEC>`"]
pub type SRCSGADDR3 = crate::Reg<srcsgaddr3::SRCSGADDR3_SPEC>;
#[doc = "SRCSGADDR3 register"]
pub mod srcsgaddr3;
#[doc = "DSTSR (r) register accessor: an alias for `Reg<DSTSR_SPEC>`"]
pub type DSTSR = crate::Reg<dstsr::DSTSR_SPEC>;
#[doc = "DSTSR register"]
pub mod dstsr;
#[doc = "SRCSR (r) register accessor: an alias for `Reg<SRCSR_SPEC>`"]
pub type SRCSR = crate::Reg<srcsr::SRCSR_SPEC>;
#[doc = "SRCSR register"]
pub mod srcsr;
