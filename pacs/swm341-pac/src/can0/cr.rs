#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST` reader - RST field"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - RST field"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LOM` reader - LOM field"]
pub type LOM_R = crate::BitReader<bool>;
#[doc = "Field `LOM` writer - LOM field"]
pub type LOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STM` reader - STM field"]
pub type STM_R = crate::BitReader<bool>;
#[doc = "Field `STM` writer - STM field"]
pub type STM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - SLEEP field"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - SLEEP field"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RST field"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOM field"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STM field"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SLEEP field"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST field"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<0> {
        RST_W::new(self)
    }
    #[doc = "Bit 1 - LOM field"]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W<1> {
        LOM_W::new(self)
    }
    #[doc = "Bit 2 - STM field"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W<2> {
        STM_W::new(self)
    }
    #[doc = "Bit 4 - SLEEP field"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<4> {
        SLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
