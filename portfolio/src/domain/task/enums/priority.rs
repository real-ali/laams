#[derive(Debug,Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Medium
    }
}

impl Priority {
    /// Returns `true` if the priority is [`Low`].
    ///
    /// [`Low`]: Priority::Low
    #[must_use]
    pub fn is_low(&self) -> bool {
        matches!(self, Self::Low)
    }

    /// Returns `true` if the priority is [`Medium`].
    ///
    /// [`Medium`]: Priority::Medium
    #[must_use]
    pub fn is_medium(&self) -> bool {
        matches!(self, Self::Medium)
    }

    /// Returns `true` if the priority is [`High`].
    ///
    /// [`High`]: Priority::High
    #[must_use]
    pub fn is_high(&self) -> bool {
        matches!(self, Self::High)
    }
}
