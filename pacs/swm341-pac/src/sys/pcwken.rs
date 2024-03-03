#[doc = "Register `PCWKEN` reader"]
pub struct R(crate::R<PCWKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCWKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCWKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCWKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCWKEN` writer"]
pub struct W(crate::W<PCWKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCWKEN_SPEC>;
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
impl From<crate::W<PCWKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCWKEN_SPEC>) -> Self {
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
#[doc = "PCWKEN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcwken](index.html) module"]
pub struct PCWKEN_SPEC;
impl crate::RegisterSpec for PCWKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcwken::R](R) reader structure"]
impl crate::Readable for PCWKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcwken::W](W) writer structure"]
impl crate::Writable for PCWKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCWKEN to value 0"]
impl crate::Resettable for PCWKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
