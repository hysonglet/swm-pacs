#[doc = "Register `CLKSEL` reader"]
pub struct R(crate::R<CLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSEL` writer"]
pub struct W(crate::W<CLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSEL_SPEC>;
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
impl From<crate::W<CLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS` reader - SYS field"]
pub type SYS_R = crate::BitReader<bool>;
#[doc = "Field `SYS` writer - SYS field"]
pub type SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSEL_SPEC, bool, O>;
#[doc = "Field `CLK_DIVx` reader - CLK_DIVx field"]
pub type CLK_DIVX_R = crate::BitReader<bool>;
#[doc = "Field `CLK_DIVx` writer - CLK_DIVx field"]
pub type CLK_DIVX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSEL_SPEC, bool, O>;
#[doc = "Field `CLK` reader - CLK field"]
pub type CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK` writer - CLK field"]
pub type CLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RTC` reader - RTC field"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - RTC field"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSEL_SPEC, bool, O>;
#[doc = "Field `IOFILT` reader - IOFILT field"]
pub type IOFILT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOFILT` writer - IOFILT field"]
pub type IOFILT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDIO` reader - SDIO field"]
pub type SDIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDIO` writer - SDIO field"]
pub type SDIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDT` reader - WDT field"]
pub type WDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT` writer - WDT field"]
pub type WDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AD0` reader - AD0 field"]
pub type AD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD0` writer - AD0 field"]
pub type AD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AD0DIV` reader - AD0DIV field"]
pub type AD0DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD0DIV` writer - AD0DIV field"]
pub type AD0DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AD1` reader - AD1 field"]
pub type AD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD1` writer - AD1 field"]
pub type AD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AD1DIV` reader - AD1DIV field"]
pub type AD1DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD1DIV` writer - AD1DIV field"]
pub type AD1DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SLEEP` reader - SLEEP field"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - SLEEP field"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYS field"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLK_DIVx field"]
    #[inline(always)]
    pub fn clk_divx(&self) -> CLK_DIVX_R {
        CLK_DIVX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - CLK field"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - RTC field"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IOFILT field"]
    #[inline(always)]
    pub fn iofilt(&self) -> IOFILT_R {
        IOFILT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SDIO field"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - WDT field"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - AD0 field"]
    #[inline(always)]
    pub fn ad0(&self) -> AD0_R {
        AD0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - AD0DIV field"]
    #[inline(always)]
    pub fn ad0div(&self) -> AD0DIV_R {
        AD0DIV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - AD1 field"]
    #[inline(always)]
    pub fn ad1(&self) -> AD1_R {
        AD1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - AD1DIV field"]
    #[inline(always)]
    pub fn ad1div(&self) -> AD1DIV_R {
        AD1DIV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - SLEEP field"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS field"]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W<0> {
        SYS_W::new(self)
    }
    #[doc = "Bit 1 - CLK_DIVx field"]
    #[inline(always)]
    pub fn clk_divx(&mut self) -> CLK_DIVX_W<1> {
        CLK_DIVX_W::new(self)
    }
    #[doc = "Bits 2:4 - CLK field"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W<2> {
        CLK_W::new(self)
    }
    #[doc = "Bit 5 - RTC field"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<5> {
        RTC_W::new(self)
    }
    #[doc = "Bits 6:7 - IOFILT field"]
    #[inline(always)]
    pub fn iofilt(&mut self) -> IOFILT_W<6> {
        IOFILT_W::new(self)
    }
    #[doc = "Bits 10:11 - SDIO field"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W<10> {
        SDIO_W::new(self)
    }
    #[doc = "Bits 12:13 - WDT field"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<12> {
        WDT_W::new(self)
    }
    #[doc = "Bits 16:17 - AD0 field"]
    #[inline(always)]
    pub fn ad0(&mut self) -> AD0_W<16> {
        AD0_W::new(self)
    }
    #[doc = "Bits 18:19 - AD0DIV field"]
    #[inline(always)]
    pub fn ad0div(&mut self) -> AD0DIV_W<18> {
        AD0DIV_W::new(self)
    }
    #[doc = "Bits 20:21 - AD1 field"]
    #[inline(always)]
    pub fn ad1(&mut self) -> AD1_W<20> {
        AD1_W::new(self)
    }
    #[doc = "Bits 22:23 - AD1DIV field"]
    #[inline(always)]
    pub fn ad1div(&mut self) -> AD1DIV_W<22> {
        AD1DIV_W::new(self)
    }
    #[doc = "Bit 24 - SLEEP field"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<24> {
        SLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKSEL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel](index.html) module"]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clksel::R](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksel::W](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
