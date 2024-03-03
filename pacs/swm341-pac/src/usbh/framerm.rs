#[doc = "Register `FRAMERM` reader"]
pub struct R(crate::R<FRAMERM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMERM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMERM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMERM` writer"]
pub struct W(crate::W<FRAMERM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMERM_SPEC>;
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
impl From<crate::W<FRAMERM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMERM_SPEC>) -> Self {
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
#[doc = "FRAMERM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framerm](index.html) module"]
pub struct FRAMERM_SPEC;
impl crate::RegisterSpec for FRAMERM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framerm::R](R) reader structure"]
impl crate::Readable for FRAMERM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framerm::W](W) writer structure"]
impl crate::Writable for FRAMERM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMERM to value 0"]
impl crate::Resettable for FRAMERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
