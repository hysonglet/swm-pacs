#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOC` reader - EOC field"]
pub type EOC_R = crate::BitReader<bool>;
#[doc = "Field `EOC` writer - EOC field"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OVF` reader - OVF field"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - OVF field"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `HALF` reader - HALF field"]
pub type HALF_R = crate::BitReader<bool>;
#[doc = "Field `HALF` writer - HALF field"]
pub type HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `FULL` reader - FULL field"]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `FULL` writer - FULL field"]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `EMPTY` reader - EMPTY field"]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY` writer - EMPTY field"]
pub type EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `LEVEL` reader - LEVEL field"]
pub type LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVEL` writer - LEVEL field"]
pub type LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - EOC field"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVF field"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HALF field"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FULL field"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EMPTY field"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - LEVEL field"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EOC field"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<0> {
        EOC_W::new(self)
    }
    #[doc = "Bit 1 - OVF field"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W<1> {
        OVF_W::new(self)
    }
    #[doc = "Bit 2 - HALF field"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W<2> {
        HALF_W::new(self)
    }
    #[doc = "Bit 3 - FULL field"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W<3> {
        FULL_W::new(self)
    }
    #[doc = "Bit 4 - EMPTY field"]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W<4> {
        EMPTY_W::new(self)
    }
    #[doc = "Bits 5:7 - LEVEL field"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W<5> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
