pub trait PrintException {
    fn print_exception(&self) -> ();
}
pub struct BaseException {
    msg: String,
    line: usize,
    col: usize,
}

pub struct SyntaxException {
    exception: BaseException,
}

impl PrintException for SyntaxException {
    fn print_exception(&self) -> () {
        println!(
            "\x1b[31mSyntax Error\x1b[0m at {}:{}\n\t {}",
            self.exception.line, self.exception.col, self.exception.msg
        );
    }
}

impl BaseException {
    pub fn new(msg: String, line: usize, col: usize) -> BaseException {
        BaseException { msg, line, col }
    }
}

impl SyntaxException {
    pub fn new(msg: String, line: usize, col: usize) -> SyntaxException {
        SyntaxException {
            exception: BaseException::new(msg, line, col),
        }
    }
}
