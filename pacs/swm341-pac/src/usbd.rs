#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x400 - TXBUF0_ register"]
    pub txbuf0_: [TXBUF0_; 256],
    #[doc = "0x400..0x800 - TXBUF1_ register"]
    pub txbuf1_: [TXBUF1_; 256],
    #[doc = "0x800..0xc00 - TXBUF2_ register"]
    pub txbuf2_: [TXBUF2_; 256],
    #[doc = "0xc00..0x1000 - TXBUF3_ register"]
    pub txbuf3_: [TXBUF3_; 256],
    #[doc = "0x1000..0x1400 - TXBUF4_ register"]
    pub txbuf4_: [TXBUF4_; 256],
    #[doc = "0x1400..0x1800 - TXBUF5_ register"]
    pub txbuf5_: [TXBUF5_; 256],
    #[doc = "0x1800..0x1c00 - TXBUF6_ register"]
    pub txbuf6_: [TXBUF6_; 256],
    #[doc = "0x1c00..0x2000 - TXBUF7_ register"]
    pub txbuf7_: [TXBUF7_; 256],
    #[doc = "0x2000..0x2400 - TXBUF8_ register"]
    pub txbuf8_: [TXBUF8_; 256],
    #[doc = "0x2400..0x2800 - TXBUF9_ register"]
    pub txbuf9_: [TXBUF9_; 256],
    #[doc = "0x2800..0x2c00 - TXBUF10_ register"]
    pub txbuf10_: [TXBUF10_; 256],
    #[doc = "0x2c00..0x3000 - TXBUF11_ register"]
    pub txbuf11_: [TXBUF11_; 256],
    #[doc = "0x3000..0x3400 - TXBUF12_ register"]
    pub txbuf12_: [TXBUF12_; 256],
    #[doc = "0x3400..0x3800 - TXBUF13_ register"]
    pub txbuf13_: [TXBUF13_; 256],
    #[doc = "0x3800..0x3c00 - TXBUF14_ register"]
    pub txbuf14_: [TXBUF14_; 256],
    #[doc = "0x3c00..0x4000 - TXBUF15_ register"]
    pub txbuf15_: [TXBUF15_; 256],
    #[doc = "0x4000..0x4400 - RXBUF register"]
    pub rxbuf: [RXBUF; 256],
    #[doc = "0x4400..0x4410 - register cluster"]
    pub inep0: INEP0,
    #[doc = "0x4410..0x4420 - register cluster"]
    pub inep1: INEP1,
    #[doc = "0x4420..0x4430 - register cluster"]
    pub inep2: INEP2,
    #[doc = "0x4430..0x4440 - register cluster"]
    pub inep3: INEP3,
    #[doc = "0x4440..0x4450 - register cluster"]
    pub inep4: INEP4,
    #[doc = "0x4450..0x4460 - register cluster"]
    pub inep5: INEP5,
    #[doc = "0x4460..0x4470 - register cluster"]
    pub inep6: INEP6,
    #[doc = "0x4470..0x4480 - register cluster"]
    pub inep7: INEP7,
    #[doc = "0x4480..0x4490 - register cluster"]
    pub inep8: INEP8,
    #[doc = "0x4490..0x44a0 - register cluster"]
    pub inep9: INEP9,
    #[doc = "0x44a0..0x44b0 - register cluster"]
    pub inep10: INEP10,
    #[doc = "0x44b0..0x44c0 - register cluster"]
    pub inep11: INEP11,
    #[doc = "0x44c0..0x44d0 - register cluster"]
    pub inep12: INEP12,
    #[doc = "0x44d0..0x44e0 - register cluster"]
    pub inep13: INEP13,
    #[doc = "0x44e0..0x44f0 - register cluster"]
    pub inep14: INEP14,
    #[doc = "0x44f0..0x4500 - register cluster"]
    pub inep15: INEP15,
    #[doc = "0x4500 - register cluster"]
    pub outep0: OUTEP0,
    _reserved34: [u8; 0x0c],
    #[doc = "0x4510 - register cluster"]
    pub outep1: OUTEP1,
    _reserved35: [u8; 0x0c],
    #[doc = "0x4520 - register cluster"]
    pub outep2: OUTEP2,
    _reserved36: [u8; 0x0c],
    #[doc = "0x4530 - register cluster"]
    pub outep3: OUTEP3,
    _reserved37: [u8; 0x0c],
    #[doc = "0x4540 - register cluster"]
    pub outep4: OUTEP4,
    _reserved38: [u8; 0x0c],
    #[doc = "0x4550 - register cluster"]
    pub outep5: OUTEP5,
    _reserved39: [u8; 0x0c],
    #[doc = "0x4560 - register cluster"]
    pub outep6: OUTEP6,
    _reserved40: [u8; 0x0c],
    #[doc = "0x4570 - register cluster"]
    pub outep7: OUTEP7,
    _reserved41: [u8; 0x0c],
    #[doc = "0x4580 - register cluster"]
    pub outep8: OUTEP8,
    _reserved42: [u8; 0x0c],
    #[doc = "0x4590 - register cluster"]
    pub outep9: OUTEP9,
    _reserved43: [u8; 0x0c],
    #[doc = "0x45a0 - register cluster"]
    pub outep10: OUTEP10,
    _reserved44: [u8; 0x0c],
    #[doc = "0x45b0 - register cluster"]
    pub outep11: OUTEP11,
    _reserved45: [u8; 0x0c],
    #[doc = "0x45c0 - register cluster"]
    pub outep12: OUTEP12,
    _reserved46: [u8; 0x0c],
    #[doc = "0x45d0 - register cluster"]
    pub outep13: OUTEP13,
    _reserved47: [u8; 0x0c],
    #[doc = "0x45e0 - register cluster"]
    pub outep14: OUTEP14,
    _reserved48: [u8; 0x0c],
    #[doc = "0x45f0 - register cluster"]
    pub outep15: OUTEP15,
    _reserved49: [u8; 0x0c],
    #[doc = "0x4600 - DEVCR register"]
    pub devcr: DEVCR,
    #[doc = "0x4604 - DEVSR register"]
    pub devsr: DEVSR,
    #[doc = "0x4608 - DEVIF register"]
    pub devif: DEVIF,
    #[doc = "0x460c - DEVIE register"]
    pub devie: DEVIE,
    _reserved53: [u8; 0x04],
    #[doc = "0x4614 - EPIE register"]
    pub epie: EPIE,
    #[doc = "0x4618 - FFTHR register"]
    pub ffthr: FFTHR,
    #[doc = "0x461c - RXSR register"]
    pub rxsr: RXSR,
    #[doc = "0x4620 - SETUPSR register"]
    pub setupsr: SETUPSR,
    #[doc = "0x4624 - EPIF register"]
    pub epif: EPIF,
    #[doc = "0x4628 - FRAMENR register"]
    pub framenr: FRAMENR,
    _reserved59: [u8; 0xd4],
    #[doc = "0x4700 - SETUPD1 register"]
    pub setupd1: SETUPD1,
    #[doc = "0x4704 - SETUPD2 register"]
    pub setupd2: SETUPD2,
    _reserved61: [u8; 0xfc],
    #[doc = "0x4804..0x4844 - EPCFG register"]
    pub epcfg: [EPCFG; 16],
}
#[doc = "TXBUF0_ (rw) register accessor: an alias for `Reg<TXBUF0__SPEC>`"]
pub type TXBUF0_ = crate::Reg<txbuf0_::TXBUF0__SPEC>;
#[doc = "TXBUF0_ register"]
pub mod txbuf0_;
#[doc = "TXBUF1_ (rw) register accessor: an alias for `Reg<TXBUF1__SPEC>`"]
pub type TXBUF1_ = crate::Reg<txbuf1_::TXBUF1__SPEC>;
#[doc = "TXBUF1_ register"]
pub mod txbuf1_;
#[doc = "TXBUF2_ (rw) register accessor: an alias for `Reg<TXBUF2__SPEC>`"]
pub type TXBUF2_ = crate::Reg<txbuf2_::TXBUF2__SPEC>;
#[doc = "TXBUF2_ register"]
pub mod txbuf2_;
#[doc = "TXBUF3_ (rw) register accessor: an alias for `Reg<TXBUF3__SPEC>`"]
pub type TXBUF3_ = crate::Reg<txbuf3_::TXBUF3__SPEC>;
#[doc = "TXBUF3_ register"]
pub mod txbuf3_;
#[doc = "TXBUF4_ (rw) register accessor: an alias for `Reg<TXBUF4__SPEC>`"]
pub type TXBUF4_ = crate::Reg<txbuf4_::TXBUF4__SPEC>;
#[doc = "TXBUF4_ register"]
pub mod txbuf4_;
#[doc = "TXBUF5_ (rw) register accessor: an alias for `Reg<TXBUF5__SPEC>`"]
pub type TXBUF5_ = crate::Reg<txbuf5_::TXBUF5__SPEC>;
#[doc = "TXBUF5_ register"]
pub mod txbuf5_;
#[doc = "TXBUF6_ (rw) register accessor: an alias for `Reg<TXBUF6__SPEC>`"]
pub type TXBUF6_ = crate::Reg<txbuf6_::TXBUF6__SPEC>;
#[doc = "TXBUF6_ register"]
pub mod txbuf6_;
#[doc = "TXBUF7_ (rw) register accessor: an alias for `Reg<TXBUF7__SPEC>`"]
pub type TXBUF7_ = crate::Reg<txbuf7_::TXBUF7__SPEC>;
#[doc = "TXBUF7_ register"]
pub mod txbuf7_;
#[doc = "TXBUF8_ (rw) register accessor: an alias for `Reg<TXBUF8__SPEC>`"]
pub type TXBUF8_ = crate::Reg<txbuf8_::TXBUF8__SPEC>;
#[doc = "TXBUF8_ register"]
pub mod txbuf8_;
#[doc = "TXBUF9_ (rw) register accessor: an alias for `Reg<TXBUF9__SPEC>`"]
pub type TXBUF9_ = crate::Reg<txbuf9_::TXBUF9__SPEC>;
#[doc = "TXBUF9_ register"]
pub mod txbuf9_;
#[doc = "TXBUF10_ (rw) register accessor: an alias for `Reg<TXBUF10__SPEC>`"]
pub type TXBUF10_ = crate::Reg<txbuf10_::TXBUF10__SPEC>;
#[doc = "TXBUF10_ register"]
pub mod txbuf10_;
#[doc = "TXBUF11_ (rw) register accessor: an alias for `Reg<TXBUF11__SPEC>`"]
pub type TXBUF11_ = crate::Reg<txbuf11_::TXBUF11__SPEC>;
#[doc = "TXBUF11_ register"]
pub mod txbuf11_;
#[doc = "TXBUF12_ (rw) register accessor: an alias for `Reg<TXBUF12__SPEC>`"]
pub type TXBUF12_ = crate::Reg<txbuf12_::TXBUF12__SPEC>;
#[doc = "TXBUF12_ register"]
pub mod txbuf12_;
#[doc = "TXBUF13_ (rw) register accessor: an alias for `Reg<TXBUF13__SPEC>`"]
pub type TXBUF13_ = crate::Reg<txbuf13_::TXBUF13__SPEC>;
#[doc = "TXBUF13_ register"]
pub mod txbuf13_;
#[doc = "TXBUF14_ (rw) register accessor: an alias for `Reg<TXBUF14__SPEC>`"]
pub type TXBUF14_ = crate::Reg<txbuf14_::TXBUF14__SPEC>;
#[doc = "TXBUF14_ register"]
pub mod txbuf14_;
#[doc = "TXBUF15_ (rw) register accessor: an alias for `Reg<TXBUF15__SPEC>`"]
pub type TXBUF15_ = crate::Reg<txbuf15_::TXBUF15__SPEC>;
#[doc = "TXBUF15_ register"]
pub mod txbuf15_;
#[doc = "RXBUF (rw) register accessor: an alias for `Reg<RXBUF_SPEC>`"]
pub type RXBUF = crate::Reg<rxbuf::RXBUF_SPEC>;
#[doc = "RXBUF register"]
pub mod rxbuf;
#[doc = "register cluster"]
pub use inep0::INEP0;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep0;
#[doc = "register cluster"]
pub use inep1::INEP1;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep1;
#[doc = "register cluster"]
pub use inep2::INEP2;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep2;
#[doc = "register cluster"]
pub use inep3::INEP3;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep3;
#[doc = "register cluster"]
pub use inep4::INEP4;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep4;
#[doc = "register cluster"]
pub use inep5::INEP5;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep5;
#[doc = "register cluster"]
pub use inep6::INEP6;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep6;
#[doc = "register cluster"]
pub use inep7::INEP7;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep7;
#[doc = "register cluster"]
pub use inep8::INEP8;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep8;
#[doc = "register cluster"]
pub use inep9::INEP9;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep9;
#[doc = "register cluster"]
pub use inep10::INEP10;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep10;
#[doc = "register cluster"]
pub use inep11::INEP11;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep11;
#[doc = "register cluster"]
pub use inep12::INEP12;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep12;
#[doc = "register cluster"]
pub use inep13::INEP13;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep13;
#[doc = "register cluster"]
pub use inep14::INEP14;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep14;
#[doc = "register cluster"]
pub use inep15::INEP15;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod inep15;
#[doc = "register cluster"]
pub use outep0::OUTEP0;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep0;
#[doc = "register cluster"]
pub use outep1::OUTEP1;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep1;
#[doc = "register cluster"]
pub use outep2::OUTEP2;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep2;
#[doc = "register cluster"]
pub use outep3::OUTEP3;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep3;
#[doc = "register cluster"]
pub use outep4::OUTEP4;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep4;
#[doc = "register cluster"]
pub use outep5::OUTEP5;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep5;
#[doc = "register cluster"]
pub use outep6::OUTEP6;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep6;
#[doc = "register cluster"]
pub use outep7::OUTEP7;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep7;
#[doc = "register cluster"]
pub use outep8::OUTEP8;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep8;
#[doc = "register cluster"]
pub use outep9::OUTEP9;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep9;
#[doc = "register cluster"]
pub use outep10::OUTEP10;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep10;
#[doc = "register cluster"]
pub use outep11::OUTEP11;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep11;
#[doc = "register cluster"]
pub use outep12::OUTEP12;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep12;
#[doc = "register cluster"]
pub use outep13::OUTEP13;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep13;
#[doc = "register cluster"]
pub use outep14::OUTEP14;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep14;
#[doc = "register cluster"]
pub use outep15::OUTEP15;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod outep15;
#[doc = "DEVCR (rw) register accessor: an alias for `Reg<DEVCR_SPEC>`"]
pub type DEVCR = crate::Reg<devcr::DEVCR_SPEC>;
#[doc = "DEVCR register"]
pub mod devcr;
#[doc = "DEVSR (r) register accessor: an alias for `Reg<DEVSR_SPEC>`"]
pub type DEVSR = crate::Reg<devsr::DEVSR_SPEC>;
#[doc = "DEVSR register"]
pub mod devsr;
#[doc = "DEVIF (rw) register accessor: an alias for `Reg<DEVIF_SPEC>`"]
pub type DEVIF = crate::Reg<devif::DEVIF_SPEC>;
#[doc = "DEVIF register"]
pub mod devif;
#[doc = "DEVIE (rw) register accessor: an alias for `Reg<DEVIE_SPEC>`"]
pub type DEVIE = crate::Reg<devie::DEVIE_SPEC>;
#[doc = "DEVIE register"]
pub mod devie;
#[doc = "EPIE (rw) register accessor: an alias for `Reg<EPIE_SPEC>`"]
pub type EPIE = crate::Reg<epie::EPIE_SPEC>;
#[doc = "EPIE register"]
pub mod epie;
#[doc = "FFTHR (rw) register accessor: an alias for `Reg<FFTHR_SPEC>`"]
pub type FFTHR = crate::Reg<ffthr::FFTHR_SPEC>;
#[doc = "FFTHR register"]
pub mod ffthr;
#[doc = "RXSR (rw) register accessor: an alias for `Reg<RXSR_SPEC>`"]
pub type RXSR = crate::Reg<rxsr::RXSR_SPEC>;
#[doc = "RXSR register"]
pub mod rxsr;
#[doc = "SETUPSR (rw) register accessor: an alias for `Reg<SETUPSR_SPEC>`"]
pub type SETUPSR = crate::Reg<setupsr::SETUPSR_SPEC>;
#[doc = "SETUPSR register"]
pub mod setupsr;
#[doc = "EPIF (rw) register accessor: an alias for `Reg<EPIF_SPEC>`"]
pub type EPIF = crate::Reg<epif::EPIF_SPEC>;
#[doc = "EPIF register"]
pub mod epif;
#[doc = "FRAMENR (rw) register accessor: an alias for `Reg<FRAMENR_SPEC>`"]
pub type FRAMENR = crate::Reg<framenr::FRAMENR_SPEC>;
#[doc = "FRAMENR register"]
pub mod framenr;
#[doc = "SETUPD1 (rw) register accessor: an alias for `Reg<SETUPD1_SPEC>`"]
pub type SETUPD1 = crate::Reg<setupd1::SETUPD1_SPEC>;
#[doc = "SETUPD1 register"]
pub mod setupd1;
#[doc = "SETUPD2 (rw) register accessor: an alias for `Reg<SETUPD2_SPEC>`"]
pub type SETUPD2 = crate::Reg<setupd2::SETUPD2_SPEC>;
#[doc = "SETUPD2 register"]
pub mod setupd2;
#[doc = "EPCFG (rw) register accessor: an alias for `Reg<EPCFG_SPEC>`"]
pub type EPCFG = crate::Reg<epcfg::EPCFG_SPEC>;
#[doc = "EPCFG register"]
pub mod epcfg;
