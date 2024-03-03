#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN0` reader - PIN0 field"]
pub type PIN0_R = crate::BitReader<bool>;
#[doc = "Field `PIN0` writer - PIN0 field"]
pub type PIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN1` reader - PIN1 field"]
pub type PIN1_R = crate::BitReader<bool>;
#[doc = "Field `PIN1` writer - PIN1 field"]
pub type PIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN2` reader - PIN2 field"]
pub type PIN2_R = crate::BitReader<bool>;
#[doc = "Field `PIN2` writer - PIN2 field"]
pub type PIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN3` reader - PIN3 field"]
pub type PIN3_R = crate::BitReader<bool>;
#[doc = "Field `PIN3` writer - PIN3 field"]
pub type PIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN4` reader - PIN4 field"]
pub type PIN4_R = crate::BitReader<bool>;
#[doc = "Field `PIN4` writer - PIN4 field"]
pub type PIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN5` reader - PIN5 field"]
pub type PIN5_R = crate::BitReader<bool>;
#[doc = "Field `PIN5` writer - PIN5 field"]
pub type PIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN6` reader - PIN6 field"]
pub type PIN6_R = crate::BitReader<bool>;
#[doc = "Field `PIN6` writer - PIN6 field"]
pub type PIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN7` reader - PIN7 field"]
pub type PIN7_R = crate::BitReader<bool>;
#[doc = "Field `PIN7` writer - PIN7 field"]
pub type PIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN8` reader - PIN8 field"]
pub type PIN8_R = crate::BitReader<bool>;
#[doc = "Field `PIN8` writer - PIN8 field"]
pub type PIN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN9` reader - PIN9 field"]
pub type PIN9_R = crate::BitReader<bool>;
#[doc = "Field `PIN9` writer - PIN9 field"]
pub type PIN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN10` reader - PIN10 field"]
pub type PIN10_R = crate::BitReader<bool>;
#[doc = "Field `PIN10` writer - PIN10 field"]
pub type PIN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN11` reader - PIN11 field"]
pub type PIN11_R = crate::BitReader<bool>;
#[doc = "Field `PIN11` writer - PIN11 field"]
pub type PIN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN12` reader - PIN12 field"]
pub type PIN12_R = crate::BitReader<bool>;
#[doc = "Field `PIN12` writer - PIN12 field"]
pub type PIN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN13` reader - PIN13 field"]
pub type PIN13_R = crate::BitReader<bool>;
#[doc = "Field `PIN13` writer - PIN13 field"]
pub type PIN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN14` reader - PIN14 field"]
pub type PIN14_R = crate::BitReader<bool>;
#[doc = "Field `PIN14` writer - PIN14 field"]
pub type PIN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `PIN15` reader - PIN15 field"]
pub type PIN15_R = crate::BitReader<bool>;
#[doc = "Field `PIN15` writer - PIN15 field"]
pub type PIN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PIN0 field"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIN1 field"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PIN2 field"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PIN3 field"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PIN4 field"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PIN5 field"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PIN6 field"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PIN7 field"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PIN8 field"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PIN9 field"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PIN10 field"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PIN11 field"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PIN12 field"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PIN13 field"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PIN14 field"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PIN15 field"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIN0 field"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W<0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - PIN1 field"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W<1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2 - PIN2 field"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W<2> {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3 - PIN3 field"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W<3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4 - PIN4 field"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W<4> {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5 - PIN5 field"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W<5> {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6 - PIN6 field"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W<6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - PIN7 field"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W<7> {
        PIN7_W::new(self)
    }
    #[doc = "Bit 8 - PIN8 field"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W<8> {
        PIN8_W::new(self)
    }
    #[doc = "Bit 9 - PIN9 field"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W<9> {
        PIN9_W::new(self)
    }
    #[doc = "Bit 10 - PIN10 field"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W<10> {
        PIN10_W::new(self)
    }
    #[doc = "Bit 11 - PIN11 field"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W<11> {
        PIN11_W::new(self)
    }
    #[doc = "Bit 12 - PIN12 field"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W<12> {
        PIN12_W::new(self)
    }
    #[doc = "Bit 13 - PIN13 field"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W<13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - PIN14 field"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W<14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - PIN15 field"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W<15> {
        PIN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ODR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
