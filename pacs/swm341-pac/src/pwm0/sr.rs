#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - STAT field"]
pub type STAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT` writer - STAT field"]
pub type STAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIR` reader - DIR field"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR field"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OUTA` reader - OUTA field"]
pub type OUTA_R = crate::BitReader<bool>;
#[doc = "Field `OUTA` writer - OUTA field"]
pub type OUTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OUTB` reader - OUTB field"]
pub type OUTB_R = crate::BitReader<bool>;
#[doc = "Field `OUTB` writer - OUTB field"]
pub type OUTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OUTAN` reader - OUTAN field"]
pub type OUTAN_R = crate::BitReader<bool>;
#[doc = "Field `OUTAN` writer - OUTAN field"]
pub type OUTAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OUTBN` reader - OUTBN field"]
pub type OUTBN_R = crate::BitReader<bool>;
#[doc = "Field `OUTBN` writer - OUTBN field"]
pub type OUTBN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - STAT field"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - DIR field"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OUTA field"]
    #[inline(always)]
    pub fn outa(&self) -> OUTA_R {
        OUTA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OUTB field"]
    #[inline(always)]
    pub fn outb(&self) -> OUTB_R {
        OUTB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OUTAN field"]
    #[inline(always)]
    pub fn outan(&self) -> OUTAN_R {
        OUTAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OUTBN field"]
    #[inline(always)]
    pub fn outbn(&self) -> OUTBN_R {
        OUTBN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - STAT field"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bit 4 - DIR field"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - OUTA field"]
    #[inline(always)]
    pub fn outa(&mut self) -> OUTA_W<5> {
        OUTA_W::new(self)
    }
    #[doc = "Bit 6 - OUTB field"]
    #[inline(always)]
    pub fn outb(&mut self) -> OUTB_W<6> {
        OUTB_W::new(self)
    }
    #[doc = "Bit 7 - OUTAN field"]
    #[inline(always)]
    pub fn outan(&mut self) -> OUTAN_W<7> {
        OUTAN_W::new(self)
    }
    #[doc = "Bit 8 - OUTBN field"]
    #[inline(always)]
    pub fn outbn(&mut self) -> OUTBN_W<8> {
        OUTBN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
