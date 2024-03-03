#[doc = "Register `SWBRK` reader"]
pub struct R(crate::R<SWBRK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWBRK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWBRK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWBRK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWBRK` writer"]
pub struct W(crate::W<SWBRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWBRK_SPEC>;
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
impl From<crate::W<SWBRK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWBRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM0A` reader - PWM0A field"]
pub type PWM0A_R = crate::BitReader<bool>;
#[doc = "Field `PWM0A` writer - PWM0A field"]
pub type PWM0A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM1A` reader - PWM1A field"]
pub type PWM1A_R = crate::BitReader<bool>;
#[doc = "Field `PWM1A` writer - PWM1A field"]
pub type PWM1A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM2A` reader - PWM2A field"]
pub type PWM2A_R = crate::BitReader<bool>;
#[doc = "Field `PWM2A` writer - PWM2A field"]
pub type PWM2A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM3A` reader - PWM3A field"]
pub type PWM3A_R = crate::BitReader<bool>;
#[doc = "Field `PWM3A` writer - PWM3A field"]
pub type PWM3A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM4A` reader - PWM4A field"]
pub type PWM4A_R = crate::BitReader<bool>;
#[doc = "Field `PWM4A` writer - PWM4A field"]
pub type PWM4A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM0B` reader - PWM0B field"]
pub type PWM0B_R = crate::BitReader<bool>;
#[doc = "Field `PWM0B` writer - PWM0B field"]
pub type PWM0B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM1B` reader - PWM1B field"]
pub type PWM1B_R = crate::BitReader<bool>;
#[doc = "Field `PWM1B` writer - PWM1B field"]
pub type PWM1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM2B` reader - PWM2B field"]
pub type PWM2B_R = crate::BitReader<bool>;
#[doc = "Field `PWM2B` writer - PWM2B field"]
pub type PWM2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM3B` reader - PWM3B field"]
pub type PWM3B_R = crate::BitReader<bool>;
#[doc = "Field `PWM3B` writer - PWM3B field"]
pub type PWM3B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
#[doc = "Field `PWM4B` reader - PWM4B field"]
pub type PWM4B_R = crate::BitReader<bool>;
#[doc = "Field `PWM4B` writer - PWM4B field"]
pub type PWM4B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWBRK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWM0A field"]
    #[inline(always)]
    pub fn pwm0a(&self) -> PWM0A_R {
        PWM0A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1A field"]
    #[inline(always)]
    pub fn pwm1a(&self) -> PWM1A_R {
        PWM1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2A field"]
    #[inline(always)]
    pub fn pwm2a(&self) -> PWM2A_R {
        PWM2A_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3A field"]
    #[inline(always)]
    pub fn pwm3a(&self) -> PWM3A_R {
        PWM3A_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4A field"]
    #[inline(always)]
    pub fn pwm4a(&self) -> PWM4A_R {
        PWM4A_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM0B field"]
    #[inline(always)]
    pub fn pwm0b(&self) -> PWM0B_R {
        PWM0B_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM1B field"]
    #[inline(always)]
    pub fn pwm1b(&self) -> PWM1B_R {
        PWM1B_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM2B field"]
    #[inline(always)]
    pub fn pwm2b(&self) -> PWM2B_R {
        PWM2B_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM3B field"]
    #[inline(always)]
    pub fn pwm3b(&self) -> PWM3B_R {
        PWM3B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM4B field"]
    #[inline(always)]
    pub fn pwm4b(&self) -> PWM4B_R {
        PWM4B_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0A field"]
    #[inline(always)]
    pub fn pwm0a(&mut self) -> PWM0A_W<0> {
        PWM0A_W::new(self)
    }
    #[doc = "Bit 1 - PWM1A field"]
    #[inline(always)]
    pub fn pwm1a(&mut self) -> PWM1A_W<1> {
        PWM1A_W::new(self)
    }
    #[doc = "Bit 2 - PWM2A field"]
    #[inline(always)]
    pub fn pwm2a(&mut self) -> PWM2A_W<2> {
        PWM2A_W::new(self)
    }
    #[doc = "Bit 3 - PWM3A field"]
    #[inline(always)]
    pub fn pwm3a(&mut self) -> PWM3A_W<3> {
        PWM3A_W::new(self)
    }
    #[doc = "Bit 4 - PWM4A field"]
    #[inline(always)]
    pub fn pwm4a(&mut self) -> PWM4A_W<4> {
        PWM4A_W::new(self)
    }
    #[doc = "Bit 8 - PWM0B field"]
    #[inline(always)]
    pub fn pwm0b(&mut self) -> PWM0B_W<8> {
        PWM0B_W::new(self)
    }
    #[doc = "Bit 9 - PWM1B field"]
    #[inline(always)]
    pub fn pwm1b(&mut self) -> PWM1B_W<9> {
        PWM1B_W::new(self)
    }
    #[doc = "Bit 10 - PWM2B field"]
    #[inline(always)]
    pub fn pwm2b(&mut self) -> PWM2B_W<10> {
        PWM2B_W::new(self)
    }
    #[doc = "Bit 11 - PWM3B field"]
    #[inline(always)]
    pub fn pwm3b(&mut self) -> PWM3B_W<11> {
        PWM3B_W::new(self)
    }
    #[doc = "Bit 12 - PWM4B field"]
    #[inline(always)]
    pub fn pwm4b(&mut self) -> PWM4B_W<12> {
        PWM4B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SWBRK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swbrk](index.html) module"]
pub struct SWBRK_SPEC;
impl crate::RegisterSpec for SWBRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swbrk::R](R) reader structure"]
impl crate::Readable for SWBRK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swbrk::W](W) writer structure"]
impl crate::Writable for SWBRK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWBRK to value 0"]
impl crate::Resettable for SWBRK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
