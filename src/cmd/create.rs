use cargo_toml::{DependencyDetail, Manifest};
use cliclack::input;
use console::style;
use lazy_static::lazy_static;
use std::{
    collections::HashMap, fs, path::{Path, PathBuf}
};

use crate::{
    error::Result,
    utils::{clone_repo, re_init_repository},
};

#[derive(Default)]
struct Component<'a> {
    value: &'a str,
    label: &'a str,
    hint: &'a str,
    dependencies: Vec<Dependency>,
}

#[derive(Default)]
struct Dependency {
    name: String,
    features: Vec<String>,
    version: String,
    optional: bool,
}

impl From<(&'_ str, &'_ str)> for Dependency {
    fn from(value: (&'_ str, &'_ str)) -> Self {
        Dependency {
            name: value.0.into(),
            version: value.1.into(),
            features: vec![],
            optional: false,
        }
    }
}

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
lazy_static! {
    static ref COMPONENTS: Vec<Component<'static>> = vec![
        Component {
            value: "sea-orm",
            label: "SeaORM",
            hint: "recommended",
            dependencies: vec![Dependency {
                name: "sea-orm".into(),
                version: "0.12".into(),
                features: vec!["tokio".into(), "rustls".into(), "sqlx-sqlite".into()],
                optional: false,
            }],
        },
        Component {
            value: "jwt",
            label: "JWT",
            hint: "https://github.com/keats/jsonwebtoken",
            dependencies: vec![("jsonwebtoken", "9").into(),],
        },
        Component {
            value: "websocket",
            label: "WebSocket",
            hint: "https://docs.rs/tokio-tungstenite",
            dependencies: vec![("tokio-tungstenite", "0.23.0").into(),],
        },
    ];
}

pub(crate) fn create_project(project_name: &Option<String>) -> Result<()> {
    // ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");
    cliclack::clear_screen().unwrap();

    cliclack::intro(style(" create-app ").on_cyan().black()).unwrap();

    let project_name = get_project_name(project_name);
    let web_framework = get_template_name();
    let components = mult_select_components();

    let (_, repo_url) = GLOBAL_TEMPLATES.get_key_value(&web_framework).unwrap();

    let spinner = cliclack::spinner();
    spinner.start("初始化项目中...");
    let _ = clone_repo(repo_url, &project_name)?;

    remove_template_git(&project_name);

    // 重新初始化 git
    re_init_repository(&project_name)?;

    //添加组件
    let _ = add_components(&project_name, &components);
    spinner.stop("初始化完成");

    let next_steps = format!(
        "cd {project_name}\n{install}cargo run\n",
        install =
            style("cargo install").magenta().to_string() + &style(" # 🚀").dim().to_string() + "\n"
    );

    cliclack::note("Next steps. 🌲🍉🐓", next_steps).unwrap();

    cliclack::outro(format!(
        "Problems? {}\n",
        style("https://github.com/kingzcheung/nana/issues")
            .cyan()
            .underlined()
    ))
    .unwrap();
    Ok(())
}

/// 多选组件交互函数
///
/// 该函数通过cliclack库提供的接口，实现了一个多选组件选择的交互式界面。
/// 用户可以在提供的组件列表中选择一个或多个组件，最终函数返回用户选择的组件名称列表。
fn mult_select_components() -> Vec<String> {
    // 初始化多选组件选择界面，设置初始选中项为"sea-orm"
    let mut comps =
        cliclack::multiselect("Select additional components").initial_values(vec!["sea-orm"]);

    for c in COMPONENTS.iter() {
        comps = comps.item(c.value, c.label, c.hint);
    }

    // 与用户进行交互，获取用户的选择
    let comps = comps.interact().unwrap();
    // 将用户选择的组件转换为String类型，并收集到一个Vec中返回
    comps.into_iter().map(|x| x.into()).collect()
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
fn get_project_name(name: &Option<String>) -> String {
    match name {
        Some(n) => n.clone(),
        None => {
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
    cliclack::select("选择你喜欢的 WEB 框架".to_string())
        .initial_value("axum")
        .item("axum", "Axum", "https://github.com/tokio-rs/axum")
        .item("salvo", "Salvo", "https://salvo.rs/")
        .item("actix", "actix-web", "https://actix.rs/")
        .interact()
        .unwrap()
        .into()
}

fn add_components(project_name: &str, components: &[String]) -> anyhow::Result<()> {
    update_cargo_toml(project_name, components)?;

    Ok(())
}

fn update_cargo_toml(project_name: &str, components: &[String]) -> anyhow::Result<()> {
    let mut cargo_toml_path = PathBuf::new();
    cargo_toml_path.push(project_name);
    cargo_toml_path.push("Cargo.toml");

    let toml = std::fs::read(&cargo_toml_path)?;

    let mut manifest = Manifest::from_slice(&toml)?;
    
    for name in components {
        let find = COMPONENTS.iter().find(|x| x.value == name);
        if let Some(comp) = find {
            for dep in comp.dependencies.iter() {
                manifest.dependencies.insert(
                    dep.name.clone(),
                    cargo_toml::Dependency::Detailed(Box::new(DependencyDetail {
                        version: Some(dep.version.clone()),
                        registry: None,
                        registry_index: None,
                        path: None,
                        inherited: false,
                        git: None,
                        branch: None,
                        tag: None,
                        rev: None,
                        features:dep.features.clone(),
                        optional: dep.optional,
                        default_features: true,
                        package: None,
                        unstable: Default::default(),
                    })),
                );
            }
        }
    }

    let toml_str = toml::to_string(&manifest)?;
    fs::write(cargo_toml_path, toml_str)?;

    Ok(())
}
