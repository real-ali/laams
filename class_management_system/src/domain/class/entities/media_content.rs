use crate::domain::class::enums::MediaType;

pub struct MediaContent{
    id:String,
    title: String,
    sub_title: String,
    description: String,
    media_type: MediaType,
    url: String,
}