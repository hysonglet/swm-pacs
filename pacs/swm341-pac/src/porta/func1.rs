#[doc = "Register `FUNC1` reader"]
pub struct R(crate::R<FUNC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC1` writer"]
pub struct W(crate::W<FUNC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC1_SPEC>;
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
impl From<crate::W<FUNC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN8` reader - PIN8 field"]
pub type PIN8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN8` writer - PIN8 field"]
pub type PIN8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN9` reader - PIN9 field"]
pub type PIN9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN9` writer - PIN9 field"]
pub type PIN9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN10` reader - PIN10 field"]
pub type PIN10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN10` writer - PIN10 field"]
pub type PIN10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN11` reader - PIN11 field"]
pub type PIN11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN11` writer - PIN11 field"]
pub type PIN11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN12` reader - PIN12 field"]
pub type PIN12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN12` writer - PIN12 field"]
pub type PIN12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN13` reader - PIN13 field"]
pub type PIN13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN13` writer - PIN13 field"]
pub type PIN13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN14` reader - PIN14 field"]
pub type PIN14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN14` writer - PIN14 field"]
pub type PIN14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PIN15` reader - PIN15 field"]
pub type PIN15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN15` writer - PIN15 field"]
pub type PIN15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNC1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PIN8 field"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PIN9 field"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PIN10 field"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PIN11 field"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PIN12 field"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PIN13 field"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PIN14 field"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PIN15 field"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PIN8 field"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W<0> {
        PIN8_W::new(self)
    }
    #[doc = "Bits 4:7 - PIN9 field"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W<4> {
        PIN9_W::new(self)
    }
    #[doc = "Bits 8:11 - PIN10 field"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W<8> {
        PIN10_W::new(self)
    }
    #[doc = "Bits 12:15 - PIN11 field"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W<12> {
        PIN11_W::new(self)
    }
    #[doc = "Bits 16:19 - PIN12 field"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W<16> {
        PIN12_W::new(self)
    }
    #[doc = "Bits 20:23 - PIN13 field"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W<20> {
        PIN13_W::new(self)
    }
    #[doc = "Bits 24:27 - PIN14 field"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W<24> {
        PIN14_W::new(self)
    }
    #[doc = "Bits 28:31 - PIN15 field"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W<28> {
        PIN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FUNC1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func1](index.html) module"]
pub struct FUNC1_SPEC;
impl crate::RegisterSpec for FUNC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func1::R](R) reader structure"]
impl crate::Readable for FUNC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func1::W](W) writer structure"]
impl crate::Writable for FUNC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC1 to value 0"]
impl crate::Resettable for FUNC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
