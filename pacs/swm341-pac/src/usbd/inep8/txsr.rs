#[doc = "Register `TXSR` reader"]
pub struct R(crate::R<TXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXSR` writer"]
pub struct W(crate::W<TXSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXSR_SPEC>;
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
impl From<crate::W<TXSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUCC` reader - SUCC field"]
pub type SUCC_R = crate::BitReader<bool>;
#[doc = "Field `SUCC` writer - SUCC field"]
pub type SUCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXSR_SPEC, bool, O>;
#[doc = "Field `DATSNT` reader - DATSNT field"]
pub type DATSNT_R = crate::BitReader<bool>;
#[doc = "Field `DATSNT` writer - DATSNT field"]
pub type DATSNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXSR_SPEC, bool, O>;
#[doc = "Field `NAKSNT` reader - NAKSNT field"]
pub type NAKSNT_R = crate::BitReader<bool>;
#[doc = "Field `NAKSNT` writer - NAKSNT field"]
pub type NAKSNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SUCC field"]
    #[inline(always)]
    pub fn succ(&self) -> SUCC_R {
        SUCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DATSNT field"]
    #[inline(always)]
    pub fn datsnt(&self) -> DATSNT_R {
        DATSNT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKSNT field"]
    #[inline(always)]
    pub fn naksnt(&self) -> NAKSNT_R {
        NAKSNT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SUCC field"]
    #[inline(always)]
    pub fn succ(&mut self) -> SUCC_W<0> {
        SUCC_W::new(self)
    }
    #[doc = "Bit 1 - DATSNT field"]
    #[inline(always)]
    pub fn datsnt(&mut self) -> DATSNT_W<1> {
        DATSNT_W::new(self)
    }
    #[doc = "Bit 3 - NAKSNT field"]
    #[inline(always)]
    pub fn naksnt(&mut self) -> NAKSNT_W<3> {
        NAKSNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txsr](index.html) module"]
pub struct TXSR_SPEC;
impl crate::RegisterSpec for TXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txsr::R](R) reader structure"]
impl crate::Readable for TXSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txsr::W](W) writer structure"]
impl crate::Writable for TXSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXSR to value 0"]
impl crate::Resettable for TXSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
