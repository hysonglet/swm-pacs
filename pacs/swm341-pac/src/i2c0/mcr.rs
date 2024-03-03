#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STA` reader - STA field"]
pub type STA_R = crate::BitReader<bool>;
#[doc = "Field `STA` writer - STA field"]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `RD` reader - RD field"]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - RD field"]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `WR` reader - WR field"]
pub type WR_R = crate::BitReader<bool>;
#[doc = "Field `WR` writer - WR field"]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `STO` reader - STO field"]
pub type STO_R = crate::BitReader<bool>;
#[doc = "Field `STO` writer - STO field"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - STA field"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RD field"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WR field"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STO field"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STA field"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W<0> {
        STA_W::new(self)
    }
    #[doc = "Bit 1 - RD field"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W<1> {
        RD_W::new(self)
    }
    #[doc = "Bit 2 - WR field"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W<2> {
        WR_W::new(self)
    }
    #[doc = "Bit 3 - STO field"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W<3> {
        STO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
