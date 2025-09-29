//! SIMD accelerated Temperature Scaling operations for paraphym
//!
//! This module provides vectorized implementations of temperature scaling operations
//! optimized for AI inference tasks, focusing on zero-allocation and lock-free design.

use once_cell::sync::Lazy;

use crate::error::SimdResult;
use crate::runtime::TemperatureDispatch;

/// Scalar implementation of temperature scaling as a fallback
fn scalar_temperature_scale(logits: &mut [f32], temperature: f32) -> SimdResult<()> {
    if temperature <= 0.0 {
        return Err(crate::error::SimdError::InvalidInput(
            "Temperature must be positive".to_string(),
        ));
    }

    let inv_temp = 1.0 / temperature;
    for logit in logits.iter_mut() {
        *logit *= inv_temp;
    }
    Ok(())
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn avx2_temperature_scale(logits: &mut [f32], temperature: f32) -> SimdResult<()> {
    use std::arch::x86_64::*;

    if temperature <= 0.0 {
        return Err(crate::error::SimdError::InvalidInput(
            "Temperature must be positive".to_string(),
        ));
    }

    let inv_temp = _mm256_set1_ps(1.0 / temperature);
    let len = logits.len();
    let mut i = 0;

    while i + 8 <= len {
        let ptr = logits.as_mut_ptr().add(i) as *mut f32;
        let val = _mm256_loadu_ps(ptr);
        let scaled = _mm256_mul_ps(val, inv_temp);
        _mm256_storeu_ps(ptr, scaled);
        i += 8;
    }

    // Handle remainder scalar
    let inv_temp_scalar = 1.0 / temperature;
    for logit in logits.iter_mut().skip(i) {
        *logit *= inv_temp_scalar;
    }

    Ok(())
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse4.1")]
unsafe fn sse41_temperature_scale(logits: &mut [f32], temperature: f32) -> SimdResult<()> {
    use std::arch::x86_64::*;

    if temperature <= 0.0 {
        return Err(crate::error::SimdError::InvalidInput(
            "Temperature must be positive".to_string(),
        ));
    }

    let inv_temp = _mm_set1_ps(1.0 / temperature);
    let len = logits.len();
    let mut i = 0;

    while i + 4 <= len {
        let ptr = logits.as_mut_ptr().add(i) as *mut f32;
        let val = _mm_loadu_ps(ptr);
        let scaled = _mm_mul_ps(val, inv_temp);
        _mm_storeu_ps(ptr, scaled);
        i += 4;
    }

    // Handle remainder scalar
    let inv_temp_scalar = 1.0 / temperature;
    for logit in logits.iter_mut().skip(i) {
        *logit *= inv_temp_scalar;
    }

    Ok(())
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn neon_temperature_scale(logits: &mut [f32], temperature: f32) -> SimdResult<()> {
    use std::arch::aarch64::*;

    if temperature <= 0.0 {
        return Err(crate::error::SimdError::InvalidInput(
            "Temperature must be positive".to_string(),
        ));
    }

    let inv_temp = vdupq_n_f32(1.0 / temperature);
    let len = logits.len();
    let mut i = 0;

    while i + 4 <= len {
        let ptr = unsafe { logits.as_mut_ptr().add(i) };
        let val = unsafe { vld1q_f32(ptr) };
        let scaled = vmulq_f32(val, inv_temp);
        unsafe { vst1q_f32(ptr, scaled) };
        i += 4;
    }

    // Handle remainder scalar
    let inv_temp_scalar = 1.0 / temperature;
    for logit in logits.iter_mut().skip(i) {
        *logit *= inv_temp_scalar;
    }

    Ok(())
}

/// Applies temperature scaling to logits using the best available implementation.
/// Temperature scaling adjusts the logits by dividing them by the temperature value.
/// Lower temperature (<1.0) makes the distribution sharper (more confident),
/// while higher temperature (>1.0) makes it softer (more uniform).
/// Dispatch table for temperature scaling operations across different CPU capabilities
pub static TEMPERATURE_DISPATCH: Lazy<TemperatureDispatch> = Lazy::new(create_temperature_dispatch);

/// Scale logits by temperature using the best available SIMD implementation
///
/// Applies temperature scaling to logits in-place: logit /= temperature
/// Automatically selects optimal SIMD implementation based on runtime CPU features.
///
/// # Arguments
/// * `logits` - Mutable slice of logits to scale
/// * `temperature` - Temperature scaling factor (must be > 0.0)
///
/// # Returns
/// * `SimdResult<()>` - Success or error if temperature is invalid
pub fn scale_temperature(logits: &mut [f32], temperature: f32) -> SimdResult<()> {
    if logits.is_empty() {
        return Err(crate::error::SimdError::InvalidInput(
            "Logits array cannot be empty".to_string(),
        ));
    }
    TEMPERATURE_DISPATCH.call(logits, temperature)
}

fn create_temperature_dispatch() -> TemperatureDispatch {
    TemperatureDispatch {
        avx512: None, // To be implemented if AVX512 support is added

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        avx2: Some(avx2_temperature_scale),
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        avx2: None,

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        sse41: Some(sse41_temperature_scale),
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        sse41: None,

        #[cfg(target_arch = "aarch64")]
        neon: Some(neon_temperature_scale),
        #[cfg(not(target_arch = "aarch64"))]
        neon: None,

        scalar: scalar_temperature_scale,
    }
}
