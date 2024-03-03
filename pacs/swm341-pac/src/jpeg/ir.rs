#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEDONE` reader - IEDONE field"]
pub type IEDONE_R = crate::BitReader<bool>;
#[doc = "Field `IEDONE` writer - IEDONE field"]
pub type IEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `IEEMPTY` reader - IEEMPTY field"]
pub type IEEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `IEEMPTY` writer - IEEMPTY field"]
pub type IEEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `IEERROR` reader - IEERROR field"]
pub type IEERROR_R = crate::BitReader<bool>;
#[doc = "Field `IEERROR` writer - IEERROR field"]
pub type IEERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ICDONE` reader - ICDONE field"]
pub type ICDONE_R = crate::BitReader<bool>;
#[doc = "Field `ICDONE` writer - ICDONE field"]
pub type ICDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ICEMPTY` reader - ICEMPTY field"]
pub type ICEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ICEMPTY` writer - ICEMPTY field"]
pub type ICEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ICERROR` reader - ICERROR field"]
pub type ICERROR_R = crate::BitReader<bool>;
#[doc = "Field `ICERROR` writer - ICERROR field"]
pub type ICERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `IFDONE` reader - IFDONE field"]
pub type IFDONE_R = crate::BitReader<bool>;
#[doc = "Field `IFDONE` writer - IFDONE field"]
pub type IFDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `IFEMPTY` reader - IFEMPTY field"]
pub type IFEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `IFEMPTY` writer - IFEMPTY field"]
pub type IFEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `IFERROR` reader - IFERROR field"]
pub type IFERROR_R = crate::BitReader<bool>;
#[doc = "Field `IFERROR` writer - IFERROR field"]
pub type IFERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IEDONE field"]
    #[inline(always)]
    pub fn iedone(&self) -> IEDONE_R {
        IEDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IEEMPTY field"]
    #[inline(always)]
    pub fn ieempty(&self) -> IEEMPTY_R {
        IEEMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IEERROR field"]
    #[inline(always)]
    pub fn ieerror(&self) -> IEERROR_R {
        IEERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - ICDONE field"]
    #[inline(always)]
    pub fn icdone(&self) -> ICDONE_R {
        ICDONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ICEMPTY field"]
    #[inline(always)]
    pub fn icempty(&self) -> ICEMPTY_R {
        ICEMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ICERROR field"]
    #[inline(always)]
    pub fn icerror(&self) -> ICERROR_R {
        ICERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - IFDONE field"]
    #[inline(always)]
    pub fn ifdone(&self) -> IFDONE_R {
        IFDONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - IFEMPTY field"]
    #[inline(always)]
    pub fn ifempty(&self) -> IFEMPTY_R {
        IFEMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IFERROR field"]
    #[inline(always)]
    pub fn iferror(&self) -> IFERROR_R {
        IFERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IEDONE field"]
    #[inline(always)]
    pub fn iedone(&mut self) -> IEDONE_W<0> {
        IEDONE_W::new(self)
    }
    #[doc = "Bit 2 - IEEMPTY field"]
    #[inline(always)]
    pub fn ieempty(&mut self) -> IEEMPTY_W<2> {
        IEEMPTY_W::new(self)
    }
    #[doc = "Bit 3 - IEERROR field"]
    #[inline(always)]
    pub fn ieerror(&mut self) -> IEERROR_W<3> {
        IEERROR_W::new(self)
    }
    #[doc = "Bit 5 - ICDONE field"]
    #[inline(always)]
    pub fn icdone(&mut self) -> ICDONE_W<5> {
        ICDONE_W::new(self)
    }
    #[doc = "Bit 7 - ICEMPTY field"]
    #[inline(always)]
    pub fn icempty(&mut self) -> ICEMPTY_W<7> {
        ICEMPTY_W::new(self)
    }
    #[doc = "Bit 8 - ICERROR field"]
    #[inline(always)]
    pub fn icerror(&mut self) -> ICERROR_W<8> {
        ICERROR_W::new(self)
    }
    #[doc = "Bit 10 - IFDONE field"]
    #[inline(always)]
    pub fn ifdone(&mut self) -> IFDONE_W<10> {
        IFDONE_W::new(self)
    }
    #[doc = "Bit 12 - IFEMPTY field"]
    #[inline(always)]
    pub fn ifempty(&mut self) -> IFEMPTY_W<12> {
        IFEMPTY_W::new(self)
    }
    #[doc = "Bit 13 - IFERROR field"]
    #[inline(always)]
    pub fn iferror(&mut self) -> IFERROR_W<13> {
        IFERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
