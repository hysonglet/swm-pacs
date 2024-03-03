#[doc = r"Register block"]
#[repr(C)]
pub struct HTABLE0 {
    #[doc = "0x00..0x18 - DC_CODEWORD register"]
    pub dc_codeword: [DC_CODEWORD; 6],
    #[doc = "0x18..0x20 - DC_CODELEN register"]
    pub dc_codelen: [DC_CODELEN; 2],
    #[doc = "0x20..0x28 - DC_CODEVAL register"]
    pub dc_codeval: [DC_CODEVAL; 2],
    #[doc = "0x28..0x48 - AC_CODEWORD register"]
    pub ac_codeword: [AC_CODEWORD; 8],
    #[doc = "0x48..0x58 - AC_CODEADDR register"]
    pub ac_codeaddr: [AC_CODEADDR; 4],
    #[doc = "0x58..0xfc - AC_CODEVAL register"]
    pub ac_codeval: [AC_CODEVAL; 41],
}
#[doc = "DC_CODEWORD (w) register accessor: an alias for `Reg<DC_CODEWORD_SPEC>`"]
pub type DC_CODEWORD = crate::Reg<dc_codeword::DC_CODEWORD_SPEC>;
#[doc = "DC_CODEWORD register"]
pub mod dc_codeword;
#[doc = "DC_CODELEN (w) register accessor: an alias for `Reg<DC_CODELEN_SPEC>`"]
pub type DC_CODELEN = crate::Reg<dc_codelen::DC_CODELEN_SPEC>;
#[doc = "DC_CODELEN register"]
pub mod dc_codelen;
#[doc = "DC_CODEVAL (w) register accessor: an alias for `Reg<DC_CODEVAL_SPEC>`"]
pub type DC_CODEVAL = crate::Reg<dc_codeval::DC_CODEVAL_SPEC>;
#[doc = "DC_CODEVAL register"]
pub mod dc_codeval;
#[doc = "AC_CODEWORD (w) register accessor: an alias for `Reg<AC_CODEWORD_SPEC>`"]
pub type AC_CODEWORD = crate::Reg<ac_codeword::AC_CODEWORD_SPEC>;
#[doc = "AC_CODEWORD register"]
pub mod ac_codeword;
#[doc = "AC_CODEADDR (w) register accessor: an alias for `Reg<AC_CODEADDR_SPEC>`"]
pub type AC_CODEADDR = crate::Reg<ac_codeaddr::AC_CODEADDR_SPEC>;
#[doc = "AC_CODEADDR register"]
pub mod ac_codeaddr;
#[doc = "AC_CODEVAL (w) register accessor: an alias for `Reg<AC_CODEVAL_SPEC>`"]
pub type AC_CODEVAL = crate::Reg<ac_codeval::AC_CODEVAL_SPEC>;
#[doc = "AC_CODEVAL register"]
pub mod ac_codeval;
