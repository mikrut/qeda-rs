mod kicad;

use std::collections::HashMap;

use crate::errors::*;
use crate::library::Library;
use kicad::KicadGenerator;

pub trait GeneratorHandler {
    fn render(&self, name: &str, library: &Library) -> Result<()>;
}

pub struct Generators<'a> {
    handlers: HashMap<&'a str, Box<dyn GeneratorHandler>>,
}

impl<'a> Generators<'a> {
    pub fn new() -> Generators<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn GeneratorHandler>> = HashMap::new();
        handlers.insert("kicad", Box::new(KicadGenerator::new()));

        Generators {
            handlers,
        }
    }

    pub fn get(&self, key: &str) -> Result<&Box<dyn GeneratorHandler>> {
        self.handlers.get(key).ok_or(ErrorKind::InvalidGeneratorType(key.to_string()).into())
    }
}
