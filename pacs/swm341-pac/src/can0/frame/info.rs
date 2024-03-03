#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFO` writer"]
pub struct W(crate::W<INFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFO_SPEC>;
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
impl From<crate::W<INFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - DLC field"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - DLC field"]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INFO_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTR` reader - RTR field"]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - RTR field"]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `FF` reader - FF field"]
pub type FF_R = crate::BitReader<bool>;
#[doc = "Field `FF` writer - FF field"]
pub type FF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INFO_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - DLC field"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - RTR field"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FF field"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DLC field"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W<0> {
        DLC_W::new(self)
    }
    #[doc = "Bit 6 - RTR field"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W<6> {
        RTR_W::new(self)
    }
    #[doc = "Bit 7 - FF field"]
    #[inline(always)]
    pub fn ff(&mut self) -> FF_W<7> {
        FF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INFO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [info::W](W) writer structure"]
impl crate::Writable for INFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
