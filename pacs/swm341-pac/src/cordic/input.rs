#[doc = "Register `INPUT` reader"]
pub struct R(crate::R<INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUT` writer"]
pub struct W(crate::W<INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUT_SPEC>;
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
impl From<crate::W<INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F` reader - F field"]
pub type F_R = crate::FieldReader<u16, u16>;
#[doc = "Field `F` writer - F field"]
pub type F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUT_SPEC, u16, u16, 14, O>;
#[doc = "Field `I` reader - I field"]
pub type I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I` writer - I field"]
pub type I_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `F2` reader - F2 field"]
pub type F2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `F2` writer - F2 field"]
pub type F2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUT_SPEC, u16, u16, 14, O>;
#[doc = "Field `I2` reader - I2 field"]
pub type I2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2` writer - I2 field"]
pub type I2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPUT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:13 - F field"]
    #[inline(always)]
    pub fn f(&self) -> F_R {
        F_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - I field"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:29 - F2 field"]
    #[inline(always)]
    pub fn f2(&self) -> F2_R {
        F2_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 30:31 - I2 field"]
    #[inline(always)]
    pub fn i2(&self) -> I2_R {
        I2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - F field"]
    #[inline(always)]
    pub fn f(&mut self) -> F_W<0> {
        F_W::new(self)
    }
    #[doc = "Bits 14:15 - I field"]
    #[inline(always)]
    pub fn i(&mut self) -> I_W<14> {
        I_W::new(self)
    }
    #[doc = "Bits 16:29 - F2 field"]
    #[inline(always)]
    pub fn f2(&mut self) -> F2_W<16> {
        F2_W::new(self)
    }
    #[doc = "Bits 30:31 - I2 field"]
    #[inline(always)]
    pub fn i2(&mut self) -> I2_W<30> {
        I2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INPUT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](index.html) module"]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input::R](R) reader structure"]
impl crate::Readable for INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [input::W](W) writer structure"]
impl crate::Writable for INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
