use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub fn generate_etag<P: AsRef<Path>>(path: P) -> Option<String> {
    let metadata = fs::metadata(path.as_ref()).ok()?;
    let modified = metadata.modified().ok()?;
    let content = fs::read(path.as_ref()).ok()?;

    let mut hasher = Sha256::new();
    hasher.update(&content);
    hasher.update(
        modified
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs()
            .to_string(),
    );

    let result = hasher.finalize();
    Some(format!(
        "\"{}\"",
        general_purpose::URL_SAFE_NO_PAD.encode(&result[..16])
    ))
}
