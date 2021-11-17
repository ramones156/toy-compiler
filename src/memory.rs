use std::collections::HashMap;
use std::io::ErrorKind;
use crate::ast::{Expr, Var};

pub struct Memory {
    vars: HashMap<String,Box<Expr>>,
}

impl Memory {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }
    pub fn add(&mut self, v: &Var) {
        let v = v.clone();
        self.vars.insert(v.name,Box::new(v.expr));
    }

    pub fn find(&self,name:&str) -> Result<&Expr, std::io::Error> {
        if let Some(e) = self.vars.get(name) {
            Ok(&**e)
        } else {
            Err(std::io::Error::new(ErrorKind::NotFound, "Couldn't find referenced variable"))
        }
    }
}
