#[doc = "Register `EVSR` reader"]
pub struct R(crate::R<EVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSR` writer"]
pub struct W(crate::W<EVSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSR_SPEC>;
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
impl From<crate::W<EVSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV0` reader - EV0 field"]
pub type EV0_R = crate::BitReader<bool>;
#[doc = "Field `EV0` writer - EV0 field"]
pub type EV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
#[doc = "Field `EV1` reader - EV1 field"]
pub type EV1_R = crate::BitReader<bool>;
#[doc = "Field `EV1` writer - EV1 field"]
pub type EV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
#[doc = "Field `EV2` reader - EV2 field"]
pub type EV2_R = crate::BitReader<bool>;
#[doc = "Field `EV2` writer - EV2 field"]
pub type EV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
#[doc = "Field `EV3` reader - EV3 field"]
pub type EV3_R = crate::BitReader<bool>;
#[doc = "Field `EV3` writer - EV3 field"]
pub type EV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
#[doc = "Field `EV4` reader - EV4 field"]
pub type EV4_R = crate::BitReader<bool>;
#[doc = "Field `EV4` writer - EV4 field"]
pub type EV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
#[doc = "Field `EV5` reader - EV5 field"]
pub type EV5_R = crate::BitReader<bool>;
#[doc = "Field `EV5` writer - EV5 field"]
pub type EV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
#[doc = "Field `EV6` reader - EV6 field"]
pub type EV6_R = crate::BitReader<bool>;
#[doc = "Field `EV6` writer - EV6 field"]
pub type EV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EV0 field"]
    #[inline(always)]
    pub fn ev0(&self) -> EV0_R {
        EV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EV1 field"]
    #[inline(always)]
    pub fn ev1(&self) -> EV1_R {
        EV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EV2 field"]
    #[inline(always)]
    pub fn ev2(&self) -> EV2_R {
        EV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EV3 field"]
    #[inline(always)]
    pub fn ev3(&self) -> EV3_R {
        EV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EV4 field"]
    #[inline(always)]
    pub fn ev4(&self) -> EV4_R {
        EV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EV5 field"]
    #[inline(always)]
    pub fn ev5(&self) -> EV5_R {
        EV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EV6 field"]
    #[inline(always)]
    pub fn ev6(&self) -> EV6_R {
        EV6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EV0 field"]
    #[inline(always)]
    pub fn ev0(&mut self) -> EV0_W<0> {
        EV0_W::new(self)
    }
    #[doc = "Bit 1 - EV1 field"]
    #[inline(always)]
    pub fn ev1(&mut self) -> EV1_W<1> {
        EV1_W::new(self)
    }
    #[doc = "Bit 2 - EV2 field"]
    #[inline(always)]
    pub fn ev2(&mut self) -> EV2_W<2> {
        EV2_W::new(self)
    }
    #[doc = "Bit 3 - EV3 field"]
    #[inline(always)]
    pub fn ev3(&mut self) -> EV3_W<3> {
        EV3_W::new(self)
    }
    #[doc = "Bit 4 - EV4 field"]
    #[inline(always)]
    pub fn ev4(&mut self) -> EV4_W<4> {
        EV4_W::new(self)
    }
    #[doc = "Bit 5 - EV5 field"]
    #[inline(always)]
    pub fn ev5(&mut self) -> EV5_W<5> {
        EV5_W::new(self)
    }
    #[doc = "Bit 6 - EV6 field"]
    #[inline(always)]
    pub fn ev6(&mut self) -> EV6_W<6> {
        EV6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evsr](index.html) module"]
pub struct EVSR_SPEC;
impl crate::RegisterSpec for EVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evsr::R](R) reader structure"]
impl crate::Readable for EVSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evsr::W](W) writer structure"]
impl crate::Writable for EVSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVSR to value 0"]
impl crate::Resettable for EVSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
