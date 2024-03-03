#[doc = "Register `PAWKSR` reader"]
pub struct R(crate::R<PAWKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAWKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAWKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAWKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAWKSR` writer"]
pub struct W(crate::W<PAWKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAWKSR_SPEC>;
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
impl From<crate::W<PAWKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAWKSR_SPEC>) -> Self {
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
#[doc = "PAWKSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pawksr](index.html) module"]
pub struct PAWKSR_SPEC;
impl crate::RegisterSpec for PAWKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pawksr::R](R) reader structure"]
impl crate::Readable for PAWKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pawksr::W](W) writer structure"]
impl crate::Writable for PAWKSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAWKSR to value 0"]
impl crate::Resettable for PAWKSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
