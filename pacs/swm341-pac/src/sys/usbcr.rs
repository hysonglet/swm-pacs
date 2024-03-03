#[doc = "Register `USBCR` reader"]
pub struct R(crate::R<USBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCR` writer"]
pub struct W(crate::W<USBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCR_SPEC>;
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
impl From<crate::W<USBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST48M` reader - RST48M field"]
pub type RST48M_R = crate::BitReader<bool>;
#[doc = "Field `RST48M` writer - RST48M field"]
pub type RST48M_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCR_SPEC, bool, O>;
#[doc = "Field `RST12M` reader - RST12M field"]
pub type RST12M_R = crate::BitReader<bool>;
#[doc = "Field `RST12M` writer - RST12M field"]
pub type RST12M_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCR_SPEC, bool, O>;
#[doc = "Field `RSTPLL` reader - RSTPLL field"]
pub type RSTPLL_R = crate::BitReader<bool>;
#[doc = "Field `RSTPLL` writer - RSTPLL field"]
pub type RSTPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCR_SPEC, bool, O>;
#[doc = "Field `ROLE` reader - ROLE field"]
pub type ROLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROLE` writer - ROLE field"]
pub type ROLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VBUS` reader - VBUS field"]
pub type VBUS_R = crate::BitReader<bool>;
#[doc = "Field `VBUS` writer - VBUS field"]
pub type VBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RST48M field"]
    #[inline(always)]
    pub fn rst48m(&self) -> RST48M_R {
        RST48M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RST12M field"]
    #[inline(always)]
    pub fn rst12m(&self) -> RST12M_R {
        RST12M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSTPLL field"]
    #[inline(always)]
    pub fn rstpll(&self) -> RSTPLL_R {
        RSTPLL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - ROLE field"]
    #[inline(always)]
    pub fn role(&self) -> ROLE_R {
        ROLE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - VBUS field"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST48M field"]
    #[inline(always)]
    pub fn rst48m(&mut self) -> RST48M_W<0> {
        RST48M_W::new(self)
    }
    #[doc = "Bit 1 - RST12M field"]
    #[inline(always)]
    pub fn rst12m(&mut self) -> RST12M_W<1> {
        RST12M_W::new(self)
    }
    #[doc = "Bit 2 - RSTPLL field"]
    #[inline(always)]
    pub fn rstpll(&mut self) -> RSTPLL_W<2> {
        RSTPLL_W::new(self)
    }
    #[doc = "Bits 3:4 - ROLE field"]
    #[inline(always)]
    pub fn role(&mut self) -> ROLE_W<3> {
        ROLE_W::new(self)
    }
    #[doc = "Bit 5 - VBUS field"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W<5> {
        VBUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcr](index.html) module"]
pub struct USBCR_SPEC;
impl crate::RegisterSpec for USBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcr::R](R) reader structure"]
impl crate::Readable for USBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcr::W](W) writer structure"]
impl crate::Writable for USBCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCR to value 0"]
impl crate::Resettable for USBCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
