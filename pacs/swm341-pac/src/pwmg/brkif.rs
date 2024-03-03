#[doc = "Register `BRKIF` reader"]
pub struct R(crate::R<BRKIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRKIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRKIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRKIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRKIF` writer"]
pub struct W(crate::W<BRKIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRKIF_SPEC>;
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
impl From<crate::W<BRKIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRKIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK0` reader - BRK0 field"]
pub type BRK0_R = crate::BitReader<bool>;
#[doc = "Field `BRK0` writer - BRK0 field"]
pub type BRK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIF_SPEC, bool, O>;
#[doc = "Field `BRK1` reader - BRK1 field"]
pub type BRK1_R = crate::BitReader<bool>;
#[doc = "Field `BRK1` writer - BRK1 field"]
pub type BRK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIF_SPEC, bool, O>;
#[doc = "Field `BRK2` reader - BRK2 field"]
pub type BRK2_R = crate::BitReader<bool>;
#[doc = "Field `BRK2` writer - BRK2 field"]
pub type BRK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BRK0 field"]
    #[inline(always)]
    pub fn brk0(&self) -> BRK0_R {
        BRK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK1 field"]
    #[inline(always)]
    pub fn brk1(&self) -> BRK1_R {
        BRK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 field"]
    #[inline(always)]
    pub fn brk2(&self) -> BRK2_R {
        BRK2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK0 field"]
    #[inline(always)]
    pub fn brk0(&mut self) -> BRK0_W<0> {
        BRK0_W::new(self)
    }
    #[doc = "Bit 1 - BRK1 field"]
    #[inline(always)]
    pub fn brk1(&mut self) -> BRK1_W<1> {
        BRK1_W::new(self)
    }
    #[doc = "Bit 2 - BRK2 field"]
    #[inline(always)]
    pub fn brk2(&mut self) -> BRK2_W<2> {
        BRK2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRKIF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brkif](index.html) module"]
pub struct BRKIF_SPEC;
impl crate::RegisterSpec for BRKIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brkif::R](R) reader structure"]
impl crate::Readable for BRKIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brkif::W](W) writer structure"]
impl crate::Writable for BRKIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRKIF to value 0"]
impl crate::Resettable for BRKIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
