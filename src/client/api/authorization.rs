use tonic::metadata::MetadataValue;
use tonic::{Request, Status};

pub(super) fn insert_token(mut request: Request<()>, token: &str) -> Result<Request<()>, Status> {
    // Retrieve token.
    let metadata = MetadataValue::from_str(token).unwrap();

    // Insert token to request metadata.
    request.metadata_mut().insert("token", metadata);
    Ok(request)
}
