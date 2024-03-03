#[doc = "Register `RESET` reader"]
pub struct R(crate::R<RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET` writer"]
pub struct W(crate::W<RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_SPEC>;
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
impl From<crate::W<RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM0` reader - PWM0 field"]
pub type PWM0_R = crate::BitReader<bool>;
#[doc = "Field `PWM0` writer - PWM0 field"]
pub type PWM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `PWM1` reader - PWM1 field"]
pub type PWM1_R = crate::BitReader<bool>;
#[doc = "Field `PWM1` writer - PWM1 field"]
pub type PWM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `PWM2` reader - PWM2 field"]
pub type PWM2_R = crate::BitReader<bool>;
#[doc = "Field `PWM2` writer - PWM2 field"]
pub type PWM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `PWM3` reader - PWM3 field"]
pub type PWM3_R = crate::BitReader<bool>;
#[doc = "Field `PWM3` writer - PWM3 field"]
pub type PWM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
#[doc = "Field `PWM4` reader - PWM4 field"]
pub type PWM4_R = crate::BitReader<bool>;
#[doc = "Field `PWM4` writer - PWM4 field"]
pub type PWM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWM0 field"]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 field"]
    #[inline(always)]
    pub fn pwm1(&self) -> PWM1_R {
        PWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 field"]
    #[inline(always)]
    pub fn pwm2(&self) -> PWM2_R {
        PWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 field"]
    #[inline(always)]
    pub fn pwm3(&self) -> PWM3_R {
        PWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 field"]
    #[inline(always)]
    pub fn pwm4(&self) -> PWM4_R {
        PWM4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 field"]
    #[inline(always)]
    pub fn pwm0(&mut self) -> PWM0_W<0> {
        PWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 field"]
    #[inline(always)]
    pub fn pwm1(&mut self) -> PWM1_W<1> {
        PWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 field"]
    #[inline(always)]
    pub fn pwm2(&mut self) -> PWM2_W<2> {
        PWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 field"]
    #[inline(always)]
    pub fn pwm3(&mut self) -> PWM3_W<3> {
        PWM3_W::new(self)
    }
    #[doc = "Bit 4 - PWM4 field"]
    #[inline(always)]
    pub fn pwm4(&mut self) -> PWM4_W<4> {
        PWM4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](index.html) module"]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset::R](R) reader structure"]
impl crate::Readable for RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset::W](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
