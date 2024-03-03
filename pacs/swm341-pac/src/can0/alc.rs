#[doc = "Register `ALC` reader"]
pub struct R(crate::R<ALC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ALC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alc](index.html) module"]
pub struct ALC_SPEC;
impl crate::RegisterSpec for ALC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alc::R](R) reader structure"]
impl crate::Readable for ALC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ALC to value 0"]
impl crate::Resettable for ALC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
