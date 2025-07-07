// Benchmarking utility for search engine performance testing
// Measures query response times and result counts across multiple queries

use crate::search::tantivy_processor;
use crate::filter::FilterManager;
use std::fs::{self, OpenOptions};
use std::sync::Mutex;
use std::io::Write;
use std::time::{Duration, Instant, SystemTime};
use std::process::Command;
use std::thread;

pub struct SystemInfo {
    os: String,
    cpu_info: String,
    total_memory_mb: u64,
}

pub struct BenchmarkMetrics {
    query: String,
    duration_ms: u128,
    results_count: usize,
    timestamp: SystemTime,
    thread_id: String,
}

pub struct BenchmarkSummary {
    total_queries: usize,
    avg_duration_ms: f64,
    min_duration_ms: u128,
    max_duration_ms: u128,
    total_results: usize,
    avg_results_per_query: f64,
    total_duration: Duration,
    system_info: SystemInfo,
}

fn get_system_info() -> SystemInfo {
    let os = std::env::consts::OS.to_string();

    let cpu_info = Command::new("sh")
        .arg("-c")
        .arg("grep 'model name' /proc/cpuinfo | head -1 || sysctl -n machdep.cpu.brand_string || echo 'Unknown CPU'")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown CPU".to_string());

    let total_memory_mb = if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("grep MemTotal /proc/meminfo | awk '{print $2}'")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<u64>()
                    .map(|kb| kb / 1024)
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("sysctl -n hw.memsize | awk '{print $1/1024/1024}'")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<u64>()
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    } else {
        0
    };

    SystemInfo {
        os,
        cpu_info,
        total_memory_mb,
    }
}

pub fn run_search_benchmark() -> Result<(), std::io::Error> {
    println!("Starting search engine benchmark...");

    let system_info = get_system_info();
    println!("System Info:");
    println!("OS: {}", system_info.os);
    println!("CPU Info: {}", system_info.cpu_info);
    println!("Total Memory: {} MB", system_info.total_memory_mb);

    let queries = fs::read_to_string("random.txt")?
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.len() >= 1 && !trimmed.starts_with('-') && !trimmed.starts_with('+')
        })
        .map(String::from)
        .collect::<Vec<String>>();

    if queries.is_empty() {
        println!("No valid queries found in random.txt");
        let mut queries = Vec::new();
        queries.push("privacy".to_string());
        queries.push("security".to_string());
        queries.push("encryption".to_string());
    }

    let mut metrics = Vec::new();
    let benchmark_start = Instant::now();

    // Run benchmarks
    for query in &queries {
        if query.trim().len() < 1 {
            println!("Skipping empty query");
            continue;
        }

        println!("Testing query: {}", query);

        for _ in 0..5 {
            let filter_manager = FilterManager::new();

            match tantivy_processor(&query.trim(), &filter_manager) {
                Ok((results, duration)) => {
                    metrics.push(BenchmarkMetrics {
                        query: query.clone(),
                        duration_ms: duration.as_millis(),
                        results_count: results.len(),
                        timestamp: SystemTime::now(),
                        thread_id: format!("{:?}", thread::current().id()),
                    });
                }
                Err(e) => {
                    println!("Error processing query '{}': {:?}", query, e);
                }
            }
        }
    }

    let total_queries = metrics.len();
    let total_duration = benchmark_start.elapsed();

    let durations: Vec<u128> = metrics.iter()
        .map(|m| m.duration_ms)
        .collect();

    let total_results: usize = metrics.iter()
        .map(|m| m.results_count)
        .sum();

    let summary = BenchmarkSummary {
        total_queries,
        avg_duration_ms: durations.iter().sum::<u128>() as f64 / total_queries as f64,
        min_duration_ms: *durations.iter().min().unwrap_or(&0),
        max_duration_ms: *durations.iter().max().unwrap_or(&0),
        total_results,
        avg_results_per_query: total_results as f64 / total_queries as f64,
        total_duration,
        system_info,
    };

    save_benchmark_results(&metrics, &summary)?;

    print_summary(&summary);

    Ok(())
}

fn save_benchmark_results(
    metrics: &[BenchmarkMetrics],
    summary: &BenchmarkSummary,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("benchmark_results.txt")?;

    writeln!(file, "\n=== Benchmark Results - {} ===",
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs())?;

    writeln!(file, "\nServer Information:")?;
    writeln!(file, "OS: {}", summary.system_info.os)?;
    writeln!(file, "CPU Info: {}", summary.system_info.cpu_info)?;
    writeln!(file, "Total Memory: {} MB", summary.system_info.total_memory_mb)?;

    writeln!(file, "\nSummary:")?;
    writeln!(file, "Total Queries: {}", summary.total_queries)?;
    writeln!(file, "Average Duration: {:.2}ms", summary.avg_duration_ms)?;
    writeln!(file, "Min Duration: {}ms", summary.min_duration_ms)?;
    writeln!(file, "Max Duration: {}ms", summary.max_duration_ms)?;
    writeln!(file, "Total Results: {}", summary.total_results)?;
    writeln!(file, "Avg Results Per Query: {:.2}", summary.avg_results_per_query)?;
    writeln!(file, "Total Benchmark Duration: {:?}", summary.total_duration)?;

    writeln!(file, "\nDetailed Metrics:")?;
    for metric in metrics {
        writeln!(file, "Query: '{}' - Duration: {}ms, Results: {}, Thread: {}",
            metric.query,
            metric.duration_ms,
            metric.results_count,
            metric.thread_id)?;
    }

    Ok(())
}

fn print_summary(summary: &BenchmarkSummary) {
    println!("\nBenchmark Summary:");
    println!("Server Information:");
    println!("OS: {}", summary.system_info.os);
    println!("CPU Info: {}", summary.system_info.cpu_info);
    println!("Total Memory: {} MB", summary.system_info.total_memory_mb);

    println!("\nPerformance Metrics:");
    println!("Total Queries: {}", summary.total_queries);
    println!("Average Duration: {:.2}ms", summary.avg_duration_ms);
    println!("Min Duration: {}ms", summary.min_duration_ms);
    println!("Max Duration: {}ms", summary.max_duration_ms);
    println!("Total Results: {}", summary.total_results);
    println!("Avg Results Per Query: {:.2}", summary.avg_results_per_query);
    println!("Total Benchmark Duration: {:?}", summary.total_duration);
    println!("\nDetailed results saved to benchmark_results.txt");
}
