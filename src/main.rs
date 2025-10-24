use anyhow::{Context, Result};
use clap::Parser;

/// يمكنك البحث داخل ملف عن طريق ادخال النص
#[derive(Parser)]
struct Cli {
    /// ادخل النمط المراد البحث عنه
    pattern: String,
    /// ادخل المسار المراد البحث فيه
    path: std::path::PathBuf
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("لا يمكن قراءة الملف {}", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
