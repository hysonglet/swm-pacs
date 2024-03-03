#[doc = "Register `BODSR` reader"]
pub struct R(crate::R<BODSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODSR` writer"]
pub struct W(crate::W<BODSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODSR_SPEC>;
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
impl From<crate::W<BODSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IF` reader - IF field"]
pub type IF_R = crate::BitReader<bool>;
#[doc = "Field `IF` writer - IF field"]
pub type IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODSR_SPEC, bool, O>;
#[doc = "Field `ST` reader - ST field"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - ST field"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IF field"]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ST field"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IF field"]
    #[inline(always)]
    pub fn if_(&mut self) -> IF_W<0> {
        IF_W::new(self)
    }
    #[doc = "Bit 1 - ST field"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<1> {
        ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BODSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodsr](index.html) module"]
pub struct BODSR_SPEC;
impl crate::RegisterSpec for BODSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodsr::R](R) reader structure"]
impl crate::Readable for BODSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodsr::W](W) writer structure"]
impl crate::Writable for BODSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODSR to value 0"]
impl crate::Resettable for BODSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
