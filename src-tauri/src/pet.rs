use crate::monitor::Metrics;
use serde::{Deserialize, Serialize};

/// Pet state based on system health
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PetState {
    /// All metrics are good
    Happy,
    /// One metric is in warning range
    Okay,
    /// Multiple metrics in warning or one in critical
    Stressed,
    /// Any metric is critical
    Critical,
}

impl PetState {
    /// Get emoji representation of pet state
    pub fn emoji(&self) -> &str {
        match self {
            PetState::Happy => "😊",
            PetState::Okay => "😐",
            PetState::Stressed => "😰",
            PetState::Critical => "🔥",
        }
    }

    /// Get description of pet state
    pub fn description(&self) -> &str {
        match self {
            PetState::Happy => "Pet is happy - system healthy!",
            PetState::Okay => "Pet is okay - minor resource usage",
            PetState::Stressed => "Pet is stressed - high resource usage",
            PetState::Critical => "Pet is critical - system overloaded!",
        }
    }
}

/// Pet that reflects system health
pub struct Pet {
    state: PetState,
}

impl Pet {
    /// Create a new pet with default Happy state
    pub fn new() -> Self {
        Self {
            state: PetState::Happy,
        }
    }

    /// Update pet state based on current metrics
    /// Returns the new state
    ///
    /// Updated thresholds per Domain Expert recommendations:
    /// - RAM: 70/85/95% (was 60/80/90%)
    /// - CPU: 70/85/95% (was 60/80/90%)
    /// - Disk: 5/10/20% of total disk (percentage-based instead of absolute MB)
    pub fn update(&mut self, metrics: &Metrics) -> PetState {
        // Updated RAM thresholds: 70/85/95%
        let ram_status = Self::classify_metric(metrics.ram_percent, 70.0, 85.0, 95.0);

        // Updated CPU thresholds: 70/85/95% (with temporal smoothing already applied in monitor)
        let cpu_status = Self::classify_metric(metrics.cpu_percent, 70.0, 85.0, 95.0);

        // Updated disk thresholds: percentage-based (5/10/20% of total disk)
        let disk_status = Self::classify_disk_percent(metrics.disk_junk_percent);

        // Determine overall state based on worst metric
        let statuses = [ram_status, cpu_status, disk_status];

        // If any metric is critical, pet is critical
        if statuses.iter().any(|&s| s == MetricStatus::Critical) {
            self.state = PetState::Critical;
        }
        // If 2+ metrics are warning, or 1 warning + 1 critical, pet is stressed
        else if statuses.iter().filter(|&&s| s != MetricStatus::Good).count() >= 2 {
            self.state = PetState::Stressed;
        }
        // If one metric is warning, pet is okay
        else if statuses.iter().any(|&s| s == MetricStatus::Warning) {
            self.state = PetState::Okay;
        }
        // All metrics good
        else {
            self.state = PetState::Happy;
        }

        self.state
    }

    /// Get current pet state
    pub fn get_state(&self) -> PetState {
        self.state
    }

    /// Get a description of the pet's current state
    pub fn get_icon_description(&self) -> &str {
        self.state.description()
    }

    /// Get emoji for current state
    pub fn get_emoji(&self) -> &str {
        self.state.emoji()
    }

    /// Classify a metric based on thresholds
    /// Updated thresholds:
    /// - Good: <70%
    /// - Warning: 70-85%
    /// - Stressed: 85-95%
    /// - Critical: ≥95%
    fn classify_metric(value: f32, warning: f32, stressed: f32, critical: f32) -> MetricStatus {
        if value >= critical {
            MetricStatus::Critical
        } else if value >= stressed {
            MetricStatus::Warning
        } else if value >= warning {
            MetricStatus::Warning
        } else {
            MetricStatus::Good
        }
    }

    /// Classify disk junk based on percentage thresholds
    /// Updated per Domain Expert:
    /// - Good: <5% of total disk
    /// - Warning: 5-10% of total disk
    /// - Stressed: 10-20% of total disk
    /// - Critical: ≥20% of total disk
    fn classify_disk_percent(percent: f32) -> MetricStatus {
        if percent >= 20.0 {
            MetricStatus::Critical
        } else if percent >= 10.0 {
            MetricStatus::Warning
        } else if percent >= 5.0 {
            MetricStatus::Warning
        } else {
            MetricStatus::Good
        }
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal metric classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetricStatus {
    Good,
    Warning,
    Critical,
}
