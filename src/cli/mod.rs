use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

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
}
