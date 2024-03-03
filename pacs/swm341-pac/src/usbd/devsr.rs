#[doc = "Register `DEVSR` reader"]
pub struct R(crate::R<DEVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFG` reader - CFG field"]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTF` reader - INTF field"]
pub type INTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALT` reader - ALT field"]
pub type ALT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUSP` reader - SUSP field"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `FRNR` reader - FRNR field"]
pub type FRNR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - CFG field"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - INTF field"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ALT field"]
    #[inline(always)]
    pub fn alt(&self) -> ALT_R {
        ALT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - SUSP field"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 21:31 - FRNR field"]
    #[inline(always)]
    pub fn frnr(&self) -> FRNR_R {
        FRNR_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "DEVSR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devsr](index.html) module"]
pub struct DEVSR_SPEC;
impl crate::RegisterSpec for DEVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devsr::R](R) reader structure"]
impl crate::Readable for DEVSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVSR to value 0"]
impl crate::Resettable for DEVSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
