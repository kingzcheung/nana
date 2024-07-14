use cliclack::input;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use console::style;


use crate::{
    error::Result,
    utils::{clone_repo, re_init_repository},
};

lazy_static! {
    static ref GLOBAL_TEMPLATES: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert(
            String::from("axum"),
            "https://github.com/kingzcheung/nana-template-axum".into(),
        );
        m.insert(String::from("salvo"), "todo".into());
        m.insert(String::from("actix"), "todo".into());
        m
    };
}

pub(crate) fn create_project(project_name: &Option<String>) -> Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");
    cliclack::clear_screen().unwrap();

    cliclack::intro(style(" create-app ").on_cyan().black()).unwrap();
    // 从参数中提取项目名，如果项目名不存在，则使用默认值或进行其他处理。
    let project_name = get_project_name(project_name);
    let web_farmework = get_template_name();

    let (_, repo_url) = GLOBAL_TEMPLATES.get_key_value(&web_farmework).unwrap();

    let spinner = cliclack::spinner();
    spinner.start("初始化项目中...");
    let _ = clone_repo(repo_url, &project_name)?;

    remove_template_git(&project_name);

    // 重新初始化 git
    re_init_repository(&project_name)?;
    spinner.stop("初始化完成");

    let next_steps = format!(
        "cd {project_name}\n{install}cargo run\n",
        install = style("cargo install").magenta().to_string() + &style(" # 🚀").dim().to_string() + "\n"
    );

    cliclack::note("Next steps. 🌲🍉🐓", next_steps).unwrap();

    cliclack::outro(format!(
        "Problems? {}\n",
        style("https://example.com/issues").cyan().underlined()
    )).unwrap();
    Ok(())
}

fn remove_template_git(project_name: &str) {
    let mut project_path = PathBuf::new();
    project_path.push(project_name);
    project_path.push(".git");
    if let Ok(m) = std::fs::metadata(project_name) {
        if m.is_dir() {
            std::fs::remove_dir_all(project_path).unwrap();
        }
    }
}

/// 根据提供的项目名获取最终的项目名称。
/// 如果项目名未提供，则通过用户交互方式获取。
///
/// # 参数
/// `project_name` - 项目的名称选项，可能为空。
///
/// # 返回值
/// 返回一个字符串，表示最终确定的项目名称。
fn get_project_name(project_name: &Option<String>) -> String {
    // 根据项目名选项进行匹配处理
    match project_name {
        // 如果项目名存在，直接克隆返回
        Some(name) => name.clone(),
        // 如果项目名不存在，通过用户交互方式获取项目名
        None => {
            // 通过input函数交互式获取项目名，设置默认提示为"new_project"
            let path: String = input("Where should we create your project?")
                .placeholder("new_project")
                // 对输入的项目名进行验证
                .validate(|input: &String| {
                    // 验证项目名不能为空
                    if input.is_empty() {
                        Err("请输入项目名称.")
                    // 验证项目名不能以数字开头
                    } else if input.chars().next().map_or(false, |c| c.is_numeric()) {
                        Err("请输入正确的项目名称,名称不能以数字开头.")
                    // 验证项目名不能是已存在的路径
                    } else if Path::new(input).exists() {
                        Err("项目名称已存在.")
                    } else {
                        Ok(())
                    }
                })
                // 与用户交互获取项目名
                .interact()
                .unwrap();
            // 返回通过交互方式获取的项目名
            path
        }
    }
}

fn get_template_name() -> String {
    let kind = cliclack::select(format!("选择你喜欢的 WEB 框架"))
        .initial_value("axum")
        .item("axum", "Axum", "https://github.com/tokio-rs/axum")
        .item("salvo", "Salvo", "https://salvo.rs/")
        .item("actix", "actix-web", "https://actix.rs/")
        .interact()
        .unwrap();

    if kind != "axum" {
        panic!("暂不支持该框架")
    }
    kind.into()
}
