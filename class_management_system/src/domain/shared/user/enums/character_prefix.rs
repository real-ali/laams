#[derive(Debug, Clone, Default)]
//Honorific
pub enum CharacterPrefix {
    #[default]
    None,
    Mr,   // Used before the name of an adult male
    Miss, // Traditionally used for unmarried women
    Ms,   // Neutral title for women, regardless of marital status
    Mrs,  // Used before the name of a married woman
    Dr,   // Used for individuals with a doctoral degree, can be any gender
    Prof, // Used for individuals with a professorial rank, can be any gender
    Capt, // Used for a captain, typically in the military or maritime context
    Rev,  // Used for a reverend, typically a minister or clergy
    Col,  // Used for a colonel, typically in the military
    Sgt,  // Used for a sergeant, typically in the military or police
}

impl CharacterPrefix {
    /// Returns `true` if the honorific is [`Mr`].
    ///
    /// [`Mr`]: Honorific::Mr
    #[must_use]
    pub fn is_mr(&self) -> bool {
        matches!(self, Self::Mr)
    }

    /// Returns `true` if the honorific is [`Miss`].
    ///
    /// [`Miss`]: Honorific::Miss
    #[must_use]
    pub fn is_miss(&self) -> bool {
        matches!(self, Self::Miss)
    }

    /// Returns `true` if the honorific is [`Ms`].
    ///
    /// [`Ms`]: Honorific::Ms
    #[must_use]
    pub fn is_ms(&self) -> bool {
        matches!(self, Self::Ms)
    }

    /// Returns `true` if the honorific is [`Mrs`].
    ///
    /// [`Mrs`]: Honorific::Mrs
    #[must_use]
    pub fn is_mrs(&self) -> bool {
        matches!(self, Self::Mrs)
    }

    /// Returns `true` if the honorific is [`Dr`].
    ///
    /// [`Dr`]: Honorific::Dr
    #[must_use]
    pub fn is_dr(&self) -> bool {
        matches!(self, Self::Dr)
    }

    /// Returns `true` if the honorific is [`Prof`].
    ///
    /// [`Prof`]: Honorific::Prof
    #[must_use]
    pub fn is_prof(&self) -> bool {
        matches!(self, Self::Prof)
    }

    /// Returns `true` if the honorific is [`Capt`].
    ///
    /// [`Capt`]: Honorific::Capt
    #[must_use]
    pub fn is_capt(&self) -> bool {
        matches!(self, Self::Capt)
    }

    /// Returns `true` if the honorific is [`Rev`].
    ///
    /// [`Rev`]: Honorific::Rev
    #[must_use]
    pub fn is_rev(&self) -> bool {
        matches!(self, Self::Rev)
    }

    /// Returns `true` if the honorific is [`Col`].
    ///
    /// [`Col`]: Honorific::Col
    #[must_use]
    pub fn is_col(&self) -> bool {
        matches!(self, Self::Col)
    }

    /// Returns `true` if the honorific is [`Sgt`].
    ///
    /// [`Sgt`]: Honorific::Sgt
    #[must_use]
    pub fn is_sgt(&self) -> bool {
        matches!(self, Self::Sgt)
    }
}
