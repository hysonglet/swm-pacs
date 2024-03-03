#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - BUSY field"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOLVL` reader - FIFOLVL field"]
pub type FIFOLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOEMPTY` reader - FIFOEMPTY field"]
pub type FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOHFULL` reader - FIFOHFULL field"]
pub type FIFOHFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFULL` reader - FIFOFULL field"]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOOVF` reader - FIFOOVF field"]
pub type FIFOOVF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOUVF` reader - FIFOUVF field"]
pub type FIFOUVF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - BUSY field"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:26 - FIFOLVL field"]
    #[inline(always)]
    pub fn fifolvl(&self) -> FIFOLVL_R {
        FIFOLVL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - FIFOEMPTY field"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FIFOHFULL field"]
    #[inline(always)]
    pub fn fifohfull(&self) -> FIFOHFULL_R {
        FIFOHFULL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFOFULL field"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FIFOOVF field"]
    #[inline(always)]
    pub fn fifoovf(&self) -> FIFOOVF_R {
        FIFOOVF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FIFOUVF field"]
    #[inline(always)]
    pub fn fifouvf(&self) -> FIFOUVF_R {
        FIFOUVF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
