#[doc = "Register `PRSTR0` reader"]
pub struct R(crate::R<PRSTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTR0` writer"]
pub struct W(crate::W<PRSTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTR0_SPEC>;
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
impl From<crate::W<PRSTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOA` reader - GPIOA field"]
pub type GPIOA_R = crate::BitReader<bool>;
#[doc = "Field `GPIOA` writer - GPIOA field"]
pub type GPIOA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `GPIOB` reader - GPIOB field"]
pub type GPIOB_R = crate::BitReader<bool>;
#[doc = "Field `GPIOB` writer - GPIOB field"]
pub type GPIOB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `GPIOC` reader - GPIOC field"]
pub type GPIOC_R = crate::BitReader<bool>;
#[doc = "Field `GPIOC` writer - GPIOC field"]
pub type GPIOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `GPIOD` reader - GPIOD field"]
pub type GPIOD_R = crate::BitReader<bool>;
#[doc = "Field `GPIOD` writer - GPIOD field"]
pub type GPIOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `GPIOM` reader - GPIOM field"]
pub type GPIOM_R = crate::BitReader<bool>;
#[doc = "Field `GPIOM` writer - GPIOM field"]
pub type GPIOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `GPION` reader - GPION field"]
pub type GPION_R = crate::BitReader<bool>;
#[doc = "Field `GPION` writer - GPION field"]
pub type GPION_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `UART0` reader - UART0 field"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - UART0 field"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `UART1` reader - UART1 field"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - UART1 field"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `UART2` reader - UART2 field"]
pub type UART2_R = crate::BitReader<bool>;
#[doc = "Field `UART2` writer - UART2 field"]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `UART3` reader - UART3 field"]
pub type UART3_R = crate::BitReader<bool>;
#[doc = "Field `UART3` writer - UART3 field"]
pub type UART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `WDT` reader - WDT field"]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - WDT field"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `TIMR` reader - TIMR field"]
pub type TIMR_R = crate::BitReader<bool>;
#[doc = "Field `TIMR` writer - TIMR field"]
pub type TIMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `PWM` reader - PWM field"]
pub type PWM_R = crate::BitReader<bool>;
#[doc = "Field `PWM` writer - PWM field"]
pub type PWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `SPI0` reader - SPI0 field"]
pub type SPI0_R = crate::BitReader<bool>;
#[doc = "Field `SPI0` writer - SPI0 field"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `SPI1` reader - SPI1 field"]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1` writer - SPI1 field"]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `I2C0` reader - I2C0 field"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C0 field"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - I2C1 field"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - I2C1 field"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `CRC` reader - CRC field"]
pub type CRC_R = crate::BitReader<bool>;
#[doc = "Field `CRC` writer - CRC field"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `CORDIC` reader - CORDIC field"]
pub type CORDIC_R = crate::BitReader<bool>;
#[doc = "Field `CORDIC` writer - CORDIC field"]
pub type CORDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `DIV` reader - DIV field"]
pub type DIV_R = crate::BitReader<bool>;
#[doc = "Field `DIV` writer - DIV field"]
pub type DIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `SDIO` reader - SDIO field"]
pub type SDIO_R = crate::BitReader<bool>;
#[doc = "Field `SDIO` writer - SDIO field"]
pub type SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `USB` reader - USB field"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - USB field"]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `ANAC` reader - ANAC field"]
pub type ANAC_R = crate::BitReader<bool>;
#[doc = "Field `ANAC` writer - ANAC field"]
pub type ANAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `ADC0` reader - ADC0 field"]
pub type ADC0_R = crate::BitReader<bool>;
#[doc = "Field `ADC0` writer - ADC0 field"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `CAN0` reader - CAN0 field"]
pub type CAN0_R = crate::BitReader<bool>;
#[doc = "Field `CAN0` writer - CAN0 field"]
pub type CAN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `DMA2D` reader - DMA2D field"]
pub type DMA2D_R = crate::BitReader<bool>;
#[doc = "Field `DMA2D` writer - DMA2D field"]
pub type DMA2D_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
#[doc = "Field `LCD` reader - LCD field"]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - LCD field"]
pub type LCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOA field"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB field"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC field"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD field"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOM field"]
    #[inline(always)]
    pub fn gpiom(&self) -> GPIOM_R {
        GPIOM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPION field"]
    #[inline(always)]
    pub fn gpion(&self) -> GPION_R {
        GPION_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 field"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART1 field"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART2 field"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART3 field"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WDT field"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMR field"]
    #[inline(always)]
    pub fn timr(&self) -> TIMR_R {
        TIMR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM field"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI0 field"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 field"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C0 field"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C1 field"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC field"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CORDIC field"]
    #[inline(always)]
    pub fn cordic(&self) -> CORDIC_R {
        CORDIC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIV field"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO field"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USB field"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ANAC field"]
    #[inline(always)]
    pub fn anac(&self) -> ANAC_R {
        ANAC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC0 field"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - CAN0 field"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA2D field"]
    #[inline(always)]
    pub fn dma2d(&self) -> DMA2D_R {
        DMA2D_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LCD field"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA field"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GPIOA_W<0> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 1 - GPIOB field"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GPIOB_W<1> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 2 - GPIOC field"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GPIOC_W<2> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 3 - GPIOD field"]
    #[inline(always)]
    pub fn gpiod(&mut self) -> GPIOD_W<3> {
        GPIOD_W::new(self)
    }
    #[doc = "Bit 4 - GPIOM field"]
    #[inline(always)]
    pub fn gpiom(&mut self) -> GPIOM_W<4> {
        GPIOM_W::new(self)
    }
    #[doc = "Bit 5 - GPION field"]
    #[inline(always)]
    pub fn gpion(&mut self) -> GPION_W<5> {
        GPION_W::new(self)
    }
    #[doc = "Bit 6 - UART0 field"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<6> {
        UART0_W::new(self)
    }
    #[doc = "Bit 7 - UART1 field"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W<7> {
        UART1_W::new(self)
    }
    #[doc = "Bit 8 - UART2 field"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W<8> {
        UART2_W::new(self)
    }
    #[doc = "Bit 9 - UART3 field"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART3_W<9> {
        UART3_W::new(self)
    }
    #[doc = "Bit 10 - WDT field"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<10> {
        WDT_W::new(self)
    }
    #[doc = "Bit 11 - TIMR field"]
    #[inline(always)]
    pub fn timr(&mut self) -> TIMR_W<11> {
        TIMR_W::new(self)
    }
    #[doc = "Bit 12 - PWM field"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PWM_W<12> {
        PWM_W::new(self)
    }
    #[doc = "Bit 13 - SPI0 field"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W<13> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 14 - SPI1 field"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<14> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 15 - I2C0 field"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W<15> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 16 - I2C1 field"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W<16> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 19 - CRC field"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<19> {
        CRC_W::new(self)
    }
    #[doc = "Bit 20 - CORDIC field"]
    #[inline(always)]
    pub fn cordic(&mut self) -> CORDIC_W<20> {
        CORDIC_W::new(self)
    }
    #[doc = "Bit 21 - DIV field"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<21> {
        DIV_W::new(self)
    }
    #[doc = "Bit 22 - SDIO field"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W<22> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 24 - USB field"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W<24> {
        USB_W::new(self)
    }
    #[doc = "Bit 25 - ANAC field"]
    #[inline(always)]
    pub fn anac(&mut self) -> ANAC_W<25> {
        ANAC_W::new(self)
    }
    #[doc = "Bit 26 - ADC0 field"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W<26> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 28 - CAN0 field"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W<28> {
        CAN0_W::new(self)
    }
    #[doc = "Bit 29 - DMA2D field"]
    #[inline(always)]
    pub fn dma2d(&mut self) -> DMA2D_W<29> {
        DMA2D_W::new(self)
    }
    #[doc = "Bit 30 - LCD field"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W<30> {
        LCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRSTR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstr0](index.html) module"]
pub struct PRSTR0_SPEC;
impl crate::RegisterSpec for PRSTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstr0::R](R) reader structure"]
impl crate::Readable for PRSTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstr0::W](W) writer structure"]
impl crate::Writable for PRSTR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRSTR0 to value 0"]
impl crate::Resettable for PRSTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
