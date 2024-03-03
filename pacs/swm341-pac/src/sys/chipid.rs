#[doc = "Register `CHIPID[%s]` reader"]
pub struct R(crate::R<CHIPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CHIPID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid](index.html) module"]
pub struct CHIPID_SPEC;
impl crate::RegisterSpec for CHIPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chipid::R](R) reader structure"]
impl crate::Readable for CHIPID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHIPID[%s]
to value 0"]
impl crate::Resettable for CHIPID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
