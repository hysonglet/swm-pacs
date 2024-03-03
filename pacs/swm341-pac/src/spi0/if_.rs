#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFOV` reader - RFOV field"]
pub type RFOV_R = crate::BitReader<bool>;
#[doc = "Field `RFOV` writer - RFOV field"]
pub type RFOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RFF` reader - RFF field"]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `RFF` writer - RFF field"]
pub type RFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RFHF` reader - RFHF field"]
pub type RFHF_R = crate::BitReader<bool>;
#[doc = "Field `RFHF` writer - RFHF field"]
pub type RFHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TFE` reader - TFE field"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - TFE field"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TFHF` reader - TFHF field"]
pub type TFHF_R = crate::BitReader<bool>;
#[doc = "Field `TFHF` writer - TFHF field"]
pub type TFHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `RFTHR` reader - RFTHR field"]
pub type RFTHR_R = crate::BitReader<bool>;
#[doc = "Field `RFTHR` writer - RFTHR field"]
pub type RFTHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TFTHR` reader - TFTHR field"]
pub type TFTHR_R = crate::BitReader<bool>;
#[doc = "Field `TFTHR` writer - TFTHR field"]
pub type TFTHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `WTC` reader - WTC field"]
pub type WTC_R = crate::BitReader<bool>;
#[doc = "Field `WTC` writer - WTC field"]
pub type WTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FTC` reader - FTC field"]
pub type FTC_R = crate::BitReader<bool>;
#[doc = "Field `FTC` writer - FTC field"]
pub type FTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SSFALL` reader - SSFALL field"]
pub type SSFALL_R = crate::BitReader<bool>;
#[doc = "Field `SSFALL` writer - SSFALL field"]
pub type SSFALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SSRISE` reader - SSRISE field"]
pub type SSRISE_R = crate::BitReader<bool>;
#[doc = "Field `SSRISE` writer - SSRISE field"]
pub type SSRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RFOV field"]
    #[inline(always)]
    pub fn rfov(&self) -> RFOV_R {
        RFOV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RFF field"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RFHF field"]
    #[inline(always)]
    pub fn rfhf(&self) -> RFHF_R {
        RFHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TFE field"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TFHF field"]
    #[inline(always)]
    pub fn tfhf(&self) -> TFHF_R {
        TFHF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFTHR field"]
    #[inline(always)]
    pub fn rfthr(&self) -> RFTHR_R {
        RFTHR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TFTHR field"]
    #[inline(always)]
    pub fn tfthr(&self) -> TFTHR_R {
        TFTHR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - WTC field"]
    #[inline(always)]
    pub fn wtc(&self) -> WTC_R {
        WTC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FTC field"]
    #[inline(always)]
    pub fn ftc(&self) -> FTC_R {
        FTC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SSFALL field"]
    #[inline(always)]
    pub fn ssfall(&self) -> SSFALL_R {
        SSFALL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSRISE field"]
    #[inline(always)]
    pub fn ssrise(&self) -> SSRISE_R {
        SSRISE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFOV field"]
    #[inline(always)]
    pub fn rfov(&mut self) -> RFOV_W<0> {
        RFOV_W::new(self)
    }
    #[doc = "Bit 1 - RFF field"]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W<1> {
        RFF_W::new(self)
    }
    #[doc = "Bit 2 - RFHF field"]
    #[inline(always)]
    pub fn rfhf(&mut self) -> RFHF_W<2> {
        RFHF_W::new(self)
    }
    #[doc = "Bit 3 - TFE field"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<3> {
        TFE_W::new(self)
    }
    #[doc = "Bit 4 - TFHF field"]
    #[inline(always)]
    pub fn tfhf(&mut self) -> TFHF_W<4> {
        TFHF_W::new(self)
    }
    #[doc = "Bit 5 - RFTHR field"]
    #[inline(always)]
    pub fn rfthr(&mut self) -> RFTHR_W<5> {
        RFTHR_W::new(self)
    }
    #[doc = "Bit 6 - TFTHR field"]
    #[inline(always)]
    pub fn tfthr(&mut self) -> TFTHR_W<6> {
        TFTHR_W::new(self)
    }
    #[doc = "Bit 8 - WTC field"]
    #[inline(always)]
    pub fn wtc(&mut self) -> WTC_W<8> {
        WTC_W::new(self)
    }
    #[doc = "Bit 9 - FTC field"]
    #[inline(always)]
    pub fn ftc(&mut self) -> FTC_W<9> {
        FTC_W::new(self)
    }
    #[doc = "Bit 10 - SSFALL field"]
    #[inline(always)]
    pub fn ssfall(&mut self) -> SSFALL_W<10> {
        SSFALL_W::new(self)
    }
    #[doc = "Bit 11 - SSRISE field"]
    #[inline(always)]
    pub fn ssrise(&mut self) -> SSRISE_W<11> {
        SSRISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
