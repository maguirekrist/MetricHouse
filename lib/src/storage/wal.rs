use std::{fs::File, io::{BufWriter, Write}, path::Path};

use crate::storage::file;

pub const WAL_DIR: &str = "wals/";

pub struct WalWriter {
    writer: BufWriter<File>,
    counter: u64,
    flush_interval: u64
}

impl WalWriter {
    pub fn open(file: File) -> Self {
        Self {
            writer: BufWriter::new(file),
            counter: 0,
            flush_interval: 100
        }
    }

    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(file::create_file_timed(&Path::new(format!("{}{}", WAL_DIR, "wal.bin").as_str()),file::KIB * 4)),
            counter: 0,
            flush_interval: 10
        }
    }

    pub fn write(&mut self, bin: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(&bin)?;
        self.counter += 1;

        //TODO: If the incoming flush will expand the file beyond it's initial capacity, create a new WAL. 

        if (self.counter % self.flush_interval) == 0 {
            println!("Flushing writer after {} writes", self.counter);
            self.writer.flush()?; // Flush every 1000 writes
        }
        Ok(())
    }
}

impl Drop for WalWriter {
    fn drop(&mut self) {
        self.writer.flush().expect("Failed to flush writer on drop");
    }
}


//test
#[cfg(test)]
mod tests {
    use super::*; 

    fn dir_exists(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    #[test]
    fn create_wal_test()
    {
        let _wal_writer = WalWriter::new();
        assert!(dir_exists(WAL_DIR))
    }
}