#[doc = "Register `RSTSR` reader"]
pub struct R(crate::R<RSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSR` writer"]
pub struct W(crate::W<RSTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSR_SPEC>;
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
impl From<crate::W<RSTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` reader - POR field"]
pub type POR_R = crate::BitReader<bool>;
#[doc = "Field `POR` writer - POR field"]
pub type POR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTSR_SPEC, bool, O>;
#[doc = "Field `WDT` reader - WDT field"]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - WDT field"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - POR field"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT field"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POR field"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W<0> {
        POR_W::new(self)
    }
    #[doc = "Bit 1 - WDT field"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<1> {
        WDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSTSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsr](index.html) module"]
pub struct RSTSR_SPEC;
impl crate::RegisterSpec for RSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstsr::R](R) reader structure"]
impl crate::Readable for RSTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsr::W](W) writer structure"]
impl crate::Writable for RSTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTSR to value 0"]
impl crate::Resettable for RSTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
