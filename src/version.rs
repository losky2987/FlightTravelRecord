pub const VERSION_MAJOR: u32 = 0;
pub const VERSION_MINOR: u32 = 1;
pub const VERSION_PATCH: u32 = 0;
pub const VERSION_CHANNEL: &str = "Alpha";

pub fn get_version() -> String {
    return format!("{}.{}.{} {}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_CHANNEL);
}
