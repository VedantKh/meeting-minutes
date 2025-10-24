//

use log::{info, warn};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AudioQualityMetrics {
    pub rms_level: f32,           // Root Mean Square level
    pub peak_level: f32,          // Peak amplitude
    pub signal_to_noise: f32,     // Estimated SNR in dB
    pub clipping_detected: bool,  // Whether clipping was detected
    pub silence_ratio: f32,       // Ratio of silence to speech
    pub sample_rate: u32,         // Actual sample rate
    pub recommended_action: Option<String>, // Suggested improvement
}

impl AudioQualityMetrics {
    pub fn analyze(samples: &[f32], sample_rate: u32) -> Self {
        if samples.is_empty() {
            return Self::default_with_rate(sample_rate);
        }

        let rms = (samples.iter().map(|&x| x * x).sum::<f32>() / samples.len() as f32).sqrt();
        
        let peak = samples.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        
        let clipping_threshold = 0.99;
        let clipping_detected = samples.iter().any(|&x| x.abs() >= clipping_threshold);
        
        let silence_threshold = 0.01; // -40dB
        let silence_count = samples.iter().filter(|&&x| x.abs() < silence_threshold).count();
        let silence_ratio = silence_count as f32 / samples.len() as f32;
        
        let mut loud_samples = Vec::new();
        let mut quiet_samples = Vec::new();
        
        for &sample in samples {
            if sample.abs() > rms {
                loud_samples.push(sample);
            } else if sample.abs() < rms * 0.1 {
                quiet_samples.push(sample);
            }
        }
        
        let signal_rms = if !loud_samples.is_empty() {
            (loud_samples.iter().map(|&x| x * x).sum::<f32>() / loud_samples.len() as f32).sqrt()
        } else {
            rms
        };
        
        let noise_rms = if !quiet_samples.is_empty() {
            (quiet_samples.iter().map(|&x| x * x).sum::<f32>() / quiet_samples.len() as f32).sqrt()
        } else {
            0.001 // Avoid division by zero
        };
        
        let snr = 20.0 * (signal_rms / noise_rms.max(0.001)).log10();
        
        let recommended_action = Self::generate_recommendation(rms, peak, snr, clipping_detected, silence_ratio);
        
        Self {
            rms_level: rms,
            peak_level: peak,
            signal_to_noise: snr,
            clipping_detected,
            silence_ratio,
            sample_rate,
            recommended_action,
        }
    }
    
    fn generate_recommendation(rms: f32, peak: f32, snr: f32, clipping: bool, silence_ratio: f32) -> Option<String> {
        if clipping {
            return Some("Audio clipping detected! Reduce microphone gain or move away from mic.".to_string());
        }
        
        if rms < 0.01 {
            return Some("Audio level too low. Increase microphone volume or speak closer to mic.".to_string());
        }
        
        if snr < 10.0 {
            return Some("High background noise detected. Use a quieter environment or noise-canceling microphone.".to_string());
        }
        
        if silence_ratio > 0.8 {
            return Some("Mostly silence detected. Ensure microphone is not muted and positioned correctly.".to_string());
        }
        
        if peak > 0.95 && !clipping {
            return Some("Audio level near clipping. Consider reducing microphone gain slightly.".to_string());
        }
        
        None // Audio quality is good
    }
    
    fn default_with_rate(sample_rate: u32) -> Self {
        Self {
            rms_level: 0.0,
            peak_level: 0.0,
            signal_to_noise: 0.0,
            clipping_detected: false,
            silence_ratio: 1.0,
            sample_rate,
            recommended_action: Some("No audio data to analyze".to_string()),
        }
    }
    
