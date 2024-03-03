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
#[doc = "Field `SRCFMT` reader - SRCFMT field"]
pub type SRCFMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCFMT` writer - SRCFMT field"]
pub type SRCFMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCANMOD` reader - SCANMOD field"]
pub type SCANMOD_R = crate::BitReader<bool>;
#[doc = "Field `SCANMOD` writer - SCANMOD field"]
pub type SCANMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `NISCOMP` reader - NISCOMP field"]
pub type NISCOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NISCOMP` writer - NISCOMP field"]
pub type NISCOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `HT1COMP` reader - HT1COMP field"]
pub type HT1COMP_R = crate::BitReader<bool>;
#[doc = "Field `HT1COMP` writer - HT1COMP field"]
pub type HT1COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `HT2COMP` reader - HT2COMP field"]
pub type HT2COMP_R = crate::BitReader<bool>;
#[doc = "Field `HT2COMP` writer - HT2COMP field"]
pub type HT2COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `HT3COMP` reader - HT3COMP field"]
pub type HT3COMP_R = crate::BitReader<bool>;
#[doc = "Field `HT3COMP` writer - HT3COMP field"]
pub type HT3COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `QT1COMP` reader - QT1COMP field"]
pub type QT1COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT1COMP` writer - QT1COMP field"]
pub type QT1COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `QT2COMP` reader - QT2COMP field"]
pub type QT2COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT2COMP` writer - QT2COMP field"]
pub type QT2COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `QT3COMP` reader - QT3COMP field"]
pub type QT3COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT3COMP` writer - QT3COMP field"]
pub type QT3COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `OUTFMT` reader - OUTFMT field"]
pub type OUTFMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTFMT` writer - OUTFMT field"]
pub type OUTFMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `YUV2RGB` reader - YUV2RGB field"]
pub type YUV2RGB_R = crate::BitReader<bool>;
#[doc = "Field `YUV2RGB` writer - YUV2RGB field"]
pub type YUV2RGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `UVSWAP` reader - UVSWAP field"]
pub type UVSWAP_R = crate::BitReader<bool>;
#[doc = "Field `UVSWAP` writer - UVSWAP field"]
pub type UVSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RBSWAP` reader - RBSWAP field"]
pub type RBSWAP_R = crate::BitReader<bool>;
#[doc = "Field `RBSWAP` writer - RBSWAP field"]
pub type RBSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `EDSWAP` reader - EDSWAP field"]
pub type EDSWAP_R = crate::BitReader<bool>;
#[doc = "Field `EDSWAP` writer - EDSWAP field"]
pub type EDSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `_565DITH` reader - _565DITH field"]
pub type _565DITH_R = crate::BitReader<bool>;
#[doc = "Field `_565DITH` writer - _565DITH field"]
pub type _565DITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SRCFMT field"]
    #[inline(always)]
    pub fn srcfmt(&self) -> SRCFMT_R {
        SRCFMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SCANMOD field"]
    #[inline(always)]
    pub fn scanmod(&self) -> SCANMOD_R {
        SCANMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - NISCOMP field"]
    #[inline(always)]
    pub fn niscomp(&self) -> NISCOMP_R {
        NISCOMP_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - HT1COMP field"]
    #[inline(always)]
    pub fn ht1comp(&self) -> HT1COMP_R {
        HT1COMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HT2COMP field"]
    #[inline(always)]
    pub fn ht2comp(&self) -> HT2COMP_R {
        HT2COMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HT3COMP field"]
    #[inline(always)]
    pub fn ht3comp(&self) -> HT3COMP_R {
        HT3COMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - QT1COMP field"]
    #[inline(always)]
    pub fn qt1comp(&self) -> QT1COMP_R {
        QT1COMP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - QT2COMP field"]
    #[inline(always)]
    pub fn qt2comp(&self) -> QT2COMP_R {
        QT2COMP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - QT3COMP field"]
    #[inline(always)]
    pub fn qt3comp(&self) -> QT3COMP_R {
        QT3COMP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:16 - OUTFMT field"]
    #[inline(always)]
    pub fn outfmt(&self) -> OUTFMT_R {
        OUTFMT_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - YUV2RGB field"]
    #[inline(always)]
    pub fn yuv2rgb(&self) -> YUV2RGB_R {
        YUV2RGB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UVSWAP field"]
    #[inline(always)]
    pub fn uvswap(&self) -> UVSWAP_R {
        UVSWAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RBSWAP field"]
    #[inline(always)]
    pub fn rbswap(&self) -> RBSWAP_R {
        RBSWAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EDSWAP field"]
    #[inline(always)]
    pub fn edswap(&self) -> EDSWAP_R {
        EDSWAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - _565DITH field"]
    #[inline(always)]
    pub fn _565dith(&self) -> _565DITH_R {
        _565DITH_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRCFMT field"]
    #[inline(always)]
    pub fn srcfmt(&mut self) -> SRCFMT_W<0> {
        SRCFMT_W::new(self)
    }
    #[doc = "Bit 2 - SCANMOD field"]
    #[inline(always)]
    pub fn scanmod(&mut self) -> SCANMOD_W<2> {
        SCANMOD_W::new(self)
    }
    #[doc = "Bits 3:4 - NISCOMP field"]
    #[inline(always)]
    pub fn niscomp(&mut self) -> NISCOMP_W<3> {
        NISCOMP_W::new(self)
    }
    #[doc = "Bit 5 - HT1COMP field"]
    #[inline(always)]
    pub fn ht1comp(&mut self) -> HT1COMP_W<5> {
        HT1COMP_W::new(self)
    }
    #[doc = "Bit 6 - HT2COMP field"]
    #[inline(always)]
    pub fn ht2comp(&mut self) -> HT2COMP_W<6> {
        HT2COMP_W::new(self)
    }
    #[doc = "Bit 7 - HT3COMP field"]
    #[inline(always)]
    pub fn ht3comp(&mut self) -> HT3COMP_W<7> {
        HT3COMP_W::new(self)
    }
    #[doc = "Bits 8:9 - QT1COMP field"]
    #[inline(always)]
    pub fn qt1comp(&mut self) -> QT1COMP_W<8> {
        QT1COMP_W::new(self)
    }
    #[doc = "Bits 10:11 - QT2COMP field"]
    #[inline(always)]
    pub fn qt2comp(&mut self) -> QT2COMP_W<10> {
        QT2COMP_W::new(self)
    }
    #[doc = "Bits 12:13 - QT3COMP field"]
    #[inline(always)]
    pub fn qt3comp(&mut self) -> QT3COMP_W<12> {
        QT3COMP_W::new(self)
    }
    #[doc = "Bits 14:16 - OUTFMT field"]
    #[inline(always)]
    pub fn outfmt(&mut self) -> OUTFMT_W<14> {
        OUTFMT_W::new(self)
    }
    #[doc = "Bit 17 - YUV2RGB field"]
    #[inline(always)]
    pub fn yuv2rgb(&mut self) -> YUV2RGB_W<17> {
        YUV2RGB_W::new(self)
    }
    #[doc = "Bit 18 - UVSWAP field"]
    #[inline(always)]
    pub fn uvswap(&mut self) -> UVSWAP_W<18> {
        UVSWAP_W::new(self)
    }
    #[doc = "Bit 19 - RBSWAP field"]
    #[inline(always)]
    pub fn rbswap(&mut self) -> RBSWAP_W<19> {
        RBSWAP_W::new(self)
    }
    #[doc = "Bit 20 - EDSWAP field"]
    #[inline(always)]
    pub fn edswap(&mut self) -> EDSWAP_W<20> {
        EDSWAP_W::new(self)
    }
    #[doc = "Bit 21 - _565DITH field"]
    #[inline(always)]
    pub fn _565dith(&mut self) -> _565DITH_W<21> {
        _565DITH_W::new(self)
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
