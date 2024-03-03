#[doc = "Register `CMPTRG` reader"]
pub struct R(crate::R<CMPTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPTRG` writer"]
pub struct W(crate::W<CMPTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPTRG_SPEC>;
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
impl From<crate::W<CMPTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - CMP field"]
pub type CMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMP` writer - CMP field"]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTRG_SPEC, u16, u16, 16, O>;
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPTRG_SPEC, bool, O>;
#[doc = "Field `MUX` reader - MUX field"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - MUX field"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTRG_SPEC, u8, u8, 3, O>;
#[doc = "Field `WIDTH` reader - WIDTH field"]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDTH` writer - WIDTH field"]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTRG_SPEC, u8, u8, 6, O>;
#[doc = "Field `DIR` reader - DIR field"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR field"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPTRG_SPEC, bool, O>;
#[doc = "Field `ATP` reader - ATP field"]
pub type ATP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATP` writer - ATP field"]
pub type ATP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTRG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:15 - CMP field"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - MUX field"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:25 - WIDTH field"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - DIR field"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - ATP field"]
    #[inline(always)]
    pub fn atp(&self) -> ATP_R {
        ATP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CMP field"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Bit 16 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<16> {
        EN_W::new(self)
    }
    #[doc = "Bits 17:19 - MUX field"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W<17> {
        MUX_W::new(self)
    }
    #[doc = "Bits 20:25 - WIDTH field"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W<20> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 28 - DIR field"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<28> {
        DIR_W::new(self)
    }
    #[doc = "Bits 29:31 - ATP field"]
    #[inline(always)]
    pub fn atp(&mut self) -> ATP_W<29> {
        ATP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPTRG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmptrg](index.html) module"]
pub struct CMPTRG_SPEC;
impl crate::RegisterSpec for CMPTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmptrg::R](R) reader structure"]
impl crate::Readable for CMPTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmptrg::W](W) writer structure"]
impl crate::Writable for CMPTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPTRG to value 0"]
impl crate::Resettable for CMPTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
