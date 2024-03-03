#[doc = "Register `BRKIN` reader"]
pub struct R(crate::R<BRKIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRKIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRKIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRKIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRKIN` writer"]
pub struct W(crate::W<BRKIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRKIN_SPEC>;
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
impl From<crate::W<BRKIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRKIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK0A` reader - BRK0A field"]
pub type BRK0A_R = crate::BitReader<bool>;
#[doc = "Field `BRK0A` writer - BRK0A field"]
pub type BRK0A_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIN_SPEC, bool, O>;
#[doc = "Field `BRK1A` reader - BRK1A field"]
pub type BRK1A_R = crate::BitReader<bool>;
#[doc = "Field `BRK1A` writer - BRK1A field"]
pub type BRK1A_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIN_SPEC, bool, O>;
#[doc = "Field `BRK2A` reader - BRK2A field"]
pub type BRK2A_R = crate::BitReader<bool>;
#[doc = "Field `BRK2A` writer - BRK2A field"]
pub type BRK2A_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIN_SPEC, bool, O>;
#[doc = "Field `BRK0B` reader - BRK0B field"]
pub type BRK0B_R = crate::BitReader<bool>;
#[doc = "Field `BRK0B` writer - BRK0B field"]
pub type BRK0B_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIN_SPEC, bool, O>;
#[doc = "Field `BRK1B` reader - BRK1B field"]
pub type BRK1B_R = crate::BitReader<bool>;
#[doc = "Field `BRK1B` writer - BRK1B field"]
pub type BRK1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIN_SPEC, bool, O>;
#[doc = "Field `BRK2B` reader - BRK2B field"]
pub type BRK2B_R = crate::BitReader<bool>;
#[doc = "Field `BRK2B` writer - BRK2B field"]
pub type BRK2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BRK0A field"]
    #[inline(always)]
    pub fn brk0a(&self) -> BRK0A_R {
        BRK0A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK1A field"]
    #[inline(always)]
    pub fn brk1a(&self) -> BRK1A_R {
        BRK1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2A field"]
    #[inline(always)]
    pub fn brk2a(&self) -> BRK2A_R {
        BRK2A_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - BRK0B field"]
    #[inline(always)]
    pub fn brk0b(&self) -> BRK0B_R {
        BRK0B_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK1B field"]
    #[inline(always)]
    pub fn brk1b(&self) -> BRK1B_R {
        BRK1B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK2B field"]
    #[inline(always)]
    pub fn brk2b(&self) -> BRK2B_R {
        BRK2B_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK0A field"]
    #[inline(always)]
    pub fn brk0a(&mut self) -> BRK0A_W<0> {
        BRK0A_W::new(self)
    }
    #[doc = "Bit 1 - BRK1A field"]
    #[inline(always)]
    pub fn brk1a(&mut self) -> BRK1A_W<1> {
        BRK1A_W::new(self)
    }
    #[doc = "Bit 2 - BRK2A field"]
    #[inline(always)]
    pub fn brk2a(&mut self) -> BRK2A_W<2> {
        BRK2A_W::new(self)
    }
    #[doc = "Bit 4 - BRK0B field"]
    #[inline(always)]
    pub fn brk0b(&mut self) -> BRK0B_W<4> {
        BRK0B_W::new(self)
    }
    #[doc = "Bit 5 - BRK1B field"]
    #[inline(always)]
    pub fn brk1b(&mut self) -> BRK1B_W<5> {
        BRK1B_W::new(self)
    }
    #[doc = "Bit 6 - BRK2B field"]
    #[inline(always)]
    pub fn brk2b(&mut self) -> BRK2B_W<6> {
        BRK2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRKIN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brkin](index.html) module"]
pub struct BRKIN_SPEC;
impl crate::RegisterSpec for BRKIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brkin::R](R) reader structure"]
impl crate::Readable for BRKIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brkin::W](W) writer structure"]
impl crate::Writable for BRKIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRKIN to value 0"]
impl crate::Resettable for BRKIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
