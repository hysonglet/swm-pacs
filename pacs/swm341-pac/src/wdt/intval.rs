#[doc = "Register `INTVAL` reader"]
pub struct R(crate::R<INTVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTVAL` writer"]
pub struct W(crate::W<INTVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTVAL_SPEC>;
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
impl From<crate::W<INTVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTVAL_SPEC>) -> Self {
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
#[doc = "INTVAL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intval](index.html) module"]
pub struct INTVAL_SPEC;
impl crate::RegisterSpec for INTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intval::R](R) reader structure"]
impl crate::Readable for INTVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intval::W](W) writer structure"]
impl crate::Writable for INTVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTVAL to value 0"]
impl crate::Resettable for INTVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
