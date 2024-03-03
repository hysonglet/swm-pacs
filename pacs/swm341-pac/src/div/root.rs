#[doc = "Register `ROOT` reader"]
pub struct R(crate::R<ROOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROOT` writer"]
pub struct W(crate::W<ROOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROOT_SPEC>;
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
impl From<crate::W<ROOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROOT_SPEC>) -> Self {
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
#[doc = "ROOT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [root](index.html) module"]
pub struct ROOT_SPEC;
impl crate::RegisterSpec for ROOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [root::R](R) reader structure"]
impl crate::Readable for ROOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [root::W](W) writer structure"]
impl crate::Writable for ROOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROOT to value 0"]
impl crate::Resettable for ROOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
