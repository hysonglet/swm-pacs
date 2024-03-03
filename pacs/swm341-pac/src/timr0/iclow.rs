#[doc = "Register `ICLOW` reader"]
pub struct R(crate::R<ICLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLOW` writer"]
pub struct W(crate::W<ICLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLOW_SPEC>;
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
impl From<crate::W<ICLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLOW_SPEC>) -> Self {
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
#[doc = "ICLOW register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclow](index.html) module"]
pub struct ICLOW_SPEC;
impl crate::RegisterSpec for ICLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclow::R](R) reader structure"]
impl crate::Readable for ICLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclow::W](W) writer structure"]
impl crate::Writable for ICLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICLOW to value 0"]
impl crate::Resettable for ICLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
