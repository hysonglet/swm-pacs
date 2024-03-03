#[doc = "Register `IFOV` reader"]
pub struct R(crate::R<IFOV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFOV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFOV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFOV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFOV` writer"]
pub struct W(crate::W<IFOV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFOV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IFOV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFOV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` reader - INDEX field"]
pub type INDEX_R = crate::BitReader<bool>;
#[doc = "Field `INDEX` writer - INDEX field"]
pub type INDEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFOV_SPEC, bool, O>;
#[doc = "Field `MATCH` reader - MATCH field"]
pub type MATCH_R = crate::BitReader<bool>;
#[doc = "Field `MATCH` writer - MATCH field"]
pub type MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFOV_SPEC, bool, O>;
#[doc = "Field `CNTOV` reader - CNTOV field"]
pub type CNTOV_R = crate::BitReader<bool>;
#[doc = "Field `CNTOV` writer - CNTOV field"]
pub type CNTOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFOV_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - ERROR field"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - ERROR field"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFOV_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - INDEX field"]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W<0> {
        INDEX_W::new(self)
    }
    #[doc = "Bit 1 - MATCH field"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W<1> {
        MATCH_W::new(self)
    }
    #[doc = "Bit 2 - CNTOV field"]
    #[inline(always)]
    pub fn cntov(&mut self) -> CNTOV_W<2> {
        CNTOV_W::new(self)
    }
    #[doc = "Bit 3 - ERROR field"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W<3> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFOV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifov](index.html) module"]
pub struct IFOV_SPEC;
impl crate::RegisterSpec for IFOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifov::R](R) reader structure"]
impl crate::Readable for IFOV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifov::W](W) writer structure"]
impl crate::Writable for IFOV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFOV to value 0"]
impl crate::Resettable for IFOV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
