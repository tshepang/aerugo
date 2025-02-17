#[doc = "Register `RXBC` reader"]
pub type R = crate::R<RXBC_SPEC>;
#[doc = "Register `RXBC` writer"]
pub type W = crate::W<RXBC_SPEC>;
#[doc = "Field `RBSA` reader - Receive Buffer Start Address"]
pub type RBSA_R = crate::FieldReader<u16>;
#[doc = "Field `RBSA` writer - Receive Buffer Start Address"]
pub type RBSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn rbsa(&mut self) -> RBSA_W<RXBC_SPEC, 2> {
        RBSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Rx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBC_SPEC;
impl crate::RegisterSpec for RXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbc::R`](R) reader structure"]
impl crate::Readable for RXBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxbc::W`](W) writer structure"]
impl crate::Writable for RXBC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXBC to value 0"]
impl crate::Resettable for RXBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
