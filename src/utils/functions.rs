use {
    super::constants::{
        GB,
        KB,
        MB,
    },
    chrono::Local,
};

pub fn disk_size_format(bytes: u64) -> String {
    match bytes {
        0..KB => format!("{} B", bytes),
        KB..MB => format!("{:.2} KiB", bytes as f64 / KB as f64),
        MB..GB => format!("{:.2} MiB", bytes as f64 / MB as f64),
        _ => format!("{:.2} GiB", bytes as f64 / GB as f64),
    }
}

pub fn format_time() -> String {
    Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
