#[doc = "Register `PLLDIV` reader"]
pub struct R(crate::R<PLLDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLDIV` writer"]
pub struct W(crate::W<PLLDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLDIV_SPEC>;
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
impl From<crate::W<PLLDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBDIV` reader - FBDIV field"]
pub type FBDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FBDIV` writer - FBDIV field"]
pub type FBDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLDIV_SPEC, u16, u16, 9, O>;
#[doc = "Field `INDIV` reader - INDIV field"]
pub type INDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDIV` writer - INDIV field"]
pub type INDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLDIV_SPEC, u8, u8, 5, O>;
#[doc = "Field `OUTDIV` reader - OUTDIV field"]
pub type OUTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTDIV` writer - OUTDIV field"]
pub type OUTDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLDIV_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:8 - FBDIV field"]
    #[inline(always)]
    pub fn fbdiv(&self) -> FBDIV_R {
        FBDIV_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:20 - INDIV field"]
    #[inline(always)]
    pub fn indiv(&self) -> INDIV_R {
        INDIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - OUTDIV field"]
    #[inline(always)]
    pub fn outdiv(&self) -> OUTDIV_R {
        OUTDIV_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - FBDIV field"]
    #[inline(always)]
    pub fn fbdiv(&mut self) -> FBDIV_W<0> {
        FBDIV_W::new(self)
    }
    #[doc = "Bits 16:20 - INDIV field"]
    #[inline(always)]
    pub fn indiv(&mut self) -> INDIV_W<16> {
        INDIV_W::new(self)
    }
    #[doc = "Bits 24:25 - OUTDIV field"]
    #[inline(always)]
    pub fn outdiv(&mut self) -> OUTDIV_W<24> {
        OUTDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLDIV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plldiv](index.html) module"]
pub struct PLLDIV_SPEC;
impl crate::RegisterSpec for PLLDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plldiv::R](R) reader structure"]
impl crate::Readable for PLLDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plldiv::W](W) writer structure"]
impl crate::Writable for PLLDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLDIV to value 0"]
impl crate::Resettable for PLLDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
