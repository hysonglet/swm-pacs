#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - START field"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START field"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RANGE` reader - RANGE field"]
pub type RANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE` writer - RANGE field"]
pub type RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `SINCOS` reader - SINCOS field"]
pub type SINCOS_R = crate::BitReader<bool>;
#[doc = "Field `SINCOS` writer - SINCOS field"]
pub type SINCOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `MULDIV` reader - MULDIV field"]
pub type MULDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULDIV` writer - MULDIV field"]
pub type MULDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - START field"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RANGE field"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SINCOS field"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MULDIV field"]
    #[inline(always)]
    pub fn muldiv(&self) -> MULDIV_R {
        MULDIV_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - START field"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 1:2 - RANGE field"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W<1> {
        RANGE_W::new(self)
    }
    #[doc = "Bit 3 - SINCOS field"]
    #[inline(always)]
    pub fn sincos(&mut self) -> SINCOS_W<3> {
        SINCOS_W::new(self)
    }
    #[doc = "Bits 4:5 - MULDIV field"]
    #[inline(always)]
    pub fn muldiv(&mut self) -> MULDIV_W<4> {
        MULDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMD register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
