#[doc = "Register `SWMR` reader"]
pub type R = crate::R<SWMR_SPEC>;
#[doc = "Register `SWMR` writer"]
pub type W = crate::W<SWMR_SPEC>;
#[doc = "Field `SADR1` reader - Slave Address 1"]
pub type SADR1_R = crate::FieldReader;
#[doc = "Field `SADR1` writer - Slave Address 1"]
pub type SADR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SADR2` reader - Slave Address 2"]
pub type SADR2_R = crate::FieldReader;
#[doc = "Field `SADR2` writer - Slave Address 2"]
pub type SADR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SADR3` reader - Slave Address 3"]
pub type SADR3_R = crate::FieldReader;
#[doc = "Field `SADR3` writer - Slave Address 3"]
pub type SADR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DATAM` reader - Data Match"]
pub type DATAM_R = crate::FieldReader;
#[doc = "Field `DATAM` writer - Data Match"]
pub type DATAM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&self) -> SADR1_R {
        SADR1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&self) -> SADR2_R {
        SADR2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&self) -> SADR3_R {
        SADR3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn sadr1(&mut self) -> SADR1_W<SWMR_SPEC, 0> {
        SADR1_W::new(self)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn sadr2(&mut self) -> SADR2_W<SWMR_SPEC, 8> {
        SADR2_W::new(self)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    #[must_use]
    pub fn sadr3(&mut self) -> SADR3_W<SWMR_SPEC, 16> {
        SADR3_W::new(self)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    #[must_use]
    pub fn datam(&mut self) -> DATAM_W<SWMR_SPEC, 24> {
        DATAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SleepWalking Matching Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWMR_SPEC;
impl crate::RegisterSpec for SWMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swmr::R`](R) reader structure"]
impl crate::Readable for SWMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swmr::W`](W) writer structure"]
impl crate::Writable for SWMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWMR to value 0"]
impl crate::Resettable for SWMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
