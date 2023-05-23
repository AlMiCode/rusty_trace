use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref LOGGER: Logger = Logger::default();
}

#[derive(Default)]
pub struct Logger {
    latest: RwLock<String>,
}

impl Logger {
    pub fn log(&self, str: impl Into<String>) {
        let str = str.into();
        eprintln!("{}", &str);
        self.latest
            .write()
            .and_then(|mut latest| Ok(*latest = str.into()))
            .expect("Logger log MUST never fail");
    }

    pub fn with_latest(&self, f: impl FnOnce(&str)) {
        self.latest
            .read()
            .and_then(|latest| Ok(f(&latest)))
            .expect("Logger MUST always provide latest msg");
    }
}
