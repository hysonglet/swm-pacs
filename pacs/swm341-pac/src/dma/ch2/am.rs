#[doc = "Register `AM` reader"]
pub struct R(crate::R<AM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AM` writer"]
pub struct W(crate::W<AM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AM_SPEC>;
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
impl From<crate::W<AM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTAM` reader - DSTAM field"]
pub type DSTAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSTAM` writer - DSTAM field"]
pub type DSTAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSTBIT` reader - DSTBIT field"]
pub type DSTBIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSTBIT` writer - DSTBIT field"]
pub type DSTBIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSTBURST` reader - DSTBURST field"]
pub type DSTBURST_R = crate::BitReader<bool>;
#[doc = "Field `DSTBURST` writer - DSTBURST field"]
pub type DSTBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AM_SPEC, bool, O>;
#[doc = "Field `SRCAM` reader - SRCAM field"]
pub type SRCAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCAM` writer - SRCAM field"]
pub type SRCAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AM_SPEC, u8, u8, 2, O>;
#[doc = "Field `SRCBIT` reader - SRCBIT field"]
pub type SRCBIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCBIT` writer - SRCBIT field"]
pub type SRCBIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AM_SPEC, u8, u8, 2, O>;
#[doc = "Field `SRCBURST` reader - SRCBURST field"]
pub type SRCBURST_R = crate::BitReader<bool>;
#[doc = "Field `SRCBURST` writer - SRCBURST field"]
pub type SRCBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - DSTAM field"]
    #[inline(always)]
    pub fn dstam(&self) -> DSTAM_R {
        DSTAM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DSTBIT field"]
    #[inline(always)]
    pub fn dstbit(&self) -> DSTBIT_R {
        DSTBIT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DSTBURST field"]
    #[inline(always)]
    pub fn dstburst(&self) -> DSTBURST_R {
        DSTBURST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SRCAM field"]
    #[inline(always)]
    pub fn srcam(&self) -> SRCAM_R {
        SRCAM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SRCBIT field"]
    #[inline(always)]
    pub fn srcbit(&self) -> SRCBIT_R {
        SRCBIT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SRCBURST field"]
    #[inline(always)]
    pub fn srcburst(&self) -> SRCBURST_R {
        SRCBURST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSTAM field"]
    #[inline(always)]
    pub fn dstam(&mut self) -> DSTAM_W<0> {
        DSTAM_W::new(self)
    }
    #[doc = "Bits 2:3 - DSTBIT field"]
    #[inline(always)]
    pub fn dstbit(&mut self) -> DSTBIT_W<2> {
        DSTBIT_W::new(self)
    }
    #[doc = "Bit 4 - DSTBURST field"]
    #[inline(always)]
    pub fn dstburst(&mut self) -> DSTBURST_W<4> {
        DSTBURST_W::new(self)
    }
    #[doc = "Bits 8:9 - SRCAM field"]
    #[inline(always)]
    pub fn srcam(&mut self) -> SRCAM_W<8> {
        SRCAM_W::new(self)
    }
    #[doc = "Bits 10:11 - SRCBIT field"]
    #[inline(always)]
    pub fn srcbit(&mut self) -> SRCBIT_W<10> {
        SRCBIT_W::new(self)
    }
    #[doc = "Bit 12 - SRCBURST field"]
    #[inline(always)]
    pub fn srcburst(&mut self) -> SRCBURST_W<12> {
        SRCBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [am](index.html) module"]
pub struct AM_SPEC;
impl crate::RegisterSpec for AM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [am::R](R) reader structure"]
impl crate::Readable for AM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [am::W](W) writer structure"]
impl crate::Writable for AM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AM to value 0"]
impl crate::Resettable for AM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
