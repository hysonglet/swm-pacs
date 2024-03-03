#[doc = "Register `FIFO` reader"]
pub struct R(crate::R<FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO` writer"]
pub struct W(crate::W<FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SPEC>;
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
impl From<crate::W<FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXLVL` reader - RXLVL field"]
pub type RXLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXLVL` writer - RXLVL field"]
pub type RXLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXLVL` reader - TXLVL field"]
pub type TXLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXLVL` writer - TXLVL field"]
pub type TXLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u8, u8, 8, O>;
#[doc = "Field `RXTHR` reader - RXTHR field"]
pub type RXTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTHR` writer - RXTHR field"]
pub type RXTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXTHR` reader - TXTHR field"]
pub type TXTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXTHR` writer - TXTHR field"]
pub type TXTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RXLVL field"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TXLVL field"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RXTHR field"]
    #[inline(always)]
    pub fn rxthr(&self) -> RXTHR_R {
        RXTHR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TXTHR field"]
    #[inline(always)]
    pub fn txthr(&self) -> TXTHR_R {
        TXTHR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RXLVL field"]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W<0> {
        RXLVL_W::new(self)
    }
    #[doc = "Bits 8:15 - TXLVL field"]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W<8> {
        TXLVL_W::new(self)
    }
    #[doc = "Bits 16:23 - RXTHR field"]
    #[inline(always)]
    pub fn rxthr(&mut self) -> RXTHR_W<16> {
        RXTHR_W::new(self)
    }
    #[doc = "Bits 24:31 - TXTHR field"]
    #[inline(always)]
    pub fn txthr(&mut self) -> TXTHR_W<24> {
        TXTHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](index.html) module"]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo::R](R) reader structure"]
impl crate::Readable for FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo::W](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
