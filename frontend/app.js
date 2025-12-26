// Configuration
const API_URL = 'http://localhost:8080/sort';

// Algorithm complexity mapping
const COMPLEXITY_MAP = {
    'Bubble Sort': 'O(n²)',
    'Selection Sort': 'O(n²)',
    'Insertion Sort': 'O(n²)',
    'Merge Sort': 'O(n log n)',
    'Quick Sort': 'O(n log n)',
    'Heap Sort': 'O(n log n)',
    'Tim Sort': 'O(n log n)',
    'Radix Sort': 'O(nk)',
    'Bucket Sort': 'O(n+k)',
    'Shell Sort': 'O(n log²n)'
};

// DOM Elements
const startBtn = document.getElementById('startBtn');
const restartBtn = document.getElementById('restartBtn');
const startSection = document.getElementById('startSection');
const loadingSection = document.getElementById('loadingSection');
const resultsSection = document.getElementById('resultsSection');
const resultsTableBody = document.getElementById('resultsTableBody');

// Event Listeners
startBtn.addEventListener('click', runBenchmark);
restartBtn.addEventListener('click', resetApp);

// Main function to run benchmark
async function runBenchmark() {
    // Hide start button, show loading spinner
    startSection.style.display = 'none';
    loadingSection.style.display = 'block';
    resultsSection.style.display = 'none';

    try {
        // Call backend API
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        
        // Hide loading spinner, show results
        loadingSection.style.display = 'none';
        displayResults(data.results);
        resultsSection.style.display = 'block';

    } catch (error) {
        console.error('Error running benchmark:', error);
        loadingSection.style.display = 'none';
        alert('Error connecting to backend server. Please ensure the Rust server is running on http://localhost:8080');
        startSection.style.display = 'block';
    }
}

// Display results in table
function displayResults(results) {
    // Clear previous results
    resultsTableBody.innerHTML = '';

    // Sort results by execution time (fastest first)
    const sortedResults = [...results].sort((a, b) => a.duration_micros - b.duration_micros);

    // Populate table
    sortedResults.forEach((result, index) => {
        const row = document.createElement('tr');
        
        // Determine time class based on performance
        const timeClass = getTimeClass(result.duration_micros, sortedResults);
        
        row.innerHTML = `
            <th scope="row" class="row-number">${index + 1}</th>
            <td><strong>${result.algorithm}</strong></td>
            <td><code>${COMPLEXITY_MAP[result.algorithm] || 'N/A'}</code></td>
            <td class="${timeClass}">${result.duration_micros.toFixed(2)} μs</td>
        `;
        
        resultsTableBody.appendChild(row);
    });
}

// Determine CSS class based on relative performance
function getTimeClass(duration, allResults) {
    const times = allResults.map(r => r.duration_micros);
    const min = Math.min(...times);
    const max = Math.max(...times);
    const range = max - min;
    
    if (duration < min + range * 0.33) {
        return 'time-fast';
    } else if (duration < min + range * 0.66) {
        return 'time-medium';
    } else {
        return 'time-slow';
    }
}

// Reset application to initial state
function resetApp() {
    resultsSection.style.display = 'none';
    startSection.style.display = 'block';
    resultsTableBody.innerHTML = '';
}

// Log initialization
console.log('Sorting Benchmark App initialized');
console.log('Backend API URL:', API_URL);
