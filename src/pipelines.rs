mod git_repo;
mod github_actions;
mod gitlab_ci;

use crate::pipelines::git_repo::GitRepo;
use crate::pipelines::github_actions::{GithubActions, GITHUB_ACTIONS};
use crate::pipelines::gitlab_ci::{GitlabCI, GITLAB_CI};
use std::env;

pub(crate) trait Pipeline {
    fn branch_name(&self) -> String;
    fn short_commit_sha(&self) -> String;
}

pub(crate) enum PipelineType {
    GithubActions(GithubActions),
    GitlabCI(GitlabCI),
    GitRepo(GitRepo),
}

fn pipeline_type() -> PipelineType {
    if env::var(GITHUB_ACTIONS).map_or(false, |v| v == "true") {
        PipelineType::GithubActions(GithubActions)
    } else if env::var(GITLAB_CI).map_or(false, |v| v == "true") {
        PipelineType::GitlabCI(GitlabCI)
    } else {
        PipelineType::GitRepo(GitRepo)
    }
}

pub(crate) struct PipelineInfo {
    pub branch_name: String,
    pub short_commit_sha: String,
}

impl PipelineInfo {
    fn new(p: &dyn Pipeline) -> Self {
        PipelineInfo {
            branch_name: p.branch_name(),
            short_commit_sha: p.short_commit_sha(),
        }
    }
}

pub(crate) fn pipeline_info() -> PipelineInfo {
    match pipeline_type() {
        PipelineType::GithubActions(p) => PipelineInfo::new(&p),
        PipelineType::GitlabCI(p) => PipelineInfo::new(&p),
        PipelineType::GitRepo(p) => PipelineInfo::new(&p),
    }
}