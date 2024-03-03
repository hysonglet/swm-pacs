#[doc = "Register `USBPHYCR` reader"]
pub struct R(crate::R<USBPHYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYCR` writer"]
pub struct W(crate::W<USBPHYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYCR_SPEC>;
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
impl From<crate::W<USBPHYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDEN` reader - IDEN field"]
pub type IDEN_R = crate::BitReader<bool>;
#[doc = "Field `IDEN` writer - IDEN field"]
pub type IDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYCR_SPEC, bool, O>;
#[doc = "Field `OPMODE` reader - OPMODE field"]
pub type OPMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPMODE` writer - OPMODE field"]
pub type OPMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `XCVR` reader - XCVR field"]
pub type XCVR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XCVR` writer - XCVR field"]
pub type XCVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLEN` reader - PLLEN field"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLLEN field"]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IDEN field"]
    #[inline(always)]
    pub fn iden(&self) -> IDEN_R {
        IDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - OPMODE field"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - XCVR field"]
    #[inline(always)]
    pub fn xcvr(&self) -> XCVR_R {
        XCVR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PLLEN field"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDEN field"]
    #[inline(always)]
    pub fn iden(&mut self) -> IDEN_W<0> {
        IDEN_W::new(self)
    }
    #[doc = "Bits 1:2 - OPMODE field"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W<1> {
        OPMODE_W::new(self)
    }
    #[doc = "Bits 4:5 - XCVR field"]
    #[inline(always)]
    pub fn xcvr(&mut self) -> XCVR_W<4> {
        XCVR_W::new(self)
    }
    #[doc = "Bit 7 - PLLEN field"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<7> {
        PLLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBPHYCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphycr](index.html) module"]
pub struct USBPHYCR_SPEC;
impl crate::RegisterSpec for USBPHYCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphycr::R](R) reader structure"]
impl crate::Readable for USBPHYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphycr::W](W) writer structure"]
impl crate::Writable for USBPHYCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPHYCR to value 0"]
impl crate::Resettable for USBPHYCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
