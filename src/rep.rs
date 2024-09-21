pub fn read(lisp_exp: &str) -> anyhow::Result<&str> {
    println!("reading: {}", lisp_exp);
    Ok(lisp_exp)
}

pub fn eval(lisp_exp: &str) -> anyhow::Result<&str> {
    println!("eval: {}", lisp_exp);
    Ok(lisp_exp)
}

pub fn print(lisp_exp: &str) -> anyhow::Result<&str> {
    println!("print: {}", lisp_exp);
    Ok(lisp_exp)
}

pub fn rep(lisp_exp: &str) -> anyhow::Result<&str> {
    println!("rep: {}", lisp_exp);
    print(eval(read(lisp_exp)?)?)
}
