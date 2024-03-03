#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - TO field"]
pub type TO_R = crate::BitReader<bool>;
#[doc = "Field `TO` writer - TO field"]
pub type TO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `OC0` reader - OC0 field"]
pub type OC0_R = crate::BitReader<bool>;
#[doc = "Field `OC0` writer - OC0 field"]
pub type OC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `OC1` reader - OC1 field"]
pub type OC1_R = crate::BitReader<bool>;
#[doc = "Field `OC1` writer - OC1 field"]
pub type OC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ICR` reader - ICR field"]
pub type ICR_R = crate::BitReader<bool>;
#[doc = "Field `ICR` writer - ICR field"]
pub type ICR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ICF` reader - ICF field"]
pub type ICF_R = crate::BitReader<bool>;
#[doc = "Field `ICF` writer - ICF field"]
pub type ICF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TO field"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OC0 field"]
    #[inline(always)]
    pub fn oc0(&self) -> OC0_R {
        OC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OC1 field"]
    #[inline(always)]
    pub fn oc1(&self) -> OC1_R {
        OC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ICR field"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ICF field"]
    #[inline(always)]
    pub fn icf(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TO field"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W<0> {
        TO_W::new(self)
    }
    #[doc = "Bit 1 - OC0 field"]
    #[inline(always)]
    pub fn oc0(&mut self) -> OC0_W<1> {
        OC0_W::new(self)
    }
    #[doc = "Bit 2 - OC1 field"]
    #[inline(always)]
    pub fn oc1(&mut self) -> OC1_W<2> {
        OC1_W::new(self)
    }
    #[doc = "Bit 3 - ICR field"]
    #[inline(always)]
    pub fn icr(&mut self) -> ICR_W<3> {
        ICR_W::new(self)
    }
    #[doc = "Bit 4 - ICF field"]
    #[inline(always)]
    pub fn icf(&mut self) -> ICF_W<4> {
        ICF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
