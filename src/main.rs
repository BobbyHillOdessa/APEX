mod config;
mod types;
mod hardware;
mod checks;
mod fixes;
mod export;
mod utils;

use clap::{Parser, Subcommand};
use colored::*;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "APEX")]
#[command(author = "APEX Team")]
#[command(version = "11.0.0")]
#[command(about = "Military-Grade Windows System Performance Auditor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a full system audit
    Audit {
        /// Export results to JSON
        #[arg(long)]
        json: Option<String>,
        
        /// Export results to HTML
        #[arg(long)]
        html: Option<String>,
        
        /// Export results to CSV
        #[arg(long)]
        csv: Option<String>,
    },
    
    /// Apply fixes for detected issues
    Fix {
        /// Issue number to fix
        #[arg(short, long)]
        issue: Option<u32>,
        
        /// Fix all issues
        #[arg(long)]
        all: bool,
    },
    
    /// Rollback previous fixes
    Rollback {
        /// Backup file to restore
        backup: String,
    },
    
    /// Run performance benchmark
    Benchmark,
    
    /// Monitor system in real-time
    Monitor {
        /// Update interval in seconds
        #[arg(short, long, default_value = "1")]
        interval: u64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    print_banner();
    
    match cli.command {
        Commands::Audit { json, html, csv } => {
            run_audit(json, html, csv)?;
        }
        Commands::Fix { issue, all } => {
            run_fix(issue, all)?;
        }
        Commands::Rollback { backup } => {
            run_rollback(&backup)?;
        }
        Commands::Benchmark => {
            run_benchmark()?;
        }
        Commands::Monitor { interval } => {
            run_monitor(interval)?;
        }
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_green());
    println!("{}", "║                        APEX v11.0.0                          ║".bright_green());
    println!("{}", "║         Military-Grade Windows Performance Auditor          ║".bright_green());
    println!("{}", "║                  500+ Checks • Rust Powered                  ║".bright_green());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_green());
    println!();
}

fn run_audit(json_path: Option<String>, html_path: Option<String>, csv_path: Option<String>) -> anyhow::Result<()> {
    println!("{}", "🔍 Starting system audit...".bright_cyan());
    
    let start = Instant::now();
    
    // Detect hardware
    println!("{}", "📊 Detecting hardware...".bright_cyan());
    let hardware = hardware::HardwareInfo::detect();
    
    println!("   CPU: {}", hardware.cpu.name.bright_yellow());
    println!("   Cores: {} physical, {} logical", hardware.cpu.cores, hardware.cpu.threads);
    println!("   Memory: {:.2} GB", hardware.memory.total as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("   GPUs: {}", hardware.gpu.len());
    for gpu in &hardware.gpu {
        println!("      - {}", gpu.name.bright_yellow());
    }
    println!();
    
    // Run checks
    println!("{}", "🔬 Running system checks...".bright_cyan());
    let config = config::Config::new();
    let results = checks::run_all_checks(&config);
    
    let duration = start.elapsed();
    
    println!();
    println!("{}", "═══════════════════════════════════════════════════════════════".bright_green());
    println!("{}", format!("✅ Audit Complete in {:.2}s", duration.as_secs_f64()).bright_green());
    println!("{}", "═══════════════════════════════════════════════════════════════".bright_green());
    println!();
    
    // Display score
    let score_color = if results.score >= 90 {
        "green"
    } else if results.score >= 70 {
        "yellow"
    } else {
        "red"
    };
    
    println!("{}", format!("🎯 SYSTEM SCORE: {}/100", results.score).color(score_color).bold());
    println!();
    
    // Display summary
    println!("{}", format!("📋 Total Checks: {}", results.check_count).bright_white());
    println!("{}", format!("⚠️  Issues Found: {}", results.issues.len()).bright_yellow());
    println!("{}", format!("⚡ Warnings: {}", results.warnings.len()).bright_yellow());
    println!();
    
    // Display check categories
    let ok_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Ok)).count();
    let warn_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Warn)).count();
    let bad_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Bad)).count();
    let info_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Info)).count();
    
    println!("Check Results:");
    println!("   {} {}", "✓".green(), format!("{} OK", ok_count).green());
    println!("   {} {}", "⚠".yellow(), format!("{} Warnings", warn_count).yellow());
    println!("   {} {}", "✗".red(), format!("{} Critical", bad_count).red());
    println!("   {} {}", "ℹ".blue(), format!("{} Info", info_count).blue());
    println!();
    
    // Export results
    if let Some(path) = json_path {
        export::json::export(&results, &path)?;
        println!("{}", format!("💾 JSON report saved to: {}", path).bright_green());
    }
    
    if let Some(path) = html_path {
        export::html::export(&results, &path)?;
        println!("{}", format!("💾 HTML report saved to: {}", path).bright_green());
    }
    
    if let Some(path) = csv_path {
        export::csv::export(&results, &path)?;
        println!("{}", format!("💾 CSV report saved to: {}", path).bright_green());
    }
    
    Ok(())
}

fn run_fix(_issue: Option<u32>, _all: bool) -> anyhow::Result<()> {
    utils::privileges::require_admin()?;
    
    println!("{}", "🔧 Fix functionality requires detailed implementation".bright_yellow());
    println!("{}", "   This will be available in a future update.".bright_white());
    
    Ok(())
}

fn run_rollback(backup: &str) -> anyhow::Result<()> {
    utils::privileges::require_admin()?;
    
    println!("{}", format!("⏮️  Rolling back from: {}", backup).bright_cyan());
    fixes::backup::rollback_from_file(backup)?;
    println!("{}", "✅ Rollback complete!".bright_green());
    
    Ok(())
}

fn run_benchmark() -> anyhow::Result<()> {
    println!("{}", "🏁 Running performance benchmark...".bright_cyan());
    
    let iterations = 10;
    let mut total_time = 0.0;
    
    for i in 1..=iterations {
        let start = Instant::now();
        let config = config::Config::new();
        let _ = checks::run_all_checks(&config);
        let duration = start.elapsed();
        total_time += duration.as_secs_f64();
        
        println!("   Run {}/{}: {:.3}s", i, iterations, duration.as_secs_f64());
    }
    
    let avg_time = total_time / iterations as f64;
    
    println!();
    println!("{}", format!("📊 Average audit time: {:.3}s", avg_time).bright_green());
    println!("{}", format!("📊 Checks per second: {:.0}", 500.0 / avg_time).bright_green());
    
    Ok(())
}

fn run_monitor(interval: u64) -> anyhow::Result<()> {
    println!("{}", format!("👁️  Monitoring system (update every {}s, Ctrl+C to exit)...", interval).bright_cyan());
    println!();
    
    loop {
        let start = Instant::now();
        let config = config::Config::new();
        let results = checks::run_all_checks(&config);
        let duration = start.elapsed();
        
        // Clear screen (simple version)
        print!("\x1B[2J\x1B[1;1H");
        
        println!("{}", format!("APEX Monitor - Score: {}/100 - Scan: {:.2}s", results.score, duration.as_secs_f64()).bright_green());
        println!("{}", "═══════════════════════════════════════════════════════════════".bright_green());
        
        let ok_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Ok)).count();
        let warn_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Warn)).count();
        let bad_count = results.checks.iter().filter(|c| matches!(c.status, types::CheckStatus::Bad)).count();
        
        println!("   {} OK   {} WARN   {} CRITICAL", 
            format!("{}", ok_count).green(),
            format!("{}", warn_count).yellow(),
            format!("{}", bad_count).red()
        );
        
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}
