use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

use crate::types::error::BadArgumentError;

/// Represents a parsed command
pub struct Command {
    pub args: Vec<String>,
    pub args_set: HashSet<String>,
    pub kwargs: HashMap<String, String>,
}

impl Command {
    /// parses args into a `Command`
    pub fn new(args: std::env::Args) -> Self {
        let args: Vec<String> = args.collect();
        let mut args_set = HashSet::<String>::new();
        let mut kwargs = HashMap::<String, String>::new();

        lazy_static! {
            /// splits keyword from arg
            static ref RE: regex::Regex = regex::Regex::new(r"([a-zA-Z\-_0-9]+?)=(.+)").unwrap();
        }
        for arg in &args {
            args_set.insert(arg.to_owned());
            if let Some(captures) = RE.captures(arg) {
                kwargs.insert(
                    captures.get(1).unwrap().as_str().to_owned(),
                    captures.get(2).unwrap().as_str().to_owned(),
                );
            };
        }

        Command {
            args,
            args_set,
            kwargs,
        }
    }

    /// tries to parse the speciifed kwarg into an int
    /// returns None if the key is not present
    /// returns Some(Ok(T)) if the key is present and the value for it can be
    /// parsed as an int
    /// returns Some(Err(E)) if the key is present and the value for ti cannot
    /// be parsed as an int
    pub fn uint_kwarg<T: From<u64>>(&self, key: &str) -> Option<Result<T, BadArgumentError>> {
        if let Some(n_str) = self.kwargs.get(key) {
            Some(if let Ok(n) = n_str.parse::<u64>() {
                Ok(n.into())
            } else {
                Err(BadArgumentError(format!(
                    "kwarg `{}` must be a valid integer, but `{}`, is not",
                    key, n_str
                )))
            })
        } else {
            None
        }
    }
}

/// Represents a builder for creating ansi strings
pub struct AnsiStringBuilder {
    bold: bool,
    foreground_color: Option<(u8, u8, u8)>,
    strikethrough: bool,
    s: String,
}

impl AnsiStringBuilder {
    pub fn new() -> Self {
        AnsiStringBuilder {
            bold: false,
            foreground_color: None,
            strikethrough: false,
            s: String::new(),
        }
    }

    pub fn clear_all(&mut self) {
        self.s.push_str("\x1b[0m");
    }

    pub fn set_foreground_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.foreground_color = Some((r, g, b));
        self
    }

    pub fn set_bold(mut self, set: bool) -> Self {
        self.bold = set;
        self
    }

    pub fn set_strikethrough(mut self, set: bool) -> Self {
        self.strikethrough = set;
        self
    }

    pub fn push_str(mut self, string: &str) -> Self {
        let mut escape_codes = Vec::<String>::new();
        if self.bold {
            escape_codes.push("1".to_string());
        }
        if let Some(color) = self.foreground_color {
            escape_codes.extend([
                "38".to_string(),
                "2".to_string(),
                color.0.to_string(),
                color.1.to_string(),
                color.2.to_string(),
            ]);
        }
        if self.strikethrough {
            escape_codes.push("9".to_string());
        }
        self.s.push_str(&format!("\x1b[{}m{}", escape_codes.join(";"), string));
        self.clear_all();
        self
    }

    pub fn build(self) -> String {
        self.s
    }
}
