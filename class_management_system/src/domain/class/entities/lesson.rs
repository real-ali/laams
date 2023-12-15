use super::LessonContent;
#[derive(Debug,Clone,Default)]
pub struct Lesson{
    id:String,
    title:String,
    description:String,
    contents: Vec<LessonContent>
}