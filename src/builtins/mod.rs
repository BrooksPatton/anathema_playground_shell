use std::fmt::Display;

pub mod builtin_echo;
pub mod builtin_exit;
pub mod builtin_type;

#[derive(Debug, Clone, Copy)]
pub enum Builtin {
    Exit,
    Notfound,
    Echo,
    Type,
}

impl From<&String> for Builtin {
    fn from(command_name: &String) -> Self {
        match command_name.as_str() {
            "exit" => Self::Exit,
            "echo" => Self::Echo,
            "type" => Self::Type,
            _ => Self::Notfound,
        }
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let builtin_name = format!("{self:?}").to_lowercase();
        write!(f, "{builtin_name}")
    }
}
