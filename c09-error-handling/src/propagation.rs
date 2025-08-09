use std::fs::{self, File};
use std::io::{self, Read};

// Shows different ways to propagate errors up the call stack.

// Verbose version: manual match and early return.
pub fn username_from_file_verbose(path: &str) -> Result<String, io::Error> {
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    if let Err(e) = f.read_to_string(&mut s) {
        return Err(e);
    }
    Ok(s)
}

// Idiomatic version using the `?` operator.
pub fn username_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

// Even shorter by delegating to fs::read_to_string.
pub fn username_from_file_short(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_existing_file() {
        // Prepare a temp file in target dir to avoid polluting repo
        let path = "target/tmp_username.txt";
        std::fs::create_dir_all("target").unwrap();
        std::fs::write(path, "alice").unwrap();

        let name = username_from_file(path).unwrap();
        assert_eq!(name, "alice");
    }

    #[test]
    fn returns_error_for_missing_file() {
        let err = username_from_file("no_such_file.txt").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }
}
