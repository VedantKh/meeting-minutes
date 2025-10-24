//! Dynamic resampling with anti-aliasing
//! 
//! This module provides high-quality dynamic resampling capabilities that handle
//! 

use anyhow::Result;

/// Dynamic resampler that handles rate changes gracefully with anti-aliasing
pub struct DynamicResampler {
    target_rate: u32,
    last_samples: Vec<f32>, // Buffer for continuity between chunks
}

impl DynamicResampler {
    /// Create a new dynamic resampler
    pub fn new(target_rate: u32) -> Self {
        Self { 
            target_rate,
            last_samples: Vec::new(),
        }
    }

    /// Handle sample rate changes
    pub fn handle_rate_change(&mut self) {
        self.last_samples.clear();
    }

    /// Resample audio to target rate with anti-aliasing
    /// 
    pub fn resample(&mut self, audio: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
        if from_rate == to_rate {
            return audio.to_vec();
        }

        let ratio = from_rate as f64 / to_rate as f64;
        
        let filtered_audio = if ratio > 1.0 {
            self.apply_antialiasing_filter(audio, ratio)
        } else {
            audio.to_vec()
        };

        self.resample_cubic(&filtered_audio, ratio)
    }
    
    fn apply_antialiasing_filter(&self, audio: &[f32], ratio: f64) -> Vec<f32> {
        if audio.len() < 3 {
            return audio.to_vec();
        }
        
        let cutoff_normalized = 0.4 / ratio;
        
        // Simple moving average filter (basic low-pass)
        let filter_size = ((1.0 / cutoff_normalized) as usize).max(1).min(7);
        
        let mut filtered = Vec::with_capacity(audio.len());
        
        for i in 0..audio.len() {
            let start = if i >= filter_size { i - filter_size } else { 0 };
            let end = (i + filter_size + 1).min(audio.len());
            
            let sum: f32 = audio[start..end].iter().sum();
            let avg = sum / (end - start) as f32;
            filtered.push(avg);
        }
        
        filtered
    }
    
    /// Resample using cubic interpolation for better quality
    fn resample_cubic(&self, audio: &[f32], ratio: f64) -> Vec<f32> {
        let new_len = (audio.len() as f64 / ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let src_pos = i as f64 * ratio;
            let src_idx = src_pos as usize;
            let fraction = (src_pos - src_idx as f64) as f32;

            let p0 = if src_idx > 0 { 
                audio[src_idx - 1] 
            } else { 
                audio[src_idx] 
            };
            
            let p1 = audio[src_idx];
            
            let p2 = if src_idx + 1 < audio.len() { 
                audio[src_idx + 1] 
            } else { 
                audio[src_idx] 
            };
            
            let p3 = if src_idx + 2 < audio.len() { 
                audio[src_idx + 2] 
            } else if src_idx + 1 < audio.len() { 
                audio[src_idx + 1] 
            } else { 
                audio[src_idx] 
            };

            let interpolated = cubic_interpolate(p0, p1, p2, p3, fraction);
            resampled.push(interpolated);
        }

        resampled
    }
}

/// Cubic Hermite interpolation for smooth resampling
/// 
fn cubic_interpolate(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let a = -0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3;
    let b = p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3;
    let c = -0.5 * p0 + 0.5 * p2;
    let d = p1;
    
    a * t * t * t + b * t * t + c * t + d
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resample_same_rate() {
        let mut resampler = DynamicResampler::new(48000);
        let audio = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let result = resampler.resample(&audio, 48000, 48000);
        assert_eq!(result, audio);
    }
    
    #[test]
    fn test_resample_upsample() {
        let mut resampler = DynamicResampler::new(48000);
        let audio = vec![0.0, 1.0, 0.0];
        let result = resampler.resample(&audio, 16000, 48000);
        
        assert!(result.len() > audio.len());
        
        let max_val = result.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        assert!(max_val > 0.9);
    }
    
    #[test]
    fn test_resample_downsample() {
        let mut resampler = DynamicResampler::new(16000);
        let audio = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let result = resampler.resample(&audio, 48000, 16000);
        
        assert!(result.len() < audio.len());
    }
    
    #[test]
    fn test_cubic_interpolation() {
        let result = cubic_interpolate(0.0, 0.0, 1.0, 1.0, 0.5);
        
        assert!(result > 0.4 && result < 0.6);
    }
}
