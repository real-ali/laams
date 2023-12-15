use super::MediaContent;
#[derive(Debug,Clone,Default)]
pub struct LessonContent {
    id: String,
    title: String,
    sub_title: String,
    description: String,
    media: MediaContent,
}
