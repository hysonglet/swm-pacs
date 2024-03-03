#[doc = "Register `ECC` reader"]
pub struct R(crate::R<ECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEGCODE` reader - SEGCODE field"]
pub type SEGCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIR` reader - DIR field"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `ERRCODE` reader - ERRCODE field"]
pub type ERRCODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - SEGCODE field"]
    #[inline(always)]
    pub fn segcode(&self) -> SEGCODE_R {
        SEGCODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - DIR field"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - ERRCODE field"]
    #[inline(always)]
    pub fn errcode(&self) -> ERRCODE_R {
        ERRCODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "ECC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc](index.html) module"]
pub struct ECC_SPEC;
impl crate::RegisterSpec for ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc::R](R) reader structure"]
impl crate::Readable for ECC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC to value 0"]
impl crate::Resettable for ECC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
