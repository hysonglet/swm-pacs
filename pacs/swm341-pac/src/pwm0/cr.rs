#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - MODE field"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - MODE field"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MULT` reader - MULT field"]
pub type MULT_R = crate::BitReader<bool>;
#[doc = "Field `MULT` writer - MULT field"]
pub type MULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DIR` reader - DIR field"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR field"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CLKSRC` reader - CLKSRC field"]
pub type CLKSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSRC` writer - CLKSRC field"]
pub type CLKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLKDIV` reader - CLKDIV field"]
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV` writer - CLKDIV field"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u16, u16, 10, O>;
#[doc = "Field `RPTNUM` reader - RPTNUM field"]
pub type RPTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPTNUM` writer - RPTNUM field"]
pub type RPTNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - MODE field"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - MULT field"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIR field"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CLKSRC field"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:15 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - RPTNUM field"]
    #[inline(always)]
    pub fn rptnum(&self) -> RPTNUM_R {
        RPTNUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE field"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - MULT field"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W<2> {
        MULT_W::new(self)
    }
    #[doc = "Bit 3 - DIR field"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<3> {
        DIR_W::new(self)
    }
    #[doc = "Bits 4:5 - CLKSRC field"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W<4> {
        CLKSRC_W::new(self)
    }
    #[doc = "Bits 6:15 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<6> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 16:23 - RPTNUM field"]
    #[inline(always)]
    pub fn rptnum(&mut self) -> RPTNUM_W<16> {
        RPTNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
