#[doc = "Register `RXBUF[%s]` reader"]
pub struct R(crate::R<RXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXBUF[%s]` writer"]
pub struct W(crate::W<RXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXBUF_SPEC>;
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
impl From<crate::W<RXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXBUF_SPEC>) -> Self {
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
#[doc = "RXBUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf](index.html) module"]
pub struct RXBUF_SPEC;
impl crate::RegisterSpec for RXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbuf::R](R) reader structure"]
impl crate::Readable for RXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxbuf::W](W) writer structure"]
impl crate::Writable for RXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXBUF[%s]
to value 0"]
impl crate::Resettable for RXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
