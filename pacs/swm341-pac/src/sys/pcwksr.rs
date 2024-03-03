#[doc = "Register `PCWKSR` reader"]
pub struct R(crate::R<PCWKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCWKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCWKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCWKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCWKSR` writer"]
pub struct W(crate::W<PCWKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCWKSR_SPEC>;
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
impl From<crate::W<PCWKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCWKSR_SPEC>) -> Self {
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
#[doc = "PCWKSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcwksr](index.html) module"]
pub struct PCWKSR_SPEC;
impl crate::RegisterSpec for PCWKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcwksr::R](R) reader structure"]
impl crate::Readable for PCWKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcwksr::W](W) writer structure"]
impl crate::Writable for PCWKSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCWKSR to value 0"]
impl crate::Resettable for PCWKSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
