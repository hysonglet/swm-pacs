#[doc = "Register `BRKSR` reader"]
pub struct R(crate::R<BRKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRKSR` writer"]
pub struct W(crate::W<BRKSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRKSR_SPEC>;
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
impl From<crate::W<BRKSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRKSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK0` reader - BRK0 field"]
pub type BRK0_R = crate::BitReader<bool>;
#[doc = "Field `BRK0` writer - BRK0 field"]
pub type BRK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKSR_SPEC, bool, O>;
#[doc = "Field `BRK1` reader - BRK1 field"]
pub type BRK1_R = crate::BitReader<bool>;
#[doc = "Field `BRK1` writer - BRK1 field"]
pub type BRK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKSR_SPEC, bool, O>;
#[doc = "Field `BRK2` reader - BRK2 field"]
pub type BRK2_R = crate::BitReader<bool>;
#[doc = "Field `BRK2` writer - BRK2 field"]
pub type BRK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - BRK0 field"]
    #[inline(always)]
    pub fn brk0(&self) -> BRK0_R {
        BRK0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK1 field"]
    #[inline(always)]
    pub fn brk1(&self) -> BRK1_R {
        BRK1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK2 field"]
    #[inline(always)]
    pub fn brk2(&self) -> BRK2_R {
        BRK2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - BRK0 field"]
    #[inline(always)]
    pub fn brk0(&mut self) -> BRK0_W<4> {
        BRK0_W::new(self)
    }
    #[doc = "Bit 5 - BRK1 field"]
    #[inline(always)]
    pub fn brk1(&mut self) -> BRK1_W<5> {
        BRK1_W::new(self)
    }
    #[doc = "Bit 6 - BRK2 field"]
    #[inline(always)]
    pub fn brk2(&mut self) -> BRK2_W<6> {
        BRK2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRKSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brksr](index.html) module"]
pub struct BRKSR_SPEC;
impl crate::RegisterSpec for BRKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brksr::R](R) reader structure"]
impl crate::Readable for BRKSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brksr::W](W) writer structure"]
impl crate::Writable for BRKSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRKSR to value 0"]
impl crate::Resettable for BRKSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
