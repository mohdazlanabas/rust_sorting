use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_files::Files;
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::collections::HashSet;
use std::time::Instant;

mod sorts;
use sorts::*;

#[derive(Serialize, Deserialize)]
struct SortResult {
    algorithm: String,
    duration_micros: f64,
}

#[derive(Serialize)]
struct BenchmarkResponse {
    results: Vec<SortResult>,
}

// Generate 10000 unique random numbers between 1 and 1000000
fn generate_unique_random_numbers() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut numbers = HashSet::new();

    while numbers.len() < 10000 {
        numbers.insert(rng.gen_range(1..=1000000));
    }

    numbers.into_iter().collect()
}

// Benchmark a sorting algorithm
fn benchmark_sort<F>(mut data: Vec<i32>, sort_fn: F, name: &str) -> SortResult 
where
    F: FnOnce(&mut Vec<i32>)
{
    let start = Instant::now();
    sort_fn(&mut data);
    let duration = start.elapsed();
    
    SortResult {
        algorithm: name.to_string(),
        duration_micros: duration.as_micros() as f64,
    }
}

#[post("/sort")]
async fn sort_benchmark() -> impl Responder {
    // Generate random numbers
    let numbers = generate_unique_random_numbers();
    
    // Run all sorting algorithms
    let mut results = Vec::new();
    
    results.push(benchmark_sort(numbers.clone(), bubble_sort, "Bubble Sort"));
    results.push(benchmark_sort(numbers.clone(), selection_sort, "Selection Sort"));
    results.push(benchmark_sort(numbers.clone(), insertion_sort, "Insertion Sort"));
    results.push(benchmark_sort(numbers.clone(), merge_sort, "Merge Sort"));
    results.push(benchmark_sort(numbers.clone(), quick_sort, "Quick Sort"));
    results.push(benchmark_sort(numbers.clone(), heap_sort, "Heap Sort"));
    results.push(benchmark_sort(numbers.clone(), tim_sort, "Tim Sort"));
    results.push(benchmark_sort(numbers.clone(), radix_sort, "Radix Sort"));
    results.push(benchmark_sort(numbers.clone(), bucket_sort, "Bucket Sort"));
    results.push(benchmark_sort(numbers.clone(), shell_sort, "Shell Sort"));
    
    HttpResponse::Ok().json(BenchmarkResponse { results })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Sorting Benchmark API Server");
    println!("📡 Server running at: http://localhost:8080");
    println!("🔗 CORS enabled for local development");
    println!("✅ Ready to accept requests at POST /sort\n");

    HttpServer::new(|| {
        // Configure CORS for local development
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(sort_benchmark)
            .service(Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
