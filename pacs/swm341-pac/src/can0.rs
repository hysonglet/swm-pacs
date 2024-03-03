#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - CMD register"]
    pub cmd: CMD,
    #[doc = "0x08 - SR register"]
    pub sr: SR,
    #[doc = "0x0c - IF register"]
    pub if_: IF,
    #[doc = "0x10 - IE register"]
    pub ie: IE,
    #[doc = "0x14 - BT2 register"]
    pub bt2: BT2,
    #[doc = "0x18 - BT0 register"]
    pub bt0: BT0,
    #[doc = "0x1c - BT1 register"]
    pub bt1: BT1,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - AFM register"]
    pub afm: AFM,
    #[doc = "0x28 - AFE register"]
    pub afe: AFE,
    #[doc = "0x2c - ALC register"]
    pub alc: ALC,
    #[doc = "0x30 - ECC register"]
    pub ecc: ECC,
    #[doc = "0x34 - EWLIM register"]
    pub ewlim: EWLIM,
    #[doc = "0x38 - RXERR register"]
    pub rxerr: RXERR,
    #[doc = "0x3c - TXERR register"]
    pub txerr: TXERR,
    #[doc = "0x40..0x74 - register cluster"]
    pub frame: FRAME,
    #[doc = "0x74 - RMCNT register"]
    pub rmcnt: RMCNT,
    _reserved17: [u8; 0x0288],
    #[doc = "0x300..0x340 - ACR register"]
    pub acr: [ACR; 16],
    _reserved18: [u8; 0x40],
    #[doc = "0x380..0x3c0 - AMR register"]
    pub amr: [AMR; 16],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "CMD register"]
pub mod cmd;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "BT2 (rw) register accessor: an alias for `Reg<BT2_SPEC>`"]
pub type BT2 = crate::Reg<bt2::BT2_SPEC>;
#[doc = "BT2 register"]
pub mod bt2;
#[doc = "BT0 (rw) register accessor: an alias for `Reg<BT0_SPEC>`"]
pub type BT0 = crate::Reg<bt0::BT0_SPEC>;
#[doc = "BT0 register"]
pub mod bt0;
#[doc = "BT1 (rw) register accessor: an alias for `Reg<BT1_SPEC>`"]
pub type BT1 = crate::Reg<bt1::BT1_SPEC>;
#[doc = "BT1 register"]
pub mod bt1;
#[doc = "AFM (rw) register accessor: an alias for `Reg<AFM_SPEC>`"]
pub type AFM = crate::Reg<afm::AFM_SPEC>;
#[doc = "AFM register"]
pub mod afm;
#[doc = "AFE (rw) register accessor: an alias for `Reg<AFE_SPEC>`"]
pub type AFE = crate::Reg<afe::AFE_SPEC>;
#[doc = "AFE register"]
pub mod afe;
#[doc = "ALC (r) register accessor: an alias for `Reg<ALC_SPEC>`"]
pub type ALC = crate::Reg<alc::ALC_SPEC>;
#[doc = "ALC register"]
pub mod alc;
#[doc = "ECC (r) register accessor: an alias for `Reg<ECC_SPEC>`"]
pub type ECC = crate::Reg<ecc::ECC_SPEC>;
#[doc = "ECC register"]
pub mod ecc;
#[doc = "EWLIM (rw) register accessor: an alias for `Reg<EWLIM_SPEC>`"]
pub type EWLIM = crate::Reg<ewlim::EWLIM_SPEC>;
#[doc = "EWLIM register"]
pub mod ewlim;
#[doc = "RXERR (rw) register accessor: an alias for `Reg<RXERR_SPEC>`"]
pub type RXERR = crate::Reg<rxerr::RXERR_SPEC>;
#[doc = "RXERR register"]
pub mod rxerr;
#[doc = "TXERR (rw) register accessor: an alias for `Reg<TXERR_SPEC>`"]
pub type TXERR = crate::Reg<txerr::TXERR_SPEC>;
#[doc = "TXERR register"]
pub mod txerr;
#[doc = "register cluster"]
pub use frame::FRAME;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod frame;
#[doc = "RMCNT (r) register accessor: an alias for `Reg<RMCNT_SPEC>`"]
pub type RMCNT = crate::Reg<rmcnt::RMCNT_SPEC>;
#[doc = "RMCNT register"]
pub mod rmcnt;
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "ACR register"]
pub mod acr;
#[doc = "AMR (rw) register accessor: an alias for `Reg<AMR_SPEC>`"]
pub type AMR = crate::Reg<amr::AMR_SPEC>;
#[doc = "AMR register"]
pub mod amr;
