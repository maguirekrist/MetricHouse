use crate::models::metric::Metric;


pub struct Chunk {
    pub id: String,
    pub start_time: u64,
    pub end_time: u64,
    pub metrics: Vec<Metric>,
}

