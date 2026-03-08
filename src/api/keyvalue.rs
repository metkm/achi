use std::{
    ffi::c_int,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use log::error;

use crate::error::AppError;

#[derive(PartialEq, Clone, Debug, Default)]
enum KeyValueType {
    #[default]
    None = 0,
    String = 1,
    Int32 = 2,
    Float32 = 3,
    Pointer = 4,
    WideString = 5,
    Color = 6,
    UInt64 = 7,
    End = 8,
}

impl From<u8> for KeyValueType {
    fn from(value: u8) -> Self {
        match value {
            0 => KeyValueType::None,
            1 => KeyValueType::String,
            2 => KeyValueType::Int32,
            3 => KeyValueType::Float32,
            4 => KeyValueType::Pointer,
            5 => KeyValueType::WideString,
            6 => KeyValueType::Color,
            7 => KeyValueType::UInt64,
            8 => KeyValueType::End,
            _ => Self::None,
        }
    }
}

fn read_string_from_bufreader<R: Read>(reader: &mut BufReader<R>) -> Result<String, AppError> {
    let mut buffer = Vec::with_capacity(128);
    let read_count = reader.read_until(0, &mut buffer)?;

    unsafe {
        buffer.set_len(read_count - 1); // -1 is for \0
    }

    Ok(String::from_utf8(buffer).unwrap_or("".to_string()))
}

fn read_i32_from_bufreader<R: Read>(reader: &mut BufReader<R>) -> Result<i32, AppError> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer).ok();

    Ok(i32::from_le_bytes(buffer))
}

fn read_u64_from_bufreader<R: Read>(reader: &mut BufReader<R>) -> Result<u64, AppError> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer).ok();

    Ok(u64::from_le_bytes(buffer))
}

fn read_f32_from_bufreader<R: Read>(reader: &mut BufReader<R>) -> Result<f32, AppError> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer).ok();

    Ok(f32::from_le_bytes(buffer))
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct KeyValue {
    pub children: Vec<KeyValue>,
    kvt: KeyValueType,
    pub name: String,
    pub value: String,
}

impl KeyValue {
    pub fn get_kv_by_name(&self, key: &str) -> Option<&KeyValue> {
        if self.children.is_empty() {
            return None;
        }

        self.children.iter().find(|c| c.name == key)
    }

    pub fn parse_buffer(&mut self, reader: &mut BufReader<File>) {
        loop {
            let mut type_buffer = [0];

            if reader.read_exact(&mut type_buffer).is_err() {
                break;
            }

            let kvt = KeyValueType::from(type_buffer[0]);
            if kvt == KeyValueType::End {
                break;
            }

            let mut current = KeyValue {
                kvt: kvt.clone(),
                name: read_string_from_bufreader(reader).unwrap_or("unknown".to_string()),
                ..Default::default()
            };

            match kvt {
                KeyValueType::None => {
                    current.parse_buffer(reader);
                }
                KeyValueType::String => {
                    current.value = read_string_from_bufreader(reader)
                        .unwrap_or_else(|_| "unknown".to_string());
                }
                KeyValueType::WideString => {
                    error!("Widestring is not supported");
                    break;
                }
                KeyValueType::Int32 => {
                    current.value = format!("{}", read_i32_from_bufreader(reader).unwrap_or(0));
                }
                KeyValueType::UInt64 => {
                    current.value = format!("{}", read_u64_from_bufreader(reader).unwrap_or(0));
                }
                KeyValueType::Float32 => {
                    current.value = format!("{}", read_f32_from_bufreader(reader).unwrap_or(0.0));
                }
                KeyValueType::Color => {
                    current.value = format!("{}", read_i32_from_bufreader(reader).unwrap_or(0));
                }
                KeyValueType::Pointer => {
                    current.value = format!("{}", read_i32_from_bufreader(reader).unwrap_or(0));
                }
                _ => break,
            };

            self.children.push(current);
        }
    }

    pub fn from_install_path(install_path: &str, app_id: c_int) -> Result<KeyValue, AppError> {
        let target = Path::new(install_path)
            .join("appcache")
            .join("stats")
            .join(format!("UserGameStatsSchema_{app_id}.bin"));

        let file = File::open(&target)?;
        let mut reader = BufReader::new(file);

        let mut root = KeyValue {
            name: String::from("<Root>"),
            kvt: KeyValueType::None,
            ..Default::default()
        };

        root.parse_buffer(&mut reader);

        Ok(root)
    }
}
