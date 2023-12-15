use super::LessonContent;

pub struct Lesson{
    id:String,
    title:String,
    description:String,
    contents: Vec<LessonContent>
}