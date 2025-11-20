use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    src: R,
    total_bytes: usize,
    call_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(inner: R) -> Self {
        ReadStats {
            src: inner,
            total_bytes: 0,
            call_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.src
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn reads(&self) -> usize {
        self.call_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_now = self.src.read(buf)?;
        self.call_count += 1;
        self.total_bytes += read_now;
        Ok(read_now)
    }
}



pub struct WriteStats<W> {
    dst: W,
    written: usize,
    ops: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(inner: W) -> Self {
        WriteStats {
            dst: inner,
            written: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.dst
    }

    pub fn bytes_through(&self) -> usize {
        self.written
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.dst.write(buf)?;
        self.ops += 1;
        self.written += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.dst.flush()
    }
}