#[doc = "Register `DC_CODELEN[%s]` writer"]
pub struct W(crate::W<DC_CODELEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_CODELEN_SPEC>;
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
impl From<crate::W<DC_CODELEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_CODELEN_SPEC>) -> Self {
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
#[doc = "DC_CODELEN register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_codelen](index.html) module"]
pub struct DC_CODELEN_SPEC;
impl crate::RegisterSpec for DC_CODELEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dc_codelen::W](W) writer structure"]
impl crate::Writable for DC_CODELEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_CODELEN[%s]
to value 0"]
impl crate::Resettable for DC_CODELEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
