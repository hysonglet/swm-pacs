#[doc = "Register `BGC` reader"]
pub struct R(crate::R<BGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGC` writer"]
pub struct W(crate::W<BGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGC_SPEC>;
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
impl From<crate::W<BGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGC_SPEC>) -> Self {
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
#[doc = "BGC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgc](index.html) module"]
pub struct BGC_SPEC;
impl crate::RegisterSpec for BGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgc::R](R) reader structure"]
impl crate::Readable for BGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgc::W](W) writer structure"]
impl crate::Writable for BGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGC to value 0"]
impl crate::Resettable for BGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
