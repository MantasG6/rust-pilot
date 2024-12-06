use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
            .with_context(|| format!("could not write to provided writer"))?;
        }
    }
    Ok(())
}