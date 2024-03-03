#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTC` reader - WTC field"]
pub type WTC_R = crate::BitReader<bool>;
#[doc = "Field `WTC` writer - WTC field"]
pub type WTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TFE` reader - TFE field"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - TFE field"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TFNF` reader - TFNF field"]
pub type TFNF_R = crate::BitReader<bool>;
#[doc = "Field `TFNF` writer - TFNF field"]
pub type TFNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RFNE` reader - RFNE field"]
pub type RFNE_R = crate::BitReader<bool>;
#[doc = "Field `RFNE` writer - RFNE field"]
pub type RFNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RFF` reader - RFF field"]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `RFF` writer - RFF field"]
pub type RFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RFOV` reader - RFOV field"]
pub type RFOV_R = crate::BitReader<bool>;
#[doc = "Field `RFOV` writer - RFOV field"]
pub type RFOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TFLVL` reader - TFLVL field"]
pub type TFLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFLVL` writer - TFLVL field"]
pub type TFLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFLVL` reader - RFLVL field"]
pub type RFLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFLVL` writer - RFLVL field"]
pub type RFLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUSY` reader - BUSY field"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY field"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WTC field"]
    #[inline(always)]
    pub fn wtc(&self) -> WTC_R {
        WTC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFE field"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TFNF field"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFNE field"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RFF field"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOV field"]
    #[inline(always)]
    pub fn rfov(&self) -> RFOV_R {
        RFOV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - TFLVL field"]
    #[inline(always)]
    pub fn tflvl(&self) -> TFLVL_R {
        TFLVL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - RFLVL field"]
    #[inline(always)]
    pub fn rflvl(&self) -> RFLVL_R {
        RFLVL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 15 - BUSY field"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WTC field"]
    #[inline(always)]
    pub fn wtc(&mut self) -> WTC_W<0> {
        WTC_W::new(self)
    }
    #[doc = "Bit 1 - TFE field"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<1> {
        TFE_W::new(self)
    }
    #[doc = "Bit 2 - TFNF field"]
    #[inline(always)]
    pub fn tfnf(&mut self) -> TFNF_W<2> {
        TFNF_W::new(self)
    }
    #[doc = "Bit 3 - RFNE field"]
    #[inline(always)]
    pub fn rfne(&mut self) -> RFNE_W<3> {
        RFNE_W::new(self)
    }
    #[doc = "Bit 4 - RFF field"]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W<4> {
        RFF_W::new(self)
    }
    #[doc = "Bit 5 - RFOV field"]
    #[inline(always)]
    pub fn rfov(&mut self) -> RFOV_W<5> {
        RFOV_W::new(self)
    }
    #[doc = "Bits 6:8 - TFLVL field"]
    #[inline(always)]
    pub fn tflvl(&mut self) -> TFLVL_W<6> {
        TFLVL_W::new(self)
    }
    #[doc = "Bits 9:11 - RFLVL field"]
    #[inline(always)]
    pub fn rflvl(&mut self) -> RFLVL_W<9> {
        RFLVL_W::new(self)
    }
    #[doc = "Bit 15 - BUSY field"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<15> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
