#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - VALUE field"]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - VALUE field"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u16, u16, 12, O>;
#[doc = "Field `CHNUM` reader - CHNUM field"]
pub type CHNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHNUM` writer - CHNUM field"]
pub type CHNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - VALUE field"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - CHNUM field"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - VALUE field"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 12:15 - CHNUM field"]
    #[inline(always)]
    pub fn chnum(&mut self) -> CHNUM_W<12> {
        CHNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
