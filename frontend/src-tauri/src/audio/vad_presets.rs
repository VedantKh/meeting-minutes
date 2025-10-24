//
//

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VadPreset {
    OneOnOne,
    
    GroupMeeting,
    
    Presentation,
    
    NoisyEnvironment,
    
    Custom {
        positive_threshold: f32,
        negative_threshold: f32,
        redemption_ms: u32,
        min_speech_ms: u32,
    },
}

impl VadPreset {
    pub fn positive_threshold(&self) -> f32 {
        match self {
            VadPreset::OneOnOne => 0.50,           // Silero default - balanced
            VadPreset::GroupMeeting => 0.45,       // More lenient for overlapping speech
            VadPreset::Presentation => 0.40,       // Very lenient for continuous speech
            VadPreset::NoisyEnvironment => 0.60,   // Higher threshold to filter noise
            VadPreset::Custom { positive_threshold, .. } => *positive_threshold,
        }
    }
    
    pub fn negative_threshold(&self) -> f32 {
        match self {
            VadPreset::OneOnOne => 0.35,           // Silero default - balanced
            VadPreset::GroupMeeting => 0.30,       // More lenient for natural pauses
            VadPreset::Presentation => 0.25,       // Very lenient for thinking pauses
            VadPreset::NoisyEnvironment => 0.40,   // Higher threshold to filter noise
            VadPreset::Custom { negative_threshold, .. } => *negative_threshold,
        }
    }
    
    pub fn redemption_time(&self) -> Duration {
        match self {
            VadPreset::OneOnOne => Duration::from_millis(2000),        // 2 seconds
            VadPreset::GroupMeeting => Duration::from_millis(3000),    // 3 seconds
            VadPreset::Presentation => Duration::from_millis(4000),    // 4 seconds
            VadPreset::NoisyEnvironment => Duration::from_millis(2500), // 2.5 seconds
            VadPreset::Custom { redemption_ms, .. } => Duration::from_millis(*redemption_ms as u64),
        }
    }
    
    pub fn min_speech_time(&self) -> Duration {
        match self {
            VadPreset::OneOnOne => Duration::from_millis(250),         // 250ms minimum
            VadPreset::GroupMeeting => Duration::from_millis(300),     // 300ms minimum
            VadPreset::Presentation => Duration::from_millis(400),     // 400ms minimum
            VadPreset::NoisyEnvironment => Duration::from_millis(350), // 350ms minimum
            VadPreset::Custom { min_speech_ms, .. } => Duration::from_millis(*min_speech_ms as u64),
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            VadPreset::OneOnOne => "Optimized for one-on-one conversations with natural pauses",
            VadPreset::GroupMeeting => "Optimized for group meetings with multiple speakers and turn-taking",
            VadPreset::Presentation => "Optimized for presentations and lectures with long continuous speech",
            VadPreset::NoisyEnvironment => "Optimized for noisy environments (cafes, open offices)",
            VadPreset::Custom { .. } => "Custom user-defined settings",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            VadPreset::OneOnOne => "One-on-One",
            VadPreset::GroupMeeting => "Group Meeting",
            VadPreset::Presentation => "Presentation",
            VadPreset::NoisyEnvironment => "Noisy Environment",
            VadPreset::Custom { .. } => "Custom",
        }
    }
    
    pub fn all_presets() -> Vec<VadPreset> {
        vec![
            VadPreset::OneOnOne,
            VadPreset::GroupMeeting,
            VadPreset::Presentation,
            VadPreset::NoisyEnvironment,
        ]
    }
    
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "one-on-one" | "oneOnOne" | "one_on_one" => Some(VadPreset::OneOnOne),
            "group-meeting" | "groupMeeting" | "group_meeting" => Some(VadPreset::GroupMeeting),
            "presentation" => Some(VadPreset::Presentation),
            "noisy-environment" | "noisyEnvironment" | "noisy_environment" => Some(VadPreset::NoisyEnvironment),
            _ => None,
        }
    }
}

impl Default for VadPreset {
    fn default() -> Self {
        VadPreset::OneOnOne
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_preset_thresholds() {
        let one_on_one = VadPreset::OneOnOne;
        assert_eq!(one_on_one.positive_threshold(), 0.50);
        assert_eq!(one_on_one.negative_threshold(), 0.35);
        
        let noisy = VadPreset::NoisyEnvironment;
        assert!(noisy.positive_threshold() > one_on_one.positive_threshold());
        assert!(noisy.negative_threshold() > one_on_one.negative_threshold());
    }
    
    #[test]
    fn test_preset_redemption_times() {
        let one_on_one = VadPreset::OneOnOne;
        assert_eq!(one_on_one.redemption_time(), Duration::from_millis(2000));
        
        let presentation = VadPreset::Presentation;
        assert_eq!(presentation.redemption_time(), Duration::from_millis(4000));
        assert!(presentation.redemption_time() > one_on_one.redemption_time());
    }
    
    #[test]
    fn test_preset_parsing() {
        assert_eq!(VadPreset::from_name("one-on-one"), Some(VadPreset::OneOnOne));
        assert_eq!(VadPreset::from_name("groupMeeting"), Some(VadPreset::GroupMeeting));
        assert_eq!(VadPreset::from_name("presentation"), Some(VadPreset::Presentation));
        assert_eq!(VadPreset::from_name("invalid"), None);
    }
    
    #[test]
    fn test_custom_preset() {
        let custom = VadPreset::Custom {
            positive_threshold: 0.55,
            negative_threshold: 0.38,
            redemption_ms: 2500,
            min_speech_ms: 300,
        };
        
        assert_eq!(custom.positive_threshold(), 0.55);
        assert_eq!(custom.negative_threshold(), 0.38);
        assert_eq!(custom.redemption_time(), Duration::from_millis(2500));
        assert_eq!(custom.min_speech_time(), Duration::from_millis(300));
    }
}
