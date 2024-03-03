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
#[doc = "Field `ENA` reader - ENA field"]
pub type ENA_R = crate::BitReader<bool>;
#[doc = "Field `ENA` writer - ENA field"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ABSWAP` reader - ABSWAP field"]
pub type ABSWAP_R = crate::BitReader<bool>;
#[doc = "Field `ABSWAP` writer - ABSWAP field"]
pub type ABSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `X2X4` reader - X2X4 field"]
pub type X2X4_R = crate::BitReader<bool>;
#[doc = "Field `X2X4` writer - X2X4 field"]
pub type X2X4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSTSRC` reader - RSTSRC field"]
pub type RSTSRC_R = crate::BitReader<bool>;
#[doc = "Field `RSTSRC` writer - RSTSRC field"]
pub type RSTSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - MODE field"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - MODE field"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `INDEX` reader - INDEX field"]
pub type INDEX_R = crate::BitReader<bool>;
#[doc = "Field `INDEX` writer - INDEX field"]
pub type INDEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PAUSE` reader - PAUSE field"]
pub type PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE` writer - PAUSE field"]
pub type PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ENA field"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ABSWAP field"]
    #[inline(always)]
    pub fn abswap(&self) -> ABSWAP_R {
        ABSWAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - X2X4 field"]
    #[inline(always)]
    pub fn x2x4(&self) -> X2X4_R {
        X2X4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RSTSRC field"]
    #[inline(always)]
    pub fn rstsrc(&self) -> RSTSRC_R {
        RSTSRC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MODE field"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - INDEX field"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAUSE field"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENA field"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<0> {
        ENA_W::new(self)
    }
    #[doc = "Bit 4 - ABSWAP field"]
    #[inline(always)]
    pub fn abswap(&mut self) -> ABSWAP_W<4> {
        ABSWAP_W::new(self)
    }
    #[doc = "Bit 5 - X2X4 field"]
    #[inline(always)]
    pub fn x2x4(&mut self) -> X2X4_W<5> {
        X2X4_W::new(self)
    }
    #[doc = "Bit 6 - RSTSRC field"]
    #[inline(always)]
    pub fn rstsrc(&mut self) -> RSTSRC_W<6> {
        RSTSRC_W::new(self)
    }
    #[doc = "Bit 7 - MODE field"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<7> {
        MODE_W::new(self)
    }
    #[doc = "Bit 9 - INDEX field"]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W<9> {
        INDEX_W::new(self)
    }
    #[doc = "Bit 10 - PAUSE field"]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W<10> {
        PAUSE_W::new(self)
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
