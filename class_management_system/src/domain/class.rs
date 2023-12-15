


use self::entities::Subject;

use super::shared::user::entities::Student;


pub mod entities;
pub mod enums;
pub mod values;
pub mod shared;

pub struct Class {
    id : String,
    title: String,
    description: String,
    level: i8,
    subjects: Vec<Subject>,
    students:Vec<Student>,
    start_date: String,
    expiry_date: String,
    
}
