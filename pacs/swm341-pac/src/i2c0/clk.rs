#[doc = "Register `CLK` reader"]
pub struct R(crate::R<CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK` writer"]
pub struct W(crate::W<CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SPEC>;
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
impl From<crate::W<CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - SCLL field"]
pub type SCLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLL` writer - SCLL field"]
pub type SCLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLH` reader - SCLH field"]
pub type SCLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLH` writer - SCLH field"]
pub type SCLH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIV` reader - DIV field"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - DIV field"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `SDAH` reader - SDAH field"]
pub type SDAH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDAH` writer - SDAH field"]
pub type SDAH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - SCLL field"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCLH field"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DIV field"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - SDAH field"]
    #[inline(always)]
    pub fn sdah(&self) -> SDAH_R {
        SDAH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCLL field"]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W<0> {
        SCLL_W::new(self)
    }
    #[doc = "Bits 8:15 - SCLH field"]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<8> {
        SCLH_W::new(self)
    }
    #[doc = "Bits 16:23 - DIV field"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Bits 24:27 - SDAH field"]
    #[inline(always)]
    pub fn sdah(&mut self) -> SDAH_W<24> {
        SDAH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
