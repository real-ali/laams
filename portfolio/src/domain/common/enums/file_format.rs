#[derive(Debug,Clone)]
pub enum FileFormat {
    PDF,
    JPG,
    PNG,
    DOC,
    JPEG,
    UNKNOWN
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl FileFormat {
    /// Returns `true` if the file format is [`PDF`].
    ///
    /// [`PDF`]: FileFormat::PDF
    #[must_use]
    pub fn is_pdf(&self) -> bool {
        matches!(self, Self::PDF)
    }

    /// Returns `true` if the file format is [`JPG`].
    ///
    /// [`JPG`]: FileFormat::JPG
    #[must_use]
    pub fn is_jpg(&self) -> bool {
        matches!(self, Self::JPG)
    }

    /// Returns `true` if the file format is [`PNG`].
    ///
    /// [`PNG`]: FileFormat::PNG
    #[must_use]
    pub fn is_png(&self) -> bool {
        matches!(self, Self::PNG)
    }

    /// Returns `true` if the file format is [`DOC`].
    ///
    /// [`DOC`]: FileFormat::DOC
    #[must_use]
    pub fn is_doc(&self) -> bool {
        matches!(self, Self::DOC)
    }

    /// Returns `true` if the file format is [`JPEG`].
    ///
    /// [`JPEG`]: FileFormat::JPEG
    #[must_use]
    pub fn is_jpeg(&self) -> bool {
        matches!(self, Self::JPEG)
    }

    /// Returns `true` if the file format is [`UNKNOWN`].
    ///
    /// [`UNKNOWN`]: FileFormat::UNKNOWN
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::UNKNOWN)
    }
}
