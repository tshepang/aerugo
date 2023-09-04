#[doc = "Register `ST2RPQ[%s]` reader"]
pub type R = crate::R<ST2RPQ_SPEC>;
#[doc = "Register `ST2RPQ[%s]` writer"]
pub type W = crate::W<ST2RPQ_SPEC>;
#[doc = "Field `QNB` reader - Queue Number (0-5)"]
pub type QNB_R = crate::FieldReader;
#[doc = "Field `QNB` writer - Queue Number (0-5)"]
pub type QNB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `VLANP` reader - VLAN Priority"]
pub type VLANP_R = crate::FieldReader;
#[doc = "Field `VLANP` writer - VLAN Priority"]
pub type VLANP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `VLANE` reader - VLAN Enable"]
pub type VLANE_R = crate::BitReader;
#[doc = "Field `VLANE` writer - VLAN Enable"]
pub type VLANE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2ETH` reader - Index of Screening Type 2 EtherType register x"]
pub type I2ETH_R = crate::FieldReader;
#[doc = "Field `I2ETH` writer - Index of Screening Type 2 EtherType register x"]
pub type I2ETH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ETHE` reader - EtherType Enable"]
pub type ETHE_R = crate::BitReader;
#[doc = "Field `ETHE` writer - EtherType Enable"]
pub type ETHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPA` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type COMPA_R = crate::FieldReader;
#[doc = "Field `COMPA` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type COMPA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `COMPAE` reader - Compare A Enable"]
pub type COMPAE_R = crate::BitReader;
#[doc = "Field `COMPAE` writer - Compare A Enable"]
pub type COMPAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPB` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type COMPB_R = crate::FieldReader;
#[doc = "Field `COMPB` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type COMPB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `COMPBE` reader - Compare B Enable"]
pub type COMPBE_R = crate::BitReader;
#[doc = "Field `COMPBE` writer - Compare B Enable"]
pub type COMPBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPC` reader - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type COMPC_R = crate::FieldReader;
#[doc = "Field `COMPC` writer - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
pub type COMPC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `COMPCE` reader - Compare C Enable"]
pub type COMPCE_R = crate::BitReader;
#[doc = "Field `COMPCE` writer - Compare C Enable"]
pub type COMPCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QNB_R {
        QNB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&self) -> VLANP_R {
        VLANP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&self) -> VLANE_R {
        VLANE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&self) -> I2ETH_R {
        I2ETH_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&self) -> ETHE_R {
        ETHE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&self) -> COMPA_R {
        COMPA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&self) -> COMPAE_R {
        COMPAE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&self) -> COMPB_R {
        COMPB_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&self) -> COMPBE_R {
        COMPBE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&self) -> COMPC_R {
        COMPC_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&self) -> COMPCE_R {
        COMPCE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    #[must_use]
    pub fn qnb(&mut self) -> QNB_W<ST2RPQ_SPEC, 0> {
        QNB_W::new(self)
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    #[must_use]
    pub fn vlanp(&mut self) -> VLANP_W<ST2RPQ_SPEC, 4> {
        VLANP_W::new(self)
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlane(&mut self) -> VLANE_W<ST2RPQ_SPEC, 8> {
        VLANE_W::new(self)
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    #[must_use]
    pub fn i2eth(&mut self) -> I2ETH_W<ST2RPQ_SPEC, 9> {
        I2ETH_W::new(self)
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ethe(&mut self) -> ETHE_W<ST2RPQ_SPEC, 12> {
        ETHE_W::new(self)
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    #[must_use]
    pub fn compa(&mut self) -> COMPA_W<ST2RPQ_SPEC, 13> {
        COMPA_W::new(self)
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compae(&mut self) -> COMPAE_W<ST2RPQ_SPEC, 18> {
        COMPAE_W::new(self)
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    #[must_use]
    pub fn compb(&mut self) -> COMPB_W<ST2RPQ_SPEC, 19> {
        COMPB_W::new(self)
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compbe(&mut self) -> COMPBE_W<ST2RPQ_SPEC, 24> {
        COMPBE_W::new(self)
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    #[must_use]
    pub fn compc(&mut self) -> COMPC_W<ST2RPQ_SPEC, 25> {
        COMPC_W::new(self)
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compce(&mut self) -> COMPCE_W<ST2RPQ_SPEC, 30> {
        COMPCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Screening Type 2 Register Priority Queue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2rpq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2rpq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST2RPQ_SPEC;
impl crate::RegisterSpec for ST2RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2rpq::R`](R) reader structure"]
impl crate::Readable for ST2RPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st2rpq::W`](W) writer structure"]
impl crate::Writable for ST2RPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2RPQ[%s]
to value 0"]
impl crate::Resettable for ST2RPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
