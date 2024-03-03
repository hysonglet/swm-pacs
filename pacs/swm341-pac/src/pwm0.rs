#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - OCR register"]
    pub ocr: OCR,
    #[doc = "0x08 - BRKCR register"]
    pub brkcr: BRKCR,
    #[doc = "0x0c - BRKIN register"]
    pub brkin: BRKIN,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - PERIOD register"]
    pub period: PERIOD,
    #[doc = "0x24 - CMPA register"]
    pub cmpa: CMPA,
    #[doc = "0x28 - CMPB register"]
    pub cmpb: CMPB,
    #[doc = "0x2c - DZA register"]
    pub dza: DZA,
    #[doc = "0x30 - DZB register"]
    pub dzb: DZB,
    #[doc = "0x34 - CMPA2 register"]
    pub cmpa2: CMPA2,
    #[doc = "0x38 - CMPB2 register"]
    pub cmpb2: CMPB2,
    _reserved11: [u8; 0x14],
    #[doc = "0x50 - OVFTRG register"]
    pub ovftrg: OVFTRG,
    #[doc = "0x54 - CMPTRG register"]
    pub cmptrg: CMPTRG,
    _reserved13: [u8; 0x08],
    #[doc = "0x60 - EVMUX register"]
    pub evmux: EVMUX,
    #[doc = "0x64 - EVMSK register"]
    pub evmsk: EVMSK,
    _reserved15: [u8; 0x08],
    #[doc = "0x70 - IE register"]
    pub ie: IE,
    #[doc = "0x74 - IF register"]
    pub if_: IF,
    #[doc = "0x78 - VALUE register"]
    pub value: VALUE,
    #[doc = "0x7c - SR register"]
    pub sr: SR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "OCR (rw) register accessor: an alias for `Reg<OCR_SPEC>`"]
pub type OCR = crate::Reg<ocr::OCR_SPEC>;
#[doc = "OCR register"]
pub mod ocr;
#[doc = "BRKCR (rw) register accessor: an alias for `Reg<BRKCR_SPEC>`"]
pub type BRKCR = crate::Reg<brkcr::BRKCR_SPEC>;
#[doc = "BRKCR register"]
pub mod brkcr;
#[doc = "BRKIN (rw) register accessor: an alias for `Reg<BRKIN_SPEC>`"]
pub type BRKIN = crate::Reg<brkin::BRKIN_SPEC>;
#[doc = "BRKIN register"]
pub mod brkin;
#[doc = "PERIOD (rw) register accessor: an alias for `Reg<PERIOD_SPEC>`"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "PERIOD register"]
pub mod period;
#[doc = "CMPA (rw) register accessor: an alias for `Reg<CMPA_SPEC>`"]
pub type CMPA = crate::Reg<cmpa::CMPA_SPEC>;
#[doc = "CMPA register"]
pub mod cmpa;
#[doc = "CMPB (rw) register accessor: an alias for `Reg<CMPB_SPEC>`"]
pub type CMPB = crate::Reg<cmpb::CMPB_SPEC>;
#[doc = "CMPB register"]
pub mod cmpb;
#[doc = "DZA (rw) register accessor: an alias for `Reg<DZA_SPEC>`"]
pub type DZA = crate::Reg<dza::DZA_SPEC>;
#[doc = "DZA register"]
pub mod dza;
#[doc = "DZB (rw) register accessor: an alias for `Reg<DZB_SPEC>`"]
pub type DZB = crate::Reg<dzb::DZB_SPEC>;
#[doc = "DZB register"]
pub mod dzb;
#[doc = "CMPA2 (rw) register accessor: an alias for `Reg<CMPA2_SPEC>`"]
pub type CMPA2 = crate::Reg<cmpa2::CMPA2_SPEC>;
#[doc = "CMPA2 register"]
pub mod cmpa2;
#[doc = "CMPB2 (rw) register accessor: an alias for `Reg<CMPB2_SPEC>`"]
pub type CMPB2 = crate::Reg<cmpb2::CMPB2_SPEC>;
#[doc = "CMPB2 register"]
pub mod cmpb2;
#[doc = "OVFTRG (rw) register accessor: an alias for `Reg<OVFTRG_SPEC>`"]
pub type OVFTRG = crate::Reg<ovftrg::OVFTRG_SPEC>;
#[doc = "OVFTRG register"]
pub mod ovftrg;
#[doc = "CMPTRG (rw) register accessor: an alias for `Reg<CMPTRG_SPEC>`"]
pub type CMPTRG = crate::Reg<cmptrg::CMPTRG_SPEC>;
#[doc = "CMPTRG register"]
pub mod cmptrg;
#[doc = "EVMUX (rw) register accessor: an alias for `Reg<EVMUX_SPEC>`"]
pub type EVMUX = crate::Reg<evmux::EVMUX_SPEC>;
#[doc = "EVMUX register"]
pub mod evmux;
#[doc = "EVMSK (rw) register accessor: an alias for `Reg<EVMSK_SPEC>`"]
pub type EVMSK = crate::Reg<evmsk::EVMSK_SPEC>;
#[doc = "EVMSK register"]
pub mod evmsk;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "VALUE (rw) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "VALUE register"]
pub mod value;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
