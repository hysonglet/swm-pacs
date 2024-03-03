#[doc = "Register `RTCWKSR` reader"]
pub struct R(crate::R<RTCWKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCWKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCWKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCWKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCWKSR` writer"]
pub struct W(crate::W<RTCWKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCWKSR_SPEC>;
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
impl From<crate::W<RTCWKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCWKSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - FLAG field"]
pub type FLAG_R = crate::BitReader<bool>;
#[doc = "Field `FLAG` writer - FLAG field"]
pub type FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCWKSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLAG field"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLAG field"]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<0> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCWKSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcwksr](index.html) module"]
pub struct RTCWKSR_SPEC;
impl crate::RegisterSpec for RTCWKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcwksr::R](R) reader structure"]
impl crate::Readable for RTCWKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcwksr::W](W) writer structure"]
impl crate::Writable for RTCWKSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCWKSR to value 0"]
impl crate::Resettable for RTCWKSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
