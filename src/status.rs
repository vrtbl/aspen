use colored::*;

pub enum Kind {
    Info,
    Success,
    Warn,
    Fatal,
}

pub struct Status(pub Kind, pub &'static str);

impl Status {
    pub fn info()    -> Status { Status(Kind::Info, "Info") }
    pub fn success() -> Status { Status(Kind::Success, "Success") }
    pub fn warn()    -> Status { Status(Kind::Warn, "Warning") }
    pub fn fatal()    -> Status { Status(Kind::Fatal, "Fatal") }

    pub fn log(&self, message: String) {
        let tag = match self.0 {
            Kind::Info    => self.1.white(),
            Kind::Success => self.1.blue(),
            Kind::Warn    => self.1.yellow(),
            Kind::Fatal   => self.1.red(),
        };

        println!("{:>12} {}", tag.bold(), message);
    }
}
