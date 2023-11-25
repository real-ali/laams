#[derive(Debug, Clone)]
pub enum Status {
    Pending,
    Inprogress,
    Completed,
    Faild,
}

impl Default for Status {
    fn default() -> Self {
        Self::Pending
    }
}

impl Status {
    /// Returns `true` if the status is [`Pending`].
    ///
    /// [`Pending`]: Status::Pending
    #[must_use]
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the status is [`Inprogress`].
    ///
    /// [`Inprogress`]: Status::Inprogress
    #[must_use]
    pub fn is_inprogress(&self) -> bool {
        matches!(self, Self::Inprogress)
    }

    /// Returns `true` if the status is [`Completed`].
    ///
    /// [`Completed`]: Status::Completed
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns `true` if the status is [`Faild`].
    ///
    /// [`Faild`]: Status::Faild
    #[must_use]
    pub fn is_faild(&self) -> bool {
        matches!(self, Self::Faild)
    }
}
