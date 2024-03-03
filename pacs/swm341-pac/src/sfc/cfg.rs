#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDTYPE` reader - CMDTYPE field"]
pub type CMDTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDTYPE` writer - CMDTYPE field"]
pub type CMDTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMDWREN` reader - CMDWREN field"]
pub type CMDWREN_R = crate::BitReader<bool>;
#[doc = "Field `CMDWREN` writer - CMDWREN field"]
pub type CMDWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CLKDIV` reader - CLKDIV field"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - CLKDIV field"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADDR4L` reader - ADDR4L field"]
pub type ADDR4L_R = crate::BitReader<bool>;
#[doc = "Field `ADDR4L` writer - ADDR4L field"]
pub type ADDR4L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DATA4L_PP` reader - DATA4L_PP field"]
pub type DATA4L_PP_R = crate::BitReader<bool>;
#[doc = "Field `DATA4L_PP` writer - DATA4L_PP field"]
pub type DATA4L_PP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DATA4L_RD` reader - DATA4L_RD field"]
pub type DATA4L_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA4L_RD` writer - DATA4L_RD field"]
pub type DATA4L_RD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `WREN` reader - WREN field"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - WREN field"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - CMDTYPE field"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CMDTYPE_R {
        CMDTYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - CMDWREN field"]
    #[inline(always)]
    pub fn cmdwren(&self) -> CMDWREN_R {
        CMDWREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - ADDR4L field"]
    #[inline(always)]
    pub fn addr4l(&self) -> ADDR4L_R {
        ADDR4L_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DATA4L_PP field"]
    #[inline(always)]
    pub fn data4l_pp(&self) -> DATA4L_PP_R {
        DATA4L_PP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - DATA4L_RD field"]
    #[inline(always)]
    pub fn data4l_rd(&self) -> DATA4L_RD_R {
        DATA4L_RD_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - WREN field"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CMDTYPE field"]
    #[inline(always)]
    pub fn cmdtype(&mut self) -> CMDTYPE_W<0> {
        CMDTYPE_W::new(self)
    }
    #[doc = "Bit 5 - CMDWREN field"]
    #[inline(always)]
    pub fn cmdwren(&mut self) -> CMDWREN_W<5> {
        CMDWREN_W::new(self)
    }
    #[doc = "Bits 6:7 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<6> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - ADDR4L field"]
    #[inline(always)]
    pub fn addr4l(&mut self) -> ADDR4L_W<8> {
        ADDR4L_W::new(self)
    }
    #[doc = "Bit 9 - DATA4L_PP field"]
    #[inline(always)]
    pub fn data4l_pp(&mut self) -> DATA4L_PP_W<9> {
        DATA4L_PP_W::new(self)
    }
    #[doc = "Bits 10:11 - DATA4L_RD field"]
    #[inline(always)]
    pub fn data4l_rd(&mut self) -> DATA4L_RD_W<10> {
        DATA4L_RD_W::new(self)
    }
    #[doc = "Bit 12 - WREN field"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
