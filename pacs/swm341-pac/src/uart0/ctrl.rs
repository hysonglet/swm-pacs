#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXIDLE` reader - TXIDLE field"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` writer - TXIDLE field"]
pub type TXIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXFF` reader - TXFF field"]
pub type TXFF_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` writer - TXFF field"]
pub type TXFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXIE` reader - TXIE field"]
pub type TXIE_R = crate::BitReader<bool>;
#[doc = "Field `TXIE` writer - TXIE field"]
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXNE` reader - RXNE field"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` writer - RXNE field"]
pub type RXNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXIE` reader - RXIE field"]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - RXIE field"]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXOV` reader - RXOV field"]
pub type RXOV_R = crate::BitReader<bool>;
#[doc = "Field `RXOV` writer - RXOV field"]
pub type RXOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXDOIE` reader - TXDOIE field"]
pub type TXDOIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDOIE` writer - TXDOIE field"]
pub type TXDOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LOOP` reader - LOOP field"]
pub type LOOP_R = crate::BitReader<bool>;
#[doc = "Field `LOOP` writer - LOOP field"]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TOIE` reader - TOIE field"]
pub type TOIE_R = crate::BitReader<bool>;
#[doc = "Field `TOIE` writer - TOIE field"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DATA9b` reader - DATA9b field"]
pub type DATA9B_R = crate::BitReader<bool>;
#[doc = "Field `DATA9b` writer - DATA9b field"]
pub type DATA9B_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PARITY` reader - PARITY field"]
pub type PARITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PARITY` writer - PARITY field"]
pub type PARITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `STOP2b` reader - STOP2b field"]
pub type STOP2B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP2b` writer - STOP2b field"]
pub type STOP2B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - TXIDLE field"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFF field"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXIE field"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXNE field"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXIE field"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXDOIE field"]
    #[inline(always)]
    pub fn txdoie(&self) -> TXDOIE_R {
        TXDOIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LOOP field"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - TOIE field"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - DATA9b field"]
    #[inline(always)]
    pub fn data9b(&self) -> DATA9B_R {
        DATA9B_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - PARITY field"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - STOP2b field"]
    #[inline(always)]
    pub fn stop2b(&self) -> STOP2B_R {
        STOP2B_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXIDLE field"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TXIDLE_W<0> {
        TXIDLE_W::new(self)
    }
    #[doc = "Bit 1 - TXFF field"]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W<1> {
        TXFF_W::new(self)
    }
    #[doc = "Bit 2 - TXIE field"]
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W<2> {
        TXIE_W::new(self)
    }
    #[doc = "Bit 3 - RXNE field"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<3> {
        RXNE_W::new(self)
    }
    #[doc = "Bit 4 - RXIE field"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W<4> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 5 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&mut self) -> RXOV_W<5> {
        RXOV_W::new(self)
    }
    #[doc = "Bit 6 - TXDOIE field"]
    #[inline(always)]
    pub fn txdoie(&mut self) -> TXDOIE_W<6> {
        TXDOIE_W::new(self)
    }
    #[doc = "Bit 9 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<9> {
        EN_W::new(self)
    }
    #[doc = "Bit 10 - LOOP field"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W<10> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 14 - TOIE field"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<14> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 18 - DATA9b field"]
    #[inline(always)]
    pub fn data9b(&mut self) -> DATA9B_W<18> {
        DATA9B_W::new(self)
    }
    #[doc = "Bits 19:21 - PARITY field"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<19> {
        PARITY_W::new(self)
    }
    #[doc = "Bits 22:23 - STOP2b field"]
    #[inline(always)]
    pub fn stop2b(&mut self) -> STOP2B_W<22> {
        STOP2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
