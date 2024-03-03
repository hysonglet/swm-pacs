#[doc = "Register `CLKEN1` reader"]
pub struct R(crate::R<CLKEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKEN1` writer"]
pub struct W(crate::W<CLKEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKEN1_SPEC>;
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
impl From<crate::W<CLKEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOE` reader - GPIOE field"]
pub type GPIOE_R = crate::BitReader<bool>;
#[doc = "Field `GPIOE` writer - GPIOE field"]
pub type GPIOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `SPI2` reader - SPI2 field"]
pub type SPI2_R = crate::BitReader<bool>;
#[doc = "Field `SPI2` writer - SPI2 field"]
pub type SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `SDRAM` reader - SDRAM field"]
pub type SDRAM_R = crate::BitReader<bool>;
#[doc = "Field `SDRAM` writer - SDRAM field"]
pub type SDRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `SFC` reader - SFC field"]
pub type SFC_R = crate::BitReader<bool>;
#[doc = "Field `SFC` writer - SFC field"]
pub type SFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `ADC1` reader - ADC1 field"]
pub type ADC1_R = crate::BitReader<bool>;
#[doc = "Field `ADC1` writer - ADC1 field"]
pub type ADC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `CAN1` reader - CAN1 field"]
pub type CAN1_R = crate::BitReader<bool>;
#[doc = "Field `CAN1` writer - CAN1 field"]
pub type CAN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `RTC` reader - RTC field"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - RTC field"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `IOFILT` reader - IOFILT field"]
pub type IOFILT_R = crate::BitReader<bool>;
#[doc = "Field `IOFILT` writer - IOFILT field"]
pub type IOFILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `BTIMR` reader - BTIMR field"]
pub type BTIMR_R = crate::BitReader<bool>;
#[doc = "Field `BTIMR` writer - BTIMR field"]
pub type BTIMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `JPEG` reader - JPEG field"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` writer - JPEG field"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `DAC` reader - DAC field"]
pub type DAC_R = crate::BitReader<bool>;
#[doc = "Field `DAC` writer - DAC field"]
pub type DAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
#[doc = "Field `QEI` reader - QEI field"]
pub type QEI_R = crate::BitReader<bool>;
#[doc = "Field `QEI` writer - QEI field"]
pub type QEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, O>;
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
    #[doc = "Bit 13 - SFC field"]
    #[inline(always)]
    pub fn sfc(&self) -> SFC_R {
        SFC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC1 field"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CAN1 field"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 26 - DAC field"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - QEI field"]
    #[inline(always)]
    pub fn qei(&self) -> QEI_R {
        QEI_R::new(((self.bits >> 27) & 1) != 0)
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
    #[doc = "Bit 13 - SFC field"]
    #[inline(always)]
    pub fn sfc(&mut self) -> SFC_W<13> {
        SFC_W::new(self)
    }
    #[doc = "Bit 16 - ADC1 field"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W<16> {
        ADC1_W::new(self)
    }
    #[doc = "Bit 17 - CAN1 field"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W<17> {
        CAN1_W::new(self)
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
    #[doc = "Bit 26 - DAC field"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<26> {
        DAC_W::new(self)
    }
    #[doc = "Bit 27 - QEI field"]
    #[inline(always)]
    pub fn qei(&mut self) -> QEI_W<27> {
        QEI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKEN1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clken1](index.html) module"]
pub struct CLKEN1_SPEC;
impl crate::RegisterSpec for CLKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clken1::R](R) reader structure"]
impl crate::Readable for CLKEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clken1::W](W) writer structure"]
impl crate::Writable for CLKEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKEN1 to value 0"]
impl crate::Resettable for CLKEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
