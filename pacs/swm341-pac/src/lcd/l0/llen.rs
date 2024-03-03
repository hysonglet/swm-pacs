#[doc = "Register `LLEN` reader"]
pub struct R(crate::R<LLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLEN` writer"]
pub struct W(crate::W<LLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLEN_SPEC>;
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
impl From<crate::W<LLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLEN_SPEC>) -> Self {
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
#[doc = "LLEN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [llen](index.html) module"]
pub struct LLEN_SPEC;
impl crate::RegisterSpec for LLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [llen::R](R) reader structure"]
impl crate::Readable for LLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [llen::W](W) writer structure"]
impl crate::Writable for LLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LLEN to value 0"]
impl crate::Resettable for LLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
