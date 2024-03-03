#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CLKSEL register"]
    pub clksel: CLKSEL,
    #[doc = "0x04 - CLKDIVx_ON register"]
    pub clkdivx_on: CLKDIVX_ON,
    #[doc = "0x08 - CLKEN0 register"]
    pub clken0: CLKEN0,
    #[doc = "0x0c - CLKEN1 register"]
    pub clken1: CLKEN1,
    #[doc = "0x10 - SLEEP register"]
    pub sleep: SLEEP,
    _reserved5: [u8; 0x10],
    #[doc = "0x24 - RSTSR register"]
    pub rstsr: RSTSR,
    _reserved6: [u8; 0x08],
    #[doc = "0x30 - RTCWKCR register"]
    pub rtcwkcr: RTCWKCR,
    #[doc = "0x34 - RTCWKSR register"]
    pub rtcwksr: RTCWKSR,
    _reserved8: [u8; 0x38],
    #[doc = "0x70 - USBPHYCR register"]
    pub usbphycr: USBPHYCR,
    #[doc = "0x74 - USBCR register"]
    pub usbcr: USBCR,
    _reserved10: [u8; 0x08],
    #[doc = "0x80..0x90 - CHIPID register"]
    pub chipid: [CHIPID; 4],
    #[doc = "0x90..0xa0 - BACKUP register"]
    pub backup: [BACKUP; 4],
    _reserved12: [u8; 0x30],
    #[doc = "0xd0 - PRNGCR register"]
    pub prngcr: PRNGCR,
    #[doc = "0xd4 - PRNGDL register"]
    pub prngdl: PRNGDL,
    #[doc = "0xd8 - PRNGDH register"]
    pub prngdh: PRNGDH,
    _reserved15: [u8; 0x24],
    #[doc = "0x100 - PAWKEN register"]
    pub pawken: PAWKEN,
    #[doc = "0x104 - PBWKEN register"]
    pub pbwken: PBWKEN,
    #[doc = "0x108 - PCWKEN register"]
    pub pcwken: PCWKEN,
    #[doc = "0x10c - PDWKEN register"]
    pub pdwken: PDWKEN,
    #[doc = "0x110 - PEWKEN register"]
    pub pewken: PEWKEN,
    _reserved20: [u8; 0x0c],
    #[doc = "0x120 - PMWKEN register"]
    pub pmwken: PMWKEN,
    #[doc = "0x124 - PNWKEN register"]
    pub pnwken: PNWKEN,
    _reserved22: [u8; 0x08],
    #[doc = "0x130 - PAWKSR register"]
    pub pawksr: PAWKSR,
    #[doc = "0x134 - PBWKSR register"]
    pub pbwksr: PBWKSR,
    #[doc = "0x138 - PCWKSR register"]
    pub pcwksr: PCWKSR,
    #[doc = "0x13c - PDWKSR register"]
    pub pdwksr: PDWKSR,
    #[doc = "0x140 - PEWKSR register"]
    pub pewksr: PEWKSR,
    _reserved27: [u8; 0x0c],
    #[doc = "0x150 - PMWKSR register"]
    pub pmwksr: PMWKSR,
    #[doc = "0x154 - PNWKSR register"]
    pub pnwksr: PNWKSR,
    _reserved29: [u8; 0x02a8],
    #[doc = "0x400 - IOFILT0 register"]
    pub iofilt0: IOFILT0,
    #[doc = "0x404 - IOFILT1 register"]
    pub iofilt1: IOFILT1,
    _reserved31: [u8; 0x0318],
    #[doc = "0x720 - PRSTEN register"]
    pub prsten: PRSTEN,
    #[doc = "0x724 - PRSTR0 register"]
    pub prstr0: PRSTR0,
    #[doc = "0x728 - PRSTR1 register"]
    pub prstr1: PRSTR1,
    _reserved34: [u8; 0x000a_98d4],
    #[doc = "0xaa000 - HRCCR register"]
    pub hrccr: HRCCR,
    _reserved35: [u8; 0x0c],
    #[doc = "0xaa010 - BODCR register"]
    pub bodcr: BODCR,
    #[doc = "0xaa014 - BODSR register"]
    pub bodsr: BODSR,
    #[doc = "0xaa018 - ADCCR register"]
    pub adccr: ADCCR,
    _reserved38: [u8; 0x04],
    #[doc = "0xaa020 - XTALCR register"]
    pub xtalcr: XTALCR,
    #[doc = "0xaa024 - XTALSR register"]
    pub xtalsr: XTALSR,
    _reserved40: [u8; 0x18],
    #[doc = "0xaa040 - PLLCR register"]
    pub pllcr: PLLCR,
    #[doc = "0xaa044 - PLLDIV register"]
    pub plldiv: PLLDIV,
    _reserved42: [u8; 0x04],
    #[doc = "0xaa04c - PLLLOCK register"]
    pub plllock: PLLLOCK,
    #[doc = "0xaa050 - LRCCR register"]
    pub lrccr: LRCCR,
    _reserved44: [u8; 0x1c],
    #[doc = "0xaa070 - OPACR register"]
    pub opacr: OPACR,
    _reserved45: [u8; 0x0c],
    #[doc = "0xaa080 - ACMPCR register"]
    pub acmpcr: ACMPCR,
    #[doc = "0xaa084 - ACMPSR register"]
    pub acmpsr: ACMPSR,
    #[doc = "0xaa088 - ACMPCR2 register"]
    pub acmpcr2: ACMPCR2,
    _reserved48: [u8; 0x04],
    #[doc = "0xaa090 - DACCR register"]
    pub daccr: DACCR,
    _reserved49: [u8; 0x04],
    #[doc = "0xaa098 - TEMPCR register"]
    pub tempcr: TEMPCR,
}
#[doc = "CLKSEL (rw) register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "CLKSEL register"]
pub mod clksel;
#[doc = "CLKDIVx_ON (rw) register accessor: an alias for `Reg<CLKDIVX_ON_SPEC>`"]
pub type CLKDIVX_ON = crate::Reg<clkdivx_on::CLKDIVX_ON_SPEC>;
#[doc = "CLKDIVx_ON register"]
pub mod clkdivx_on;
#[doc = "CLKEN0 (rw) register accessor: an alias for `Reg<CLKEN0_SPEC>`"]
pub type CLKEN0 = crate::Reg<clken0::CLKEN0_SPEC>;
#[doc = "CLKEN0 register"]
pub mod clken0;
#[doc = "CLKEN1 (rw) register accessor: an alias for `Reg<CLKEN1_SPEC>`"]
pub type CLKEN1 = crate::Reg<clken1::CLKEN1_SPEC>;
#[doc = "CLKEN1 register"]
pub mod clken1;
#[doc = "SLEEP (rw) register accessor: an alias for `Reg<SLEEP_SPEC>`"]
pub type SLEEP = crate::Reg<sleep::SLEEP_SPEC>;
#[doc = "SLEEP register"]
pub mod sleep;
#[doc = "RSTSR (rw) register accessor: an alias for `Reg<RSTSR_SPEC>`"]
pub type RSTSR = crate::Reg<rstsr::RSTSR_SPEC>;
#[doc = "RSTSR register"]
pub mod rstsr;
#[doc = "RTCWKCR (rw) register accessor: an alias for `Reg<RTCWKCR_SPEC>`"]
pub type RTCWKCR = crate::Reg<rtcwkcr::RTCWKCR_SPEC>;
#[doc = "RTCWKCR register"]
pub mod rtcwkcr;
#[doc = "RTCWKSR (rw) register accessor: an alias for `Reg<RTCWKSR_SPEC>`"]
pub type RTCWKSR = crate::Reg<rtcwksr::RTCWKSR_SPEC>;
#[doc = "RTCWKSR register"]
pub mod rtcwksr;
#[doc = "USBPHYCR (rw) register accessor: an alias for `Reg<USBPHYCR_SPEC>`"]
pub type USBPHYCR = crate::Reg<usbphycr::USBPHYCR_SPEC>;
#[doc = "USBPHYCR register"]
pub mod usbphycr;
#[doc = "USBCR (rw) register accessor: an alias for `Reg<USBCR_SPEC>`"]
pub type USBCR = crate::Reg<usbcr::USBCR_SPEC>;
#[doc = "USBCR register"]
pub mod usbcr;
#[doc = "CHIPID (r) register accessor: an alias for `Reg<CHIPID_SPEC>`"]
pub type CHIPID = crate::Reg<chipid::CHIPID_SPEC>;
#[doc = "CHIPID register"]
pub mod chipid;
#[doc = "BACKUP (rw) register accessor: an alias for `Reg<BACKUP_SPEC>`"]
pub type BACKUP = crate::Reg<backup::BACKUP_SPEC>;
#[doc = "BACKUP register"]
pub mod backup;
#[doc = "PRNGCR (rw) register accessor: an alias for `Reg<PRNGCR_SPEC>`"]
pub type PRNGCR = crate::Reg<prngcr::PRNGCR_SPEC>;
#[doc = "PRNGCR register"]
pub mod prngcr;
#[doc = "PRNGDL (rw) register accessor: an alias for `Reg<PRNGDL_SPEC>`"]
pub type PRNGDL = crate::Reg<prngdl::PRNGDL_SPEC>;
#[doc = "PRNGDL register"]
pub mod prngdl;
#[doc = "PRNGDH (rw) register accessor: an alias for `Reg<PRNGDH_SPEC>`"]
pub type PRNGDH = crate::Reg<prngdh::PRNGDH_SPEC>;
#[doc = "PRNGDH register"]
pub mod prngdh;
#[doc = "PAWKEN (rw) register accessor: an alias for `Reg<PAWKEN_SPEC>`"]
pub type PAWKEN = crate::Reg<pawken::PAWKEN_SPEC>;
#[doc = "PAWKEN register"]
pub mod pawken;
#[doc = "PBWKEN (rw) register accessor: an alias for `Reg<PBWKEN_SPEC>`"]
pub type PBWKEN = crate::Reg<pbwken::PBWKEN_SPEC>;
#[doc = "PBWKEN register"]
pub mod pbwken;
#[doc = "PCWKEN (rw) register accessor: an alias for `Reg<PCWKEN_SPEC>`"]
pub type PCWKEN = crate::Reg<pcwken::PCWKEN_SPEC>;
#[doc = "PCWKEN register"]
pub mod pcwken;
#[doc = "PDWKEN (rw) register accessor: an alias for `Reg<PDWKEN_SPEC>`"]
pub type PDWKEN = crate::Reg<pdwken::PDWKEN_SPEC>;
#[doc = "PDWKEN register"]
pub mod pdwken;
#[doc = "PEWKEN (rw) register accessor: an alias for `Reg<PEWKEN_SPEC>`"]
pub type PEWKEN = crate::Reg<pewken::PEWKEN_SPEC>;
#[doc = "PEWKEN register"]
pub mod pewken;
#[doc = "PMWKEN (rw) register accessor: an alias for `Reg<PMWKEN_SPEC>`"]
pub type PMWKEN = crate::Reg<pmwken::PMWKEN_SPEC>;
#[doc = "PMWKEN register"]
pub mod pmwken;
#[doc = "PNWKEN (rw) register accessor: an alias for `Reg<PNWKEN_SPEC>`"]
pub type PNWKEN = crate::Reg<pnwken::PNWKEN_SPEC>;
#[doc = "PNWKEN register"]
pub mod pnwken;
#[doc = "PAWKSR (rw) register accessor: an alias for `Reg<PAWKSR_SPEC>`"]
pub type PAWKSR = crate::Reg<pawksr::PAWKSR_SPEC>;
#[doc = "PAWKSR register"]
pub mod pawksr;
#[doc = "PBWKSR (rw) register accessor: an alias for `Reg<PBWKSR_SPEC>`"]
pub type PBWKSR = crate::Reg<pbwksr::PBWKSR_SPEC>;
#[doc = "PBWKSR register"]
pub mod pbwksr;
#[doc = "PCWKSR (rw) register accessor: an alias for `Reg<PCWKSR_SPEC>`"]
pub type PCWKSR = crate::Reg<pcwksr::PCWKSR_SPEC>;
#[doc = "PCWKSR register"]
pub mod pcwksr;
#[doc = "PDWKSR (rw) register accessor: an alias for `Reg<PDWKSR_SPEC>`"]
pub type PDWKSR = crate::Reg<pdwksr::PDWKSR_SPEC>;
#[doc = "PDWKSR register"]
pub mod pdwksr;
#[doc = "PEWKSR (rw) register accessor: an alias for `Reg<PEWKSR_SPEC>`"]
pub type PEWKSR = crate::Reg<pewksr::PEWKSR_SPEC>;
#[doc = "PEWKSR register"]
pub mod pewksr;
#[doc = "PMWKSR (rw) register accessor: an alias for `Reg<PMWKSR_SPEC>`"]
pub type PMWKSR = crate::Reg<pmwksr::PMWKSR_SPEC>;
#[doc = "PMWKSR register"]
pub mod pmwksr;
#[doc = "PNWKSR (rw) register accessor: an alias for `Reg<PNWKSR_SPEC>`"]
pub type PNWKSR = crate::Reg<pnwksr::PNWKSR_SPEC>;
#[doc = "PNWKSR register"]
pub mod pnwksr;
#[doc = "IOFILT0 (rw) register accessor: an alias for `Reg<IOFILT0_SPEC>`"]
pub type IOFILT0 = crate::Reg<iofilt0::IOFILT0_SPEC>;
#[doc = "IOFILT0 register"]
pub mod iofilt0;
#[doc = "IOFILT1 (rw) register accessor: an alias for `Reg<IOFILT1_SPEC>`"]
pub type IOFILT1 = crate::Reg<iofilt1::IOFILT1_SPEC>;
#[doc = "IOFILT1 register"]
pub mod iofilt1;
#[doc = "PRSTEN (rw) register accessor: an alias for `Reg<PRSTEN_SPEC>`"]
pub type PRSTEN = crate::Reg<prsten::PRSTEN_SPEC>;
#[doc = "PRSTEN register"]
pub mod prsten;
#[doc = "PRSTR0 (rw) register accessor: an alias for `Reg<PRSTR0_SPEC>`"]
pub type PRSTR0 = crate::Reg<prstr0::PRSTR0_SPEC>;
#[doc = "PRSTR0 register"]
pub mod prstr0;
#[doc = "PRSTR1 (rw) register accessor: an alias for `Reg<PRSTR1_SPEC>`"]
pub type PRSTR1 = crate::Reg<prstr1::PRSTR1_SPEC>;
#[doc = "PRSTR1 register"]
pub mod prstr1;
#[doc = "HRCCR (rw) register accessor: an alias for `Reg<HRCCR_SPEC>`"]
pub type HRCCR = crate::Reg<hrccr::HRCCR_SPEC>;
#[doc = "HRCCR register"]
pub mod hrccr;
#[doc = "BODCR (rw) register accessor: an alias for `Reg<BODCR_SPEC>`"]
pub type BODCR = crate::Reg<bodcr::BODCR_SPEC>;
#[doc = "BODCR register"]
pub mod bodcr;
#[doc = "BODSR (rw) register accessor: an alias for `Reg<BODSR_SPEC>`"]
pub type BODSR = crate::Reg<bodsr::BODSR_SPEC>;
#[doc = "BODSR register"]
pub mod bodsr;
#[doc = "ADCCR (rw) register accessor: an alias for `Reg<ADCCR_SPEC>`"]
pub type ADCCR = crate::Reg<adccr::ADCCR_SPEC>;
#[doc = "ADCCR register"]
pub mod adccr;
#[doc = "XTALCR (rw) register accessor: an alias for `Reg<XTALCR_SPEC>`"]
pub type XTALCR = crate::Reg<xtalcr::XTALCR_SPEC>;
#[doc = "XTALCR register"]
pub mod xtalcr;
#[doc = "XTALSR (rw) register accessor: an alias for `Reg<XTALSR_SPEC>`"]
pub type XTALSR = crate::Reg<xtalsr::XTALSR_SPEC>;
#[doc = "XTALSR register"]
pub mod xtalsr;
#[doc = "PLLCR (rw) register accessor: an alias for `Reg<PLLCR_SPEC>`"]
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
#[doc = "PLLCR register"]
pub mod pllcr;
#[doc = "PLLDIV (rw) register accessor: an alias for `Reg<PLLDIV_SPEC>`"]
pub type PLLDIV = crate::Reg<plldiv::PLLDIV_SPEC>;
#[doc = "PLLDIV register"]
pub mod plldiv;
#[doc = "PLLLOCK (rw) register accessor: an alias for `Reg<PLLLOCK_SPEC>`"]
pub type PLLLOCK = crate::Reg<plllock::PLLLOCK_SPEC>;
#[doc = "PLLLOCK register"]
pub mod plllock;
#[doc = "LRCCR (rw) register accessor: an alias for `Reg<LRCCR_SPEC>`"]
pub type LRCCR = crate::Reg<lrccr::LRCCR_SPEC>;
#[doc = "LRCCR register"]
pub mod lrccr;
#[doc = "OPACR (rw) register accessor: an alias for `Reg<OPACR_SPEC>`"]
pub type OPACR = crate::Reg<opacr::OPACR_SPEC>;
#[doc = "OPACR register"]
pub mod opacr;
#[doc = "ACMPCR (rw) register accessor: an alias for `Reg<ACMPCR_SPEC>`"]
pub type ACMPCR = crate::Reg<acmpcr::ACMPCR_SPEC>;
#[doc = "ACMPCR register"]
pub mod acmpcr;
#[doc = "ACMPSR (rw) register accessor: an alias for `Reg<ACMPSR_SPEC>`"]
pub type ACMPSR = crate::Reg<acmpsr::ACMPSR_SPEC>;
#[doc = "ACMPSR register"]
pub mod acmpsr;
#[doc = "ACMPCR2 (rw) register accessor: an alias for `Reg<ACMPCR2_SPEC>`"]
pub type ACMPCR2 = crate::Reg<acmpcr2::ACMPCR2_SPEC>;
#[doc = "ACMPCR2 register"]
pub mod acmpcr2;
#[doc = "DACCR (rw) register accessor: an alias for `Reg<DACCR_SPEC>`"]
pub type DACCR = crate::Reg<daccr::DACCR_SPEC>;
#[doc = "DACCR register"]
pub mod daccr;
#[doc = "TEMPCR (rw) register accessor: an alias for `Reg<TEMPCR_SPEC>`"]
pub type TEMPCR = crate::Reg<tempcr::TEMPCR_SPEC>;
#[doc = "TEMPCR register"]
pub mod tempcr;
