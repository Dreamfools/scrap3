pub trait FloatLibmExt: Sized + Copy {
    /// Alternative to [f32::powi]/[f64::powi] using [libm]
    fn powi_m(self, n: i32) -> Self;
    /// Alternative to [f32::powf]/[f64::powf] using [libm]
    fn powf_m(self, n: Self) -> Self;
    /// Alternative to [f32::exp]/[f64::exp] using [libm]
    fn exp_m(self) -> Self;
    /// Alternative to [f32::exp2]/[f64::exp2] using [libm]
    fn exp2_m(self) -> Self;
    /// Alternative to [f32::exp_m1]/[f64::exp_m1] using [libm]
    fn exp_m1_m(self) -> Self;
    /// Alternative to [f32::ln]/[f64::ln] using [libm]
    fn ln_m(self) -> Self;
    /// Alternative to [f32::ln_1p]/[f64::ln_1p] using [libm]
    fn ln_1p_m(self) -> Self;
    /// Alternative to [f32::log]/[f64::log] using [libm]
    fn log_m(self, base: Self) -> Self;
    /// Alternative to [f32::log2]/[f64::log2] using [libm]
    fn log2_m(self) -> Self;
    /// Alternative to [f32::log10]/[f64::log10] using [libm]
    fn log10_m(self) -> Self;
    /// Alternative to [f32::cbrt]/[f64::cbrt] using [libm]
    fn cbrt_m(self) -> Self;
    /// Alternative to [f32::hypot]/[f64::hypot] using [libm]
    fn hypot_m(self, other: Self) -> Self;
    /// Alternative to [f32::sin]/[f64::sin] using [libm]
    fn sin_m(self) -> Self;
    /// Alternative to [f32::cos]/[f64::cos] using [libm]
    fn cos_m(self) -> Self;
    /// Alternative to [f32::tan]/[f64::tan] using [libm]
    fn tan_m(self) -> Self;
    /// Alternative to [f32::asin]/[f64::asin] using [libm]
    fn asin_m(self) -> Self;
    /// Alternative to [f32::acos]/[f64::acos] using [libm]
    fn acos_m(self) -> Self;
    /// Alternative to [f32::atan]/[f64::atan] using [libm]
    fn atan_m(self) -> Self;
    /// Alternative to [f32::atan2]/[f64::atan2] using [libm]
    fn atan2_m(self, other: Self) -> Self;
    /// Alternative to [f32::sin_cos]/[f64::sin_cos] using [libm]
    fn sin_cos_m(self) -> (Self, Self) {
        (self.sin_m(), self.cos_m())
    }
    /// Alternative to [f32::sinh]/[f64::sinh] using [libm]
    fn sinh_m(self) -> Self;
    /// Alternative to [f32::cosh]/[f64::cosh] using [libm]
    fn cosh_m(self) -> Self;
    /// Alternative to [f32::tanh]/[f64::tanh] using [libm]
    fn tanh_m(self) -> Self;
    /// Alternative to [f32::asinh]/[f64::asinh] using [libm]
    fn asinh_m(self) -> Self;
    /// Alternative to [f32::acosh]/[f64::acosh] using [libm]
    fn acosh_m(self) -> Self;
    /// Alternative to [f32::atanh]/[f64::atanh] using [libm]
    fn atanh_m(self) -> Self;
}

impl FloatLibmExt for f32 {
    fn powi_m(self, n: i32) -> Self {
        self.powf_m(n as f32)
    }

    fn powf_m(self, n: Self) -> Self {
        libm::powf(self, n)
    }

    fn exp_m(self) -> Self {
        libm::expf(self)
    }

    fn exp2_m(self) -> Self {
        libm::exp2f(self)
    }

    fn exp_m1_m(self) -> Self {
        libm::expm1f(self)
    }

    fn ln_m(self) -> Self {
        libm::logf(self)
    }

    fn ln_1p_m(self) -> Self {
        libm::log1pf(self)
    }

    fn log_m(self, base: Self) -> Self {
        libm::logf(self) / libm::logf(base)
    }

    fn log2_m(self) -> Self {
        libm::log2f(self)
    }

    fn log10_m(self) -> Self {
        libm::log10f(self)
    }

    fn cbrt_m(self) -> Self {
        libm::cbrtf(self)
    }

    fn hypot_m(self, other: Self) -> Self {
        libm::hypotf(self, other)
    }

    fn sin_m(self) -> Self {
        libm::sinf(self)
    }

    fn cos_m(self) -> Self {
        libm::cosf(self)
    }

    fn tan_m(self) -> Self {
        libm::tanf(self)
    }

    fn asin_m(self) -> Self {
        libm::asinf(self)
    }

    fn acos_m(self) -> Self {
        libm::acosf(self)
    }

    fn atan_m(self) -> Self {
        libm::atanf(self)
    }

    fn atan2_m(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }

    fn sinh_m(self) -> Self {
        libm::sinhf(self)
    }

    fn cosh_m(self) -> Self {
        libm::coshf(self)
    }

    fn tanh_m(self) -> Self {
        libm::tanhf(self)
    }

    fn asinh_m(self) -> Self {
        libm::asinhf(self)
    }

    fn acosh_m(self) -> Self {
        libm::acoshf(self)
    }

    fn atanh_m(self) -> Self {
        libm::atanhf(self)
    }
}

impl FloatLibmExt for f64 {
    fn powi_m(self, n: i32) -> Self {
        self.powf_m(n as f64)
    }

    fn powf_m(self, n: Self) -> Self {
        libm::pow(self, n)
    }

    fn exp_m(self) -> Self {
        libm::exp(self)
    }

    fn exp2_m(self) -> Self {
        libm::exp2(self)
    }

    fn exp_m1_m(self) -> Self {
        libm::expm1(self)
    }

    fn ln_m(self) -> Self {
        libm::log(self)
    }

    fn ln_1p_m(self) -> Self {
        libm::log1p(self)
    }

    fn log_m(self, base: Self) -> Self {
        libm::log(self) / libm::log(base)
    }

    fn log2_m(self) -> Self {
        libm::log2(self)
    }

    fn log10_m(self) -> Self {
        libm::log10(self)
    }

    fn cbrt_m(self) -> Self {
        libm::cbrt(self)
    }

    fn hypot_m(self, other: Self) -> Self {
        libm::hypot(self, other)
    }

    fn sin_m(self) -> Self {
        libm::sin(self)
    }

    fn cos_m(self) -> Self {
        libm::cos(self)
    }

    fn tan_m(self) -> Self {
        libm::tan(self)
    }

    fn asin_m(self) -> Self {
        libm::asin(self)
    }

    fn acos_m(self) -> Self {
        libm::acos(self)
    }

    fn atan_m(self) -> Self {
        libm::atan(self)
    }

    fn atan2_m(self, other: Self) -> Self {
        libm::atan2(self, other)
    }

    fn sinh_m(self) -> Self {
        libm::sinh(self)
    }

    fn cosh_m(self) -> Self {
        libm::cosh(self)
    }

    fn tanh_m(self) -> Self {
        libm::tanh(self)
    }

    fn asinh_m(self) -> Self {
        libm::asinh(self)
    }

    fn acosh_m(self) -> Self {
        libm::acosh(self)
    }

    fn atanh_m(self) -> Self {
        libm::atanh(self)
    }
}
