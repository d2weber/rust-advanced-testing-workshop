use anyhow::Context;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::Client;
use semver::Version;

async fn get_latest_release(
    client: &Client,
    base_uri: &str,
    owner: &str,
    repo: &str,
) -> Result<Version, GetReleaseError> {
    let url = format!("{base_uri}/repos/{owner}/{repo}/releases/latest");
    let response = client
        .get(&url)
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(USER_AGENT, "tester")
        .send()
        .await
        .map_err(GetReleaseError::NetworkError)?
        .error_for_status()
        .map_err(GetReleaseError::APIError)?;
    let release = response
        .json::<serde_json::Value>()
        .await
        .map_err(GetReleaseError::DeserializationError)?;
    let tag = release["tag_name"]
        .as_str()
        .context("tag_name is not a string")
        .map_err(GetReleaseError::InvalidTag)?;
    let tag = Version::parse(tag).map_err(|e| GetReleaseError::InvalidTag(e.into()))?;
    Ok(tag)
}

#[derive(Debug, thiserror::Error)]
enum GetReleaseError {
    #[error("Failed to send a request to GitHub")]
    NetworkError(reqwest::Error),
    #[error("GitHub API returned an error")]
    APIError(reqwest::Error),
    #[error("GitHub API returned an API response that we couldn't understand")]
    DeserializationError(reqwest::Error),
    #[error("The tag for the latest release is not a valid semver version")]
    InvalidTag(anyhow::Error),
}

#[cfg(test)]
mod tests {
    use crate::GetReleaseError;
    use googletest::matchers::{err, pat};
    use googletest::assert_that;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::method;

    #[googletest::test]
    #[tokio::test]
    async fn errors_if_tag_is_not_valid_semver_version() {
        let mock_server = MockServer::start().await;

        // Precondition: do what follows only if the request method is "GET"
        Mock::given(method("GET"))
            // Response value: return a 200 OK
            .respond_with(ResponseTemplate::new(200).set_body_string("{\"tag_name\": \"not a valid semver version\"}"))
            // Expectation: panic if this mock doesn't match at least once
            .expect(1..)
            .mount(&mock_server)
            .await;

        // Arrange
        let client = reqwest::Client::new();
        let owner = "LukeMathWalker";
        let repo = "pavex";

        // Act
        let outcome = super::get_latest_release(&client, &mock_server.uri(), owner, repo).await;

        // Assert
        assert_that!(outcome, err(pat!(GetReleaseError::InvalidTag(_))));
    }
}
