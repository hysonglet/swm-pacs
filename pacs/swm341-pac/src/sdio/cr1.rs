#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `_4BIT` reader - _4BIT field"]
pub type _4BIT_R = crate::BitReader<bool>;
#[doc = "Field `_4BIT` writer - _4BIT field"]
pub type _4BIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `_8BIT` reader - _8BIT field"]
pub type _8BIT_R = crate::BitReader<bool>;
#[doc = "Field `_8BIT` writer - _8BIT field"]
pub type _8BIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CDBIT` reader - CDBIT field"]
pub type CDBIT_R = crate::BitReader<bool>;
#[doc = "Field `CDBIT` writer - CDBIT field"]
pub type CDBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CDSRC` reader - CDSRC field"]
pub type CDSRC_R = crate::BitReader<bool>;
#[doc = "Field `CDSRC` writer - CDSRC field"]
pub type CDSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PWRON` reader - PWRON field"]
pub type PWRON_R = crate::BitReader<bool>;
#[doc = "Field `PWRON` writer - PWRON field"]
pub type PWRON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `VOLT` reader - VOLT field"]
pub type VOLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOLT` writer - VOLT field"]
pub type VOLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 1 - _4BIT field"]
    #[inline(always)]
    pub fn _4bit(&self) -> _4BIT_R {
        _4BIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - _8BIT field"]
    #[inline(always)]
    pub fn _8bit(&self) -> _8BIT_R {
        _8BIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CDBIT field"]
    #[inline(always)]
    pub fn cdbit(&self) -> CDBIT_R {
        CDBIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CDSRC field"]
    #[inline(always)]
    pub fn cdsrc(&self) -> CDSRC_R {
        CDSRC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PWRON field"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - VOLT field"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - _4BIT field"]
    #[inline(always)]
    pub fn _4bit(&mut self) -> _4BIT_W<1> {
        _4BIT_W::new(self)
    }
    #[doc = "Bit 5 - _8BIT field"]
    #[inline(always)]
    pub fn _8bit(&mut self) -> _8BIT_W<5> {
        _8BIT_W::new(self)
    }
    #[doc = "Bit 6 - CDBIT field"]
    #[inline(always)]
    pub fn cdbit(&mut self) -> CDBIT_W<6> {
        CDBIT_W::new(self)
    }
    #[doc = "Bit 7 - CDSRC field"]
    #[inline(always)]
    pub fn cdsrc(&mut self) -> CDSRC_W<7> {
        CDSRC_W::new(self)
    }
    #[doc = "Bit 8 - PWRON field"]
    #[inline(always)]
    pub fn pwron(&mut self) -> PWRON_W<8> {
        PWRON_W::new(self)
    }
    #[doc = "Bits 9:11 - VOLT field"]
    #[inline(always)]
    pub fn volt(&mut self) -> VOLT_W<9> {
        VOLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
