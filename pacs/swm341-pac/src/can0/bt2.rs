#[doc = "Register `BT2` reader"]
pub struct R(crate::R<BT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT2` writer"]
pub struct W(crate::W<BT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT2_SPEC>;
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
impl From<crate::W<BT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - BRP field"]
pub type BRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRP` writer - BRP field"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - BRP field"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BRP field"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BT2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt2](index.html) module"]
pub struct BT2_SPEC;
impl crate::RegisterSpec for BT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt2::R](R) reader structure"]
impl crate::Readable for BT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt2::W](W) writer structure"]
impl crate::Writable for BT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT2 to value 0"]
impl crate::Resettable for BT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
