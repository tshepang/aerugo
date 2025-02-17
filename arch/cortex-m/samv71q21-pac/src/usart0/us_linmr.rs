#[doc = "Register `US_LINMR` reader"]
pub type R = crate::R<US_LINMR_SPEC>;
#[doc = "Register `US_LINMR` writer"]
pub type W = crate::W<US_LINMR_SPEC>;
#[doc = "Field `NACT` reader - LIN Node Action"]
pub type NACT_R = crate::FieldReader<NACTSELECT_A>;
#[doc = "LIN Node Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NACTSELECT_A {
    #[doc = "0: The USART transmits the response."]
    PUBLISH = 0,
    #[doc = "1: The USART receives the response."]
    SUBSCRIBE = 1,
    #[doc = "2: The USART does not transmit and does not receive the response."]
    IGNORE = 2,
}
impl From<NACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NACTSELECT_A {
    type Ux = u8;
}
impl NACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NACTSELECT_A> {
        match self.bits {
            0 => Some(NACTSELECT_A::PUBLISH),
            1 => Some(NACTSELECT_A::SUBSCRIBE),
            2 => Some(NACTSELECT_A::IGNORE),
            _ => None,
        }
    }
    #[doc = "The USART transmits the response."]
    #[inline(always)]
    pub fn is_publish(&self) -> bool {
        *self == NACTSELECT_A::PUBLISH
    }
    #[doc = "The USART receives the response."]
    #[inline(always)]
    pub fn is_subscribe(&self) -> bool {
        *self == NACTSELECT_A::SUBSCRIBE
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == NACTSELECT_A::IGNORE
    }
}
#[doc = "Field `NACT` writer - LIN Node Action"]
pub type NACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, NACTSELECT_A>;
impl<'a, REG, const O: u8> NACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The USART transmits the response."]
    #[inline(always)]
    pub fn publish(self) -> &'a mut crate::W<REG> {
        self.variant(NACTSELECT_A::PUBLISH)
    }
    #[doc = "The USART receives the response."]
    #[inline(always)]
    pub fn subscribe(self) -> &'a mut crate::W<REG> {
        self.variant(NACTSELECT_A::SUBSCRIBE)
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(NACTSELECT_A::IGNORE)
    }
}
#[doc = "Field `PARDIS` reader - Parity Disable"]
pub type PARDIS_R = crate::BitReader;
#[doc = "Field `PARDIS` writer - Parity Disable"]
pub type PARDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHKDIS` reader - Checksum Disable"]
pub type CHKDIS_R = crate::BitReader;
#[doc = "Field `CHKDIS` writer - Checksum Disable"]
pub type CHKDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHKTYP` reader - Checksum Type"]
pub type CHKTYP_R = crate::BitReader;
#[doc = "Field `CHKTYP` writer - Checksum Type"]
pub type CHKTYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLM` reader - Data Length Mode"]
pub type DLM_R = crate::BitReader;
#[doc = "Field `DLM` writer - Data Length Mode"]
pub type DLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FSDIS` reader - Frame Slot Mode Disable"]
pub type FSDIS_R = crate::BitReader;
#[doc = "Field `FSDIS` writer - Frame Slot Mode Disable"]
pub type FSDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPTYP` reader - Wakeup Signal Type"]
pub type WKUPTYP_R = crate::BitReader;
#[doc = "Field `WKUPTYP` writer - Wakeup Signal Type"]
pub type WKUPTYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLC` reader - Data Length Control"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Control"]
pub type DLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PDCM` reader - DMAC Mode"]
pub type PDCM_R = crate::BitReader;
#[doc = "Field `PDCM` writer - DMAC Mode"]
pub type PDCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCDIS` reader - Synchronization Disable"]
pub type SYNCDIS_R = crate::BitReader;
#[doc = "Field `SYNCDIS` writer - Synchronization Disable"]
pub type SYNCDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    pub fn nact(&self) -> NACT_R {
        NACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    pub fn pardis(&self) -> PARDIS_R {
        PARDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    pub fn chkdis(&self) -> CHKDIS_R {
        CHKDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    pub fn chktyp(&self) -> CHKTYP_R {
        CHKTYP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    pub fn dlm(&self) -> DLM_R {
        DLM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    pub fn fsdis(&self) -> FSDIS_R {
        FSDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    pub fn wkuptyp(&self) -> WKUPTYP_R {
        WKUPTYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline(always)]
    pub fn pdcm(&self) -> PDCM_R {
        PDCM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline(always)]
    pub fn syncdis(&self) -> SYNCDIS_R {
        SYNCDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    #[must_use]
    pub fn nact(&mut self) -> NACT_W<US_LINMR_SPEC, 0> {
        NACT_W::new(self)
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pardis(&mut self) -> PARDIS_W<US_LINMR_SPEC, 2> {
        PARDIS_W::new(self)
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chkdis(&mut self) -> CHKDIS_W<US_LINMR_SPEC, 3> {
        CHKDIS_W::new(self)
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    #[must_use]
    pub fn chktyp(&mut self) -> CHKTYP_W<US_LINMR_SPEC, 4> {
        CHKTYP_W::new(self)
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dlm(&mut self) -> DLM_W<US_LINMR_SPEC, 5> {
        DLM_W::new(self)
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fsdis(&mut self) -> FSDIS_W<US_LINMR_SPEC, 6> {
        FSDIS_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    #[must_use]
    pub fn wkuptyp(&mut self) -> WKUPTYP_W<US_LINMR_SPEC, 7> {
        WKUPTYP_W::new(self)
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<US_LINMR_SPEC, 8> {
        DLC_W::new(self)
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdcm(&mut self) -> PDCM_W<US_LINMR_SPEC, 16> {
        PDCM_W::new(self)
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline(always)]
    #[must_use]
    pub fn syncdis(&mut self) -> SYNCDIS_W<US_LINMR_SPEC, 17> {
        SYNCDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LIN Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_linmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_linmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LINMR_SPEC;
impl crate::RegisterSpec for US_LINMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_linmr::R`](R) reader structure"]
impl crate::Readable for US_LINMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`us_linmr::W`](W) writer structure"]
impl crate::Writable for US_LINMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LINMR to value 0"]
impl crate::Resettable for US_LINMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
