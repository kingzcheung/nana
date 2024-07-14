use std::path::Path;

use git2::Repository;

use crate::error::{CliError, Result};

/// 克隆指定的Git仓库到本地。
///
/// 此函数使用git2库来实现仓库的克隆操作。它限定了克隆的深度为1，即只克隆最近一次的提交，
/// 这有助于减少克隆的时间和资源消耗。
///
/// 参数:
/// - `repo_url`: 待克隆的Git仓库的URL。
/// - `project_name`: 克隆完成后本地仓库的名称。
///
/// 返回值:
/// - 如果克隆成功，返回一个`Repository`对象，表示本地的仓库实例。
/// - 如果克隆失败，返回一个`CliError`错误，其中包含失败的原因。
pub(crate) fn clone_repo<P: AsRef<Path>>(repo_url: &str, project_name: P) -> Result<Repository> {
    // 创建一个新的RepoBuilder实例，用于配置和执行仓库克隆操作。
    let mut builder = git2::build::RepoBuilder::new();

    // 初始化FetchOptions对象，用于配置克隆时的拉取选项。
    let mut fetch_opts = git2::FetchOptions::new();

    // 设置克隆的深度为1，即只拉取最近一次的提交。
    fetch_opts.depth(1);

    // 将拉取选项应用到RepoBuilder中，以影响克隆操作。
    builder.fetch_options(fetch_opts);

    // 尝试使用配置的选项克隆指定的仓库到本地。如果成功，返回克隆的仓库对象；如果失败，返回相应的错误。
    match builder.clone(repo_url, project_name.as_ref()) {
        Ok(r) => Ok(r),
        Err(e) => Err(CliError::CloneFailed(e.to_string())),
    }
}

pub(crate) fn re_init_repository<P: AsRef<Path>>(project_name: P) -> Result<Repository> {
    match Repository::init(project_name) {
        Ok(repo) => Ok(repo),
        Err(e) => Err(CliError::InitFailed(e.to_string())),
    }
}
