#[doc = "Register `SYSC_WPMR` reader"]
pub type R = crate::R<SYSC_WPMR_SPEC>;
#[doc = "Register `SYSC_WPMR` writer"]
pub type W = crate::W<SYSC_WPMR_SPEC>;
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WPEN_R = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WPKEY_R = crate::FieldReader<WPKEYSELECT_A>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEYSELECT_A {
    #[doc = "5395523: Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    PASSWD = 5395523,
}
impl From<WPKEYSELECT_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPKEYSELECT_A {
    type Ux = u32;
}
impl WPKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPKEYSELECT_A> {
        match self.bits {
            5395523 => Some(WPKEYSELECT_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == WPKEYSELECT_A::PASSWD
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WPKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, WPKEYSELECT_A>;
impl<'a, REG, const O: u8> WPKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(WPKEYSELECT_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<SYSC_WPMR_SPEC, 0> {
        WPEN_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<SYSC_WPMR_SPEC, 8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysc_wpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysc_wpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSC_WPMR_SPEC;
impl crate::RegisterSpec for SYSC_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysc_wpmr::R`](R) reader structure"]
impl crate::Readable for SYSC_WPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysc_wpmr::W`](W) writer structure"]
impl crate::Writable for SYSC_WPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSC_WPMR to value 0"]
impl crate::Resettable for SYSC_WPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
