use crate::domain::class::enums::MediaType;
#[derive(Debug,Clone,Default)]
pub struct MediaContent{
    id:String,
    title: String,
    sub_title: String,
    description: String,
    media_type: MediaType,
    url: String,
}