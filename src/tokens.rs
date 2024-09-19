pub enum Token {
    LeftParent,
    RightParent,
    Plus,
    Minus,
    Multi,
    Div,
    Integer(i64),
    Float(f64),
}
