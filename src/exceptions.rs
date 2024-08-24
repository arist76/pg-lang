pub trait PrintException {
    fn print_exception(&self) -> ();
}
pub struct BaseException {
    msg: String,
    line: u64,
    col: u64,
}

pub struct SyntaxException {
    exception: BaseException,
}

impl PrintException for SyntaxException {
    fn print_exception(&self) -> () {
        let line: &str = if self.exception.line == 0 {
            ""
        } else {
            &format!(" at line {}", self.exception.line)
        };
        println!("\x1b[31mSyntax Error\x1b[0m{}: {}", line, self.exception.msg);
    }
}

impl BaseException {
    pub fn new(msg: String, line: u64, col: u64) -> BaseException {
        BaseException { msg, line, col }
    }
}

impl SyntaxException {
    pub fn new(msg: String, line: u64, col: u64) -> SyntaxException {
        SyntaxException {
            exception: BaseException::new(msg, line, col),
        }
    }
}
