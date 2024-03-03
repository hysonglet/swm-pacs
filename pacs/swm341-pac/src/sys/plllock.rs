#[doc = "Register `PLLLOCK` reader"]
pub struct R(crate::R<PLLLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLLOCK` writer"]
pub struct W(crate::W<PLLLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLLOCK_SPEC>;
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
impl From<crate::W<PLLLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLLOCK_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLLOCK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plllock](index.html) module"]
pub struct PLLLOCK_SPEC;
impl crate::RegisterSpec for PLLLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plllock::R](R) reader structure"]
impl crate::Readable for PLLLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plllock::W](W) writer structure"]
impl crate::Writable for PLLLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLLOCK to value 0"]
impl crate::Resettable for PLLLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
