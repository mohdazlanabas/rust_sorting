# Sorting Algorithm Benchmark Web App

## Project Background

A web application that benchmarks 10 different sorting algorithms against the same dataset of 10,000 unique random numbers (range: 1-1,000,000). Built to demonstrate the performance characteristics of different sorting approaches with real-time visualization.

## Architecture

### Frontend
- **HTML5** - Clean, semantic structure with Bootstrap 5
- **CSS3** - Custom styling with Bootstrap components
- **Vanilla JavaScript** - No frameworks, pure DOM manipulation and fetch API

### Backend
- **Rust** with **Actix-web** framework
- Modular architecture with each sorting algorithm in its own file
- **Actix-files** for static file serving
- High-performance sorting implementations
- RESTful API with CORS support for local development
- Microsecond-precision timing using `std::time::Instant`
- Comprehensive test suite in `sorts/mod.rs`

### Sorting Algorithms Implemented
1. Bubble Sort - O(n²) comparison-based
2. Selection Sort - O(n²) comparison-based
3. Insertion Sort - O(n²) comparison-based, adaptive
4. Merge Sort - O(n log n) divide-and-conquer
5. Quick Sort - O(n log n) average, partition-based
6. Heap Sort - O(n log n) comparison-based
7. Tim Sort - O(n log n) hybrid merge-insertion
8. Radix Sort - O(nk) non-comparison-based
9. Bucket Sort - O(n+k) distribution-based
10. Shell Sort - O(n log n) gap-based insertion

### Modular Architecture Benefits
- **Isolation**: Each algorithm in its own file for easy maintenance and testing
- **Scalability**: New algorithms can be added without modifying existing code
- **Testability**: Centralized test suite in `mod.rs` validates all algorithms
- **Readability**: Clear file structure makes it easy to locate specific implementations
- **Reusability**: Public exports allow algorithms to be used independently

## Project Structure

```
rust_sorting/
├── README.md
├── frontend/
│   ├── index.html          # Main UI
│   ├── style.css           # Styling
│   └── app.js              # Client-side logic
├── backend/
│   ├── Cargo.toml          # Rust dependencies & build config
│   └── src/
│       ├── main.rs         # Web server, API endpoints & benchmarking
│       └── sorts/
│           ├── mod.rs              # Module exports & test suite
│           ├── bubble_sort.rs      # Bubble sort implementation
│           ├── selection_sort.rs   # Selection sort implementation
│           ├── insertion_sort.rs   # Insertion sort implementation
│           ├── merge_sort.rs       # Merge sort implementation
│           ├── quick_sort.rs       # Quick sort implementation
│           ├── heap_sort.rs        # Heap sort implementation
│           ├── tim_sort.rs         # Tim sort implementation
│           ├── radix_sort.rs       # Radix sort implementation
│           ├── bucket_sort.rs      # Bucket sort implementation
│           └── shell_sort.rs       # Shell sort implementation
└── references/
    └── algorithm-notes.md  # Technical documentation
```

## Prerequisites

- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **Modern Browser** (Chrome, Firefox, Safari, Edge)

## Installation & Deployment

### Backend Setup

```bash
cd backend
cargo build --release
cargo run --release
```

The server will start on `http://localhost:8080`

### Running Tests

```bash
cd backend
cargo test
```

This runs the comprehensive test suite in `sorts/mod.rs` that validates all 10 sorting algorithms.

### Frontend Access

Simply open `frontend/index.html` in your browser, or serve it:

```bash
# Using Python 3
cd frontend
python3 -m http.server 3000

# Or using any static file server
# Then visit http://localhost:3000
```

## Usage

1. Click **"Run Sorting Benchmark"** button
2. Wait while the spinning wheel indicates processing
3. View results showing execution time for each algorithm
4. Click **"Run Again"** to generate new random numbers and re-benchmark

## API Endpoints

### POST `/sort`
Generates 10,000 unique random numbers (range: 1-1,000,000) and sorts them using all 10 algorithms.

**Response:**
```json
{
  "results": [
    {
      "algorithm": "Bubble Sort",
      "duration_micros": 42.5
    },
    ...
  ]
}
```

## Performance Notes

- Times are in microseconds (μs)
- Each algorithm sorts a fresh copy of the same array
- Radix and Bucket sorts are typically fastest for this data range
- Bubble/Selection sorts are slowest but included for educational comparison
- Results vary based on CPU, system load, and random number distribution

## Development

Built with focus on:
- Clean separation of concerns with modular architecture
- Each sorting algorithm isolated in its own module for maintainability
- Type safety with Rust
- Minimal dependencies (actix-web, actix-cors, actix-files, serde, rand)
- Cross-platform compatibility
- Comprehensive test coverage in `sorts/mod.rs`
- Educational value demonstrating algorithm complexity
- Optimized release builds with LTO and codegen-units=1

---

**Author:** Roger  
**Tech Stack:** Rust + Actix-web + Bootstrap 5 + Vanilla JS
