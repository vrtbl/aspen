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

    fn tag(&self) -> ColoredString {
        match self.0 {
            Kind::Info    => self.1.blue(),
            Kind::Success => self.1.green(),
            Kind::Warn    => self.1.yellow(),
            Kind::Fatal   => self.1.red(),
        }.bold()
    }

    pub fn log(&self, message: &str) {
        let lines = message.lines().collect::<Vec<&str>>();
        let multiline = lines.len() > 1;
        if multiline { eprintln!() }

        let mut tag = self.tag();
        let blank   = " ".repeat(tag.len()).hidden();

        for line in lines {
            eprintln!("{:>12} {}", tag, line);
            tag = blank.clone();
        }

        if multiline { eprintln!() }
    }
}
