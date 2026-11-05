use std::mem::type_info::Str;

pub enum Command {
    SET(String, String),
    GET(String),
    DEL(String),
    INVALID(String)
}

impl Command {
    fn find(line: String) -> Self {
        let mut parsed = line.trim().splitn(2, " ");

        let cmd = parsed.next().unwrap_or("").to_uppercase();
        let args = parsed.next().unwrap_or("").split(" ");

        match cmd {
            "SET" => Command::SET(args.next(), args.next()),
            "GET" => Command::GET(args.next()),
            "DEL" => Command::DEL(args.next()),
            _ => Command::INVALID(cmd)
        }

    }
}
