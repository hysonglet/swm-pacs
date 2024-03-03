#[doc = "Register `XTALCR` reader"]
pub struct R(crate::R<XTALCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALCR` writer"]
pub struct W(crate::W<XTALCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALCR_SPEC>;
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
impl From<crate::W<XTALCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `_32KON` reader - _32KON field"]
pub type _32KON_R = crate::BitReader<bool>;
#[doc = "Field `_32KON` writer - _32KON field"]
pub type _32KON_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALCR_SPEC, bool, O>;
#[doc = "Field `ON` reader - ON field"]
pub type ON_R = crate::BitReader<bool>;
#[doc = "Field `ON` writer - ON field"]
pub type ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALCR_SPEC, bool, O>;
#[doc = "Field `_32KDET` reader - _32KDET field"]
pub type _32KDET_R = crate::BitReader<bool>;
#[doc = "Field `_32KDET` writer - _32KDET field"]
pub type _32KDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALCR_SPEC, bool, O>;
#[doc = "Field `DET` reader - DET field"]
pub type DET_R = crate::BitReader<bool>;
#[doc = "Field `DET` writer - DET field"]
pub type DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALCR_SPEC, bool, O>;
#[doc = "Field `_32KDRV` reader - _32KDRV field"]
pub type _32KDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `_32KDRV` writer - _32KDRV field"]
pub type _32KDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTALCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DRV` reader - DRV field"]
pub type DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRV` writer - DRV field"]
pub type DRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTALCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - _32KON field"]
    #[inline(always)]
    pub fn _32kon(&self) -> _32KON_R {
        _32KON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ON field"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - _32KDET field"]
    #[inline(always)]
    pub fn _32kdet(&self) -> _32KDET_R {
        _32KDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DET field"]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - _32KDRV field"]
    #[inline(always)]
    pub fn _32kdrv(&self) -> _32KDRV_R {
        _32KDRV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - DRV field"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - _32KON field"]
    #[inline(always)]
    pub fn _32kon(&mut self) -> _32KON_W<0> {
        _32KON_W::new(self)
    }
    #[doc = "Bit 1 - ON field"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W<1> {
        ON_W::new(self)
    }
    #[doc = "Bit 4 - _32KDET field"]
    #[inline(always)]
    pub fn _32kdet(&mut self) -> _32KDET_W<4> {
        _32KDET_W::new(self)
    }
    #[doc = "Bit 5 - DET field"]
    #[inline(always)]
    pub fn det(&mut self) -> DET_W<5> {
        DET_W::new(self)
    }
    #[doc = "Bits 8:11 - _32KDRV field"]
    #[inline(always)]
    pub fn _32kdrv(&mut self) -> _32KDRV_W<8> {
        _32KDRV_W::new(self)
    }
    #[doc = "Bits 16:20 - DRV field"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<16> {
        DRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTALCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalcr](index.html) module"]
pub struct XTALCR_SPEC;
impl crate::RegisterSpec for XTALCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalcr::R](R) reader structure"]
impl crate::Readable for XTALCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalcr::W](W) writer structure"]
impl crate::Writable for XTALCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALCR to value 0"]
impl crate::Resettable for XTALCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
