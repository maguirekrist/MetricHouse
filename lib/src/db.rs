use std::{fs::read_dir, io::Read, path::Path};

use crate::{models::Metric, storage::{store::InMemoryStore, wal::{WalWriter, WAL_DIR}}, traits::serializable::BinarySerializable};

pub struct MetricsDb {
    memory_store: InMemoryStore,
    _wal_writer: WalWriter
}

impl MetricsDb {
    pub fn new() -> Self {
        let mut db = MetricsDb {
            memory_store: InMemoryStore::new(),
            _wal_writer: WalWriter::new()
        };
        db.recover();
        return db
    }

    fn recover(&mut self)
    {
        let wal_dir = Path::new(WAL_DIR);
        if wal_dir.is_dir()
        {
            let mut entries = read_dir(WAL_DIR).unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

            entries.sort();
            
            for entry in &entries
            {
                println!("{}", entry.as_path().to_string_lossy());
                let mut entry_file = std::fs::File::open(entry).unwrap();
                let file_meta = entry.metadata().unwrap();
                let file_len = file_meta.len() as usize;

                if file_len == 0
                {
                    println!("File: {} is empty! Deleting...", entry.to_string_lossy());
                    continue;
                }

                let mut buffer: Vec<u8> = Vec::with_capacity(file_len);
                entry_file.read_to_end(&mut buffer).unwrap();

                let mut byte_offset: usize = 0;
                while byte_offset < file_len
                {
                    let result = Metric::deserialize(&buffer, &mut byte_offset);

                    match result {
                        Ok(metric) => self.ingest(metric),
                        Err(_) => byte_offset = file_len
                    }
                }

                std::fs::remove_file(entry).unwrap();
            }
        } else {
            println!("Path {} not found!", wal_dir.to_string_lossy());
        }
    }

    pub fn ingest(&mut self, metric: Metric) {
        self.memory_store.insert(metric);
    }

    pub fn query(&self, name: &str) -> &Vec<Metric> {
        // Implementation for retrieving metrics from the database
        self.memory_store.query(name).expect(format!("Metric not found: {}", name).as_str())
    }
}

impl Drop for MetricsDb
{
    fn drop(&mut self) {
        println!("Shutdown!");
        todo!();
    }
}