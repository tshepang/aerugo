#[doc = "Register `HSTFNUM` reader"]
pub type R = crate::R<HSTFNUM_SPEC>;
#[doc = "Register `HSTFNUM` writer"]
pub type W = crate::W<HSTFNUM_SPEC>;
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MFNUM_R = crate::FieldReader;
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub type MFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FNUM` writer - Frame Number"]
pub type FNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub type FLENHIGH_R = crate::FieldReader;
#[doc = "Field `FLENHIGH` writer - Frame Length"]
pub type FLENHIGH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn mfnum(&mut self) -> MFNUM_W<HSTFNUM_SPEC, 0> {
        MFNUM_W::new(self)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FNUM_W<HSTFNUM_SPEC, 3> {
        FNUM_W::new(self)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn flenhigh(&mut self) -> FLENHIGH_W<HSTFNUM_SPEC, 16> {
        FLENHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstfnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstfnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTFNUM_SPEC;
impl crate::RegisterSpec for HSTFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstfnum::R`](R) reader structure"]
impl crate::Readable for HSTFNUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstfnum::W`](W) writer structure"]
impl crate::Writable for HSTFNUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTFNUM to value 0"]
impl crate::Resettable for HSTFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
