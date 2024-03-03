#[doc = "Register `AC_CODEVAL[%s]` writer"]
pub struct W(crate::W<AC_CODEVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_CODEVAL_SPEC>;
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
impl From<crate::W<AC_CODEVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_CODEVAL_SPEC>) -> Self {
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
#[doc = "AC_CODEVAL register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_codeval](index.html) module"]
pub struct AC_CODEVAL_SPEC;
impl crate::RegisterSpec for AC_CODEVAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ac_codeval::W](W) writer structure"]
impl crate::Writable for AC_CODEVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AC_CODEVAL[%s]
to value 0"]
impl crate::Resettable for AC_CODEVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
