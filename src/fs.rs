use std::io::Read;
use std::path::Path;

use eyre::{eyre, Context};

pub fn read_file(path: impl AsRef<Path>) -> eyre::Result<Vec<u8>> {
    let mut file = std::fs::File::open(&path)
        .wrap_err_with(|| eyre!("Couldn't open {}", path.as_ref().to_string_lossy()))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .wrap_err_with(|| eyre!("Couldn't read {}", path.as_ref().to_string_lossy()))?;
    tracing::info!(
        bytes = bytes.len(),
        path = path.as_ref().to_str(),
        "Read file",
    );
    Ok(bytes)
}

pub fn write_file(path: impl AsRef<Path>, data: Vec<u8>) -> eyre::Result<()> {
    let data_len = &data.len();
    std::fs::write(&path, data)
        .wrap_err_with(|| eyre!("Couldn't write to {}", path.as_ref().to_string_lossy()))?;
    tracing::info!(
        bytes = data_len,
        path = path.as_ref().to_str(),
        "Wrote file",
    );
    Ok(())
}
