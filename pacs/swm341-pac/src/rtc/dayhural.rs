#[doc = "Register `DAYHURAL` reader"]
pub struct R(crate::R<DAYHURAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAYHURAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAYHURAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAYHURAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAYHURAL` writer"]
pub struct W(crate::W<DAYHURAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAYHURAL_SPEC>;
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
impl From<crate::W<DAYHURAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAYHURAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOUR` reader - HOUR field"]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOUR` writer - HOUR field"]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAYHURAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `SUN` reader - SUN field"]
pub type SUN_R = crate::BitReader<bool>;
#[doc = "Field `SUN` writer - SUN field"]
pub type SUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
#[doc = "Field `MON` reader - MON field"]
pub type MON_R = crate::BitReader<bool>;
#[doc = "Field `MON` writer - MON field"]
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
#[doc = "Field `TUE` reader - TUE field"]
pub type TUE_R = crate::BitReader<bool>;
#[doc = "Field `TUE` writer - TUE field"]
pub type TUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
#[doc = "Field `WED` reader - WED field"]
pub type WED_R = crate::BitReader<bool>;
#[doc = "Field `WED` writer - WED field"]
pub type WED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
#[doc = "Field `THU` reader - THU field"]
pub type THU_R = crate::BitReader<bool>;
#[doc = "Field `THU` writer - THU field"]
pub type THU_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
#[doc = "Field `FRI` reader - FRI field"]
pub type FRI_R = crate::BitReader<bool>;
#[doc = "Field `FRI` writer - FRI field"]
pub type FRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
#[doc = "Field `SAT` reader - SAT field"]
pub type SAT_R = crate::BitReader<bool>;
#[doc = "Field `SAT` writer - SAT field"]
pub type SAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYHURAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - HOUR field"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - SUN field"]
    #[inline(always)]
    pub fn sun(&self) -> SUN_R {
        SUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MON field"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TUE field"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WED field"]
    #[inline(always)]
    pub fn wed(&self) -> WED_R {
        WED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - THU field"]
    #[inline(always)]
    pub fn thu(&self) -> THU_R {
        THU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FRI field"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SAT field"]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - HOUR field"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W<0> {
        HOUR_W::new(self)
    }
    #[doc = "Bit 5 - SUN field"]
    #[inline(always)]
    pub fn sun(&mut self) -> SUN_W<5> {
        SUN_W::new(self)
    }
    #[doc = "Bit 6 - MON field"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<6> {
        MON_W::new(self)
    }
    #[doc = "Bit 7 - TUE field"]
    #[inline(always)]
    pub fn tue(&mut self) -> TUE_W<7> {
        TUE_W::new(self)
    }
    #[doc = "Bit 8 - WED field"]
    #[inline(always)]
    pub fn wed(&mut self) -> WED_W<8> {
        WED_W::new(self)
    }
    #[doc = "Bit 9 - THU field"]
    #[inline(always)]
    pub fn thu(&mut self) -> THU_W<9> {
        THU_W::new(self)
    }
    #[doc = "Bit 10 - FRI field"]
    #[inline(always)]
    pub fn fri(&mut self) -> FRI_W<10> {
        FRI_W::new(self)
    }
    #[doc = "Bit 11 - SAT field"]
    #[inline(always)]
    pub fn sat(&mut self) -> SAT_W<11> {
        SAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAYHURAL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dayhural](index.html) module"]
pub struct DAYHURAL_SPEC;
impl crate::RegisterSpec for DAYHURAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dayhural::R](R) reader structure"]
impl crate::Readable for DAYHURAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dayhural::W](W) writer structure"]
impl crate::Writable for DAYHURAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAYHURAL to value 0"]
impl crate::Resettable for DAYHURAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
