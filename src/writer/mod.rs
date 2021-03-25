pub use error::Error;
use std::fs;
use std::path::Path;
use std::io::Write;

mod error;

pub(super) fn write(path: &Path, binary: &[u8]) -> Result<(), Error> {
    let mut file = fs::File::create(path)
        .map_err(|io_error| Error::CouldNotWriteFile { file_path: path.to_path_buf(), io_error })?;
    file.write_all(binary)
        .map_err(|io_error| Error::CouldNotWriteFile { file_path: path.to_path_buf(), io_error })?;
    Ok(())
}
