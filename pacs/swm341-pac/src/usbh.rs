#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x400 - TXBUF register"]
    pub txbuf: [TXBUF; 256],
    _reserved1: [u8; 0x3c00],
    #[doc = "0x4000..0x4400 - RXBUF register"]
    pub rxbuf: [RXBUF; 256],
    _reserved2: [u8; 0x0800],
    #[doc = "0x4c00 - IF register"]
    pub if_: IF,
    #[doc = "0x4c04 - IE register"]
    pub ie: IE,
    #[doc = "0x4c08 - SR register"]
    pub sr: SR,
    #[doc = "0x4c0c - CR register"]
    pub cr: CR,
    #[doc = "0x4c10 - FFSZ register"]
    pub ffsz: FFSZ,
    #[doc = "0x4c14 - FFTHR register"]
    pub ffthr: FFTHR,
    #[doc = "0x4c18 - TXTRSZ register"]
    pub txtrsz: TXTRSZ,
    _reserved9: [u8; 0x0418],
    #[doc = "0x5034 - FRAMEIV register"]
    pub frameiv: FRAMEIV,
    #[doc = "0x5038 - FRAMERM register"]
    pub framerm: FRAMERM,
    #[doc = "0x503c - FRAMENR register"]
    pub framenr: FRAMENR,
    _reserved12: [u8; 0x14],
    #[doc = "0x5054 - PORTSR register"]
    pub portsr: PORTSR,
    _reserved13: [u8; 0x38],
    #[doc = "0x5090 - TOKEN register"]
    pub token: TOKEN,
    #[doc = "0x5094 - OTGCSR register"]
    pub otgcsr: OTGCSR,
}
#[doc = "TXBUF (rw) register accessor: an alias for `Reg<TXBUF_SPEC>`"]
pub type TXBUF = crate::Reg<txbuf::TXBUF_SPEC>;
#[doc = "TXBUF register"]
pub mod txbuf;
#[doc = "RXBUF (rw) register accessor: an alias for `Reg<RXBUF_SPEC>`"]
pub type RXBUF = crate::Reg<rxbuf::RXBUF_SPEC>;
#[doc = "RXBUF register"]
pub mod rxbuf;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR register"]
pub mod cr;
#[doc = "FFSZ (rw) register accessor: an alias for `Reg<FFSZ_SPEC>`"]
pub type FFSZ = crate::Reg<ffsz::FFSZ_SPEC>;
#[doc = "FFSZ register"]
pub mod ffsz;
#[doc = "FFTHR (rw) register accessor: an alias for `Reg<FFTHR_SPEC>`"]
pub type FFTHR = crate::Reg<ffthr::FFTHR_SPEC>;
#[doc = "FFTHR register"]
pub mod ffthr;
#[doc = "TXTRSZ (rw) register accessor: an alias for `Reg<TXTRSZ_SPEC>`"]
pub type TXTRSZ = crate::Reg<txtrsz::TXTRSZ_SPEC>;
#[doc = "TXTRSZ register"]
pub mod txtrsz;
#[doc = "FRAMEIV (rw) register accessor: an alias for `Reg<FRAMEIV_SPEC>`"]
pub type FRAMEIV = crate::Reg<frameiv::FRAMEIV_SPEC>;
#[doc = "FRAMEIV register"]
pub mod frameiv;
#[doc = "FRAMERM (rw) register accessor: an alias for `Reg<FRAMERM_SPEC>`"]
pub type FRAMERM = crate::Reg<framerm::FRAMERM_SPEC>;
#[doc = "FRAMERM register"]
pub mod framerm;
#[doc = "FRAMENR (rw) register accessor: an alias for `Reg<FRAMENR_SPEC>`"]
pub type FRAMENR = crate::Reg<framenr::FRAMENR_SPEC>;
#[doc = "FRAMENR register"]
pub mod framenr;
#[doc = "PORTSR (rw) register accessor: an alias for `Reg<PORTSR_SPEC>`"]
pub type PORTSR = crate::Reg<portsr::PORTSR_SPEC>;
#[doc = "PORTSR register"]
pub mod portsr;
#[doc = "TOKEN (rw) register accessor: an alias for `Reg<TOKEN_SPEC>`"]
pub type TOKEN = crate::Reg<token::TOKEN_SPEC>;
#[doc = "TOKEN register"]
pub mod token;
#[doc = "OTGCSR (rw) register accessor: an alias for `Reg<OTGCSR_SPEC>`"]
pub type OTGCSR = crate::Reg<otgcsr::OTGCSR_SPEC>;
#[doc = "OTGCSR register"]
pub mod otgcsr;
