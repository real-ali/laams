use crate::domain::common::enums::FileFormat;

#[derive(Debug, Clone, Default)]
pub struct Attachment {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub format: FileFormat,
}

impl Attachment {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }
}
impl Attachment {
    pub fn set_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn set_format(mut self, format: FileFormat) -> Self {
        self.format = format;
        self
    }
}
