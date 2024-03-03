#[doc = "Register `EN` reader"]
pub struct R(crate::R<EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN` writer"]
pub struct W(crate::W<EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SPEC>;
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
impl From<crate::W<EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMR0` reader - TIMR0 field"]
pub type TIMR0_R = crate::BitReader<bool>;
#[doc = "Field `TIMR0` writer - TIMR0 field"]
pub type TIMR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR1` reader - TIMR1 field"]
pub type TIMR1_R = crate::BitReader<bool>;
#[doc = "Field `TIMR1` writer - TIMR1 field"]
pub type TIMR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR2` reader - TIMR2 field"]
pub type TIMR2_R = crate::BitReader<bool>;
#[doc = "Field `TIMR2` writer - TIMR2 field"]
pub type TIMR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR3` reader - TIMR3 field"]
pub type TIMR3_R = crate::BitReader<bool>;
#[doc = "Field `TIMR3` writer - TIMR3 field"]
pub type TIMR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR4` reader - TIMR4 field"]
pub type TIMR4_R = crate::BitReader<bool>;
#[doc = "Field `TIMR4` writer - TIMR4 field"]
pub type TIMR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR5` reader - TIMR5 field"]
pub type TIMR5_R = crate::BitReader<bool>;
#[doc = "Field `TIMR5` writer - TIMR5 field"]
pub type TIMR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR6` reader - TIMR6 field"]
pub type TIMR6_R = crate::BitReader<bool>;
#[doc = "Field `TIMR6` writer - TIMR6 field"]
pub type TIMR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR7` reader - TIMR7 field"]
pub type TIMR7_R = crate::BitReader<bool>;
#[doc = "Field `TIMR7` writer - TIMR7 field"]
pub type TIMR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR8` reader - TIMR8 field"]
pub type TIMR8_R = crate::BitReader<bool>;
#[doc = "Field `TIMR8` writer - TIMR8 field"]
pub type TIMR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR9` reader - TIMR9 field"]
pub type TIMR9_R = crate::BitReader<bool>;
#[doc = "Field `TIMR9` writer - TIMR9 field"]
pub type TIMR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR10` reader - TIMR10 field"]
pub type TIMR10_R = crate::BitReader<bool>;
#[doc = "Field `TIMR10` writer - TIMR10 field"]
pub type TIMR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
#[doc = "Field `TIMR11` reader - TIMR11 field"]
pub type TIMR11_R = crate::BitReader<bool>;
#[doc = "Field `TIMR11` writer - TIMR11 field"]
pub type TIMR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIMR0 field"]
    #[inline(always)]
    pub fn timr0(&self) -> TIMR0_R {
        TIMR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMR1 field"]
    #[inline(always)]
    pub fn timr1(&self) -> TIMR1_R {
        TIMR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMR2 field"]
    #[inline(always)]
    pub fn timr2(&self) -> TIMR2_R {
        TIMR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMR3 field"]
    #[inline(always)]
    pub fn timr3(&self) -> TIMR3_R {
        TIMR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMR4 field"]
    #[inline(always)]
    pub fn timr4(&self) -> TIMR4_R {
        TIMR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMR5 field"]
    #[inline(always)]
    pub fn timr5(&self) -> TIMR5_R {
        TIMR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIMR6 field"]
    #[inline(always)]
    pub fn timr6(&self) -> TIMR6_R {
        TIMR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIMR7 field"]
    #[inline(always)]
    pub fn timr7(&self) -> TIMR7_R {
        TIMR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIMR8 field"]
    #[inline(always)]
    pub fn timr8(&self) -> TIMR8_R {
        TIMR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMR9 field"]
    #[inline(always)]
    pub fn timr9(&self) -> TIMR9_R {
        TIMR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIMR10 field"]
    #[inline(always)]
    pub fn timr10(&self) -> TIMR10_R {
        TIMR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMR11 field"]
    #[inline(always)]
    pub fn timr11(&self) -> TIMR11_R {
        TIMR11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMR0 field"]
    #[inline(always)]
    pub fn timr0(&mut self) -> TIMR0_W<0> {
        TIMR0_W::new(self)
    }
    #[doc = "Bit 1 - TIMR1 field"]
    #[inline(always)]
    pub fn timr1(&mut self) -> TIMR1_W<1> {
        TIMR1_W::new(self)
    }
    #[doc = "Bit 2 - TIMR2 field"]
    #[inline(always)]
    pub fn timr2(&mut self) -> TIMR2_W<2> {
        TIMR2_W::new(self)
    }
    #[doc = "Bit 3 - TIMR3 field"]
    #[inline(always)]
    pub fn timr3(&mut self) -> TIMR3_W<3> {
        TIMR3_W::new(self)
    }
    #[doc = "Bit 4 - TIMR4 field"]
    #[inline(always)]
    pub fn timr4(&mut self) -> TIMR4_W<4> {
        TIMR4_W::new(self)
    }
    #[doc = "Bit 5 - TIMR5 field"]
    #[inline(always)]
    pub fn timr5(&mut self) -> TIMR5_W<5> {
        TIMR5_W::new(self)
    }
    #[doc = "Bit 6 - TIMR6 field"]
    #[inline(always)]
    pub fn timr6(&mut self) -> TIMR6_W<6> {
        TIMR6_W::new(self)
    }
    #[doc = "Bit 7 - TIMR7 field"]
    #[inline(always)]
    pub fn timr7(&mut self) -> TIMR7_W<7> {
        TIMR7_W::new(self)
    }
    #[doc = "Bit 8 - TIMR8 field"]
    #[inline(always)]
    pub fn timr8(&mut self) -> TIMR8_W<8> {
        TIMR8_W::new(self)
    }
    #[doc = "Bit 9 - TIMR9 field"]
    #[inline(always)]
    pub fn timr9(&mut self) -> TIMR9_W<9> {
        TIMR9_W::new(self)
    }
    #[doc = "Bit 10 - TIMR10 field"]
    #[inline(always)]
    pub fn timr10(&mut self) -> TIMR10_W<10> {
        TIMR10_W::new(self)
    }
    #[doc = "Bit 11 - TIMR11 field"]
    #[inline(always)]
    pub fn timr11(&mut self) -> TIMR11_W<11> {
        TIMR11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](index.html) module"]
pub struct EN_SPEC;
impl crate::RegisterSpec for EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en::R](R) reader structure"]
impl crate::Readable for EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en::W](W) writer structure"]
impl crate::Writable for EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
