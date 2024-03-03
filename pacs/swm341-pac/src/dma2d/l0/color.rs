#[doc = "Register `COLOR` reader"]
pub struct R(crate::R<COLOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLOR` writer"]
pub struct W(crate::W<COLOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLOR_SPEC>;
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
impl From<crate::W<COLOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLOR_SPEC>) -> Self {
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
#[doc = "COLOR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [color](index.html) module"]
pub struct COLOR_SPEC;
impl crate::RegisterSpec for COLOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [color::R](R) reader structure"]
impl crate::Readable for COLOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [color::W](W) writer structure"]
impl crate::Writable for COLOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COLOR to value 0"]
impl crate::Resettable for COLOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
