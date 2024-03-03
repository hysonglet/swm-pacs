#[doc = "Register `DEVCR` reader"]
pub struct R(crate::R<DEVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCR` writer"]
pub struct W(crate::W<DEVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCR_SPEC>;
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
impl From<crate::W<DEVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPEED` reader - SPEED field"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - SPEED field"]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `REMOTEWKUP` reader - REMOTEWKUP field"]
pub type REMOTEWKUP_R = crate::BitReader<bool>;
#[doc = "Field `REMOTEWKUP` writer - REMOTEWKUP field"]
pub type REMOTEWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCR_SPEC, bool, O>;
#[doc = "Field `SELFPWRD` reader - SELFPWRD field"]
pub type SELFPWRD_R = crate::BitReader<bool>;
#[doc = "Field `SELFPWRD` writer - SELFPWRD field"]
pub type SELFPWRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCR_SPEC, bool, O>;
#[doc = "Field `SYNCFRAME` reader - SYNCFRAME field"]
pub type SYNCFRAME_R = crate::BitReader<bool>;
#[doc = "Field `SYNCFRAME` writer - SYNCFRAME field"]
pub type SYNCFRAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCR_SPEC, bool, O>;
#[doc = "Field `CSRDONE` reader - CSRDONE field"]
pub type CSRDONE_R = crate::BitReader<bool>;
#[doc = "Field `CSRDONE` writer - CSRDONE field"]
pub type CSRDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCR_SPEC, bool, O>;
#[doc = "Field `SETDESC` reader - SETDESC field"]
pub type SETDESC_R = crate::BitReader<bool>;
#[doc = "Field `SETDESC` writer - SETDESC field"]
pub type SETDESC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCR_SPEC, bool, O>;
#[doc = "Field `DEVICE` reader - DEVICE field"]
pub type DEVICE_R = crate::BitReader<bool>;
#[doc = "Field `DEVICE` writer - DEVICE field"]
pub type DEVICE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SPEED field"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - REMOTEWKUP field"]
    #[inline(always)]
    pub fn remotewkup(&self) -> REMOTEWKUP_R {
        REMOTEWKUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SELFPWRD field"]
    #[inline(always)]
    pub fn selfpwrd(&self) -> SELFPWRD_R {
        SELFPWRD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SYNCFRAME field"]
    #[inline(always)]
    pub fn syncframe(&self) -> SYNCFRAME_R {
        SYNCFRAME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CSRDONE field"]
    #[inline(always)]
    pub fn csrdone(&self) -> CSRDONE_R {
        CSRDONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SETDESC field"]
    #[inline(always)]
    pub fn setdesc(&self) -> SETDESC_R {
        SETDESC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DEVICE field"]
    #[inline(always)]
    pub fn device(&self) -> DEVICE_R {
        DEVICE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPEED field"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W<0> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 2 - REMOTEWKUP field"]
    #[inline(always)]
    pub fn remotewkup(&mut self) -> REMOTEWKUP_W<2> {
        REMOTEWKUP_W::new(self)
    }
    #[doc = "Bit 3 - SELFPWRD field"]
    #[inline(always)]
    pub fn selfpwrd(&mut self) -> SELFPWRD_W<3> {
        SELFPWRD_W::new(self)
    }
    #[doc = "Bit 4 - SYNCFRAME field"]
    #[inline(always)]
    pub fn syncframe(&mut self) -> SYNCFRAME_W<4> {
        SYNCFRAME_W::new(self)
    }
    #[doc = "Bit 6 - CSRDONE field"]
    #[inline(always)]
    pub fn csrdone(&mut self) -> CSRDONE_W<6> {
        CSRDONE_W::new(self)
    }
    #[doc = "Bit 7 - SETDESC field"]
    #[inline(always)]
    pub fn setdesc(&mut self) -> SETDESC_W<7> {
        SETDESC_W::new(self)
    }
    #[doc = "Bit 8 - DEVICE field"]
    #[inline(always)]
    pub fn device(&mut self) -> DEVICE_W<8> {
        DEVICE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcr](index.html) module"]
pub struct DEVCR_SPEC;
impl crate::RegisterSpec for DEVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devcr::R](R) reader structure"]
impl crate::Readable for DEVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devcr::W](W) writer structure"]
impl crate::Writable for DEVCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVCR to value 0"]
impl crate::Resettable for DEVCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
