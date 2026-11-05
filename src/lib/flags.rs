use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Путь до файла с обоями
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
}

/// Разбирает аргументы и возвращает абсолютный путь (если задан -f).
/// `cwd` нужен, потому что второй процесс может быть запущен из другой папки.
pub fn parse(args: Vec<OsString>, cwd: Option<PathBuf>) -> Result<Option<String>, clap::Error> {
    let args = Args::try_parse_from(args)?;
    Ok(args.file.map(|p| {
        let abs = if p.is_absolute() {
            p
        } else {
            cwd.unwrap_or_default().join(p)
        };
        abs.to_string_lossy().into_owned()
    }))
}
