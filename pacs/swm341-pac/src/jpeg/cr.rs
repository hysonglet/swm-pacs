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
#[doc = "Field `START` reader - START field"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START field"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RESTART` reader - RESTART field"]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - RESTART field"]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RESET` reader - RESET field"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET field"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REINTRV` reader - REINTRV field"]
pub type REINTRV_R = crate::BitReader<bool>;
#[doc = "Field `REINTRV` writer - REINTRV field"]
pub type REINTRV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LASTBUF` reader - LASTBUF field"]
pub type LASTBUF_R = crate::BitReader<bool>;
#[doc = "Field `LASTBUF` writer - LASTBUF field"]
pub type LASTBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `QTAUTO` reader - QTAUTO field"]
pub type QTAUTO_R = crate::BitReader<bool>;
#[doc = "Field `QTAUTO` writer - QTAUTO field"]
pub type QTAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HTAUTO` reader - HTAUTO field"]
pub type HTAUTO_R = crate::BitReader<bool>;
#[doc = "Field `HTAUTO` writer - HTAUTO field"]
pub type HTAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `QTCNT` reader - QTCNT field"]
pub type QTCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QTCNT` writer - QTCNT field"]
pub type QTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `HTCNT` reader - HTCNT field"]
pub type HTCNT_R = crate::BitReader<bool>;
#[doc = "Field `HTCNT` writer - HTCNT field"]
pub type HTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CUCNT` reader - CUCNT field"]
pub type CUCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CUCNT` writer - CUCNT field"]
pub type CUCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - START field"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESTART field"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RESET field"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REINTRV field"]
    #[inline(always)]
    pub fn reintrv(&self) -> REINTRV_R {
        REINTRV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LASTBUF field"]
    #[inline(always)]
    pub fn lastbuf(&self) -> LASTBUF_R {
        LASTBUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - QTAUTO field"]
    #[inline(always)]
    pub fn qtauto(&self) -> QTAUTO_R {
        QTAUTO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HTAUTO field"]
    #[inline(always)]
    pub fn htauto(&self) -> HTAUTO_R {
        HTAUTO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - QTCNT field"]
    #[inline(always)]
    pub fn qtcnt(&self) -> QTCNT_R {
        QTCNT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - HTCNT field"]
    #[inline(always)]
    pub fn htcnt(&self) -> HTCNT_R {
        HTCNT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:27 - CUCNT field"]
    #[inline(always)]
    pub fn cucnt(&self) -> CUCNT_R {
        CUCNT_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - START field"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - RESTART field"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W<1> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 3 - RESET field"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<3> {
        RESET_W::new(self)
    }
    #[doc = "Bit 4 - REINTRV field"]
    #[inline(always)]
    pub fn reintrv(&mut self) -> REINTRV_W<4> {
        REINTRV_W::new(self)
    }
    #[doc = "Bit 5 - LASTBUF field"]
    #[inline(always)]
    pub fn lastbuf(&mut self) -> LASTBUF_W<5> {
        LASTBUF_W::new(self)
    }
    #[doc = "Bit 7 - QTAUTO field"]
    #[inline(always)]
    pub fn qtauto(&mut self) -> QTAUTO_W<7> {
        QTAUTO_W::new(self)
    }
    #[doc = "Bit 8 - HTAUTO field"]
    #[inline(always)]
    pub fn htauto(&mut self) -> HTAUTO_W<8> {
        HTAUTO_W::new(self)
    }
    #[doc = "Bits 9:10 - QTCNT field"]
    #[inline(always)]
    pub fn qtcnt(&mut self) -> QTCNT_W<9> {
        QTCNT_W::new(self)
    }
    #[doc = "Bit 11 - HTCNT field"]
    #[inline(always)]
    pub fn htcnt(&mut self) -> HTCNT_W<11> {
        HTCNT_W::new(self)
    }
    #[doc = "Bits 12:27 - CUCNT field"]
    #[inline(always)]
    pub fn cucnt(&mut self) -> CUCNT_W<12> {
        CUCNT_W::new(self)
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
