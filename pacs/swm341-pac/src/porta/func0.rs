#[doc = "Register `FUNC0` reader"]
pub struct R(crate::R<FUNC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC0` writer"]
pub struct W(crate::W<FUNC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC0_SPEC>;
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
impl From<crate::W<FUNC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN0` reader - PIN0 field"]
pub type PIN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN0` writer - PIN0 field"]
pub type PIN0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN1` reader - PIN1 field"]
pub type PIN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN1` writer - PIN1 field"]
pub type PIN1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN2` reader - PIN2 field"]
pub type PIN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN2` writer - PIN2 field"]
pub type PIN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN3` reader - PIN3 field"]
pub type PIN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN3` writer - PIN3 field"]
pub type PIN3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN4` reader - PIN4 field"]
pub type PIN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN4` writer - PIN4 field"]
pub type PIN4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN5` reader - PIN5 field"]
pub type PIN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN5` writer - PIN5 field"]
pub type PIN5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN6` reader - PIN6 field"]
pub type PIN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN6` writer - PIN6 field"]
pub type PIN6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN7` reader - PIN7 field"]
pub type PIN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN7` writer - PIN7 field"]
pub type PIN7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PIN0 field"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PIN1 field"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PIN2 field"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PIN3 field"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PIN4 field"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PIN5 field"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PIN6 field"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PIN7 field"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PIN0 field"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W<0> {
        PIN0_W::new(self)
    }
    #[doc = "Bits 4:7 - PIN1 field"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W<4> {
        PIN1_W::new(self)
    }
    #[doc = "Bits 8:11 - PIN2 field"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W<8> {
        PIN2_W::new(self)
    }
    #[doc = "Bits 12:15 - PIN3 field"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W<12> {
        PIN3_W::new(self)
    }
    #[doc = "Bits 16:19 - PIN4 field"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W<16> {
        PIN4_W::new(self)
    }
    #[doc = "Bits 20:23 - PIN5 field"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W<20> {
        PIN5_W::new(self)
    }
    #[doc = "Bits 24:27 - PIN6 field"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W<24> {
        PIN6_W::new(self)
    }
    #[doc = "Bits 28:31 - PIN7 field"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W<28> {
        PIN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FUNC0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func0](index.html) module"]
pub struct FUNC0_SPEC;
impl crate::RegisterSpec for FUNC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func0::R](R) reader structure"]
impl crate::Readable for FUNC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func0::W](W) writer structure"]
impl crate::Writable for FUNC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC0 to value 0"]
impl crate::Resettable for FUNC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
