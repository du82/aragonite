use crate::search::tantivy_processor;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::time::{Duration, Instant, SystemTime};

pub struct BenchmarkMetrics {
    query: String,
    duration_ms: u128,
    results_count: usize,
    timestamp: SystemTime,
}

pub struct BenchmarkSummary {
    total_queries: usize,
    avg_duration_ms: f64,
    min_duration_ms: u128,
    max_duration_ms: u128,
    total_results: usize,
    avg_results_per_query: f64,
    total_duration: Duration,
}

pub fn run_search_benchmark() -> Result<(), std::io::Error> {
    println!("Starting search engine benchmark...");

    // Load test queries from random.txt
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
        // Add some default test queries
        let mut queries = Vec::new();
        queries.push("privacy".to_string());
        queries.push("security".to_string());
        queries.push("encryption".to_string());
    }

    let mut metrics = Vec::new();
    let benchmark_start = Instant::now();

    // Run benchmarks
    for query in &queries {
        // Skip empty queries or those that are too short
        if query.trim().len() < 1 {
            println!("Skipping empty query");
            continue;
        }

        println!("Testing query: {}", query);

        // Run each query 5 times to get better performance metrics
        for _ in 0..5 {
            match tantivy_processor(&query.trim()) {
                Ok((results, duration)) => {
                    metrics.push(BenchmarkMetrics {
                        query: query.clone(),
                        duration_ms: duration.as_millis(),
                        results_count: results.len(),
                        timestamp: SystemTime::now(),
                    });
                }
                Err(e) => {
                    println!("Error processing query '{}': {:?}", query, e);
                }
            }
        }
    }

    // Calculate summary statistics
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
    };

    // Save results to file
    save_benchmark_results(&metrics, &summary)?;

    // Print summary to console
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

    // Write timestamp
    writeln!(file, "\n=== Benchmark Results - {} ===",
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs())?;

    // Write summary
    writeln!(file, "\nSummary:")?;
    writeln!(file, "Total Queries: {}", summary.total_queries)?;
    writeln!(file, "Average Duration: {:.2}ms", summary.avg_duration_ms)?;
    writeln!(file, "Min Duration: {}ms", summary.min_duration_ms)?;
    writeln!(file, "Max Duration: {}ms", summary.max_duration_ms)?;
    writeln!(file, "Total Results: {}", summary.total_results)?;
    writeln!(file, "Avg Results Per Query: {:.2}", summary.avg_results_per_query)?;
    writeln!(file, "Total Benchmark Duration: {:?}", summary.total_duration)?;

    // Write detailed metrics
    writeln!(file, "\nDetailed Metrics:")?;
    for metric in metrics {
        writeln!(file, "Query: '{}' - Duration: {}ms, Results: {}",
            metric.query,
            metric.duration_ms,
            metric.results_count)?;
    }

    Ok(())
}

fn print_summary(summary: &BenchmarkSummary) {
    println!("\nBenchmark Summary:");
    println!("Total Queries: {}", summary.total_queries);
    println!("Average Duration: {:.2}ms", summary.avg_duration_ms);
    println!("Min Duration: {}ms", summary.min_duration_ms);
    println!("Max Duration: {}ms", summary.max_duration_ms);
    println!("Total Results: {}", summary.total_results);
    println!("Avg Results Per Query: {:.2}", summary.avg_results_per_query);
    println!("Total Benchmark Duration: {:?}", summary.total_duration);
    println!("\nDetailed results saved to benchmark_results.txt");
}
