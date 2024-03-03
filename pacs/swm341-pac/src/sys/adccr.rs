#[doc = "Register `ADCCR` reader"]
pub struct R(crate::R<ADCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCR` writer"]
pub struct W(crate::W<ADCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCR_SPEC>;
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
impl From<crate::W<ADCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `_0IVREN` reader - _0IVREN field"]
pub type _0IVREN_R = crate::BitReader<bool>;
#[doc = "Field `_0IVREN` writer - _0IVREN field"]
pub type _0IVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCR_SPEC, bool, O>;
#[doc = "Field `_1IVREN` reader - _1IVREN field"]
pub type _1IVREN_R = crate::BitReader<bool>;
#[doc = "Field `_1IVREN` writer - _1IVREN field"]
pub type _1IVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCR_SPEC, bool, O>;
#[doc = "Field `_0IVRSEL` reader - _0IVRSEL field"]
pub type _0IVRSEL_R = crate::BitReader<bool>;
#[doc = "Field `_0IVRSEL` writer - _0IVRSEL field"]
pub type _0IVRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCR_SPEC, bool, O>;
#[doc = "Field `_1IVRSEL` reader - _1IVRSEL field"]
pub type _1IVRSEL_R = crate::BitReader<bool>;
#[doc = "Field `_1IVRSEL` writer - _1IVRSEL field"]
pub type _1IVRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCR_SPEC, bool, O>;
#[doc = "Field `IVRFLT` reader - IVRFLT field"]
pub type IVRFLT_R = crate::BitReader<bool>;
#[doc = "Field `IVRFLT` writer - IVRFLT field"]
pub type IVRFLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCR_SPEC, bool, O>;
#[doc = "Field `IVROuA` reader - IVROuA field"]
pub type IVROU_A_R = crate::BitReader<bool>;
#[doc = "Field `IVROuA` writer - IVROuA field"]
pub type IVROU_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCR_SPEC, bool, O>;
#[doc = "Field `IVRLVL` reader - IVRLVL field"]
pub type IVRLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IVRLVL` writer - IVRLVL field"]
pub type IVRLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IVRTRIM` reader - IVRTRIM field"]
pub type IVRTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IVRTRIM` writer - IVRTRIM field"]
pub type IVRTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - _0IVREN field"]
    #[inline(always)]
    pub fn _0ivren(&self) -> _0IVREN_R {
        _0IVREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - _1IVREN field"]
    #[inline(always)]
    pub fn _1ivren(&self) -> _1IVREN_R {
        _1IVREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - _0IVRSEL field"]
    #[inline(always)]
    pub fn _0ivrsel(&self) -> _0IVRSEL_R {
        _0IVRSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - _1IVRSEL field"]
    #[inline(always)]
    pub fn _1ivrsel(&self) -> _1IVRSEL_R {
        _1IVRSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IVRFLT field"]
    #[inline(always)]
    pub fn ivrflt(&self) -> IVRFLT_R {
        IVRFLT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IVROuA field"]
    #[inline(always)]
    pub fn ivrou_a(&self) -> IVROU_A_R {
        IVROU_A_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - IVRLVL field"]
    #[inline(always)]
    pub fn ivrlvl(&self) -> IVRLVL_R {
        IVRLVL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - IVRTRIM field"]
    #[inline(always)]
    pub fn ivrtrim(&self) -> IVRTRIM_R {
        IVRTRIM_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - _0IVREN field"]
    #[inline(always)]
    pub fn _0ivren(&mut self) -> _0IVREN_W<0> {
        _0IVREN_W::new(self)
    }
    #[doc = "Bit 1 - _1IVREN field"]
    #[inline(always)]
    pub fn _1ivren(&mut self) -> _1IVREN_W<1> {
        _1IVREN_W::new(self)
    }
    #[doc = "Bit 2 - _0IVRSEL field"]
    #[inline(always)]
    pub fn _0ivrsel(&mut self) -> _0IVRSEL_W<2> {
        _0IVRSEL_W::new(self)
    }
    #[doc = "Bit 3 - _1IVRSEL field"]
    #[inline(always)]
    pub fn _1ivrsel(&mut self) -> _1IVRSEL_W<3> {
        _1IVRSEL_W::new(self)
    }
    #[doc = "Bit 4 - IVRFLT field"]
    #[inline(always)]
    pub fn ivrflt(&mut self) -> IVRFLT_W<4> {
        IVRFLT_W::new(self)
    }
    #[doc = "Bit 5 - IVROuA field"]
    #[inline(always)]
    pub fn ivrou_a(&mut self) -> IVROU_A_W<5> {
        IVROU_A_W::new(self)
    }
    #[doc = "Bits 6:8 - IVRLVL field"]
    #[inline(always)]
    pub fn ivrlvl(&mut self) -> IVRLVL_W<6> {
        IVRLVL_W::new(self)
    }
    #[doc = "Bits 9:11 - IVRTRIM field"]
    #[inline(always)]
    pub fn ivrtrim(&mut self) -> IVRTRIM_W<9> {
        IVRTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccr](index.html) module"]
pub struct ADCCR_SPEC;
impl crate::RegisterSpec for ADCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adccr::R](R) reader structure"]
impl crate::Readable for ADCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adccr::W](W) writer structure"]
impl crate::Writable for ADCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCR to value 0"]
impl crate::Resettable for ADCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
