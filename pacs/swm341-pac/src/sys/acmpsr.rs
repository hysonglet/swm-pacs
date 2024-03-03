#[doc = "Register `ACMPSR` reader"]
pub struct R(crate::R<ACMPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMPSR` writer"]
pub struct W(crate::W<ACMPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMPSR_SPEC>;
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
impl From<crate::W<ACMPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0OUT` reader - CMP0OUT field"]
pub type CMP0OUT_R = crate::BitReader<bool>;
#[doc = "Field `CMP0OUT` writer - CMP0OUT field"]
pub type CMP0OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPSR_SPEC, bool, O>;
#[doc = "Field `CMP1OUT` reader - CMP1OUT field"]
pub type CMP1OUT_R = crate::BitReader<bool>;
#[doc = "Field `CMP1OUT` writer - CMP1OUT field"]
pub type CMP1OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPSR_SPEC, bool, O>;
#[doc = "Field `CMP2OUT` reader - CMP2OUT field"]
pub type CMP2OUT_R = crate::BitReader<bool>;
#[doc = "Field `CMP2OUT` writer - CMP2OUT field"]
pub type CMP2OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPSR_SPEC, bool, O>;
#[doc = "Field `CMP0IF` reader - CMP0IF field"]
pub type CMP0IF_R = crate::BitReader<bool>;
#[doc = "Field `CMP0IF` writer - CMP0IF field"]
pub type CMP0IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPSR_SPEC, bool, O>;
#[doc = "Field `CMP1IF` reader - CMP1IF field"]
pub type CMP1IF_R = crate::BitReader<bool>;
#[doc = "Field `CMP1IF` writer - CMP1IF field"]
pub type CMP1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPSR_SPEC, bool, O>;
#[doc = "Field `CMP2IF` reader - CMP2IF field"]
pub type CMP2IF_R = crate::BitReader<bool>;
#[doc = "Field `CMP2IF` writer - CMP2IF field"]
pub type CMP2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMPSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMP0OUT field"]
    #[inline(always)]
    pub fn cmp0out(&self) -> CMP0OUT_R {
        CMP0OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP1OUT field"]
    #[inline(always)]
    pub fn cmp1out(&self) -> CMP1OUT_R {
        CMP1OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP2OUT field"]
    #[inline(always)]
    pub fn cmp2out(&self) -> CMP2OUT_R {
        CMP2OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CMP0IF field"]
    #[inline(always)]
    pub fn cmp0if(&self) -> CMP0IF_R {
        CMP0IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMP1IF field"]
    #[inline(always)]
    pub fn cmp1if(&self) -> CMP1IF_R {
        CMP1IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CMP2IF field"]
    #[inline(always)]
    pub fn cmp2if(&self) -> CMP2IF_R {
        CMP2IF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP0OUT field"]
    #[inline(always)]
    pub fn cmp0out(&mut self) -> CMP0OUT_W<0> {
        CMP0OUT_W::new(self)
    }
    #[doc = "Bit 1 - CMP1OUT field"]
    #[inline(always)]
    pub fn cmp1out(&mut self) -> CMP1OUT_W<1> {
        CMP1OUT_W::new(self)
    }
    #[doc = "Bit 2 - CMP2OUT field"]
    #[inline(always)]
    pub fn cmp2out(&mut self) -> CMP2OUT_W<2> {
        CMP2OUT_W::new(self)
    }
    #[doc = "Bit 8 - CMP0IF field"]
    #[inline(always)]
    pub fn cmp0if(&mut self) -> CMP0IF_W<8> {
        CMP0IF_W::new(self)
    }
    #[doc = "Bit 9 - CMP1IF field"]
    #[inline(always)]
    pub fn cmp1if(&mut self) -> CMP1IF_W<9> {
        CMP1IF_W::new(self)
    }
    #[doc = "Bit 10 - CMP2IF field"]
    #[inline(always)]
    pub fn cmp2if(&mut self) -> CMP2IF_W<10> {
        CMP2IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMPSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmpsr](index.html) module"]
pub struct ACMPSR_SPEC;
impl crate::RegisterSpec for ACMPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmpsr::R](R) reader structure"]
impl crate::Readable for ACMPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmpsr::W](W) writer structure"]
impl crate::Writable for ACMPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMPSR to value 0"]
impl crate::Resettable for ACMPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
