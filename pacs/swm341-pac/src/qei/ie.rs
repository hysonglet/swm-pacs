#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` reader - INDEX field"]
pub type INDEX_R = crate::BitReader<bool>;
#[doc = "Field `INDEX` writer - INDEX field"]
pub type INDEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MATCH` reader - MATCH field"]
pub type MATCH_R = crate::BitReader<bool>;
#[doc = "Field `MATCH` writer - MATCH field"]
pub type MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `CNTOV` reader - CNTOV field"]
pub type CNTOV_R = crate::BitReader<bool>;
#[doc = "Field `CNTOV` writer - CNTOV field"]
pub type CNTOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - ERROR field"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - ERROR field"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
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
#[doc = "IE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
