use crate::monitor::Metrics;

/// Pet state based on system health
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn update(&mut self, metrics: &Metrics) -> PetState {
        let ram_status = Self::classify_metric(metrics.ram_percent, 60.0, 80.0, 90.0);
        let cpu_status = Self::classify_metric(metrics.cpu_percent, 60.0, 80.0, 90.0);
        let disk_status = Self::classify_disk(metrics.disk_junk_mb);

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
        match self.state {
            PetState::Happy => "😊 Pet is happy - system healthy!",
            PetState::Okay => "😐 Pet is okay - minor resource usage",
            PetState::Stressed => "😰 Pet is stressed - high resource usage",
            PetState::Critical => "🔥 Pet is critical - system overloaded!",
        }
    }

    /// Classify a metric based on thresholds
    /// Thresholds match README.md:
    /// - Happy: <60%
    /// - Okay: 60-80%
    /// - Stressed: 80-90%
    /// - Critical: >90%
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

    /// Classify disk junk based on MB thresholds
    /// Per README.md:
    /// - Happy: <500MB
    /// - Okay: 500-2000MB
    /// - Stressed: 2000-5000MB
    /// - Critical: >5000MB
    fn classify_disk(mb: u64) -> MetricStatus {
        if mb > 5000 {
            MetricStatus::Critical
        } else if mb > 2000 {
            MetricStatus::Warning
        } else if mb > 500 {
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
