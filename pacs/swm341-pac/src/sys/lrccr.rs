#[doc = "Register `LRCCR` reader"]
pub struct R(crate::R<LRCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LRCCR` writer"]
pub struct W(crate::W<LRCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRCCR_SPEC>;
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
impl From<crate::W<LRCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ON` reader - ON field"]
pub type ON_R = crate::BitReader<bool>;
#[doc = "Field `ON` writer - ON field"]
pub type ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, LRCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ON field"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ON field"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W<0> {
        ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LRCCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrccr](index.html) module"]
pub struct LRCCR_SPEC;
impl crate::RegisterSpec for LRCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrccr::R](R) reader structure"]
impl crate::Readable for LRCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrccr::W](W) writer structure"]
impl crate::Writable for LRCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LRCCR to value 0"]
impl crate::Resettable for LRCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
