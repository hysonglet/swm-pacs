#[doc = "Register `CFGABLE` reader"]
pub struct R(crate::R<CFGABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGABLE` writer"]
pub struct W(crate::W<CFGABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGABLE_SPEC>;
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
impl From<crate::W<CFGABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGABLE_SPEC>) -> Self {
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
#[doc = "CFGABLE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgable](index.html) module"]
pub struct CFGABLE_SPEC;
impl crate::RegisterSpec for CFGABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgable::R](R) reader structure"]
impl crate::Readable for CFGABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgable::W](W) writer structure"]
impl crate::Writable for CFGABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGABLE to value 0"]
impl crate::Resettable for CFGABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
