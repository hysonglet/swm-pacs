#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EN register"]
    pub en: EN,
    #[doc = "0x04 - IE register"]
    pub ie: IE,
    #[doc = "0x08 - IM register"]
    pub im: IM,
    #[doc = "0x0c - IF register"]
    pub if_: IF,
    #[doc = "0x10 - DSTSGIE register"]
    pub dstsgie: DSTSGIE,
    #[doc = "0x14 - DSTSGIM register"]
    pub dstsgim: DSTSGIM,
    #[doc = "0x18 - DSTSGIF register"]
    pub dstsgif: DSTSGIF,
    #[doc = "0x1c - SRCSGIE register"]
    pub srcsgie: SRCSGIE,
    #[doc = "0x20 - SRCSGIM register"]
    pub srcsgim: SRCSGIM,
    #[doc = "0x24 - SRCSGIF register"]
    pub srcsgif: SRCSGIF,
    _reserved10: [u8; 0x14],
    #[doc = "0x3c - PRI register"]
    pub pri: PRI,
    #[doc = "0x40..0x74 - register cluster"]
    pub ch0: CH0,
    _reserved12: [u8; 0x0c],
    #[doc = "0x80..0xb4 - register cluster"]
    pub ch1: CH1,
    _reserved13: [u8; 0x0c],
    #[doc = "0xc0..0xf4 - register cluster"]
    pub ch2: CH2,
    _reserved14: [u8; 0x0c],
    #[doc = "0x100..0x134 - register cluster"]
    pub ch3: CH3,
}
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "EN register"]
pub mod en;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IM (rw) register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "IM register"]
pub mod im;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "DSTSGIE (rw) register accessor: an alias for `Reg<DSTSGIE_SPEC>`"]
pub type DSTSGIE = crate::Reg<dstsgie::DSTSGIE_SPEC>;
#[doc = "DSTSGIE register"]
pub mod dstsgie;
#[doc = "DSTSGIM (rw) register accessor: an alias for `Reg<DSTSGIM_SPEC>`"]
pub type DSTSGIM = crate::Reg<dstsgim::DSTSGIM_SPEC>;
#[doc = "DSTSGIM register"]
pub mod dstsgim;
#[doc = "DSTSGIF (rw) register accessor: an alias for `Reg<DSTSGIF_SPEC>`"]
pub type DSTSGIF = crate::Reg<dstsgif::DSTSGIF_SPEC>;
#[doc = "DSTSGIF register"]
pub mod dstsgif;
#[doc = "SRCSGIE (rw) register accessor: an alias for `Reg<SRCSGIE_SPEC>`"]
pub type SRCSGIE = crate::Reg<srcsgie::SRCSGIE_SPEC>;
#[doc = "SRCSGIE register"]
pub mod srcsgie;
#[doc = "SRCSGIM (rw) register accessor: an alias for `Reg<SRCSGIM_SPEC>`"]
pub type SRCSGIM = crate::Reg<srcsgim::SRCSGIM_SPEC>;
#[doc = "SRCSGIM register"]
pub mod srcsgim;
#[doc = "SRCSGIF (rw) register accessor: an alias for `Reg<SRCSGIF_SPEC>`"]
pub type SRCSGIF = crate::Reg<srcsgif::SRCSGIF_SPEC>;
#[doc = "SRCSGIF register"]
pub mod srcsgif;
#[doc = "PRI (rw) register accessor: an alias for `Reg<PRI_SPEC>`"]
pub type PRI = crate::Reg<pri::PRI_SPEC>;
#[doc = "PRI register"]
pub mod pri;
#[doc = "register cluster"]
pub use ch0::CH0;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod ch0;
#[doc = "register cluster"]
pub use ch1::CH1;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod ch1;
#[doc = "register cluster"]
pub use ch2::CH2;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod ch2;
#[doc = "register cluster"]
pub use ch3::CH3;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod ch3;
