use std::fs::{self, create_dir_all, DirBuilder, File, OpenOptions, ReadDir};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;


pub const KIB: u64 = 1024;

pub fn create_file_timed(file_name: &Path, capacity: u64) -> File {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();


    if let Some(parent) = file_name.parent() {
        create_dir_all(parent).expect("Failed to create parent directories.");
    }    

    let stem = file_name.file_stem().unwrap_or_default().to_string_lossy();
    let ext = file_name.extension().map_or(String::new(), |e| format!(".{}", e.to_string_lossy()));

    let new_file_name = format!("{}_{}{}", stem, timestamp, ext);
    let new_path = file_name.with_file_name(new_file_name);

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&new_path)
        .unwrap_or_else(|e| panic!("Failed to open file {}: {}", new_path.display(), e));

    file.set_len(capacity).expect(format!("Unable to expand file size to capcity: {}", capacity).as_str());

    file
}

pub fn open_or_create(file_name: &str) -> File
{
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .append(true)
    .create(true)
    .open(file_name).unwrap();
    file
}

pub fn open_or_create_directory(path: &Path) -> std::io::Result<ReadDir>
{
    if path.exists()
    {
        fs::read_dir(path)
    }
    else {
        DirBuilder::new()
        .create(path)?;
        fs::read_dir(path)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;

    use super::*; 

    fn dir_exists(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    #[test]
    fn can_create_file_sized()
    {
        let test_dir = "wals";
        let file_name = "wals/test.wal";
        let _test_file = create_file_timed(Path::new(file_name), KIB * 4);

        assert!(dir_exists(test_dir));
        //cleanup
        remove_dir_all(test_dir).unwrap()
    }
}
