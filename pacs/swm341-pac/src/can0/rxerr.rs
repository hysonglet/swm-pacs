#[doc = "Register `RXERR` reader"]
pub struct R(crate::R<RXERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXERR` writer"]
pub struct W(crate::W<RXERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXERR_SPEC>;
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
impl From<crate::W<RXERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXERR_SPEC>) -> Self {
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
#[doc = "RXERR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxerr](index.html) module"]
pub struct RXERR_SPEC;
impl crate::RegisterSpec for RXERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxerr::R](R) reader structure"]
impl crate::Readable for RXERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxerr::W](W) writer structure"]
impl crate::Writable for RXERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXERR to value 0"]
impl crate::Resettable for RXERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
