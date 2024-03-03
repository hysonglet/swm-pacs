#[doc = "Register `BRKCR` reader"]
pub struct R(crate::R<BRKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRKCR` writer"]
pub struct W(crate::W<BRKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRKCR_SPEC>;
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
impl From<crate::W<BRKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTA` reader - OUTA field"]
pub type OUTA_R = crate::BitReader<bool>;
#[doc = "Field `OUTA` writer - OUTA field"]
pub type OUTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `OFFA` reader - OFFA field"]
pub type OFFA_R = crate::BitReader<bool>;
#[doc = "Field `OFFA` writer - OFFA field"]
pub type OFFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `OUTB` reader - OUTB field"]
pub type OUTB_R = crate::BitReader<bool>;
#[doc = "Field `OUTB` writer - OUTB field"]
pub type OUTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `OFFB` reader - OFFB field"]
pub type OFFB_R = crate::BitReader<bool>;
#[doc = "Field `OFFB` writer - OFFB field"]
pub type OFFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `OUTAN` reader - OUTAN field"]
pub type OUTAN_R = crate::BitReader<bool>;
#[doc = "Field `OUTAN` writer - OUTAN field"]
pub type OUTAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `OUTBN` reader - OUTBN field"]
pub type OUTBN_R = crate::BitReader<bool>;
#[doc = "Field `OUTBN` writer - OUTBN field"]
pub type OUTBN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `STPCNT` reader - STPCNT field"]
pub type STPCNT_R = crate::BitReader<bool>;
#[doc = "Field `STPCNT` writer - STPCNT field"]
pub type STPCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
#[doc = "Field `ACTIVE` reader - ACTIVE field"]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - ACTIVE field"]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OUTA field"]
    #[inline(always)]
    pub fn outa(&self) -> OUTA_R {
        OUTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OFFA field"]
    #[inline(always)]
    pub fn offa(&self) -> OFFA_R {
        OFFA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - OUTB field"]
    #[inline(always)]
    pub fn outb(&self) -> OUTB_R {
        OUTB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OFFB field"]
    #[inline(always)]
    pub fn offb(&self) -> OFFB_R {
        OFFB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - OUTAN field"]
    #[inline(always)]
    pub fn outan(&self) -> OUTAN_R {
        OUTAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OUTBN field"]
    #[inline(always)]
    pub fn outbn(&self) -> OUTBN_R {
        OUTBN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STPCNT field"]
    #[inline(always)]
    pub fn stpcnt(&self) -> STPCNT_R {
        STPCNT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 17 - ACTIVE field"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUTA field"]
    #[inline(always)]
    pub fn outa(&mut self) -> OUTA_W<0> {
        OUTA_W::new(self)
    }
    #[doc = "Bit 1 - OFFA field"]
    #[inline(always)]
    pub fn offa(&mut self) -> OFFA_W<1> {
        OFFA_W::new(self)
    }
    #[doc = "Bit 4 - OUTB field"]
    #[inline(always)]
    pub fn outb(&mut self) -> OUTB_W<4> {
        OUTB_W::new(self)
    }
    #[doc = "Bit 5 - OFFB field"]
    #[inline(always)]
    pub fn offb(&mut self) -> OFFB_W<5> {
        OFFB_W::new(self)
    }
    #[doc = "Bit 8 - OUTAN field"]
    #[inline(always)]
    pub fn outan(&mut self) -> OUTAN_W<8> {
        OUTAN_W::new(self)
    }
    #[doc = "Bit 9 - OUTBN field"]
    #[inline(always)]
    pub fn outbn(&mut self) -> OUTBN_W<9> {
        OUTBN_W::new(self)
    }
    #[doc = "Bit 10 - STPCNT field"]
    #[inline(always)]
    pub fn stpcnt(&mut self) -> STPCNT_W<10> {
        STPCNT_W::new(self)
    }
    #[doc = "Bit 17 - ACTIVE field"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W<17> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRKCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brkcr](index.html) module"]
pub struct BRKCR_SPEC;
impl crate::RegisterSpec for BRKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brkcr::R](R) reader structure"]
impl crate::Readable for BRKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brkcr::W](W) writer structure"]
impl crate::Writable for BRKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRKCR to value 0"]
impl crate::Resettable for BRKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
