use std::f64::consts::PI;
use std::iter;

const FRACTIONAL_BITS: i64 = 32;
const FRACTIONAL_HALF: i64 = 1i64 << (FRACTIONAL_BITS - 1);
const FRACTIONAL_FACTOR: f64 = (1i64 << FRACTIONAL_BITS) as f64;

/// A one-dimensional discrete kernel
#[derive(Debug)]
pub struct Kernel {
    values: Vec<i64>,
    center_index: usize,
}

impl Kernel {
    /// Creates a ́Lánczos resampling kernel with the given `radius`
    /// that performs translation by `offset` pixels (possibly fractional).
    /// Absolute value of `offset` shouldn’t be greater than `radius`.
    pub fn translation_lanczos(radius: usize, offset: f64) -> Self {
        assert!(offset.abs() <= radius as f64);
        
        let center_index = (radius as f64 + offset).ceil() as usize - 1;
        let len = 2 * radius;
        
        let float_values: Vec<_> = (0..len)
            .map(|index| lanczos(radius as f64, index as f64 - center_index as f64 + offset))
            .collect();
        // Normalizing
        let sum: f64 = float_values.iter().sum();
        
        let values = float_values.iter()
            .map(|v| (FRACTIONAL_FACTOR * v / sum).round() as i64)
            .collect();
        Self { values, center_index }
    }
    
    /// Number of elements to the left from the center element.
    pub fn left_radius(&self) -> usize {
        self.center_index
    }
    
    /// Number of elements to the right from the center element
    /// including the center element.
    pub fn right_radius(&self) -> usize {
        self.values.len() - self.center_index
    }
    
    /// Calculates the sum of component-wise product of the kernel and the given window.
    pub fn apply<'a, I: Iterator<Item = &'a u8>>(&self, window: I) -> u8 {
        let sum: i64 = iter::zip(window, self.values.iter())
            .map(|(&w, &k)| w as i64 * k)
            .sum();
        ((sum + FRACTIONAL_HALF) >> FRACTIONAL_BITS).clamp(0, 255) as u8
    }
}

fn sinc(x: f64) -> f64 {
    let phi = PI * x;
    let sinc = phi.sin() / phi;
    if sinc.is_finite() { sinc } else { 1.0 }
}

fn lanczos(radius: f64, x: f64) -> f64 {
    if -radius < x && x < radius {
        sinc(x) * sinc(x / radius)
    } else {
        0.0
    }
}
