#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - CLKDIV field"]
pub type CLKDIV_R = crate::BitReader<bool>;
#[doc = "Field `CLKDIV` writer - CLKDIV field"]
pub type CLKDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CASDELAY` reader - CASDELAY field"]
pub type CASDELAY_R = crate::BitReader<bool>;
#[doc = "Field `CASDELAY` writer - CASDELAY field"]
pub type CASDELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `HIGHFREQ` reader - HIGHFREQ field"]
pub type HIGHFREQ_R = crate::BitReader<bool>;
#[doc = "Field `HIGHFREQ` writer - HIGHFREQ field"]
pub type HIGHFREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - SIZE field"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - SIZE field"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CASDELAY field"]
    #[inline(always)]
    pub fn casdelay(&self) -> CASDELAY_R {
        CASDELAY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HIGHFREQ field"]
    #[inline(always)]
    pub fn highfreq(&self) -> HIGHFREQ_R {
        HIGHFREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - SIZE field"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 1 - CASDELAY field"]
    #[inline(always)]
    pub fn casdelay(&mut self) -> CASDELAY_W<1> {
        CASDELAY_W::new(self)
    }
    #[doc = "Bit 2 - HIGHFREQ field"]
    #[inline(always)]
    pub fn highfreq(&mut self) -> HIGHFREQ_W<2> {
        HIGHFREQ_W::new(self)
    }
    #[doc = "Bits 3:4 - SIZE field"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<3> {
        SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
