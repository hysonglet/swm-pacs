#[doc = "Register `PORTSR` reader"]
pub struct R(crate::R<PORTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTSR` writer"]
pub struct W(crate::W<PORTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSR_SPEC>;
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
impl From<crate::W<PORTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN` reader - CONN field"]
pub type CONN_R = crate::BitReader<bool>;
#[doc = "Field `CONN` writer - CONN field"]
pub type CONN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `ENA` reader - ENA field"]
pub type ENA_R = crate::BitReader<bool>;
#[doc = "Field `ENA` writer - ENA field"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - SUSP field"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - SUSP field"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `CLRSUSP` reader - CLRSUSP field"]
pub type CLRSUSP_R = crate::BitReader<bool>;
#[doc = "Field `CLRSUSP` writer - CLRSUSP field"]
pub type CLRSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `RESET` reader - RESET field"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET field"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `POWER` reader - POWER field"]
pub type POWER_R = crate::BitReader<bool>;
#[doc = "Field `POWER` writer - POWER field"]
pub type POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - SPEED field"]
pub type SPEED_R = crate::BitReader<bool>;
#[doc = "Field `SPEED` writer - SPEED field"]
pub type SPEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `CONNCHG` reader - CONNCHG field"]
pub type CONNCHG_R = crate::BitReader<bool>;
#[doc = "Field `CONNCHG` writer - CONNCHG field"]
pub type CONNCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `ENACHG` reader - ENACHG field"]
pub type ENACHG_R = crate::BitReader<bool>;
#[doc = "Field `ENACHG` writer - ENACHG field"]
pub type ENACHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `SUSPCHG` reader - SUSPCHG field"]
pub type SUSPCHG_R = crate::BitReader<bool>;
#[doc = "Field `SUSPCHG` writer - SUSPCHG field"]
pub type SUSPCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
#[doc = "Field `RSTCHG` reader - RSTCHG field"]
pub type RSTCHG_R = crate::BitReader<bool>;
#[doc = "Field `RSTCHG` writer - RSTCHG field"]
pub type RSTCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CONN field"]
    #[inline(always)]
    pub fn conn(&self) -> CONN_R {
        CONN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENA field"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SUSP field"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLRSUSP field"]
    #[inline(always)]
    pub fn clrsusp(&self) -> CLRSUSP_R {
        CLRSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESET field"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - POWER field"]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPEED field"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - CONNCHG field"]
    #[inline(always)]
    pub fn connchg(&self) -> CONNCHG_R {
        CONNCHG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ENACHG field"]
    #[inline(always)]
    pub fn enachg(&self) -> ENACHG_R {
        ENACHG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SUSPCHG field"]
    #[inline(always)]
    pub fn suspchg(&self) -> SUSPCHG_R {
        SUSPCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - RSTCHG field"]
    #[inline(always)]
    pub fn rstchg(&self) -> RSTCHG_R {
        RSTCHG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CONN field"]
    #[inline(always)]
    pub fn conn(&mut self) -> CONN_W<0> {
        CONN_W::new(self)
    }
    #[doc = "Bit 1 - ENA field"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<1> {
        ENA_W::new(self)
    }
    #[doc = "Bit 2 - SUSP field"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<2> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 3 - CLRSUSP field"]
    #[inline(always)]
    pub fn clrsusp(&mut self) -> CLRSUSP_W<3> {
        CLRSUSP_W::new(self)
    }
    #[doc = "Bit 4 - RESET field"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<4> {
        RESET_W::new(self)
    }
    #[doc = "Bit 8 - POWER field"]
    #[inline(always)]
    pub fn power(&mut self) -> POWER_W<8> {
        POWER_W::new(self)
    }
    #[doc = "Bit 9 - SPEED field"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W<9> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 16 - CONNCHG field"]
    #[inline(always)]
    pub fn connchg(&mut self) -> CONNCHG_W<16> {
        CONNCHG_W::new(self)
    }
    #[doc = "Bit 17 - ENACHG field"]
    #[inline(always)]
    pub fn enachg(&mut self) -> ENACHG_W<17> {
        ENACHG_W::new(self)
    }
    #[doc = "Bit 18 - SUSPCHG field"]
    #[inline(always)]
    pub fn suspchg(&mut self) -> SUSPCHG_W<18> {
        SUSPCHG_W::new(self)
    }
    #[doc = "Bit 20 - RSTCHG field"]
    #[inline(always)]
    pub fn rstchg(&mut self) -> RSTCHG_W<20> {
        RSTCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORTSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsr](index.html) module"]
pub struct PORTSR_SPEC;
impl crate::RegisterSpec for PORTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsr::R](R) reader structure"]
impl crate::Readable for PORTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsr::W](W) writer structure"]
impl crate::Writable for PORTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTSR to value 0"]
impl crate::Resettable for PORTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
