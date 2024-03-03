#[doc = "Register `IC` writer"]
pub struct W(crate::W<IC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SPEC>;
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
impl From<crate::W<IC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` writer - INDEX field"]
pub type INDEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `MATCH` writer - MATCH field"]
pub type MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `CNTOV` writer - CNTOV field"]
pub type CNTOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
#[doc = "Field `ERROR` writer - ERROR field"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_SPEC, bool, O>;
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
#[doc = "IC register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ic::W](W) writer structure"]
impl crate::Writable for IC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
