use actix_multipart::form::{MultipartForm, tempfile::TempFile};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}
