pub mod log;

pub const TEST_ADDRESS: &str = "127.0.0.1:81";

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
