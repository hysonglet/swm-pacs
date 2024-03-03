#[doc = "Register `RXSR` reader"]
pub struct R(crate::R<RXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXSR` writer"]
pub struct W(crate::W<RXSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXSR_SPEC>;
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
impl From<crate::W<RXSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUCC` reader - SUCC field"]
pub type SUCC_R = crate::BitReader<bool>;
#[doc = "Field `SUCC` writer - SUCC field"]
pub type SUCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXSR_SPEC, bool, O>;
#[doc = "Field `DONE` reader - DONE field"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - DONE field"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXSR_SPEC, bool, O>;
#[doc = "Field `EPNR` reader - EPNR field"]
pub type EPNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNR` writer - EPNR field"]
pub type EPNR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRSZ` reader - TRSZ field"]
pub type TRSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRSZ` writer - TRSZ field"]
pub type TRSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXSR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - SUCC field"]
    #[inline(always)]
    pub fn succ(&self) -> SUCC_R {
        SUCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DONE field"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&self) -> EPNR_R {
        EPNR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 22:31 - TRSZ field"]
    #[inline(always)]
    pub fn trsz(&self) -> TRSZ_R {
        TRSZ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - SUCC field"]
    #[inline(always)]
    pub fn succ(&mut self) -> SUCC_W<0> {
        SUCC_W::new(self)
    }
    #[doc = "Bit 1 - DONE field"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<1> {
        DONE_W::new(self)
    }
    #[doc = "Bits 16:19 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&mut self) -> EPNR_W<16> {
        EPNR_W::new(self)
    }
    #[doc = "Bits 22:31 - TRSZ field"]
    #[inline(always)]
    pub fn trsz(&mut self) -> TRSZ_W<22> {
        TRSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RXSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxsr](index.html) module"]
pub struct RXSR_SPEC;
impl crate::RegisterSpec for RXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxsr::R](R) reader structure"]
impl crate::Readable for RXSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxsr::W](W) writer structure"]
impl crate::Writable for RXSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXSR to value 0"]
impl crate::Resettable for RXSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
