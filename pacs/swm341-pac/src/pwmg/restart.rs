#[doc = "Register `RESTART` reader"]
pub struct R(crate::R<RESTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESTART` writer"]
pub struct W(crate::W<RESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESTART_SPEC>;
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
impl From<crate::W<RESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM0` reader - PWM0 field"]
pub type PWM0_R = crate::BitReader<bool>;
#[doc = "Field `PWM0` writer - PWM0 field"]
pub type PWM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
#[doc = "Field `PWM1` reader - PWM1 field"]
pub type PWM1_R = crate::BitReader<bool>;
#[doc = "Field `PWM1` writer - PWM1 field"]
pub type PWM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
#[doc = "Field `PWM2` reader - PWM2 field"]
pub type PWM2_R = crate::BitReader<bool>;
#[doc = "Field `PWM2` writer - PWM2 field"]
pub type PWM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
#[doc = "Field `PWM3` reader - PWM3 field"]
pub type PWM3_R = crate::BitReader<bool>;
#[doc = "Field `PWM3` writer - PWM3 field"]
pub type PWM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
#[doc = "Field `PWM4` reader - PWM4 field"]
pub type PWM4_R = crate::BitReader<bool>;
#[doc = "Field `PWM4` writer - PWM4 field"]
pub type PWM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESTART_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - PWM0 field"]
    #[inline(always)]
    pub fn pwm0(&self) -> PWM0_R {
        PWM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM1 field"]
    #[inline(always)]
    pub fn pwm1(&self) -> PWM1_R {
        PWM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM2 field"]
    #[inline(always)]
    pub fn pwm2(&self) -> PWM2_R {
        PWM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM3 field"]
    #[inline(always)]
    pub fn pwm3(&self) -> PWM3_R {
        PWM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM4 field"]
    #[inline(always)]
    pub fn pwm4(&self) -> PWM4_R {
        PWM4_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PWM0 field"]
    #[inline(always)]
    pub fn pwm0(&mut self) -> PWM0_W<8> {
        PWM0_W::new(self)
    }
    #[doc = "Bit 9 - PWM1 field"]
    #[inline(always)]
    pub fn pwm1(&mut self) -> PWM1_W<9> {
        PWM1_W::new(self)
    }
    #[doc = "Bit 10 - PWM2 field"]
    #[inline(always)]
    pub fn pwm2(&mut self) -> PWM2_W<10> {
        PWM2_W::new(self)
    }
    #[doc = "Bit 11 - PWM3 field"]
    #[inline(always)]
    pub fn pwm3(&mut self) -> PWM3_W<11> {
        PWM3_W::new(self)
    }
    #[doc = "Bit 12 - PWM4 field"]
    #[inline(always)]
    pub fn pwm4(&mut self) -> PWM4_W<12> {
        PWM4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESTART register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [restart](index.html) module"]
pub struct RESTART_SPEC;
impl crate::RegisterSpec for RESTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [restart::R](R) reader structure"]
impl crate::Readable for RESTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [restart::W](W) writer structure"]
impl crate::Writable for RESTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RESTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
