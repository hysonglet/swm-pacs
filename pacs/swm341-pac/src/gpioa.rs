#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ODR register"]
    pub odr: ODR,
    #[doc = "0x04 - DIR register"]
    pub dir: DIR,
    #[doc = "0x08 - INTLVLTRG register"]
    pub intlvltrg: INTLVLTRG,
    #[doc = "0x0c - INTBE register"]
    pub intbe: INTBE,
    #[doc = "0x10 - INTRISEEN register"]
    pub intriseen: INTRISEEN,
    #[doc = "0x14 - INTEN register"]
    pub inten: INTEN,
    #[doc = "0x18 - INTRAWSTAT register"]
    pub intrawstat: INTRAWSTAT,
    #[doc = "0x1c - INTSTAT register"]
    pub intstat: INTSTAT,
    #[doc = "0x20 - INTCLR register"]
    pub intclr: INTCLR,
    #[doc = "0x24 - DMAEN register"]
    pub dmaen: DMAEN,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - IDR register"]
    pub idr: IDR,
    _reserved11: [u8; 0x0c],
    #[doc = "0x40 - DATAPIN0 register"]
    pub datapin0: DATAPIN0,
    #[doc = "0x44 - DATAPIN1 register"]
    pub datapin1: DATAPIN1,
    #[doc = "0x48 - DATAPIN2 register"]
    pub datapin2: DATAPIN2,
    #[doc = "0x4c - DATAPIN3 register"]
    pub datapin3: DATAPIN3,
    #[doc = "0x50 - DATAPIN4 register"]
    pub datapin4: DATAPIN4,
    #[doc = "0x54 - DATAPIN5 register"]
    pub datapin5: DATAPIN5,
    #[doc = "0x58 - DATAPIN6 register"]
    pub datapin6: DATAPIN6,
    #[doc = "0x5c - DATAPIN7 register"]
    pub datapin7: DATAPIN7,
    #[doc = "0x60 - DATAPIN8 register"]
    pub datapin8: DATAPIN8,
    #[doc = "0x64 - DATAPIN9 register"]
    pub datapin9: DATAPIN9,
    #[doc = "0x68 - DATAPIN10 register"]
    pub datapin10: DATAPIN10,
    #[doc = "0x6c - DATAPIN11 register"]
    pub datapin11: DATAPIN11,
    #[doc = "0x70 - DATAPIN12 register"]
    pub datapin12: DATAPIN12,
    #[doc = "0x74 - DATAPIN13 register"]
    pub datapin13: DATAPIN13,
    #[doc = "0x78 - DATAPIN14 register"]
    pub datapin14: DATAPIN14,
    #[doc = "0x7c - DATAPIN15 register"]
    pub datapin15: DATAPIN15,
}
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "ODR register"]
pub mod odr;
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "DIR register"]
pub mod dir;
#[doc = "INTLVLTRG (rw) register accessor: an alias for `Reg<INTLVLTRG_SPEC>`"]
pub type INTLVLTRG = crate::Reg<intlvltrg::INTLVLTRG_SPEC>;
#[doc = "INTLVLTRG register"]
pub mod intlvltrg;
#[doc = "INTBE (rw) register accessor: an alias for `Reg<INTBE_SPEC>`"]
pub type INTBE = crate::Reg<intbe::INTBE_SPEC>;
#[doc = "INTBE register"]
pub mod intbe;
#[doc = "INTRISEEN (rw) register accessor: an alias for `Reg<INTRISEEN_SPEC>`"]
pub type INTRISEEN = crate::Reg<intriseen::INTRISEEN_SPEC>;
#[doc = "INTRISEEN register"]
pub mod intriseen;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "INTEN register"]
pub mod inten;
#[doc = "INTRAWSTAT (rw) register accessor: an alias for `Reg<INTRAWSTAT_SPEC>`"]
pub type INTRAWSTAT = crate::Reg<intrawstat::INTRAWSTAT_SPEC>;
#[doc = "INTRAWSTAT register"]
pub mod intrawstat;
#[doc = "INTSTAT (rw) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "INTSTAT register"]
pub mod intstat;
#[doc = "INTCLR (rw) register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "INTCLR register"]
pub mod intclr;
#[doc = "DMAEN (rw) register accessor: an alias for `Reg<DMAEN_SPEC>`"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMAEN register"]
pub mod dmaen;
#[doc = "IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "IDR register"]
pub mod idr;
#[doc = "DATAPIN0 (rw) register accessor: an alias for `Reg<DATAPIN0_SPEC>`"]
pub type DATAPIN0 = crate::Reg<datapin0::DATAPIN0_SPEC>;
#[doc = "DATAPIN0 register"]
pub mod datapin0;
#[doc = "DATAPIN1 (rw) register accessor: an alias for `Reg<DATAPIN1_SPEC>`"]
pub type DATAPIN1 = crate::Reg<datapin1::DATAPIN1_SPEC>;
#[doc = "DATAPIN1 register"]
pub mod datapin1;
#[doc = "DATAPIN2 (rw) register accessor: an alias for `Reg<DATAPIN2_SPEC>`"]
pub type DATAPIN2 = crate::Reg<datapin2::DATAPIN2_SPEC>;
#[doc = "DATAPIN2 register"]
pub mod datapin2;
#[doc = "DATAPIN3 (rw) register accessor: an alias for `Reg<DATAPIN3_SPEC>`"]
pub type DATAPIN3 = crate::Reg<datapin3::DATAPIN3_SPEC>;
#[doc = "DATAPIN3 register"]
pub mod datapin3;
#[doc = "DATAPIN4 (rw) register accessor: an alias for `Reg<DATAPIN4_SPEC>`"]
pub type DATAPIN4 = crate::Reg<datapin4::DATAPIN4_SPEC>;
#[doc = "DATAPIN4 register"]
pub mod datapin4;
#[doc = "DATAPIN5 (rw) register accessor: an alias for `Reg<DATAPIN5_SPEC>`"]
pub type DATAPIN5 = crate::Reg<datapin5::DATAPIN5_SPEC>;
#[doc = "DATAPIN5 register"]
pub mod datapin5;
#[doc = "DATAPIN6 (rw) register accessor: an alias for `Reg<DATAPIN6_SPEC>`"]
pub type DATAPIN6 = crate::Reg<datapin6::DATAPIN6_SPEC>;
#[doc = "DATAPIN6 register"]
pub mod datapin6;
#[doc = "DATAPIN7 (rw) register accessor: an alias for `Reg<DATAPIN7_SPEC>`"]
pub type DATAPIN7 = crate::Reg<datapin7::DATAPIN7_SPEC>;
#[doc = "DATAPIN7 register"]
pub mod datapin7;
#[doc = "DATAPIN8 (rw) register accessor: an alias for `Reg<DATAPIN8_SPEC>`"]
pub type DATAPIN8 = crate::Reg<datapin8::DATAPIN8_SPEC>;
#[doc = "DATAPIN8 register"]
pub mod datapin8;
#[doc = "DATAPIN9 (rw) register accessor: an alias for `Reg<DATAPIN9_SPEC>`"]
pub type DATAPIN9 = crate::Reg<datapin9::DATAPIN9_SPEC>;
#[doc = "DATAPIN9 register"]
pub mod datapin9;
#[doc = "DATAPIN10 (rw) register accessor: an alias for `Reg<DATAPIN10_SPEC>`"]
pub type DATAPIN10 = crate::Reg<datapin10::DATAPIN10_SPEC>;
#[doc = "DATAPIN10 register"]
pub mod datapin10;
#[doc = "DATAPIN11 (rw) register accessor: an alias for `Reg<DATAPIN11_SPEC>`"]
pub type DATAPIN11 = crate::Reg<datapin11::DATAPIN11_SPEC>;
#[doc = "DATAPIN11 register"]
pub mod datapin11;
#[doc = "DATAPIN12 (rw) register accessor: an alias for `Reg<DATAPIN12_SPEC>`"]
pub type DATAPIN12 = crate::Reg<datapin12::DATAPIN12_SPEC>;
#[doc = "DATAPIN12 register"]
pub mod datapin12;
#[doc = "DATAPIN13 (rw) register accessor: an alias for `Reg<DATAPIN13_SPEC>`"]
pub type DATAPIN13 = crate::Reg<datapin13::DATAPIN13_SPEC>;
#[doc = "DATAPIN13 register"]
pub mod datapin13;
#[doc = "DATAPIN14 (rw) register accessor: an alias for `Reg<DATAPIN14_SPEC>`"]
pub type DATAPIN14 = crate::Reg<datapin14::DATAPIN14_SPEC>;
#[doc = "DATAPIN14 register"]
pub mod datapin14;
#[doc = "DATAPIN15 (rw) register accessor: an alias for `Reg<DATAPIN15_SPEC>`"]
pub type DATAPIN15 = crate::Reg<datapin15::DATAPIN15_SPEC>;
#[doc = "DATAPIN15 register"]
pub mod datapin15;
