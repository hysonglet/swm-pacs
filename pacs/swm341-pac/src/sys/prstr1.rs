#[doc = "Register `PRSTR1` reader"]
pub struct R(crate::R<PRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTR1` writer"]
pub struct W(crate::W<PRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTR1_SPEC>;
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
impl From<crate::W<PRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOE` reader - GPIOE field"]
pub type GPIOE_R = crate::BitReader<bool>;
#[doc = "Field `GPIOE` writer - GPIOE field"]
pub type GPIOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `SPI2` reader - SPI2 field"]
pub type SPI2_R = crate::BitReader<bool>;
#[doc = "Field `SPI2` writer - SPI2 field"]
pub type SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `SDRAM` reader - SDRAM field"]
pub type SDRAM_R = crate::BitReader<bool>;
#[doc = "Field `SDRAM` writer - SDRAM field"]
pub type SDRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `ADC1` reader - ADC1 field"]
pub type ADC1_R = crate::BitReader<bool>;
#[doc = "Field `ADC1` writer - ADC1 field"]
pub type ADC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `RTC` reader - RTC field"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - RTC field"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `IOFILT` reader - IOFILT field"]
pub type IOFILT_R = crate::BitReader<bool>;
#[doc = "Field `IOFILT` writer - IOFILT field"]
pub type IOFILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `BTIMR` reader - BTIMR field"]
pub type BTIMR_R = crate::BitReader<bool>;
#[doc = "Field `BTIMR` writer - BTIMR field"]
pub type BTIMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
#[doc = "Field `JPEG` reader - JPEG field"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` writer - JPEG field"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOE field"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - SPI2 field"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SDRAM field"]
    #[inline(always)]
    pub fn sdram(&self) -> SDRAM_R {
        SDRAM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC1 field"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - RTC field"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IOFILT field"]
    #[inline(always)]
    pub fn iofilt(&self) -> IOFILT_R {
        IOFILT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - BTIMR field"]
    #[inline(always)]
    pub fn btimr(&self) -> BTIMR_R {
        BTIMR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - JPEG field"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOE field"]
    #[inline(always)]
    pub fn gpioe(&mut self) -> GPIOE_W<0> {
        GPIOE_W::new(self)
    }
    #[doc = "Bit 8 - SPI2 field"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W<8> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 12 - SDRAM field"]
    #[inline(always)]
    pub fn sdram(&mut self) -> SDRAM_W<12> {
        SDRAM_W::new(self)
    }
    #[doc = "Bit 16 - ADC1 field"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W<16> {
        ADC1_W::new(self)
    }
    #[doc = "Bit 19 - RTC field"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<19> {
        RTC_W::new(self)
    }
    #[doc = "Bit 20 - IOFILT field"]
    #[inline(always)]
    pub fn iofilt(&mut self) -> IOFILT_W<20> {
        IOFILT_W::new(self)
    }
    #[doc = "Bit 22 - BTIMR field"]
    #[inline(always)]
    pub fn btimr(&mut self) -> BTIMR_W<22> {
        BTIMR_W::new(self)
    }
    #[doc = "Bit 25 - JPEG field"]
    #[inline(always)]
    pub fn jpeg(&mut self) -> JPEG_W<25> {
        JPEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRSTR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstr1](index.html) module"]
pub struct PRSTR1_SPEC;
impl crate::RegisterSpec for PRSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstr1::R](R) reader structure"]
impl crate::Readable for PRSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstr1::W](W) writer structure"]
impl crate::Writable for PRSTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRSTR1 to value 0"]
impl crate::Resettable for PRSTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
