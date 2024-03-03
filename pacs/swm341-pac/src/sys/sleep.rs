#[doc = "Register `SLEEP` reader"]
pub struct R(crate::R<SLEEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEP` writer"]
pub struct W(crate::W<SLEEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEP_SPEC>;
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
impl From<crate::W<SLEEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP` reader - SLEEP field"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - SLEEP field"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEP_SPEC, bool, O>;
#[doc = "Field `STOP` reader - STOP field"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - STOP field"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLEEP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SLEEP field"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP field"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLEEP field"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<0> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 1 - STOP field"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLEEP register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep](index.html) module"]
pub struct SLEEP_SPEC;
impl crate::RegisterSpec for SLEEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleep::R](R) reader structure"]
impl crate::Readable for SLEEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleep::W](W) writer structure"]
impl crate::Writable for SLEEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLEEP to value 0"]
impl crate::Resettable for SLEEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
