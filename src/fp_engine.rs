use bitfield::bitfield;

bitfield! {
    struct LLFPR(u32);
    impl Debug;
    u32;
    zero, set_zero: 0;
    mantissa, set_mantissa: 23, 1;
    exponent, set_exponent: 30, 24;
    sign, set_sign: 31;
}

bitfield! {
    pub struct SFPR(u32);
    impl Debug;
    u32;
    pub mantissa, set_mantissa: 22, 0;
    pub exponent, set_exponent: 30, 23;
    pub sign, set_sign: 31;
}

#[derive(Debug)]
pub struct LowLatencyFloat(LLFPR);

impl LowLatencyFloat {
    pub fn zero() -> Self {
        LowLatencyFloat(LLFPR(0))
    }

    pub fn representation(&self) -> u32 {
        self.0.0
    }
}

impl From<u32> for LowLatencyFloat {
    fn from(bits: u32) -> Self {
        LowLatencyFloat(LLFPR(bits))
    }
}

#[derive(Debug)]
pub struct StandardFloat(SFPR);

impl StandardFloat {
    pub fn zero() -> Self {
        StandardFloat(SFPR(0))
    }

    pub fn representation(&self) -> u32 {
        self.0.0
    }
}

impl From<u32> for StandardFloat {
    fn from(bits: u32) -> Self {
        StandardFloat(SFPR(bits))
    }
}

impl From<LowLatencyFloat> for StandardFloat {
    fn from(llf: LowLatencyFloat) -> Self {
        if llf.representation() == 0 {
            return StandardFloat::zero();
        }

        let sign = if llf.0.sign() { 0 } else { 1 };
        let exponent = llf.0.exponent();
        let mantissa = llf.0.mantissa();

        let sf_exponent = exponent + 127;
        let sf_mantissa = mantissa << (23 - 22);

        let sf_bits = (sign << 31) | (sf_exponent << 23) | sf_mantissa;
        StandardFloat(SFPR(sf_bits))
    }
}

impl From<StandardFloat> for LowLatencyFloat {
    fn from(sf: StandardFloat) -> Self {
        if sf.representation() == 0 {
            return LowLatencyFloat::zero();
        }

        let sign = if sf.0.sign() { 0 } else { 1 };
        let exponent = sf.0.exponent();
        let mantissa = sf.0.mantissa();

        let llf_exponent = exponent.checked_sub(127).unwrap_or(0);
        let llf_mantissa = mantissa >> (23 - 22);
        
        let llf_bits = (sign << 31) | (llf_exponent << 24) | (llf_mantissa << 1);
        LowLatencyFloat(LLFPR(llf_bits))
    }
}