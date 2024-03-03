#[doc = "Register `SRCSR` reader"]
pub struct R(crate::R<SRCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LEN` reader - LEN field"]
pub type LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ERR` reader - ERR field"]
pub type ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - LEN field"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 31 - ERR field"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SRCSR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcsr](index.html) module"]
pub struct SRCSR_SPEC;
impl crate::RegisterSpec for SRCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcsr::R](R) reader structure"]
impl crate::Readable for SRCSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRCSR to value 0"]
impl crate::Resettable for SRCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
