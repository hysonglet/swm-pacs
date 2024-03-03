#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD` reader - BAUD field"]
pub type BAUD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BAUD` writer - BAUD field"]
pub type BAUD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u16, u16, 14, O>;
#[doc = "Field `TXD` reader - TXD field"]
pub type TXD_R = crate::BitReader<bool>;
#[doc = "Field `TXD` writer - TXD field"]
pub type TXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `RXD` reader - RXD field"]
pub type RXD_R = crate::BitReader<bool>;
#[doc = "Field `RXD` writer - RXD field"]
pub type RXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `RXTOIF` reader - RXTOIF field"]
pub type RXTOIF_R = crate::BitReader<bool>;
#[doc = "Field `RXTOIF` writer - RXTOIF field"]
pub type RXTOIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `TXIF` reader - TXIF field"]
pub type TXIF_R = crate::BitReader<bool>;
#[doc = "Field `TXIF` writer - TXIF field"]
pub type TXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `RXTHRF` reader - RXTHRF field"]
pub type RXTHRF_R = crate::BitReader<bool>;
#[doc = "Field `RXTHRF` writer - RXTHRF field"]
pub type RXTHRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `TXTHRF` reader - TXTHRF field"]
pub type TXTHRF_R = crate::BitReader<bool>;
#[doc = "Field `TXTHRF` writer - TXTHRF field"]
pub type TXTHRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `TOIF` reader - TOIF field"]
pub type TOIF_R = crate::BitReader<bool>;
#[doc = "Field `TOIF` writer - TOIF field"]
pub type TOIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `RXIF` reader - RXIF field"]
pub type RXIF_R = crate::BitReader<bool>;
#[doc = "Field `RXIF` writer - RXIF field"]
pub type RXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `ABREN` reader - ABREN field"]
pub type ABREN_R = crate::BitReader<bool>;
#[doc = "Field `ABREN` writer - ABREN field"]
pub type ABREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `ABRBIT` reader - ABRBIT field"]
pub type ABRBIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABRBIT` writer - ABRBIT field"]
pub type ABRBIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u8, u8, 2, O>;
#[doc = "Field `ABRERR` reader - ABRERR field"]
pub type ABRERR_R = crate::BitReader<bool>;
#[doc = "Field `ABRERR` writer - ABRERR field"]
pub type ABRERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `TXDOIF` reader - TXDOIF field"]
pub type TXDOIF_R = crate::BitReader<bool>;
#[doc = "Field `TXDOIF` writer - TXDOIF field"]
pub type TXDOIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `FRAC` reader - FRAC field"]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC` writer - FRAC field"]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:13 - BAUD field"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - TXD field"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXD field"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RXTOIF field"]
    #[inline(always)]
    pub fn rxtoif(&self) -> RXTOIF_R {
        RXTOIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXIF field"]
    #[inline(always)]
    pub fn txif(&self) -> TXIF_R {
        TXIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RXTHRF field"]
    #[inline(always)]
    pub fn rxthrf(&self) -> RXTHRF_R {
        RXTHRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXTHRF field"]
    #[inline(always)]
    pub fn txthrf(&self) -> TXTHRF_R {
        TXTHRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TOIF field"]
    #[inline(always)]
    pub fn toif(&self) -> TOIF_R {
        TOIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RXIF field"]
    #[inline(always)]
    pub fn rxif(&self) -> RXIF_R {
        RXIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ABREN field"]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - ABRBIT field"]
    #[inline(always)]
    pub fn abrbit(&self) -> ABRBIT_R {
        ABRBIT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - ABRERR field"]
    #[inline(always)]
    pub fn abrerr(&self) -> ABRERR_R {
        ABRERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXDOIF field"]
    #[inline(always)]
    pub fn txdoif(&self) -> TXDOIF_R {
        TXDOIF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - FRAC field"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - BAUD field"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W<0> {
        BAUD_W::new(self)
    }
    #[doc = "Bit 14 - TXD field"]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W<14> {
        TXD_W::new(self)
    }
    #[doc = "Bit 15 - RXD field"]
    #[inline(always)]
    pub fn rxd(&mut self) -> RXD_W<15> {
        RXD_W::new(self)
    }
    #[doc = "Bit 16 - RXTOIF field"]
    #[inline(always)]
    pub fn rxtoif(&mut self) -> RXTOIF_W<16> {
        RXTOIF_W::new(self)
    }
    #[doc = "Bit 17 - TXIF field"]
    #[inline(always)]
    pub fn txif(&mut self) -> TXIF_W<17> {
        TXIF_W::new(self)
    }
    #[doc = "Bit 19 - RXTHRF field"]
    #[inline(always)]
    pub fn rxthrf(&mut self) -> RXTHRF_W<19> {
        RXTHRF_W::new(self)
    }
    #[doc = "Bit 20 - TXTHRF field"]
    #[inline(always)]
    pub fn txthrf(&mut self) -> TXTHRF_W<20> {
        TXTHRF_W::new(self)
    }
    #[doc = "Bit 21 - TOIF field"]
    #[inline(always)]
    pub fn toif(&mut self) -> TOIF_W<21> {
        TOIF_W::new(self)
    }
    #[doc = "Bit 22 - RXIF field"]
    #[inline(always)]
    pub fn rxif(&mut self) -> RXIF_W<22> {
        RXIF_W::new(self)
    }
    #[doc = "Bit 23 - ABREN field"]
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W<23> {
        ABREN_W::new(self)
    }
    #[doc = "Bits 24:25 - ABRBIT field"]
    #[inline(always)]
    pub fn abrbit(&mut self) -> ABRBIT_W<24> {
        ABRBIT_W::new(self)
    }
    #[doc = "Bit 26 - ABRERR field"]
    #[inline(always)]
    pub fn abrerr(&mut self) -> ABRERR_W<26> {
        ABRERR_W::new(self)
    }
    #[doc = "Bit 27 - TXDOIF field"]
    #[inline(always)]
    pub fn txdoif(&mut self) -> TXDOIF_W<27> {
        TXDOIF_W::new(self)
    }
    #[doc = "Bits 28:31 - FRAC field"]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W<28> {
        FRAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BAUD register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
