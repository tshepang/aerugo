#[doc = "Register `US_CR_USART_MODE` writer"]
pub type W = crate::W<US_CR_USART_MODE_SPEC>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RSTRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RSTTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RSTSTA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STTBRK` writer - Start Break"]
pub type STTBRK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPBRK` writer - Stop Break"]
pub type STPBRK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STTTO` writer - Clear TIMEOUT Flag and Start Timeout After Next Character Received"]
pub type STTTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENDA` writer - Send Address"]
pub type SENDA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub type RSTIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub type RSTNACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RETTO` writer - Start Timeout Immediately"]
pub type RETTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTREN` writer - Data Terminal Ready Enable"]
pub type DTREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTRDIS` writer - Data Terminal Ready Disable"]
pub type DTRDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub type RTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub type RTSDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RSTRX_W<US_CR_USART_MODE_SPEC, 2> {
        RSTRX_W::new(self)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RSTTX_W<US_CR_USART_MODE_SPEC, 3> {
        RSTTX_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<US_CR_USART_MODE_SPEC, 4> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<US_CR_USART_MODE_SPEC, 5> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<US_CR_USART_MODE_SPEC, 6> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<US_CR_USART_MODE_SPEC, 7> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RSTSTA_W<US_CR_USART_MODE_SPEC, 8> {
        RSTSTA_W::new(self)
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    #[must_use]
    pub fn sttbrk(&mut self) -> STTBRK_W<US_CR_USART_MODE_SPEC, 9> {
        STTBRK_W::new(self)
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    #[must_use]
    pub fn stpbrk(&mut self) -> STPBRK_W<US_CR_USART_MODE_SPEC, 10> {
        STPBRK_W::new(self)
    }
    #[doc = "Bit 11 - Clear TIMEOUT Flag and Start Timeout After Next Character Received"]
    #[inline(always)]
    #[must_use]
    pub fn sttto(&mut self) -> STTTO_W<US_CR_USART_MODE_SPEC, 11> {
        STTTO_W::new(self)
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    #[must_use]
    pub fn senda(&mut self) -> SENDA_W<US_CR_USART_MODE_SPEC, 12> {
        SENDA_W::new(self)
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn rstit(&mut self) -> RSTIT_W<US_CR_USART_MODE_SPEC, 13> {
        RSTIT_W::new(self)
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn rstnack(&mut self) -> RSTNACK_W<US_CR_USART_MODE_SPEC, 14> {
        RSTNACK_W::new(self)
    }
    #[doc = "Bit 15 - Start Timeout Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn retto(&mut self) -> RETTO_W<US_CR_USART_MODE_SPEC, 15> {
        RETTO_W::new(self)
    }
    #[doc = "Bit 16 - Data Terminal Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtren(&mut self) -> DTREN_W<US_CR_USART_MODE_SPEC, 16> {
        DTREN_W::new(self)
    }
    #[doc = "Bit 17 - Data Terminal Ready Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtrdis(&mut self) -> DTRDIS_W<US_CR_USART_MODE_SPEC, 17> {
        DTRDIS_W::new(self)
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<US_CR_USART_MODE_SPEC, 18> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsdis(&mut self) -> RTSDIS_W<US_CR_USART_MODE_SPEC, 19> {
        RTSDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_cr_usart_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_CR_USART_MODE_SPEC;
impl crate::RegisterSpec for US_CR_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_cr_usart_mode::W`](W) writer structure"]
impl crate::Writable for US_CR_USART_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_CR_USART_MODE to value 0"]
impl crate::Resettable for US_CR_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
