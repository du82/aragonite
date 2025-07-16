// Benchmarking utility for search engine performance testing
// Measures query response times and result counts across multiple queries

use crate::search::tantivy_processor;
use crate::filter::FilterManager;
use std::fs::{self, OpenOptions};
use std::sync::{Mutex, Arc};
use std::io::Write;
use std::time::{Duration, Instant, SystemTime};
use std::process::Command;
use std::thread;

pub struct SystemInfo {
    os: String,
    cpu_info: String,
    total_memory_mb: u64,
    initial_memory_usage_mb: u64,
    peak_memory_usage_mb: u64,
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
    memory_increase_mb: u64,
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

    // Get current memory usage
    let current_memory_usage = get_current_memory_usage();

    SystemInfo {
        os,
        cpu_info,
        total_memory_mb,
        initial_memory_usage_mb: current_memory_usage,
        peak_memory_usage_mb: current_memory_usage,
    }
}

// Function to get current memory usage
fn get_current_memory_usage() -> u64 {
    if cfg!(target_os = "linux") {
        // Get process ID
        let pid = std::process::id();
        
        // Try to read memory usage from /proc/{pid}/status
        if let Ok(status) = fs::read_to_string(format!("/proc/{}/status", pid)) {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb / 1024; // Convert KB to MB
                        }
                    }
                }
            }
        }
        
        // Fallback - use ps command
        if let Ok(output) = Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "rss="])
            .output() 
        {
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(kb) = output_str.parse::<u64>() {
                return kb / 1024; // Convert KB to MB
            }
        }
    } else if cfg!(target_os = "macos") {
        let pid = std::process::id();
        if let Ok(output) = Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "rss="])
            .output() 
        {
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(kb) = output_str.parse::<u64>() {
                return kb / 1024; // Convert KB to MB
            }
        }
    }
    
    0 // Default if we couldn't determine
}

