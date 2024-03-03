#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INDEX` reader - INDEX field"]
pub type INDEX_R = crate::BitReader<bool>;
#[doc = "Field `MATCH` reader - MATCH field"]
pub type MATCH_R = crate::BitReader<bool>;
#[doc = "Field `CNTOV` reader - CNTOV field"]
pub type CNTOV_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` reader - ERROR field"]
pub type ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - INDEX field"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MATCH field"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNTOV field"]
    #[inline(always)]
    pub fn cntov(&self) -> CNTOV_R {
        CNTOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ERROR field"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "IF register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
