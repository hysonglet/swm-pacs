#[doc = "Register `EVMSK` reader"]
pub struct R(crate::R<EVMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVMSK` writer"]
pub struct W(crate::W<EVMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVMSK_SPEC>;
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
impl From<crate::W<EVMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTA` reader - OUTA field"]
pub type OUTA_R = crate::BitReader<bool>;
#[doc = "Field `OUTA` writer - OUTA field"]
pub type OUTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVMSK_SPEC, bool, O>;
#[doc = "Field `OUTB` reader - OUTB field"]
pub type OUTB_R = crate::BitReader<bool>;
#[doc = "Field `OUTB` writer - OUTB field"]
pub type OUTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVMSK_SPEC, bool, O>;
#[doc = "Field `OUTAN` reader - OUTAN field"]
pub type OUTAN_R = crate::BitReader<bool>;
#[doc = "Field `OUTAN` writer - OUTAN field"]
pub type OUTAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVMSK_SPEC, bool, O>;
#[doc = "Field `OUTBN` reader - OUTBN field"]
pub type OUTBN_R = crate::BitReader<bool>;
#[doc = "Field `OUTBN` writer - OUTBN field"]
pub type OUTBN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVMSK_SPEC, bool, O>;
#[doc = "Field `IMME` reader - IMME field"]
pub type IMME_R = crate::BitReader<bool>;
#[doc = "Field `IMME` writer - IMME field"]
pub type IMME_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVMSK_SPEC, bool, O>;
#[doc = "Field `STPCLR` reader - STPCLR field"]
pub type STPCLR_R = crate::BitReader<bool>;
#[doc = "Field `STPCLR` writer - STPCLR field"]
pub type STPCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OUTA field"]
    #[inline(always)]
    pub fn outa(&self) -> OUTA_R {
        OUTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUTB field"]
    #[inline(always)]
    pub fn outb(&self) -> OUTB_R {
        OUTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUTAN field"]
    #[inline(always)]
    pub fn outan(&self) -> OUTAN_R {
        OUTAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUTBN field"]
    #[inline(always)]
    pub fn outbn(&self) -> OUTBN_R {
        OUTBN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IMME field"]
    #[inline(always)]
    pub fn imme(&self) -> IMME_R {
        IMME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - STPCLR field"]
    #[inline(always)]
    pub fn stpclr(&self) -> STPCLR_R {
        STPCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUTA field"]
    #[inline(always)]
    pub fn outa(&mut self) -> OUTA_W<0> {
        OUTA_W::new(self)
    }
    #[doc = "Bit 1 - OUTB field"]
    #[inline(always)]
    pub fn outb(&mut self) -> OUTB_W<1> {
        OUTB_W::new(self)
    }
    #[doc = "Bit 2 - OUTAN field"]
    #[inline(always)]
    pub fn outan(&mut self) -> OUTAN_W<2> {
        OUTAN_W::new(self)
    }
    #[doc = "Bit 3 - OUTBN field"]
    #[inline(always)]
    pub fn outbn(&mut self) -> OUTBN_W<3> {
        OUTBN_W::new(self)
    }
    #[doc = "Bit 4 - IMME field"]
    #[inline(always)]
    pub fn imme(&mut self) -> IMME_W<4> {
        IMME_W::new(self)
    }
    #[doc = "Bit 8 - STPCLR field"]
    #[inline(always)]
    pub fn stpclr(&mut self) -> STPCLR_W<8> {
        STPCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVMSK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evmsk](index.html) module"]
pub struct EVMSK_SPEC;
impl crate::RegisterSpec for EVMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evmsk::R](R) reader structure"]
impl crate::Readable for EVMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evmsk::W](W) writer structure"]
impl crate::Writable for EVMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVMSK to value 0"]
impl crate::Resettable for EVMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
