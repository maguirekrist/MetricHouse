use std::{collections::HashMap, io::Write};

use crate::{models::metric::Metric, storage::file, traits::serializable::BinarySerializable};


pub struct InMemoryStore {
    flush_max: u32,
    count_table: HashMap<String, u32>,
    series: HashMap<String, Vec<Metric>>
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            flush_max: 1000,
            count_table: HashMap::new(),
            series: HashMap::new()
        }
    }

    pub fn insert(&mut self, metric: Metric) {
        let key = metric.name.to_string();
        self.series.entry(key.to_string()).or_default().push(metric);
        
        let should_flush = {
            let count = self.count_table.entry(key.to_string()).or_default();
            *count += 1;
            *count >= self.flush_max
        };

        if should_flush
        {
            self.flush_metric(key.as_str());
            *self.count_table.entry(key).or_default() = 0;
        }
    }

    pub fn query(&self, name: &str) -> Option<&Vec<Metric>> {
        self.series.get(name)
    }

    pub fn flush_metric(&mut self, name: &str)
    {   
        let file_name = format!("{}.metricdata", name);
        let mut file = file::open_or_create(&file_name);

        let mut write_data: Vec<u8> = Vec::new();
        let metrics = self.series.get(name).unwrap();

        for metric in metrics
        {
            let bin_metric = metric.serialize();
            write_data.extend(bin_metric)
        }

        file.write_all(&write_data).expect("Failed to write to file.");
    }
}