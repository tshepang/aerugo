#[doc = "Register `WOL` reader"]
pub type R = crate::R<WOL_SPEC>;
#[doc = "Register `WOL` writer"]
pub type W = crate::W<WOL_SPEC>;
#[doc = "Field `IP` reader - ARP Request IP Address"]
pub type IP_R = crate::FieldReader<u16>;
#[doc = "Field `IP` writer - ARP Request IP Address"]
pub type IP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MAG` reader - Magic Packet Event Enable"]
pub type MAG_R = crate::BitReader;
#[doc = "Field `MAG` writer - Magic Packet Event Enable"]
pub type MAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARP` reader - ARP Request IP Address"]
pub type ARP_R = crate::BitReader;
#[doc = "Field `ARP` writer - ARP Request IP Address"]
pub type ARP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SA1` reader - Specific Address Register 1 Event Enable"]
pub type SA1_R = crate::BitReader;
#[doc = "Field `SA1` writer - Specific Address Register 1 Event Enable"]
pub type SA1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTI` reader - Multicast Hash Event Enable"]
pub type MTI_R = crate::BitReader;
#[doc = "Field `MTI` writer - Multicast Hash Event Enable"]
pub type MTI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - ARP Request IP Address"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Magic Packet Event Enable"]
    #[inline(always)]
    pub fn mag(&self) -> MAG_R {
        MAG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ARP Request IP Address"]
    #[inline(always)]
    pub fn arp(&self) -> ARP_R {
        ARP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specific Address Register 1 Event Enable"]
    #[inline(always)]
    pub fn sa1(&self) -> SA1_R {
        SA1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Multicast Hash Event Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARP Request IP Address"]
    #[inline(always)]
    #[must_use]
    pub fn ip(&mut self) -> IP_W<WOL_SPEC, 0> {
        IP_W::new(self)
    }
    #[doc = "Bit 16 - Magic Packet Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mag(&mut self) -> MAG_W<WOL_SPEC, 16> {
        MAG_W::new(self)
    }
    #[doc = "Bit 17 - ARP Request IP Address"]
    #[inline(always)]
    #[must_use]
    pub fn arp(&mut self) -> ARP_W<WOL_SPEC, 17> {
        ARP_W::new(self)
    }
    #[doc = "Bit 18 - Specific Address Register 1 Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sa1(&mut self) -> SA1_W<WOL_SPEC, 18> {
        SA1_W::new(self)
    }
    #[doc = "Bit 19 - Multicast Hash Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mti(&mut self) -> MTI_W<WOL_SPEC, 19> {
        MTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wake on LAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WOL_SPEC;
impl crate::RegisterSpec for WOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wol::R`](R) reader structure"]
impl crate::Readable for WOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wol::W`](W) writer structure"]
impl crate::Writable for WOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WOL to value 0"]
impl crate::Resettable for WOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
