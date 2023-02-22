// use crate::types::KeyEvent;
// use anyhow::{Context, Ok};
// use chrono::offset::Utc;
// use chrono::DateTime;
// use std::fs;
// use std::io::Read;
// use std::time::SystemTime;
// use std::{fs::File, io::Write};

// pub struct KeyEventFile {
//     file: File,
//     content: Vec<u8>,
// }

// impl KeyEventFile {
//     pub fn create_new(key_event: &KeyEvent) -> anyhow::Result<Self> {
//         let path = Self::get_file_path_by_date();
//         let file = File::create(path)?;

//         let content = key_event.to_u8_vec()?;
//         Ok(Self { file, content })
//     }

//     pub fn create_with_file_by_default() -> anyhow::Result<Self> {
//         let path = Self::get_file_path_by_date();
//         let metadata = fs::metadata(&path).expect("Unable to read metadata");
//         let mut file = File::open(&path)?;

//         let mut content = vec![0; metadata.len() as usize];
//         file.read(&mut content).context("Buffer overflow")?;

//         Ok(Self { file, content })
//     }

//     pub fn write(&mut self) -> anyhow::Result<()> {
//         self.file
//             .write_all(&self.content)
//             .context("Failed to write file")
//     }

//     pub fn get_key_event(&self) -> anyhow::Result<KeyEvent> {

//     }

//     fn get_file_path_by_date() -> String {
//         let system_time = SystemTime::now();
//         let datetime: DateTime<Utc> = system_time.into();
//         datetime.format("%Y-%m-%d.kbd").to_string()
//     }
// }
