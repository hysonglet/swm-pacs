#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXE` reader - TXE field"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - TXE field"]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXNE` reader - RXNE field"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` writer - RXNE field"]
pub type RXNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXOV` reader - RXOV field"]
pub type RXOV_R = crate::BitReader<bool>;
#[doc = "Field `RXOV` writer - RXOV field"]
pub type RXOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TXDONE` reader - TXDONE field"]
pub type TXDONE_R = crate::BitReader<bool>;
#[doc = "Field `TXDONE` writer - TXDONE field"]
pub type TXDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXDONE` reader - RXDONE field"]
pub type RXDONE_R = crate::BitReader<bool>;
#[doc = "Field `RXDONE` writer - RXDONE field"]
pub type RXDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXSTA` reader - RXSTA field"]
pub type RXSTA_R = crate::BitReader<bool>;
#[doc = "Field `RXSTA` writer - RXSTA field"]
pub type RXSTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RXSTO` reader - RXSTO field"]
pub type RXSTO_R = crate::BitReader<bool>;
#[doc = "Field `RXSTO` writer - RXSTO field"]
pub type RXSTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `AL` reader - AL field"]
pub type AL_R = crate::BitReader<bool>;
#[doc = "Field `AL` writer - AL field"]
pub type AL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `MLTO` reader - MLTO field"]
pub type MLTO_R = crate::BitReader<bool>;
#[doc = "Field `MLTO` writer - MLTO field"]
pub type MLTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TXE field"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXNE field"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXDONE field"]
    #[inline(always)]
    pub fn txdone(&self) -> TXDONE_R {
        TXDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXDONE field"]
    #[inline(always)]
    pub fn rxdone(&self) -> RXDONE_R {
        RXDONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RXSTA field"]
    #[inline(always)]
    pub fn rxsta(&self) -> RXSTA_R {
        RXSTA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXSTO field"]
    #[inline(always)]
    pub fn rxsto(&self) -> RXSTO_R {
        RXSTO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - AL field"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MLTO field"]
    #[inline(always)]
    pub fn mlto(&self) -> MLTO_R {
        MLTO_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXE field"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W<0> {
        TXE_W::new(self)
    }
    #[doc = "Bit 1 - RXNE field"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<1> {
        RXNE_W::new(self)
    }
    #[doc = "Bit 2 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&mut self) -> RXOV_W<2> {
        RXOV_W::new(self)
    }
    #[doc = "Bit 3 - TXDONE field"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TXDONE_W<3> {
        TXDONE_W::new(self)
    }
    #[doc = "Bit 4 - RXDONE field"]
    #[inline(always)]
    pub fn rxdone(&mut self) -> RXDONE_W<4> {
        RXDONE_W::new(self)
    }
    #[doc = "Bit 8 - RXSTA field"]
    #[inline(always)]
    pub fn rxsta(&mut self) -> RXSTA_W<8> {
        RXSTA_W::new(self)
    }
    #[doc = "Bit 9 - RXSTO field"]
    #[inline(always)]
    pub fn rxsto(&mut self) -> RXSTO_W<9> {
        RXSTO_W::new(self)
    }
    #[doc = "Bit 16 - AL field"]
    #[inline(always)]
    pub fn al(&mut self) -> AL_W<16> {
        AL_W::new(self)
    }
    #[doc = "Bit 17 - MLTO field"]
    #[inline(always)]
    pub fn mlto(&mut self) -> MLTO_W<17> {
        MLTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
