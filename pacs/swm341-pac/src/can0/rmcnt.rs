#[doc = "Register `RMCNT` reader"]
pub struct R(crate::R<RMCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RMCNT register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmcnt](index.html) module"]
pub struct RMCNT_SPEC;
impl crate::RegisterSpec for RMCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmcnt::R](R) reader structure"]
impl crate::Readable for RMCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMCNT to value 0"]
impl crate::Resettable for RMCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
