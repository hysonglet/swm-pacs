#[doc = "Register `CMPA2` reader"]
pub struct R(crate::R<CMPA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPA2` writer"]
pub struct W(crate::W<CMPA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPA2_SPEC>;
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
impl From<crate::W<CMPA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPA2_SPEC>) -> Self {
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
#[doc = "CMPA2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpa2](index.html) module"]
pub struct CMPA2_SPEC;
impl crate::RegisterSpec for CMPA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpa2::R](R) reader structure"]
impl crate::Readable for CMPA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpa2::W](W) writer structure"]
impl crate::Writable for CMPA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPA2 to value 0"]
impl crate::Resettable for CMPA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
