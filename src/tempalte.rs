use std::{
    env,
    error::Error,
    fs::{self, create_dir_all, File},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use colored::Colorize;
use dialoguer::Select;

use crate::{CLI_ARGS, USERBIN_PATH};

static PS1_REL: &str = "./templates/template.ps1";

fn ps1_template() -> PathBuf {
    let exe_parent_dir = env::current_exe().unwrap();
    let exe_parent_dir = exe_parent_dir.parent().unwrap();
    Path::new(&exe_parent_dir).join(PS1_REL)
}

async fn download_ps1_template() -> Result<(), Box<dyn Error>> {
    let ps1_repo = format!(
        "https://github.com/WiresawBlade/userbin/raw/master/{}",
        PS1_REL
    );
    let response = reqwest::get(ps1_repo).await?;

    return match response.status().is_success() {
        false => panic!("无法下载模板文件，请检查你的网络是否可以连接到 github.com."),
        true => {
            let ps1_template_path = ps1_template();
            create_dir_all(ps1_template_path.parent().unwrap())?;

            let mut dest = File::create(ps1_template_path)?;
            let content = response.bytes().await?;
            let _ = io::copy(&mut content.as_ref(), &mut dest);
            Ok(())
        }
    };
}

pub async fn create_ps1<P: AsRef<Path>>(target: P) -> Result<(), Box<dyn Error>> {
    let target = target.as_ref();
    let abs_targets = find_abs_path(target)?;
    let target = match abs_targets.len() {
        0 => target.into(),
        1 => abs_targets[0].clone(),
        _ => {
            let selected = Select::new()
                .with_prompt("当前输入存在多个可能的结果，请选择其一: ")
                .items(
                    &abs_targets
                        .iter()
                        .map(|x| x.to_str().unwrap())
                        .collect::<Vec<_>>(),
                )
                .interact()
                .unwrap();

            abs_targets[selected].clone()
        }
    };

    let file_name = target.file_name().unwrap().to_str().unwrap();
    let bin_path = &PathBuf::from(USERBIN_PATH.as_str());
    let ps1_path = &bin_path.join(format!(
        "{}.ps1",
        match CLI_ARGS.alias() {
            Some(alias) => alias,
            None => Path::new(file_name).file_stem().unwrap().to_str().unwrap(),
        }
    ));

    let ps1_template_path = ps1_template();
    if !ps1_template_path.exists() {
        println!("未检测到模板文件，正在下载...");
        download_ps1_template().await?;
    }

    fs::copy(&ps1_template_path, ps1_path)?;
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(ps1_path)?;

    let template_content = {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        content
    };

    let injected_content = format!("$exePath = \"{}\"\n", &target.display());
    let ps1_content = format!("{}\n\n{}", injected_content, template_content);

    file.seek(SeekFrom::Start(0))?;
    file.write_all(ps1_content.as_bytes())?;
    println!(
        "{} {}",
        ps1_path.to_str().unwrap().green(),
        "创建成功".green()
    );
    Ok(())
}

pub fn find_abs_path<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let path = path.as_ref();
    let mut abs_vec: Vec<PathBuf> = vec![];

    match path.is_absolute() {
        true => abs_vec.push(path.into()),
        false => {
            let env_path = env::var("PATH")?;
            let env_path = env_path.split(";").collect::<Vec<&str>>();

            for prefix in env_path {
                let curr_path = Path::new(prefix).join(path);
                if curr_path.exists() && curr_path.is_absolute() {
                    abs_vec.push(curr_path);
                }
            }

            let curr_dir = env::current_dir()?;
            let curr_dir = curr_dir.join(path);
            if curr_dir.exists() {
                abs_vec.push(curr_dir);
            }
        }
    };

    Ok(abs_vec)
}
