#[doc = "Register `RTCWKCR` reader"]
pub struct R(crate::R<RTCWKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCWKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCWKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCWKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCWKCR` writer"]
pub struct W(crate::W<RTCWKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCWKCR_SPEC>;
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
impl From<crate::W<RTCWKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCWKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCWKCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCWKCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcwkcr](index.html) module"]
pub struct RTCWKCR_SPEC;
impl crate::RegisterSpec for RTCWKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcwkcr::R](R) reader structure"]
impl crate::Readable for RTCWKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcwkcr::W](W) writer structure"]
impl crate::Writable for RTCWKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCWKCR to value 0"]
impl crate::Resettable for RTCWKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
