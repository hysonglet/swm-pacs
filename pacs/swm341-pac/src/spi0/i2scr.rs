#[doc = "Register `I2SCR` reader"]
pub struct R(crate::R<I2SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCR` writer"]
pub struct W(crate::W<I2SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCR_SPEC>;
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
impl From<crate::W<I2SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIEN` reader - DIEN field"]
pub type DIEN_R = crate::BitReader<bool>;
#[doc = "Field `DIEN` writer - DIEN field"]
pub type DIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `DOEN` reader - DOEN field"]
pub type DOEN_R = crate::BitReader<bool>;
#[doc = "Field `DOEN` writer - DOEN field"]
pub type DOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `MSTR` reader - MSTR field"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - MSTR field"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `FFMT` reader - FFMT field"]
pub type FFMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFMT` writer - FFMT field"]
pub type FFMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DLEN` reader - DLEN field"]
pub type DLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLEN` writer - DLEN field"]
pub type DLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCMSYNW` reader - PCMSYNW field"]
pub type PCMSYNW_R = crate::BitReader<bool>;
#[doc = "Field `PCMSYNW` writer - PCMSYNW field"]
pub type PCMSYNW_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `MCLKOE` reader - MCLKOE field"]
pub type MCLKOE_R = crate::BitReader<bool>;
#[doc = "Field `MCLKOE` writer - MCLKOE field"]
pub type MCLKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `CHLEN` reader - CHLEN field"]
pub type CHLEN_R = crate::BitReader<bool>;
#[doc = "Field `CHLEN` writer - CHLEN field"]
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
#[doc = "Field `CHRIGHT` reader - CHRIGHT field"]
pub type CHRIGHT_R = crate::BitReader<bool>;
#[doc = "Field `CHRIGHT` writer - CHRIGHT field"]
pub type CHRIGHT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIEN field"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOEN field"]
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSTR field"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FFMT field"]
    #[inline(always)]
    pub fn ffmt(&self) -> FFMT_R {
        FFMT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DLEN field"]
    #[inline(always)]
    pub fn dlen(&self) -> DLEN_R {
        DLEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - PCMSYNW field"]
    #[inline(always)]
    pub fn pcmsynw(&self) -> PCMSYNW_R {
        PCMSYNW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCLKOE field"]
    #[inline(always)]
    pub fn mclkoe(&self) -> MCLKOE_R {
        MCLKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CHLEN field"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - CHRIGHT field"]
    #[inline(always)]
    pub fn chright(&self) -> CHRIGHT_R {
        CHRIGHT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIEN field"]
    #[inline(always)]
    pub fn dien(&mut self) -> DIEN_W<0> {
        DIEN_W::new(self)
    }
    #[doc = "Bit 1 - DOEN field"]
    #[inline(always)]
    pub fn doen(&mut self) -> DOEN_W<1> {
        DOEN_W::new(self)
    }
    #[doc = "Bit 2 - MSTR field"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<2> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 3 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<3> {
        EN_W::new(self)
    }
    #[doc = "Bits 4:5 - FFMT field"]
    #[inline(always)]
    pub fn ffmt(&mut self) -> FFMT_W<4> {
        FFMT_W::new(self)
    }
    #[doc = "Bits 6:7 - DLEN field"]
    #[inline(always)]
    pub fn dlen(&mut self) -> DLEN_W<6> {
        DLEN_W::new(self)
    }
    #[doc = "Bit 8 - PCMSYNW field"]
    #[inline(always)]
    pub fn pcmsynw(&mut self) -> PCMSYNW_W<8> {
        PCMSYNW_W::new(self)
    }
    #[doc = "Bit 9 - MCLKOE field"]
    #[inline(always)]
    pub fn mclkoe(&mut self) -> MCLKOE_W<9> {
        MCLKOE_W::new(self)
    }
    #[doc = "Bit 10 - CHLEN field"]
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<10> {
        CHLEN_W::new(self)
    }
    #[doc = "Bit 16 - CHRIGHT field"]
    #[inline(always)]
    pub fn chright(&mut self) -> CHRIGHT_W<16> {
        CHRIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scr](index.html) module"]
pub struct I2SCR_SPEC;
impl crate::RegisterSpec for I2SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2scr::R](R) reader structure"]
impl crate::Readable for I2SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2scr::W](W) writer structure"]
impl crate::Writable for I2SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SCR to value 0"]
impl crate::Resettable for I2SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
