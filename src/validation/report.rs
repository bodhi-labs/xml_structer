use console::style;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub errors: Vec<Message>,
    pub warnings: Vec<Message>,
    pub info: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub line: usize,
    pub column: usize,
    pub text: String,
}

impl Report {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            warnings: vec![],
            info: vec![],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn push(
        &mut self,
        line: usize,
        column: usize,
        text: impl Into<String>,
        severity: Severity,
    ) {
        let msg = Message {
            line,
            column,
            text: text.into(),
        };
        match severity {
            Severity::Error => self.errors.push(msg),
            Severity::Warning => self.warnings.push(msg),
            Severity::Info => self.info.push(msg),
        }
    }

    /// Pretty console output
    pub fn print(&self) {
        if self.is_valid() && self.warnings.is_empty() && self.info.is_empty() {
            println!("{}", style("✔ Validation passed").green());
            return;
        }

        println!("\n{}", style("Validation Report").bold().underlined());
        for msg in &self.errors {
            println!(
                "{} {}:{}  {}",
                style("✗").red(),
                msg.line,
                msg.column,
                msg.text
            );
        }
        for msg in &self.warnings {
            println!(
                "{} {}:{}  {}",
                style("⚠").yellow(),
                msg.line,
                msg.column,
                msg.text
            );
        }
        for msg in &self.info {
            println!(
                "{} {}:{}  {}",
                style("ℹ").blue(),
                msg.line,
                msg.column,
                msg.text
            );
        }
        println!("{:-<50}", "");
        println!(
            "Total: {} errors, {} warnings, {} info",
            self.errors.len(),
            self.warnings.len(),
            self.info.len()
        );
    }

    /// JSON output (SARIF-ready skeleton)
    pub fn to_json_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Info,
}
