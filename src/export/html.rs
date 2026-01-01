use crate::types::AuditResults;
use anyhow::Result;
use std::fs;

pub fn export(results: &AuditResults, path: &str) -> Result<()> {
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>APEX Audit Report</title>
    <style>
        body {{ font-family: 'Segoe UI', Arial, sans-serif; margin: 20px; background: #0a0a0a; color: #fff; }}
        h1 {{ color: #00ff00; }}
        .score {{ font-size: 48px; font-weight: bold; color: {}; }}
        table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
        th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #333; }}
        th {{ background: #1a1a1a; color: #00ff00; }}
        .ok {{ color: #00ff00; }}
        .warn {{ color: #ffaa00; }}
        .bad {{ color: #ff0000; }}
        .info {{ color: #00aaff; }}
    </style>
</head>
<body>
    <h1>APEX v11 Audit Report</h1>
    <div class="score">Score: {}/100</div>
    <h2>System Checks ({} total)</h2>
    <table>
        <tr>
            <th>Category</th>
            <th>Check</th>
            <th>Status</th>
            <th>Details</th>
        </tr>
        {}
    </table>
    <h2>Issues Found ({})</h2>
    <table>
        <tr>
            <th>#</th>
            <th>Category</th>
            <th>Severity</th>
            <th>Problem</th>
            <th>Solution</th>
        </tr>
        {}
    </table>
</body>
</html>"#,
        if results.score >= 90 { "#00ff00" } else if results.score >= 70 { "#ffaa00" } else { "#ff0000" },
        results.score,
        results.check_count,
        results.checks.iter().map(|c| format!(
            "<tr><td>{}</td><td>{}</td><td class=\"{}\">{:?}</td><td>{}</td></tr>",
            c.category,
            c.name,
            match c.status {
                crate::types::CheckStatus::Ok => "ok",
                crate::types::CheckStatus::Warn => "warn",
                crate::types::CheckStatus::Bad => "bad",
                crate::types::CheckStatus::Info => "info",
            },
            c.status,
            c.detail
        )).collect::<Vec<_>>().join("\n"),
        results.issues.len(),
        results.issues.iter().map(|i| format!(
            "<tr><td>{}</td><td>{}</td><td>{:?}</td><td>{}</td><td>{}</td></tr>",
            i.num,
            i.category,
            i.severity,
            i.problem,
            i.solution
        )).collect::<Vec<_>>().join("\n")
    );
    
    fs::write(path, html)?;
    Ok(())
}
