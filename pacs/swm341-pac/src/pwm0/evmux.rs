#[doc = "Register `EVMUX` reader"]
pub struct R(crate::R<EVMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVMUX` writer"]
pub struct W(crate::W<EVMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVMUX_SPEC>;
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
impl From<crate::W<EVMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - START field"]
pub type START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `START` writer - START field"]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `STOP` reader - STOP field"]
pub type STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP` writer - STOP field"]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `PAUSE` reader - PAUSE field"]
pub type PAUSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAUSE` writer - PAUSE field"]
pub type PAUSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `RELOAD` reader - RELOAD field"]
pub type RELOAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RELOAD` writer - RELOAD field"]
pub type RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `MASKA` reader - MASKA field"]
pub type MASKA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKA` writer - MASKA field"]
pub type MASKA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `MASKB` reader - MASKB field"]
pub type MASKB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKB` writer - MASKB field"]
pub type MASKB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `MASKAN` reader - MASKAN field"]
pub type MASKAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKAN` writer - MASKAN field"]
pub type MASKAN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `MASKBN` reader - MASKBN field"]
pub type MASKBN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKBN` writer - MASKBN field"]
pub type MASKBN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVMUX_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - START field"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - STOP field"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PAUSE field"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - RELOAD field"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MASKA field"]
    #[inline(always)]
    pub fn maska(&self) -> MASKA_R {
        MASKA_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - MASKB field"]
    #[inline(always)]
    pub fn maskb(&self) -> MASKB_R {
        MASKB_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - MASKAN field"]
    #[inline(always)]
    pub fn maskan(&self) -> MASKAN_R {
        MASKAN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - MASKBN field"]
    #[inline(always)]
    pub fn maskbn(&self) -> MASKBN_R {
        MASKBN_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - START field"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 4:6 - STOP field"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<4> {
        STOP_W::new(self)
    }
    #[doc = "Bits 8:10 - PAUSE field"]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W<8> {
        PAUSE_W::new(self)
    }
    #[doc = "Bits 12:14 - RELOAD field"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<12> {
        RELOAD_W::new(self)
    }
    #[doc = "Bits 16:18 - MASKA field"]
    #[inline(always)]
    pub fn maska(&mut self) -> MASKA_W<16> {
        MASKA_W::new(self)
    }
    #[doc = "Bits 20:22 - MASKB field"]
    #[inline(always)]
    pub fn maskb(&mut self) -> MASKB_W<20> {
        MASKB_W::new(self)
    }
    #[doc = "Bits 24:26 - MASKAN field"]
    #[inline(always)]
    pub fn maskan(&mut self) -> MASKAN_W<24> {
        MASKAN_W::new(self)
    }
    #[doc = "Bits 28:30 - MASKBN field"]
    #[inline(always)]
    pub fn maskbn(&mut self) -> MASKBN_W<28> {
        MASKBN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVMUX register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evmux](index.html) module"]
pub struct EVMUX_SPEC;
impl crate::RegisterSpec for EVMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evmux::R](R) reader structure"]
impl crate::Readable for EVMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evmux::W](W) writer structure"]
impl crate::Writable for EVMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVMUX to value 0"]
impl crate::Resettable for EVMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
