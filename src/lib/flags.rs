use clap::Parser;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Путь до файла с обоями (поддерживаются ~, . и ..)
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
}

/// `~` и `~/...` -> $HOME. Форма `~user` не поддерживается.
fn expand_tilde(p: &Path) -> PathBuf {
    let mut comps = p.components();
    if let Some(Component::Normal(first)) = comps.next() {
        if first == "~" {
            if let Some(home) = std::env::var_os("HOME") {
                return PathBuf::from(home).join(comps.as_path());
            }
        }
    }
    p.to_path_buf()
}

/// Схлопывает `.` и `..` без обращения к файловой системе.
fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                // на корне `..` игнорируется, как и в самой ФС
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Разбирает аргументы и возвращает абсолютный нормализованный путь (если задан -f).
/// `cwd` нужен, потому что второй процесс может быть запущен из другой папки.
pub fn parse(args: Vec<OsString>, cwd: Option<PathBuf>) -> Result<Option<String>, clap::Error> {
    let args = Args::try_parse_from(args)?;
    Ok(args.file.map(|p| {
        let p = expand_tilde(&p);
        let abs = if p.is_absolute() {
            p
        } else {
            cwd.unwrap_or_default().join(p)
        };
        normalize(&abs).to_string_lossy().into_owned()
    }))
}
