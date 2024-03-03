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
#[doc = "Field `CLKDIV` reader - CLKDIV field"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - CLKDIV field"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLKINV` reader - CLKINV field"]
pub type CLKINV_R = crate::BitReader<bool>;
#[doc = "Field `CLKINV` writer - CLKINV field"]
pub type CLKINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CLKALW` reader - CLKALW field"]
pub type CLKALW_R = crate::BitReader<bool>;
#[doc = "Field `CLKALW` writer - CLKALW field"]
pub type CLKALW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BURSTEN` reader - BURSTEN field"]
pub type BURSTEN_R = crate::BitReader<bool>;
#[doc = "Field `BURSTEN` writer - BURSTEN field"]
pub type BURSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BURSTLEN` reader - BURSTLEN field"]
pub type BURSTLEN_R = crate::BitReader<bool>;
#[doc = "Field `BURSTLEN` writer - BURSTLEN field"]
pub type BURSTLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AUTORESTA` reader - AUTORESTA field"]
pub type AUTORESTA_R = crate::BitReader<bool>;
#[doc = "Field `AUTORESTA` writer - AUTORESTA field"]
pub type AUTORESTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IMMRELOAD` reader - IMMRELOAD field"]
pub type IMMRELOAD_R = crate::BitReader<bool>;
#[doc = "Field `IMMRELOAD` writer - IMMRELOAD field"]
pub type IMMRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `VBPRELOAD` reader - VBPRELOAD field"]
pub type VBPRELOAD_R = crate::BitReader<bool>;
#[doc = "Field `VBPRELOAD` writer - VBPRELOAD field"]
pub type VBPRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FORMAT` reader - FORMAT field"]
pub type FORMAT_R = crate::BitReader<bool>;
#[doc = "Field `FORMAT` writer - FORMAT field"]
pub type FORMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SEREN` reader - SEREN field"]
pub type SEREN_R = crate::BitReader<bool>;
#[doc = "Field `SEREN` writer - SEREN field"]
pub type SEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MPUEN` reader - MPUEN field"]
pub type MPUEN_R = crate::BitReader<bool>;
#[doc = "Field `MPUEN` writer - MPUEN field"]
pub type MPUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `VSYNCINV` reader - VSYNCINV field"]
pub type VSYNCINV_R = crate::BitReader<bool>;
#[doc = "Field `VSYNCINV` writer - VSYNCINV field"]
pub type VSYNCINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSYNCINV` reader - HSYNCINV field"]
pub type HSYNCINV_R = crate::BitReader<bool>;
#[doc = "Field `HSYNCINV` writer - HSYNCINV field"]
pub type HSYNCINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - CLKINV field"]
    #[inline(always)]
    pub fn clkinv(&self) -> CLKINV_R {
        CLKINV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CLKALW field"]
    #[inline(always)]
    pub fn clkalw(&self) -> CLKALW_R {
        CLKALW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BURSTEN field"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BURSTLEN field"]
    #[inline(always)]
    pub fn burstlen(&self) -> BURSTLEN_R {
        BURSTLEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - AUTORESTA field"]
    #[inline(always)]
    pub fn autoresta(&self) -> AUTORESTA_R {
        AUTORESTA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IMMRELOAD field"]
    #[inline(always)]
    pub fn immreload(&self) -> IMMRELOAD_R {
        IMMRELOAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VBPRELOAD field"]
    #[inline(always)]
    pub fn vbpreload(&self) -> VBPRELOAD_R {
        VBPRELOAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FORMAT field"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SEREN field"]
    #[inline(always)]
    pub fn seren(&self) -> SEREN_R {
        SEREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MPUEN field"]
    #[inline(always)]
    pub fn mpuen(&self) -> MPUEN_R {
        MPUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VSYNCINV field"]
    #[inline(always)]
    pub fn vsyncinv(&self) -> VSYNCINV_R {
        VSYNCINV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HSYNCINV field"]
    #[inline(always)]
    pub fn hsyncinv(&self) -> HSYNCINV_R {
        HSYNCINV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 6 - CLKINV field"]
    #[inline(always)]
    pub fn clkinv(&mut self) -> CLKINV_W<6> {
        CLKINV_W::new(self)
    }
    #[doc = "Bit 7 - CLKALW field"]
    #[inline(always)]
    pub fn clkalw(&mut self) -> CLKALW_W<7> {
        CLKALW_W::new(self)
    }
    #[doc = "Bit 8 - BURSTEN field"]
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<8> {
        BURSTEN_W::new(self)
    }
    #[doc = "Bit 9 - BURSTLEN field"]
    #[inline(always)]
    pub fn burstlen(&mut self) -> BURSTLEN_W<9> {
        BURSTLEN_W::new(self)
    }
    #[doc = "Bit 13 - AUTORESTA field"]
    #[inline(always)]
    pub fn autoresta(&mut self) -> AUTORESTA_W<13> {
        AUTORESTA_W::new(self)
    }
    #[doc = "Bit 14 - IMMRELOAD field"]
    #[inline(always)]
    pub fn immreload(&mut self) -> IMMRELOAD_W<14> {
        IMMRELOAD_W::new(self)
    }
    #[doc = "Bit 15 - VBPRELOAD field"]
    #[inline(always)]
    pub fn vbpreload(&mut self) -> VBPRELOAD_W<15> {
        VBPRELOAD_W::new(self)
    }
    #[doc = "Bit 16 - FORMAT field"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W<16> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 17 - SEREN field"]
    #[inline(always)]
    pub fn seren(&mut self) -> SEREN_W<17> {
        SEREN_W::new(self)
    }
    #[doc = "Bit 18 - MPUEN field"]
    #[inline(always)]
    pub fn mpuen(&mut self) -> MPUEN_W<18> {
        MPUEN_W::new(self)
    }
    #[doc = "Bit 19 - VSYNCINV field"]
    #[inline(always)]
    pub fn vsyncinv(&mut self) -> VSYNCINV_W<19> {
        VSYNCINV_W::new(self)
    }
    #[doc = "Bit 20 - HSYNCINV field"]
    #[inline(always)]
    pub fn hsyncinv(&mut self) -> HSYNCINV_W<20> {
        HSYNCINV_W::new(self)
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
