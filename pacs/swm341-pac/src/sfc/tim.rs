#[doc = "Register `TIM` reader"]
pub struct R(crate::R<TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM` writer"]
pub struct W(crate::W<TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_SPEC>;
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
impl From<crate::W<TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIP_CHK_ITV` reader - WIP_CHK_ITV field"]
pub type WIP_CHK_ITV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIP_CHK_ITV` writer - WIP_CHK_ITV field"]
pub type WIP_CHK_ITV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 8, O>;
#[doc = "Field `WIP_CHK_LMT` reader - WIP_CHK_LMT field"]
pub type WIP_CHK_LMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIP_CHK_LMT` writer - WIP_CHK_LMT field"]
pub type WIP_CHK_LMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 8, O>;
#[doc = "Field `IDLETO` reader - IDLETO field"]
pub type IDLETO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDLETO` writer - IDLETO field"]
pub type IDLETO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 6, O>;
#[doc = "Field `CSHIGH` reader - CSHIGH field"]
pub type CSHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSHIGH` writer - CSHIGH field"]
pub type CSHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - WIP_CHK_ITV field"]
    #[inline(always)]
    pub fn wip_chk_itv(&self) -> WIP_CHK_ITV_R {
        WIP_CHK_ITV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WIP_CHK_LMT field"]
    #[inline(always)]
    pub fn wip_chk_lmt(&self) -> WIP_CHK_LMT_R {
        WIP_CHK_LMT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - IDLETO field"]
    #[inline(always)]
    pub fn idleto(&self) -> IDLETO_R {
        IDLETO_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - CSHIGH field"]
    #[inline(always)]
    pub fn cshigh(&self) -> CSHIGH_R {
        CSHIGH_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WIP_CHK_ITV field"]
    #[inline(always)]
    pub fn wip_chk_itv(&mut self) -> WIP_CHK_ITV_W<0> {
        WIP_CHK_ITV_W::new(self)
    }
    #[doc = "Bits 8:15 - WIP_CHK_LMT field"]
    #[inline(always)]
    pub fn wip_chk_lmt(&mut self) -> WIP_CHK_LMT_W<8> {
        WIP_CHK_LMT_W::new(self)
    }
    #[doc = "Bits 16:21 - IDLETO field"]
    #[inline(always)]
    pub fn idleto(&mut self) -> IDLETO_W<16> {
        IDLETO_W::new(self)
    }
    #[doc = "Bits 22:23 - CSHIGH field"]
    #[inline(always)]
    pub fn cshigh(&mut self) -> CSHIGH_W<22> {
        CSHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](index.html) module"]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim::R](R) reader structure"]
impl crate::Readable for TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim::W](W) writer structure"]
impl crate::Writable for TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
