#[cfg(target_os = "linux")]
const LKH_BINARY: &[u8] = include_bytes!("../resources/LKH");

#[cfg(target_os = "windows")]
const LKH_BINARY: &[u8] = include_bytes!("../resources/LKH.exe");
