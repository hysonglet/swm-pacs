#[doc = "Register `RTSCR` reader"]
pub struct R(crate::R<RTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTSCR` writer"]
pub struct W(crate::W<RTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSCR_SPEC>;
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
impl From<crate::W<RTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSCR_SPEC, bool, O>;
#[doc = "Field `POL` reader - POL field"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - POL field"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSCR_SPEC, bool, O>;
#[doc = "Field `THR` reader - THR field"]
pub type THR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THR` writer - THR field"]
pub type THR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTSCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STAT` reader - STAT field"]
pub type STAT_R = crate::BitReader<bool>;
#[doc = "Field `STAT` writer - STAT field"]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - POL field"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - THR field"]
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - STAT field"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<1> {
        EN_W::new(self)
    }
    #[doc = "Bit 3 - POL field"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<3> {
        POL_W::new(self)
    }
    #[doc = "Bits 4:6 - THR field"]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W<4> {
        THR_W::new(self)
    }
    #[doc = "Bit 8 - STAT field"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W<8> {
        STAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTSCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtscr](index.html) module"]
pub struct RTSCR_SPEC;
impl crate::RegisterSpec for RTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtscr::R](R) reader structure"]
impl crate::Readable for RTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtscr::W](W) writer structure"]
impl crate::Writable for RTSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTSCR to value 0"]
impl crate::Resettable for RTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
