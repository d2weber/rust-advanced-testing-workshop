use googletest::matcher::Matcher;
use http::StatusCode;

struct IsRedirect {
}

impl Matcher for IsRedirect {
    type ActualT = StatusCode;

    fn matches(&self, actual: &Self::ActualT) -> googletest::matcher::MatcherResult {
        match actual {
            &StatusCode::MOVED_PERMANENTLY /* TODO: and others.. */ => googletest::matcher::MatcherResult::Match,
            _ => googletest::matcher::MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: googletest::matcher::MatcherResult) -> googletest::description::Description {
        match matcher_result {
            googletest::matcher::MatcherResult::Match => "is a redirection status code".into(),
            googletest::matcher::MatcherResult::NoMatch => "isn't a redirection status code".into()
        }
        }
}

pub fn is_redirect() -> impl Matcher<ActualT = StatusCode> {
    IsRedirect{}
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
