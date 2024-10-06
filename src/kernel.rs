use std::f64::consts::PI;
use std::iter;

/// A one-dimensional discrete kernel
#[derive(Debug)]
pub struct Kernel {
    values: Vec<f64>,
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
        
        let mut values: Vec<_> = (0..len)
            .map(|index| lanczos(radius as f64, index as f64 - center_index as f64 + offset))
            .collect();
        
        // Normalizing
        let sum: f64 = values.iter().sum();
        for value in values.iter_mut() {
            *value /= sum;
        }
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
        let sum: f64 = iter::zip(window, self.values.iter())
            .map(|(&w, &k)| w as f64 * k)
            .sum();
        sum.clamp(0.0, 255.0).round() as u8
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
