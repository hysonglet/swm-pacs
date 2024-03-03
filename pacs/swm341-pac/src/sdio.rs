#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_MEM_ADDR register"]
    pub dma_mem_addr: DMA_MEM_ADDR,
    #[doc = "0x04 - BLK register"]
    pub blk: BLK,
    #[doc = "0x08 - ARG register"]
    pub arg: ARG,
    #[doc = "0x0c - CMD register"]
    pub cmd: CMD,
    #[doc = "0x10..0x20 - RESP register"]
    pub resp: [RESP; 4],
    #[doc = "0x20 - DATA register"]
    pub data: DATA,
    #[doc = "0x24 - STAT register"]
    pub stat: STAT,
    #[doc = "0x28 - CR1 register"]
    pub cr1: CR1,
    #[doc = "0x2c - CR2 register"]
    pub cr2: CR2,
    #[doc = "0x30 - IF register"]
    pub if_: IF,
    #[doc = "0x34 - IM register"]
    pub im: IM,
    #[doc = "0x38 - IE register"]
    pub ie: IE,
    #[doc = "0x3c - CMD12ERR register"]
    pub cmd12err: CMD12ERR,
}
#[doc = "DMA_MEM_ADDR (rw) register accessor: an alias for `Reg<DMA_MEM_ADDR_SPEC>`"]
pub type DMA_MEM_ADDR = crate::Reg<dma_mem_addr::DMA_MEM_ADDR_SPEC>;
#[doc = "DMA_MEM_ADDR register"]
pub mod dma_mem_addr;
#[doc = "BLK (rw) register accessor: an alias for `Reg<BLK_SPEC>`"]
pub type BLK = crate::Reg<blk::BLK_SPEC>;
#[doc = "BLK register"]
pub mod blk;
#[doc = "ARG (rw) register accessor: an alias for `Reg<ARG_SPEC>`"]
pub type ARG = crate::Reg<arg::ARG_SPEC>;
#[doc = "ARG register"]
pub mod arg;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "CMD register"]
pub mod cmd;
#[doc = "RESP (rw) register accessor: an alias for `Reg<RESP_SPEC>`"]
pub type RESP = crate::Reg<resp::RESP_SPEC>;
#[doc = "RESP register"]
pub mod resp;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA register"]
pub mod data;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "STAT register"]
pub mod stat;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CR1 register"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "CR2 register"]
pub mod cr2;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IF register"]
pub mod if_;
#[doc = "IM (rw) register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "IM register"]
pub mod im;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IE register"]
pub mod ie;
#[doc = "CMD12ERR (rw) register accessor: an alias for `Reg<CMD12ERR_SPEC>`"]
pub type CMD12ERR = crate::Reg<cmd12err::CMD12ERR_SPEC>;
#[doc = "CMD12ERR register"]
pub mod cmd12err;
