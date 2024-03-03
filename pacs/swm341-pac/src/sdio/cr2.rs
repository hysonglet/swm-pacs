#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKEN` reader - CLKEN field"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - CLKEN field"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CLKRDY` reader - CLKRDY field"]
pub type CLKRDY_R = crate::BitReader<bool>;
#[doc = "Field `CLKRDY` writer - CLKRDY field"]
pub type CLKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SDCLKEN` reader - SDCLKEN field"]
pub type SDCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `SDCLKEN` writer - SDCLKEN field"]
pub type SDCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SDCLKDIV` reader - SDCLKDIV field"]
pub type SDCLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDCLKDIV` writer - SDCLKDIV field"]
pub type SDCLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `TIMEOUT` reader - TIMEOUT field"]
pub type TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEOUT` writer - TIMEOUT field"]
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSTALL` reader - RSTALL field"]
pub type RSTALL_R = crate::BitReader<bool>;
#[doc = "Field `RSTALL` writer - RSTALL field"]
pub type RSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RSTCMD` reader - RSTCMD field"]
pub type RSTCMD_R = crate::BitReader<bool>;
#[doc = "Field `RSTCMD` writer - RSTCMD field"]
pub type RSTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RSTDAT` reader - RSTDAT field"]
pub type RSTDAT_R = crate::BitReader<bool>;
#[doc = "Field `RSTDAT` writer - RSTDAT field"]
pub type RSTDAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLKEN field"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKRDY field"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDCLKEN field"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SDCLKDIV field"]
    #[inline(always)]
    pub fn sdclkdiv(&self) -> SDCLKDIV_R {
        SDCLKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - TIMEOUT field"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - RSTALL field"]
    #[inline(always)]
    pub fn rstall(&self) -> RSTALL_R {
        RSTALL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RSTCMD field"]
    #[inline(always)]
    pub fn rstcmd(&self) -> RSTCMD_R {
        RSTCMD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RSTDAT field"]
    #[inline(always)]
    pub fn rstdat(&self) -> RSTDAT_R {
        RSTDAT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKEN field"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - CLKRDY field"]
    #[inline(always)]
    pub fn clkrdy(&mut self) -> CLKRDY_W<1> {
        CLKRDY_W::new(self)
    }
    #[doc = "Bit 2 - SDCLKEN field"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SDCLKEN_W<2> {
        SDCLKEN_W::new(self)
    }
    #[doc = "Bits 8:15 - SDCLKDIV field"]
    #[inline(always)]
    pub fn sdclkdiv(&mut self) -> SDCLKDIV_W<8> {
        SDCLKDIV_W::new(self)
    }
    #[doc = "Bits 16:19 - TIMEOUT field"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<16> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 24 - RSTALL field"]
    #[inline(always)]
    pub fn rstall(&mut self) -> RSTALL_W<24> {
        RSTALL_W::new(self)
    }
    #[doc = "Bit 25 - RSTCMD field"]
    #[inline(always)]
    pub fn rstcmd(&mut self) -> RSTCMD_W<25> {
        RSTCMD_W::new(self)
    }
    #[doc = "Bit 26 - RSTDAT field"]
    #[inline(always)]
    pub fn rstdat(&mut self) -> RSTDAT_W<26> {
        RSTDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
