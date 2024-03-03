#[doc = "Register `PLLCR` reader"]
pub struct R(crate::R<PLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCR` writer"]
pub struct W(crate::W<PLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCR_SPEC>;
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
impl From<crate::W<PLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTEN` reader - OUTEN field"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - OUTEN field"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCR_SPEC, bool, O>;
#[doc = "Field `INSEL` reader - INSEL field"]
pub type INSEL_R = crate::BitReader<bool>;
#[doc = "Field `INSEL` writer - INSEL field"]
pub type INSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCR_SPEC, bool, O>;
#[doc = "Field `OFF` reader - OFF field"]
pub type OFF_R = crate::BitReader<bool>;
#[doc = "Field `OFF` writer - OFF field"]
pub type OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCR_SPEC, bool, O>;
#[doc = "Field `RST` reader - RST field"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - RST field"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OUTEN field"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INSEL field"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OFF field"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RST field"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUTEN field"]
    #[inline(always)]
    pub fn outen(&mut self) -> OUTEN_W<0> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 1 - INSEL field"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W<1> {
        INSEL_W::new(self)
    }
    #[doc = "Bit 2 - OFF field"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W<2> {
        OFF_W::new(self)
    }
    #[doc = "Bit 3 - RST field"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<3> {
        RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcr](index.html) module"]
pub struct PLLCR_SPEC;
impl crate::RegisterSpec for PLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcr::R](R) reader structure"]
impl crate::Readable for PLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcr::W](W) writer structure"]
impl crate::Writable for PLLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCR to value 0"]
impl crate::Resettable for PLLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
