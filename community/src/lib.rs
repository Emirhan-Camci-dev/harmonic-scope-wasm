use num_complex::Complex;
use rustfft::FftPlanner;

/// HarmonicScope Community Edition (AGPLv3)
/// Provides single-phase FFT harmonic analysis up to the 25th harmonic.
pub struct HarmonicAnalyzer {
    planner: FftPlanner<f32>,
}

impl Default for HarmonicAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl HarmonicAnalyzer {
    pub fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
        }
    }

    /// Performs a high-speed real-time FFT on incoming waveform telemetry.
    /// Returns the magnitudes of the frequency bins.
    pub fn compute_fft(&mut self, samples: &[f32]) -> Vec<f32> {
        let len = samples.len();
        let mut buffer: Vec<Complex<f32>> =
            samples.iter().map(|&val| Complex::new(val, 0.0)).collect();

        let fft = self.planner.plan_fft_forward(len);
        fft.process(&mut buffer);

        // Calculate magnitudes (normalized)
        buffer
            .iter()
            .map(|c| c.norm() / (len as f32 / 2.0))
            .collect()
    }

    /// Calculates Total Harmonic Distortion (THD) using fundamental and harmonic magnitudes.
    /// Limits to the 25th harmonic for the Community Edition.
    pub fn calculate_thd(&self, magnitudes: &[f32], fundamental_idx: usize) -> f32 {
        if magnitudes.len() <= fundamental_idx || magnitudes[fundamental_idx] == 0.0 {
            return 0.0;
        }

        let fundamental = magnitudes[fundamental_idx];
        let mut sum_sq = 0.0;

        let max_harmonic = std::cmp::min(25, (magnitudes.len() - 1) / fundamental_idx);

        for h in 2..=max_harmonic {
            let idx = h * fundamental_idx;
            if idx < magnitudes.len() {
                sum_sq += magnitudes[idx] * magnitudes[idx];
            }
        }

        (sum_sq.sqrt() / fundamental) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thd_calculation() {
        let mut analyzer = HarmonicAnalyzer::new();
        // Mock data: Fundamental + 3rd harmonic
        let mut samples = vec![0.0; 1024];
        for (i, sample) in samples.iter_mut().enumerate() {
            let t = i as f32 * 2.0 * std::f32::consts::PI / 1024.0;
            // 1.0 fundamental + 0.1 3rd harmonic
            *sample = t.sin() + 0.1 * (3.0 * t).sin();
        }

        let mags = analyzer.compute_fft(&samples);
        let thd = analyzer.calculate_thd(&mags, 1);

        assert!((thd - 10.0).abs() < 1.0); // Should be roughly 10%
    }
}
