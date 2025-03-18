use wget::disk_size_format;

#[test]
fn test_disk_size_format_zero() {
    assert_eq!(disk_size_format(0), "0 B");
}

#[test]
fn test_disk_size_format_1023() {
    assert_eq!(disk_size_format(1023), "1023 B");
}

#[test]
fn test_disk_size_format_1024() {
    assert_eq!(disk_size_format(1024), "1.00 KiB");
}

#[test]
fn test_disk_size_format_5500() {
    assert_eq!(disk_size_format(5500), "5.37 KiB");
}

#[test]
fn test_disk_size_format_1048575() {
    assert_eq!(disk_size_format(1048575), "1024.00 KiB");
}

#[test]
fn test_disk_size_format_1048576() {
    assert_eq!(disk_size_format(1048576), "1.00 MiB");
}
