#[doc = "Register `TXBUF15_[%s]` reader"]
pub struct R(crate::R<TXBUF15__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBUF15__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBUF15__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBUF15__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBUF15_[%s]` writer"]
pub struct W(crate::W<TXBUF15__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBUF15__SPEC>;
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
impl From<crate::W<TXBUF15__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBUF15__SPEC>) -> Self {
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
#[doc = "TXBUF15_ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf15_](index.html) module"]
pub struct TXBUF15__SPEC;
impl crate::RegisterSpec for TXBUF15__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbuf15_::R](R) reader structure"]
impl crate::Readable for TXBUF15__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbuf15_::W](W) writer structure"]
impl crate::Writable for TXBUF15__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBUF15_[%s]
to value 0"]
impl crate::Resettable for TXBUF15__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
