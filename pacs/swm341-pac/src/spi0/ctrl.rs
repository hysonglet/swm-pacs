#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - CLKDIV field"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - CLKDIV field"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - SIZE field"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - SIZE field"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPHA` reader - CPHA field"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - CPHA field"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - CPOL field"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - CPOL field"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FFS` reader - FFS field"]
pub type FFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFS` writer - FFS field"]
pub type FFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `MSTR` reader - MSTR field"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - MSTR field"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FAST` reader - FAST field"]
pub type FAST_R = crate::BitReader<bool>;
#[doc = "Field `FAST` writer - FAST field"]
pub type FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMATXEN` reader - DMATXEN field"]
pub type DMATXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMATXEN` writer - DMATXEN field"]
pub type DMATXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMARXEN` reader - DMARXEN field"]
pub type DMARXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMARXEN` writer - DMARXEN field"]
pub type DMARXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FILTE` reader - FILTE field"]
pub type FILTE_R = crate::BitReader<bool>;
#[doc = "Field `FILTE` writer - FILTE field"]
pub type FILTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SSN_H` reader - SSN_H field"]
pub type SSN_H_R = crate::BitReader<bool>;
#[doc = "Field `SSN_H` writer - SSN_H field"]
pub type SSN_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RFTHR` reader - RFTHR field"]
pub type RFTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFTHR` writer - RFTHR field"]
pub type RFTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TFTHR` reader - TFTHR field"]
pub type TFTHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFTHR` writer - TFTHR field"]
pub type TFTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFCLR` reader - RFCLR field"]
pub type RFCLR_R = crate::BitReader<bool>;
#[doc = "Field `RFCLR` writer - RFCLR field"]
pub type RFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TFCLR` reader - TFCLR field"]
pub type TFCLR_R = crate::BitReader<bool>;
#[doc = "Field `TFCLR` writer - TFCLR field"]
pub type TFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LSBF` reader - LSBF field"]
pub type LSBF_R = crate::BitReader<bool>;
#[doc = "Field `LSBF` writer - LSBF field"]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `NSYNC` reader - NSYNC field"]
pub type NSYNC_R = crate::BitReader<bool>;
#[doc = "Field `NSYNC` writer - NSYNC field"]
pub type NSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SIZE field"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - CPHA field"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPOL field"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - FFS field"]
    #[inline(always)]
    pub fn ffs(&self) -> FFS_R {
        FFS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - MSTR field"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FAST field"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMATXEN field"]
    #[inline(always)]
    pub fn dmatxen(&self) -> DMATXEN_R {
        DMATXEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMARXEN field"]
    #[inline(always)]
    pub fn dmarxen(&self) -> DMARXEN_R {
        DMARXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FILTE field"]
    #[inline(always)]
    pub fn filte(&self) -> FILTE_R {
        FILTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SSN_H field"]
    #[inline(always)]
    pub fn ssn_h(&self) -> SSN_H_R {
        SSN_H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - RFTHR field"]
    #[inline(always)]
    pub fn rfthr(&self) -> RFTHR_R {
        RFTHR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - TFTHR field"]
    #[inline(always)]
    pub fn tfthr(&self) -> TFTHR_R {
        TFTHR_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - RFCLR field"]
    #[inline(always)]
    pub fn rfclr(&self) -> RFCLR_R {
        RFCLR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TFCLR field"]
    #[inline(always)]
    pub fn tfclr(&self) -> TFCLR_R {
        TFCLR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - LSBF field"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - NSYNC field"]
    #[inline(always)]
    pub fn nsync(&self) -> NSYNC_R {
        NSYNC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CLKDIV field"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 3 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<3> {
        EN_W::new(self)
    }
    #[doc = "Bits 4:7 - SIZE field"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<4> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 8 - CPHA field"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<8> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 9 - CPOL field"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<9> {
        CPOL_W::new(self)
    }
    #[doc = "Bits 10:11 - FFS field"]
    #[inline(always)]
    pub fn ffs(&mut self) -> FFS_W<10> {
        FFS_W::new(self)
    }
    #[doc = "Bit 12 - MSTR field"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<12> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 13 - FAST field"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W<13> {
        FAST_W::new(self)
    }
    #[doc = "Bit 14 - DMATXEN field"]
    #[inline(always)]
    pub fn dmatxen(&mut self) -> DMATXEN_W<14> {
        DMATXEN_W::new(self)
    }
    #[doc = "Bit 15 - DMARXEN field"]
    #[inline(always)]
    pub fn dmarxen(&mut self) -> DMARXEN_W<15> {
        DMARXEN_W::new(self)
    }
    #[doc = "Bit 16 - FILTE field"]
    #[inline(always)]
    pub fn filte(&mut self) -> FILTE_W<16> {
        FILTE_W::new(self)
    }
    #[doc = "Bit 17 - SSN_H field"]
    #[inline(always)]
    pub fn ssn_h(&mut self) -> SSN_H_W<17> {
        SSN_H_W::new(self)
    }
    #[doc = "Bits 18:20 - RFTHR field"]
    #[inline(always)]
    pub fn rfthr(&mut self) -> RFTHR_W<18> {
        RFTHR_W::new(self)
    }
    #[doc = "Bits 21:23 - TFTHR field"]
    #[inline(always)]
    pub fn tfthr(&mut self) -> TFTHR_W<21> {
        TFTHR_W::new(self)
    }
    #[doc = "Bit 24 - RFCLR field"]
    #[inline(always)]
    pub fn rfclr(&mut self) -> RFCLR_W<24> {
        RFCLR_W::new(self)
    }
    #[doc = "Bit 25 - TFCLR field"]
    #[inline(always)]
    pub fn tfclr(&mut self) -> TFCLR_W<25> {
        TFCLR_W::new(self)
    }
    #[doc = "Bit 28 - LSBF field"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W<28> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 29 - NSYNC field"]
    #[inline(always)]
    pub fn nsync(&mut self) -> NSYNC_W<29> {
        NSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTRL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
