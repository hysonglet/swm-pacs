#[doc = "Register `BT1` reader"]
pub struct R(crate::R<BT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT1` writer"]
pub struct W(crate::W<BT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT1_SPEC>;
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
impl From<crate::W<BT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEG1` reader - TSEG1 field"]
pub type TSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG1` writer - TSEG1 field"]
pub type TSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEG2` reader - TSEG2 field"]
pub type TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG2` writer - TSEG2 field"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SAM` reader - SAM field"]
pub type SAM_R = crate::BitReader<bool>;
#[doc = "Field `SAM` writer - SAM field"]
pub type SAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BT1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - TSEG1 field"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - TSEG2 field"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - SAM field"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TSEG1 field"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W<0> {
        TSEG1_W::new(self)
    }
    #[doc = "Bits 4:6 - TSEG2 field"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W<4> {
        TSEG2_W::new(self)
    }
    #[doc = "Bit 7 - SAM field"]
    #[inline(always)]
    pub fn sam(&mut self) -> SAM_W<7> {
        SAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BT1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt1](index.html) module"]
pub struct BT1_SPEC;
impl crate::RegisterSpec for BT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt1::R](R) reader structure"]
impl crate::Readable for BT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt1::W](W) writer structure"]
impl crate::Writable for BT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT1 to value 0"]
impl crate::Resettable for BT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
