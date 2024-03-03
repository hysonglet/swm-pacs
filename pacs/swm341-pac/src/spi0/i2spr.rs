#[doc = "Register `I2SPR` reader"]
pub struct R(crate::R<I2SPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SPR` writer"]
pub struct W(crate::W<I2SPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SPR_SPEC>;
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
impl From<crate::W<I2SPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCLKDIV` reader - MCLKDIV field"]
pub type MCLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCLKDIV` writer - MCLKDIV field"]
pub type MCLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SPR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SCLKDIV` reader - SCLKDIV field"]
pub type SCLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCLKDIV` writer - SCLKDIV field"]
pub type SCLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SPR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:5 - MCLKDIV field"]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:19 - SCLKDIV field"]
    #[inline(always)]
    pub fn sclkdiv(&self) -> SCLKDIV_R {
        SCLKDIV_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - MCLKDIV field"]
    #[inline(always)]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W<0> {
        MCLKDIV_W::new(self)
    }
    #[doc = "Bits 8:19 - SCLKDIV field"]
    #[inline(always)]
    pub fn sclkdiv(&mut self) -> SCLKDIV_W<8> {
        SCLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SPR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2spr](index.html) module"]
pub struct I2SPR_SPEC;
impl crate::RegisterSpec for I2SPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2spr::R](R) reader structure"]
impl crate::Readable for I2SPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2spr::W](W) writer structure"]
impl crate::Writable for I2SPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SPR to value 0"]
impl crate::Resettable for I2SPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
