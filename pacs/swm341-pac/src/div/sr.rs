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
#[doc = "Field `DIVEND` reader - DIVEND field"]
pub type DIVEND_R = crate::BitReader<bool>;
#[doc = "Field `DIVEND` writer - DIVEND field"]
pub type DIVEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `DIVBUSY` reader - DIVBUSY field"]
pub type DIVBUSY_R = crate::BitReader<bool>;
#[doc = "Field `DIVBUSY` writer - DIVBUSY field"]
pub type DIVBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ROOTENDI` reader - ROOTENDI field"]
pub type ROOTENDI_R = crate::BitReader<bool>;
#[doc = "Field `ROOTENDI` writer - ROOTENDI field"]
pub type ROOTENDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ROOTENDF` reader - ROOTENDF field"]
pub type ROOTENDF_R = crate::BitReader<bool>;
#[doc = "Field `ROOTENDF` writer - ROOTENDF field"]
pub type ROOTENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ROOTBUSY` reader - ROOTBUSY field"]
pub type ROOTBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ROOTBUSY` writer - ROOTBUSY field"]
pub type ROOTBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIVEND field"]
    #[inline(always)]
    pub fn divend(&self) -> DIVEND_R {
        DIVEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIVBUSY field"]
    #[inline(always)]
    pub fn divbusy(&self) -> DIVBUSY_R {
        DIVBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - ROOTENDI field"]
    #[inline(always)]
    pub fn rootendi(&self) -> ROOTENDI_R {
        ROOTENDI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ROOTENDF field"]
    #[inline(always)]
    pub fn rootendf(&self) -> ROOTENDF_R {
        ROOTENDF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ROOTBUSY field"]
    #[inline(always)]
    pub fn rootbusy(&self) -> ROOTBUSY_R {
        ROOTBUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIVEND field"]
    #[inline(always)]
    pub fn divend(&mut self) -> DIVEND_W<0> {
        DIVEND_W::new(self)
    }
    #[doc = "Bit 1 - DIVBUSY field"]
    #[inline(always)]
    pub fn divbusy(&mut self) -> DIVBUSY_W<1> {
        DIVBUSY_W::new(self)
    }
    #[doc = "Bit 8 - ROOTENDI field"]
    #[inline(always)]
    pub fn rootendi(&mut self) -> ROOTENDI_W<8> {
        ROOTENDI_W::new(self)
    }
    #[doc = "Bit 9 - ROOTENDF field"]
    #[inline(always)]
    pub fn rootendf(&mut self) -> ROOTENDF_W<9> {
        ROOTENDF_W::new(self)
    }
    #[doc = "Bit 10 - ROOTBUSY field"]
    #[inline(always)]
    pub fn rootbusy(&mut self) -> ROOTBUSY_W<10> {
        ROOTBUSY_W::new(self)
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
