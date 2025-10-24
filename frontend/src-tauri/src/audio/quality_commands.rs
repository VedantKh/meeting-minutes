//

use tauri::State;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use super::quality_diagnostics::{AudioQualityMetrics, AudioQualityMonitor};

pub struct QualityMonitorState {
    pub monitor: Arc<Mutex<Option<AudioQualityMonitor>>>,
}

impl QualityMonitorState {
    pub fn new() -> Self {
        Self {
            monitor: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioQualityReport {
    pub rms_level: f32,
    pub rms_db: f32,
    pub peak_level: f32,
    pub peak_db: f32,
    pub signal_to_noise_db: f32,
    pub clipping_detected: bool,
    pub silence_ratio_percent: f32,
    pub sample_rate: u32,
    pub quality_rating: String,  // "Excellent", "Good", "Fair", "Poor"
    pub recommended_action: Option<String>,
}

impl From<AudioQualityMetrics> for AudioQualityReport {
    fn from(metrics: AudioQualityMetrics) -> Self {
        let rms_db = if metrics.rms_level > 0.0 {
            20.0 * metrics.rms_level.log10()
        } else {
            -100.0
        };
        
        let peak_db = if metrics.peak_level > 0.0 {
            20.0 * metrics.peak_level.log10()
        } else {
            -100.0
        };
        
        let quality_rating = if metrics.clipping_detected {
            "Poor".to_string()
        } else if metrics.rms_level < 0.01 {
            "Poor".to_string()
        } else if metrics.signal_to_noise < 10.0 {
            "Fair".to_string()
        } else if metrics.signal_to_noise < 20.0 {
            "Good".to_string()
        } else {
            "Excellent".to_string()
        };
        
        Self {
            rms_level: metrics.rms_level,
            rms_db,
            peak_level: metrics.peak_level,
            peak_db,
            signal_to_noise_db: metrics.signal_to_noise,
            clipping_detected: metrics.clipping_detected,
            silence_ratio_percent: metrics.silence_ratio * 100.0,
            sample_rate: metrics.sample_rate,
            quality_rating,
            recommended_action: metrics.recommended_action,
        }
    }
}

#[tauri::command]
pub async fn init_quality_monitor(
    sample_rate: u32,
    state: State<'_, QualityMonitorState>,
) -> Result<(), String> {
    let monitor = AudioQualityMonitor::new(sample_rate, 10); // 10-sample rolling window
    
    let mut state_lock = state.monitor.lock()
        .map_err(|e| format!("Failed to lock monitor state: {}", e))?;
    
    *state_lock = Some(monitor);
    
    Ok(())
}

#[tauri::command]
pub async fn get_audio_quality(
    state: State<'_, QualityMonitorState>,
) -> Result<Option<AudioQualityReport>, String> {
    let state_lock = state.monitor.lock()
        .map_err(|e| format!("Failed to lock monitor state: {}", e))?;
    
    if let Some(ref monitor) = *state_lock {
        if let Some(metrics) = monitor.get_average_quality() {
            return Ok(Some(AudioQualityReport::from(metrics)));
        }
    }
    
    Ok(None)
}

#[tauri::command]
pub async fn is_audio_quality_acceptable(
    state: State<'_, QualityMonitorState>,
) -> Result<bool, String> {
    let state_lock = state.monitor.lock()
        .map_err(|e| format!("Failed to lock monitor state: {}", e))?;
    
    if let Some(ref monitor) = *state_lock {
        return Ok(monitor.is_quality_acceptable());
    }
    
    Ok(false)
}

#[tauri::command]
pub async fn analyze_audio_sample(
    samples: Vec<f32>,
    sample_rate: u32,
) -> Result<AudioQualityReport, String> {
    let metrics = AudioQualityMetrics::analyze(&samples, sample_rate);
    Ok(AudioQualityReport::from(metrics))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_report_conversion() {
        let metrics = AudioQualityMetrics {
            rms_level: 0.15,
            peak_level: 0.45,
            signal_to_noise: 25.0,
            clipping_detected: false,
            silence_ratio: 0.1,
            sample_rate: 48000,
            recommended_action: None,
        };
        
        let report = AudioQualityReport::from(metrics);
        assert_eq!(report.quality_rating, "Excellent");
        assert!(!report.clipping_detected);
        assert_eq!(report.sample_rate, 48000);
    }
    
    #[test]
    fn test_quality_rating_poor() {
        let metrics = AudioQualityMetrics {
            rms_level: 0.005,
            peak_level: 0.01,
            signal_to_noise: 5.0,
            clipping_detected: false,
            silence_ratio: 0.9,
            sample_rate: 48000,
            recommended_action: Some("Audio level too low".to_string()),
        };
        
        let report = AudioQualityReport::from(metrics);
        assert_eq!(report.quality_rating, "Poor");
        assert!(report.recommended_action.is_some());
    }
}
