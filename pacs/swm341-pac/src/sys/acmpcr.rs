#[doc = "Register `ACMPCR` reader"]
pub struct R(crate::R<ACMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMPCR` writer"]
pub struct W(crate::W<ACMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMPCR_SPEC>;
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
impl From<crate::W<ACMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0ON` reader - CMP0ON field"]
pub type CMP0ON_R = crate::BitReader<bool>;
#[doc = "Field `CMP0ON` writer - CMP0ON field"]
pub type CMP0ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP1ON` reader - CMP1ON field"]
pub type CMP1ON_R = crate::BitReader<bool>;
#[doc = "Field `CMP1ON` writer - CMP1ON field"]
pub type CMP1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP2ON` reader - CMP2ON field"]
pub type CMP2ON_R = crate::BitReader<bool>;
#[doc = "Field `CMP2ON` writer - CMP2ON field"]
pub type CMP2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP0HYS` reader - CMP0HYS field"]
pub type CMP0HYS_R = crate::BitReader<bool>;
#[doc = "Field `CMP0HYS` writer - CMP0HYS field"]
pub type CMP0HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP1HYS` reader - CMP1HYS field"]
pub type CMP1HYS_R = crate::BitReader<bool>;
#[doc = "Field `CMP1HYS` writer - CMP1HYS field"]
pub type CMP1HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP2HYS` reader - CMP2HYS field"]
pub type CMP2HYS_R = crate::BitReader<bool>;
#[doc = "Field `CMP2HYS` writer - CMP2HYS field"]
pub type CMP2HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `_0NVREF` reader - _0NVREF field"]
pub type _0NVREF_R = crate::BitReader<bool>;
#[doc = "Field `_0NVREF` writer - _0NVREF field"]
pub type _0NVREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `_1NVREF` reader - _1NVREF field"]
pub type _1NVREF_R = crate::BitReader<bool>;
#[doc = "Field `_1NVREF` writer - _1NVREF field"]
pub type _1NVREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `_2NVREF` reader - _2NVREF field"]
pub type _2NVREF_R = crate::BitReader<bool>;
#[doc = "Field `_2NVREF` writer - _2NVREF field"]
pub type _2NVREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP0IE` reader - CMP0IE field"]
pub type CMP0IE_R = crate::BitReader<bool>;
#[doc = "Field `CMP0IE` writer - CMP0IE field"]
pub type CMP0IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP1IE` reader - CMP1IE field"]
pub type CMP1IE_R = crate::BitReader<bool>;
#[doc = "Field `CMP1IE` writer - CMP1IE field"]
pub type CMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
#[doc = "Field `CMP2IE` reader - CMP2IE field"]
pub type CMP2IE_R = crate::BitReader<bool>;
#[doc = "Field `CMP2IE` writer - CMP2IE field"]
pub type CMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMP0ON field"]
    #[inline(always)]
    pub fn cmp0on(&self) -> CMP0ON_R {
        CMP0ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP1ON field"]
    #[inline(always)]
    pub fn cmp1on(&self) -> CMP1ON_R {
        CMP1ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP2ON field"]
    #[inline(always)]
    pub fn cmp2on(&self) -> CMP2ON_R {
        CMP2ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CMP0HYS field"]
    #[inline(always)]
    pub fn cmp0hys(&self) -> CMP0HYS_R {
        CMP0HYS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMP1HYS field"]
    #[inline(always)]
    pub fn cmp1hys(&self) -> CMP1HYS_R {
        CMP1HYS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CMP2HYS field"]
    #[inline(always)]
    pub fn cmp2hys(&self) -> CMP2HYS_R {
        CMP2HYS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - _0NVREF field"]
    #[inline(always)]
    pub fn _0nvref(&self) -> _0NVREF_R {
        _0NVREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - _1NVREF field"]
    #[inline(always)]
    pub fn _1nvref(&self) -> _1NVREF_R {
        _1NVREF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - _2NVREF field"]
    #[inline(always)]
    pub fn _2nvref(&self) -> _2NVREF_R {
        _2NVREF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - CMP0IE field"]
    #[inline(always)]
    pub fn cmp0ie(&self) -> CMP0IE_R {
        CMP0IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CMP1IE field"]
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CMP2IE field"]
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP0ON field"]
    #[inline(always)]
    pub fn cmp0on(&mut self) -> CMP0ON_W<0> {
        CMP0ON_W::new(self)
    }
    #[doc = "Bit 1 - CMP1ON field"]
    #[inline(always)]
    pub fn cmp1on(&mut self) -> CMP1ON_W<1> {
        CMP1ON_W::new(self)
    }
    #[doc = "Bit 2 - CMP2ON field"]
    #[inline(always)]
    pub fn cmp2on(&mut self) -> CMP2ON_W<2> {
        CMP2ON_W::new(self)
    }
    #[doc = "Bit 8 - CMP0HYS field"]
    #[inline(always)]
    pub fn cmp0hys(&mut self) -> CMP0HYS_W<8> {
        CMP0HYS_W::new(self)
    }
    #[doc = "Bit 9 - CMP1HYS field"]
    #[inline(always)]
    pub fn cmp1hys(&mut self) -> CMP1HYS_W<9> {
        CMP1HYS_W::new(self)
    }
    #[doc = "Bit 10 - CMP2HYS field"]
    #[inline(always)]
    pub fn cmp2hys(&mut self) -> CMP2HYS_W<10> {
        CMP2HYS_W::new(self)
    }
    #[doc = "Bit 16 - _0NVREF field"]
    #[inline(always)]
    pub fn _0nvref(&mut self) -> _0NVREF_W<16> {
        _0NVREF_W::new(self)
    }
    #[doc = "Bit 17 - _1NVREF field"]
    #[inline(always)]
    pub fn _1nvref(&mut self) -> _1NVREF_W<17> {
        _1NVREF_W::new(self)
    }
    #[doc = "Bit 18 - _2NVREF field"]
    #[inline(always)]
    pub fn _2nvref(&mut self) -> _2NVREF_W<18> {
        _2NVREF_W::new(self)
    }
    #[doc = "Bit 24 - CMP0IE field"]
    #[inline(always)]
    pub fn cmp0ie(&mut self) -> CMP0IE_W<24> {
        CMP0IE_W::new(self)
    }
    #[doc = "Bit 25 - CMP1IE field"]
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMP1IE_W<25> {
        CMP1IE_W::new(self)
    }
    #[doc = "Bit 26 - CMP2IE field"]
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMP2IE_W<26> {
        CMP2IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMPCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmpcr](index.html) module"]
pub struct ACMPCR_SPEC;
impl crate::RegisterSpec for ACMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmpcr::R](R) reader structure"]
impl crate::Readable for ACMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmpcr::W](W) writer structure"]
impl crate::Writable for ACMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMPCR to value 0"]
impl crate::Resettable for ACMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
