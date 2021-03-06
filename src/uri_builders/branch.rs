use crate::uri_builders::{BuildResult, UriBuilder, WithRepositoryUriBuilder};

#[derive(Debug, Clone)]
pub struct BranchUriBuilder<'r> {
    builder: WithRepositoryUriBuilder<'r>,
}

impl<'r> BranchUriBuilder<'r> {
    pub fn new(builder: WithRepositoryUriBuilder<'r>) -> Self {
        Self { builder }
    }

    terminal_resource_fn!(default);
}

impl<'r> UriBuilder for BranchUriBuilder<'r> {
    fn build(&self) -> BuildResult {
        let uri = format!("{}/branches", self.builder.build()?);
        Ok(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri_builders::tests::{TEST_HOST, TEST_PROJECT, TEST_REPO};
    use crate::uri_builders::ResourceUriBuilder;

    fn base_uri() -> String {
        format!(
            "{}/projects/{}/repos/{}/branches",
            crate::uri_builders::tests::base_uri(),
            TEST_PROJECT,
            TEST_REPO
        )
    }

    fn builder<'a>() -> BranchUriBuilder<'a> {
        ResourceUriBuilder::default()
            .host(TEST_HOST)
            .projects()
            .project(TEST_PROJECT)
            .repos()
            .repository(TEST_REPO)
            .branches()
    }

    #[test]
    fn branch_uri_works() {
        let uri = builder().build();
        assert_uri!(uri, base_uri());
    }

    #[test]
    fn branch_default_uri_works() {
        let uri = builder().default().build();
        assert_uri!(uri, format!("{}/default", base_uri()));
    }
}
