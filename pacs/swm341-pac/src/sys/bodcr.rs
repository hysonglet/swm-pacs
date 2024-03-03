#[doc = "Register `BODCR` reader"]
pub struct R(crate::R<BODCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCR` writer"]
pub struct W(crate::W<BODCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODCR_SPEC>;
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
impl From<crate::W<BODCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - IE field"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - IE field"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BODCR_SPEC, bool, O>;
#[doc = "Field `INTLVL` reader - INTLVL field"]
pub type INTLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTLVL` writer - INTLVL field"]
pub type INTLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BODCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSTLVL` reader - RSTLVL field"]
pub type RSTLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTLVL` writer - RSTLVL field"]
pub type RSTLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BODCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 1 - IE field"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - INTLVL field"]
    #[inline(always)]
    pub fn intlvl(&self) -> INTLVL_R {
        INTLVL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - RSTLVL field"]
    #[inline(always)]
    pub fn rstlvl(&self) -> RSTLVL_R {
        RSTLVL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - IE field"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<1> {
        IE_W::new(self)
    }
    #[doc = "Bits 4:6 - INTLVL field"]
    #[inline(always)]
    pub fn intlvl(&mut self) -> INTLVL_W<4> {
        INTLVL_W::new(self)
    }
    #[doc = "Bits 7:9 - RSTLVL field"]
    #[inline(always)]
    pub fn rstlvl(&mut self) -> RSTLVL_W<7> {
        RSTLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BODCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcr](index.html) module"]
pub struct BODCR_SPEC;
impl crate::RegisterSpec for BODCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodcr::R](R) reader structure"]
impl crate::Readable for BODCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodcr::W](W) writer structure"]
impl crate::Writable for BODCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODCR to value 0"]
impl crate::Resettable for BODCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
