#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR register"]
    pub cr: CR,
    #[doc = "0x04 - GO register"]
    pub go: GO,
    #[doc = "0x08 - IE register"]
    pub ie: IE,
    #[doc = "0x0c - IF register"]
    pub if_: IF,
    #[doc = "0x10..0x1c - register cluster"]
    pub seq0: SEQ0,
    _reserved5: [u8; 0x04],
    #[doc = "0x20..0x2c - register cluster"]
    pub seq1: SEQ1,
    _reserved6: [u8; 0x04],
    #[doc = "0x30..0x3c - register cluster"]
    pub seq2: SEQ2,
    _reserved7: [u8; 0x04],
    #[doc = "0x40..0x4c - register cluster"]
    pub seq3: SEQ3,
    _reserved8: [u8; 0x40],
    #[doc = "0x8c - SEQCHN0 register"]
    pub seqchn0: SEQCHN0,
    #[doc = "0x90 - SEQCHN1 register"]
    pub seqchn1: SEQCHN1,
    #[doc = "0x94 - SEQTRG register"]
    pub seqtrg: SEQTRG,
    #[doc = "0x98 - SEQCOV register"]
    pub seqcov: SEQCOV,
    #[doc = "0x9c - SEQSMP register"]
    pub seqsmp: SEQSMP,
    _reserved13: [u8; 0x10],
    #[doc = "0xb0 - CR2 register"]
    pub cr2: CR2,
    _reserved14: [u8; 0x3c],
    #[doc = "0xf0 - CALIB register"]
    pub calib: CALIB,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "GO (rw) register accessor: an alias for `Reg<GO_SPEC>`"]
pub type GO = crate::Reg<go::GO_SPEC>;
#[doc = "GO register"]
pub mod go;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "register cluster"]
pub use seq0::SEQ0;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod seq0;
#[doc = "register cluster"]
pub use seq1::SEQ1;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod seq1;
#[doc = "register cluster"]
pub use seq2::SEQ2;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod seq2;
#[doc = "register cluster"]
pub use seq3::SEQ3;
#[doc = r"Cluster"]
#[doc = "register cluster"]
pub mod seq3;
#[doc = "SEQCHN0 (rw) register accessor: an alias for `Reg<SEQCHN0_SPEC>`"]
pub type SEQCHN0 = crate::Reg<seqchn0::SEQCHN0_SPEC>;
#[doc = "SEQCHN0 register"]
pub mod seqchn0;
#[doc = "SEQCHN1 (rw) register accessor: an alias for `Reg<SEQCHN1_SPEC>`"]
pub type SEQCHN1 = crate::Reg<seqchn1::SEQCHN1_SPEC>;
#[doc = "SEQCHN1 register"]
pub mod seqchn1;
#[doc = "SEQTRG (rw) register accessor: an alias for `Reg<SEQTRG_SPEC>`"]
pub type SEQTRG = crate::Reg<seqtrg::SEQTRG_SPEC>;
#[doc = "SEQTRG register"]
pub mod seqtrg;
#[doc = "SEQCOV (rw) register accessor: an alias for `Reg<SEQCOV_SPEC>`"]
pub type SEQCOV = crate::Reg<seqcov::SEQCOV_SPEC>;
#[doc = "SEQCOV register"]
pub mod seqcov;
#[doc = "SEQSMP (rw) register accessor: an alias for `Reg<SEQSMP_SPEC>`"]
pub type SEQSMP = crate::Reg<seqsmp::SEQSMP_SPEC>;
#[doc = "SEQSMP register"]
pub mod seqsmp;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "CR2 register"]
pub mod cr2;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "CALIB register"]
pub mod calib;
