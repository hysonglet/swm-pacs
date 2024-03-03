#[doc = "Register `PAWKEN` reader"]
pub struct R(crate::R<PAWKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAWKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAWKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAWKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAWKEN` writer"]
pub struct W(crate::W<PAWKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAWKEN_SPEC>;
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
impl From<crate::W<PAWKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAWKEN_SPEC>) -> Self {
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
#[doc = "PAWKEN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pawken](index.html) module"]
pub struct PAWKEN_SPEC;
impl crate::RegisterSpec for PAWKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pawken::R](R) reader structure"]
impl crate::Readable for PAWKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pawken::W](W) writer structure"]
impl crate::Writable for PAWKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAWKEN to value 0"]
impl crate::Resettable for PAWKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
