use std::f64::consts::PI;
use std::iter;
use fixed::types::I32F32;

/// A one-dimensional discrete kernel
#[derive(Debug)]
pub struct Kernel {
    values: Vec<I32F32>,
    center_index: usize,
}

impl Kernel {
    /// Creates a ́Lánczos resampling kernel with the given `radius`
    /// that performs translation by `offset` pixels (possibly fractional).
    /// Absolute value of `offset` shouldn’t be greater than `radius`.
    pub fn translation_lanczos(radius: usize, offset: f64) -> Self {
        assert!(radius > 0);
        assert!(offset.abs() <= radius as f64);
        
        let center_index = (radius as f64 + offset).ceil() as usize - 1;
        let len = 2 * radius;
        
        let float_values: Vec<_> = (0..len)
            .map(|index| lanczos(radius as f64, index as f64 - center_index as f64 + offset))
            .collect();
        let sum: f64 = float_values.iter().sum();
        
        let values = float_values.iter()
            .map(|v| I32F32::from_num(v / sum))
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
    
    /// Calculates an element of the convolution of the kernel and data
    /// using a data window.
    pub fn apply<'a, I: Iterator<Item = &'a u8>>(&self, window: I) -> u8 {
        let sum: I32F32 = iter::zip(window, self.values.iter())
            .map(|(&w, &k)| I32F32::from_num(w) * k)
            .sum();
        sum.clamp(I32F32::ZERO, UPPER_BOUND).to_num()
    }
}

const UPPER_BOUND: I32F32 = I32F32::lit("255");

fn lanczos(radius: f64, x: f64) -> f64 {
    if -radius < x && x < radius {
        sinc(x) * sinc(x / radius)
    } else {
        0.0
    }
}

fn sinc(x: f64) -> f64 {
    let phi = PI * x;
    let sinc = phi.sin() / phi;
    if sinc.is_finite() { sinc } else { 1.0 }
}
