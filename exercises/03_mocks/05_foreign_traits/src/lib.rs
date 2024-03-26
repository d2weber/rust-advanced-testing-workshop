use std::str::FromStr;

pub struct Parsed;

mockall::mock! {
    pub Parsed{}
    impl FromStr for Parsed {
        type Err=&'static str;
        fn from_str(s: &str) -> Result<Self, &'static str> ;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements() {
        static_assertions::assert_impl_one!(MockParsed: FromStr);
    }
}