pub fn run_search_benchmark() -> Result<(), std::io::Error> {
    println!("Starting search engine benchmark...");

    let system_info = get_system_info();
    println!("System Info:");
    println!("OS: {}", system_info.os);
    println!("CPU Info: {}", system_info.cpu_info);
    println!("Total Memory: {} MB", system_info.total_memory_mb);
    println!("Initial Memory Usage: {} MB", system_info.initial_memory_usage_mb);
    
    // Create shared system info that can be updated during benchmark
    let system_info = Arc::new(Mutex::new(system_info));
    
    // Start memory monitoring in a separate thread
    let system_info_clone = Arc::clone(&system_info);
    let memory_monitor = std::thread::spawn(move || {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(600) { // Monitor for up to 10 minutes
            // Update peak memory usage
            let current_usage = get_current_memory_usage();
            let mut info = system_info_clone.lock().unwrap();
            if current_usage > info.peak_memory_usage_mb {
                info.peak_memory_usage_mb = current_usage;
                println!("New peak memory usage: {} MB", current_usage);
            }
            drop(info);
            
            // Check memory every second
            std::thread::sleep(Duration::from_secs(1));
        }
    });

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

            // Measure memory before query
            let pre_query_memory = get_current_memory_usage();
            
            match tantivy_processor(&query.trim(), &filter_manager) {
                Ok((results, duration)) => {
                    // Measure memory after query
                    let post_query_memory = get_current_memory_usage();
                    let memory_delta = if post_query_memory > pre_query_memory {
                        post_query_memory - pre_query_memory
                    } else {
                        0
                    };
                    
                    metrics.push(BenchmarkMetrics {
                        query: query.clone(),
                        duration_ms: duration.as_millis(),
                        results_count: results.len(),
                        timestamp: SystemTime::now(),
                        thread_id: format!("{:?}", thread::current().id()),
                    });
                    
                    println!("  Memory change: +{} MB", memory_delta);
                }
                Err(e) => {
                    println!("Error processing query '{}': {:?}", query, e);
                }
            }
        }
    }

    // Stop memory monitoring
    memory_monitor.join().unwrap_or(());
    
    let total_queries = metrics.len();
    let total_duration = benchmark_start.elapsed();

    let durations: Vec<u128> = metrics.iter()
        .map(|m| m.duration_ms)
        .collect();

    let total_results: usize = metrics.iter()
        .map(|m| m.results_count)
        .sum();
        
    // Get final system info with peak memory usage
    let system_info_locked = system_info.lock().unwrap();
    let final_system_info = SystemInfo {
        os: system_info_locked.os.clone(),
        cpu_info: system_info_locked.cpu_info.clone(),
        total_memory_mb: system_info_locked.total_memory_mb,
        initial_memory_usage_mb: system_info_locked.initial_memory_usage_mb,
        peak_memory_usage_mb: system_info_locked.peak_memory_usage_mb,
    };
    
    let memory_increase = final_system_info.peak_memory_usage_mb - final_system_info.initial_memory_usage_mb;

    let summary = BenchmarkSummary {
        total_queries,
        avg_duration_ms: durations.iter().sum::<u128>() as f64 / total_queries as f64,
        min_duration_ms: *durations.iter().min().unwrap_or(&0),
        max_duration_ms: *durations.iter().max().unwrap_or(&0),
        total_results,
        avg_results_per_query: total_results as f64 / total_queries as f64,
        total_duration,
        system_info: final_system_info,
        memory_increase_mb: memory_increase,
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
    writeln!(file, "Initial Memory Usage: {} MB", summary.system_info.initial_memory_usage_mb)?;
    writeln!(file, "Peak Memory Usage: {} MB", summary.system_info.peak_memory_usage_mb)?;
    writeln!(file, "Memory Increase: {} MB", summary.memory_increase_mb)?;

    writeln!(file, "\nSummary:")?;
    writeln!(file, "Total Queries: {}", summary.total_queries)?;
    writeln!(file, "Average Duration: {:.2}ms", summary.avg_duration_ms)?;
    writeln!(file, "Min Duration: {}ms", summary.min_duration_ms)?;
    writeln!(file, "Max Duration: {}ms", summary.max_duration_ms)?;
    writeln!(file, "Total Results: {}", summary.total_results)?;
    writeln!(file, "Avg Results Per Query: {:.2}", summary.avg_results_per_query)?;
    writeln!(file, "Total Benchmark Duration: {:?}", summary.total_duration)?;
    writeln!(file, "Memory Performance: {} MB initial → {} MB peak ({}% increase)",
        summary.system_info.initial_memory_usage_mb,
        summary.system_info.peak_memory_usage_mb,
        if summary.system_info.initial_memory_usage_mb > 0 {
            ((summary.memory_increase_mb as f64 / summary.system_info.initial_memory_usage_mb as f64) * 100.0) as u64
        } else {
            0
        }
    )?;

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
    println!("Initial Memory Usage: {} MB", summary.system_info.initial_memory_usage_mb);
    println!("Peak Memory Usage: {} MB", summary.system_info.peak_memory_usage_mb);
    println!("Memory Increase: {} MB", summary.memory_increase_mb);

    println!("\nPerformance Metrics:");
    println!("Total Queries: {}", summary.total_queries);
    println!("Average Duration: {:.2}ms", summary.avg_duration_ms);
    println!("Min Duration: {}ms", summary.min_duration_ms);
    println!("Max Duration: {}ms", summary.max_duration_ms);
    println!("Total Results: {}", summary.total_results);
    println!("Avg Results Per Query: {:.2}", summary.avg_results_per_query);
    println!("Total Benchmark Duration: {:?}", summary.total_duration);
    println!("Memory Performance: {} MB initial → {} MB peak ({}% increase)",
        summary.system_info.initial_memory_usage_mb,
        summary.system_info.peak_memory_usage_mb,
        if summary.system_info.initial_memory_usage_mb > 0 {
            ((summary.memory_increase_mb as f64 / summary.system_info.initial_memory_usage_mb as f64) * 100.0) as u64
        } else {
            0
        }
    );
    println!("\nDetailed results saved to benchmark_results.txt");
}
