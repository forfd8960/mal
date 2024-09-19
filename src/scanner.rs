use crate::tokens::Token;

pub struct Scanner {
    lisp_exp: String,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(lisp_exp: String) -> Self {
        Self {
            lisp_exp: lisp_exp,
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> anyhow::Result<Vec<Token>> {
        Ok(vec![])
    }
}
