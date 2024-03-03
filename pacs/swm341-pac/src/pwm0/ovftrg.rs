#[doc = "Register `OVFTRG` reader"]
pub struct R(crate::R<OVFTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVFTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVFTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVFTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVFTRG` writer"]
pub struct W(crate::W<OVFTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVFTRG_SPEC>;
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
impl From<crate::W<OVFTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVFTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPEN` reader - UPEN field"]
pub type UPEN_R = crate::BitReader<bool>;
#[doc = "Field `UPEN` writer - UPEN field"]
pub type UPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVFTRG_SPEC, bool, O>;
#[doc = "Field `DNEN` reader - DNEN field"]
pub type DNEN_R = crate::BitReader<bool>;
#[doc = "Field `DNEN` writer - DNEN field"]
pub type DNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVFTRG_SPEC, bool, O>;
#[doc = "Field `MUX` reader - MUX field"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - MUX field"]
pub type MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OVFTRG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - UPEN field"]
    #[inline(always)]
    pub fn upen(&self) -> UPEN_R {
        UPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DNEN field"]
    #[inline(always)]
    pub fn dnen(&self) -> DNEN_R {
        DNEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - MUX field"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UPEN field"]
    #[inline(always)]
    pub fn upen(&mut self) -> UPEN_W<0> {
        UPEN_W::new(self)
    }
    #[doc = "Bit 1 - DNEN field"]
    #[inline(always)]
    pub fn dnen(&mut self) -> DNEN_W<1> {
        DNEN_W::new(self)
    }
    #[doc = "Bits 2:4 - MUX field"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W<2> {
        MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OVFTRG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovftrg](index.html) module"]
pub struct OVFTRG_SPEC;
impl crate::RegisterSpec for OVFTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovftrg::R](R) reader structure"]
impl crate::Readable for OVFTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ovftrg::W](W) writer structure"]
impl crate::Writable for OVFTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVFTRG to value 0"]
impl crate::Resettable for OVFTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
