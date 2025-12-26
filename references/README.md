# Sorting Algorithm Benchmark Web App

## Project Background

A web application that benchmarks 10 different sorting algorithms against the same dataset of 100 unique random numbers (range: 1-1000). Built to demonstrate the performance characteristics of different sorting approaches with real-time visualization.

## Architecture

### Frontend
- **HTML5** - Clean, semantic structure with Bootstrap 5
- **CSS3** - Custom styling with Bootstrap components
- **Vanilla JavaScript** - No frameworks, pure DOM manipulation and fetch API

### Backend
- **Rust** with **Actix-web** framework
- High-performance sorting implementations
- RESTful API with CORS support for local development
- Nanosecond-precision timing using `std::time::Instant`

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

## Project Structure

```
sorting-benchmark/
├── README.md
├── frontend/
│   ├── index.html          # Main UI
│   ├── style.css           # Styling
│   └── app.js              # Client-side logic
├── backend/
│   ├── Cargo.toml          # Rust dependencies
│   └── src/
│       ├── main.rs         # Web server & API
│       └── sorts.rs        # All sorting algorithms
└── reference/
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
Generates 100 unique random numbers and sorts them using all 10 algorithms.

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
- Clean separation of concerns
- Type safety with Rust
- Minimal dependencies
- Cross-platform compatibility
- Educational value demonstrating algorithm complexity

---

**Author:** Roger  
**Tech Stack:** Rust + Actix-web + Bootstrap 5 + Vanilla JS
