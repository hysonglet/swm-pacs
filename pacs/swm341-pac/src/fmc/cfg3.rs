#[doc = "Register `CFG3` reader"]
pub struct R(crate::R<CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG3` writer"]
pub struct W(crate::W<CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG3_SPEC>;
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
impl From<crate::W<CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG3_SPEC>) -> Self {
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
#[doc = "CFG3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg3](index.html) module"]
pub struct CFG3_SPEC;
impl crate::RegisterSpec for CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg3::R](R) reader structure"]
impl crate::Readable for CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg3::W](W) writer structure"]
impl crate::Writable for CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