    pub fn log_metrics(&self, context: &str) {
        info!("🎤 Audio Quality Metrics [{}]:", context);
        info!("   RMS Level: {:.4} ({:.1} dB)", self.rms_level, 20.0 * self.rms_level.log10());
        info!("   Peak Level: {:.4} ({:.1} dB)", self.peak_level, 20.0 * self.peak_level.log10());
        info!("   SNR: {:.1} dB", self.signal_to_noise);
        info!("   Clipping: {}", if self.clipping_detected { "YES ⚠️" } else { "No" });
        info!("   Silence Ratio: {:.1}%", self.silence_ratio * 100.0);
        info!("   Sample Rate: {} Hz", self.sample_rate);
        
        if let Some(ref action) = self.recommended_action {
            warn!("   ⚠️ Recommendation: {}", action);
        } else {
            info!("   ✅ Audio quality is good");
        }
    }
}

pub struct AudioQualityMonitor {
    window_size: usize,
    recent_metrics: VecDeque<AudioQualityMetrics>,
    sample_rate: u32,
}

impl AudioQualityMonitor {
    pub fn new(sample_rate: u32, window_size: usize) -> Self {
        Self {
            window_size,
            recent_metrics: VecDeque::with_capacity(window_size),
            sample_rate,
        }
    }
    
    pub fn add_measurement(&mut self, samples: &[f32]) {
        let metrics = AudioQualityMetrics::analyze(samples, self.sample_rate);
        
        self.recent_metrics.push_back(metrics);
        if self.recent_metrics.len() > self.window_size {
            self.recent_metrics.pop_front();
        }
    }
    
    pub fn get_average_quality(&self) -> Option<AudioQualityMetrics> {
        if self.recent_metrics.is_empty() {
            return None;
        }
        
        let count = self.recent_metrics.len() as f32;
        let avg_rms = self.recent_metrics.iter().map(|m| m.rms_level).sum::<f32>() / count;
        let avg_peak = self.recent_metrics.iter().map(|m| m.peak_level).sum::<f32>() / count;
        let avg_snr = self.recent_metrics.iter().map(|m| m.signal_to_noise).sum::<f32>() / count;
        let avg_silence = self.recent_metrics.iter().map(|m| m.silence_ratio).sum::<f32>() / count;
        let any_clipping = self.recent_metrics.iter().any(|m| m.clipping_detected);
        
        Some(AudioQualityMetrics {
            rms_level: avg_rms,
            peak_level: avg_peak,
            signal_to_noise: avg_snr,
            clipping_detected: any_clipping,
            silence_ratio: avg_silence,
            sample_rate: self.sample_rate,
            recommended_action: AudioQualityMetrics::generate_recommendation(
                avg_rms, avg_peak, avg_snr, any_clipping, avg_silence
            ),
        })
    }
    
    pub fn is_quality_acceptable(&self) -> bool {
        if let Some(avg) = self.get_average_quality() {
            avg.rms_level > 0.01 
                && !avg.clipping_detected 
                && avg.signal_to_noise > 5.0 
                && avg.silence_ratio < 0.9
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_analysis_good_audio() {
        let samples: Vec<f32> = (0..1000)
            .map(|i| 0.5 * (2.0 * std::f32::consts::PI * i as f32 / 100.0).sin())
            .collect();
        
        let metrics = AudioQualityMetrics::analyze(&samples, 48000);
        assert!(metrics.rms_level > 0.3);
        assert!(metrics.rms_level < 0.4);
        assert!(!metrics.clipping_detected);
        assert!(metrics.recommended_action.is_none());
    }
    
    #[test]
    fn test_quality_analysis_clipping() {
        let samples: Vec<f32> = (0..1000)
            .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 100.0).sin())
            .collect();
        
        let metrics = AudioQualityMetrics::analyze(&samples, 48000);
        assert!(metrics.clipping_detected);
        assert!(metrics.recommended_action.is_some());
    }
    
    #[test]
    fn test_quality_analysis_silence() {
        let samples = vec![0.0; 1000];
        
        let metrics = AudioQualityMetrics::analyze(&samples, 48000);
        assert_eq!(metrics.rms_level, 0.0);
        assert_eq!(metrics.silence_ratio, 1.0);
        assert!(metrics.recommended_action.is_some());
    }
}
