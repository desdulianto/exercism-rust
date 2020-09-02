use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes: usize,
    read_count: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped,
            bytes: 0,
            read_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.data.read(buf) {
            Ok(n) => {
                self.bytes += n;
                self.read_count += 1;
                Ok(n)
            },
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    data: W,
    bytes: usize,
    write_count: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            bytes: 0,
            write_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.data.write(buf) {
            Ok(n) => {
                self.bytes += n;
                self.write_count += 1;
                Ok(n)
            },
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
