#[derive(Debug,Clone,Default)]
pub struct ExamQuestion{
    id : String,
    question:String,
    answers:Vec<String>,
    correct_answer:String,
    score:i8,
}