pub fn square(x: i32, logger: impl Logger) -> i32 {
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub trait Logger {
    fn log(&self, msg: &str);
}

pub struct PrintlnLogger;

impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
struct TestLogger {}
impl Logger for TestLogger {
    fn log(&self, msg: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        assert_eq!(square(2, TestLogger {}), 4);
    }
}
