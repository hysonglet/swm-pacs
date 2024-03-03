#[doc = "Register `DHR` reader"]
pub struct R(crate::R<DHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHR` writer"]
pub struct W(crate::W<DHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR_SPEC>;
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
impl From<crate::W<DHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR_SPEC>) -> Self {
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
#[doc = "DHR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr](index.html) module"]
pub struct DHR_SPEC;
impl crate::RegisterSpec for DHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhr::R](R) reader structure"]
impl crate::Readable for DHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhr::W](W) writer structure"]
impl crate::Writable for DHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHR to value 0"]
impl crate::Resettable for DHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
