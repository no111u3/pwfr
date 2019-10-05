//! Ebuild and Eclass utility functions
//!
//! Service types and utility functions
//! Inspired by https://crates.io/crates/nsh (github: https://github.com/seiyanuta/nsh)

use std::io::Write;
use std::os::unix::io::RawFd;

use nix::unistd;

/// `File`-like object but doesn't close the `fd`.
#[derive(Debug, PartialEq)]
pub struct FdFile {
    fd: RawFd,
}

impl FdFile {
    pub fn new(fd: RawFd) -> FdFile {
        FdFile { fd }
    }

    pub fn read_line(&self) -> Option<String> {
        let mut line = Vec::new();
        loop {
            let mut ch = vec![0; 1];
            match unistd::read(self.fd, &mut ch) {
                // EOF
                Ok(read_len) if read_len == 0 => break,
                // Read a character.
                Ok(_) => {
                    if ch[0] == 0x0a
                    /* newline */
                    {
                        break;
                    }

                    line.push(ch[0]);
                }
                // Something went wrong.
                Err(err) => {
                    trace!("read_line: error: {:?}", err);
                    break;
                }
            }
        }

        if line.is_empty() {
            None
        } else {
            Some(String::from_utf8(line).expect("binary data is not yet supported"))
        }
    }
}

impl Write for FdFile {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = unistd::write(self.fd, buf).expect("failed to write");
        Ok(len)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        unistd::fsync(self.fd).ok();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let fd_file = FdFile::new(0);
        assert_eq!(fd_file, FdFile { fd: 0 });
    }
}
