#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFG register"]
    pub cfg: CFG,
    #[doc = "0x04 - CR register"]
    pub cr: CR,
    #[doc = "0x08 - IR register"]
    pub ir: IR,
    #[doc = "0x0c - SR register"]
    pub sr: SR,
    #[doc = "0x10 - IMGSIZ register"]
    pub imgsiz: IMGSIZ,
    #[doc = "0x14 - IMGSTR register"]
    pub imgstr: IMGSTR,
    #[doc = "0x18 - CSBASE register"]
    pub csbase: CSBASE,
    _reserved_7_ybase: [u8; 0x04],
    #[doc = "0x20 - UBASE register"]
    pub ubase: UBASE,
    #[doc = "0x24 - VBASE register"]
    pub vbase: VBASE,
    #[doc = "0x28 - QTBASE register"]
    pub qtbase: QTBASE,
    #[doc = "0x2c - HTBASE register"]
    pub htbase: HTBASE,
    #[doc = "0x30 - CODLEN register"]
    pub codlen: CODLEN,
    _reserved13: [u8; 0xcc],
    #[doc = "0x100..0x140 - QTABLE0_ register"]
    pub qtable0_: [QTABLE0_; 16],
    #[doc = "0x140..0x180 - QTABLE1_ register"]
    pub qtable1_: [QTABLE1_; 16],
    #[doc = "0x180..0x1c0 - QTABLE2_ register"]
    pub qtable2_: [QTABLE2_; 16],
    _reserved16: [u8; 0x40],
    #[doc = "0x200..0x2fc - register cluster"]
    pub htable0: HTABLE0,
    _reserved17: [u8; 0x04],
    #[doc = "0x300..0x3fc - register cluster"]
    pub htable1: HTABLE1,
}
impl RegisterBlock {
    #[doc = "0x1c - RGBASE register"]
    #[inline(always)]
    pub fn rgbase(&self) -> &RGBASE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const RGBASE) }
    }
    #[doc = "0x1c - YBASE register"]
    #[inline(always)]
    pub fn ybase(&self) -> &YBASE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const YBASE) }
    }
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "CFG register"]
pub mod cfg;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "IR register"]
pub mod ir;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "IMGSIZ (rw) register accessor: an alias for `Reg<IMGSIZ_SPEC>`"]
pub type IMGSIZ = crate::Reg<imgsiz::IMGSIZ_SPEC>;
#[doc = "IMGSIZ register"]
pub mod imgsiz;
#[doc = "IMGSTR (rw) register accessor: an alias for `Reg<IMGSTR_SPEC>`"]
pub type IMGSTR = crate::Reg<imgstr::IMGSTR_SPEC>;
#[doc = "IMGSTR register"]
pub mod imgstr;
#[doc = "CSBASE (rw) register accessor: an alias for `Reg<CSBASE_SPEC>`"]
pub type CSBASE = crate::Reg<csbase::CSBASE_SPEC>;
#[doc = "CSBASE register"]
pub mod csbase;
#[doc = "YBASE (rw) register accessor: an alias for `Reg<YBASE_SPEC>`"]
pub type YBASE = crate::Reg<ybase::YBASE_SPEC>;
#[doc = "YBASE register"]
pub mod ybase;
#[doc = "RGBASE (rw) register accessor: an alias for `Reg<RGBASE_SPEC>`"]
pub type RGBASE = crate::Reg<rgbase::RGBASE_SPEC>;
#[doc = "RGBASE register"]
pub mod rgbase;
#[doc = "UBASE (rw) register accessor: an alias for `Reg<UBASE_SPEC>`"]
pub type UBASE = crate::Reg<ubase::UBASE_SPEC>;
#[doc = "UBASE register"]
pub mod ubase;
#[doc = "VBASE (rw) register accessor: an alias for `Reg<VBASE_SPEC>`"]
pub type VBASE = crate::Reg<vbase::VBASE_SPEC>;
#[doc = "VBASE register"]
pub mod vbase;
#[doc = "QTBASE (rw) register accessor: an alias for `Reg<QTBASE_SPEC>`"]
pub type QTBASE = crate::Reg<qtbase::QTBASE_SPEC>;
#[doc = "QTBASE register"]
pub mod qtbase;
#[doc = "HTBASE (rw) register accessor: an alias for `Reg<HTBASE_SPEC>`"]
pub type HTBASE = crate::Reg<htbase::HTBASE_SPEC>;
#[doc = "HTBASE register"]
pub mod htbase;
#[doc = "CODLEN (rw) register accessor: an alias for `Reg<CODLEN_SPEC>`"]
pub type CODLEN = crate::Reg<codlen::CODLEN_SPEC>;
#[doc = "CODLEN register"]
pub mod codlen;
#[doc = "QTABLE0_ (w) register accessor: an alias for `Reg<QTABLE0__SPEC>`"]
pub type QTABLE0_ = crate::Reg<qtable0_::QTABLE0__SPEC>;
#[doc = "QTABLE0_ register"]
pub mod qtable0_;
#[doc = "QTABLE1_ (w) register accessor: an alias for `Reg<QTABLE1__SPEC>`"]
pub type QTABLE1_ = crate::Reg<qtable1_::QTABLE1__SPEC>;
#[doc = "QTABLE1_ register"]
pub mod qtable1_;
#[doc = "QTABLE2_ (w) register accessor: an alias for `Reg<QTABLE2__SPEC>`"]
pub type QTABLE2_ = crate::Reg<qtable2_::QTABLE2__SPEC>;
#[doc = "QTABLE2_ register"]
pub mod qtable2_;
#[doc = "register cluster"]
pub use htable0::HTABLE0;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod htable0;
#[doc = "register cluster"]
pub use htable1::HTABLE1;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod htable1;
