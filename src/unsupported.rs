use Result;

pub fn get() -> Result<String> {
    Err("unsupported operating system".into())
}

pub fn set_from_path(_: &str) -> Result<()> {
    Err("unsupported operating system".into())
}

pub fn set_mode(_: Mode) -> Result<()> {
    Err("unsupported operating system".into())
}

pub fn get_mode() -> Result<Mode> {
    Err("unknown mode".into())
}
