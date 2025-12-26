# Sorting Algorithms Technical Reference

## Overview

This document provides detailed technical information about the 10 sorting algorithms implemented in the benchmark application.

---

## 1. Bubble Sort

**Time Complexity:** O(n²)  
**Space Complexity:** O(1)  
**Stable:** Yes

### Description
Bubble sort repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. The algorithm gets its name because smaller elements "bubble" to the top of the list.

### Characteristics
- Simple implementation
- Poor performance on large datasets
- Best case: O(n) when array is already sorted
- Worst case: O(n²) when array is reverse sorted

### Use Cases
- Educational purposes
- Small datasets (< 10 elements)
- Nearly sorted data

---

## 2. Selection Sort

**Time Complexity:** O(n²)  
**Space Complexity:** O(1)  
**Stable:** No

### Description
Selection sort divides the array into sorted and unsorted regions. It repeatedly finds the minimum element from the unsorted region and moves it to the end of the sorted region.

### Characteristics
- Minimizes number of swaps (O(n) swaps)
- Always performs O(n²) comparisons
- Not adaptive to input data

### Use Cases
- When memory write is expensive
- Small datasets
- When swap operations are costly

---

## 3. Insertion Sort

**Time Complexity:** O(n²)  
**Space Complexity:** O(1)  
**Stable:** Yes

### Description
Insertion sort builds the final sorted array one item at a time. It picks an element and inserts it into its correct position in the sorted portion of the array.

### Characteristics
- Efficient for small datasets
- Adaptive: O(n) for nearly sorted data
- Online algorithm: can sort as it receives data
- Used in Tim Sort for small subarrays

### Use Cases
- Small datasets (< 50 elements)
- Nearly sorted data
- Part of hybrid algorithms (Tim Sort)

---

## 4. Merge Sort

**Time Complexity:** O(n log n)  
**Space Complexity:** O(n)  
**Stable:** Yes

### Description
Merge sort is a divide-and-conquer algorithm that divides the array into halves, recursively sorts them, and merges the sorted halves.

### Characteristics
- Guaranteed O(n log n) performance
- Requires additional O(n) space
- Parallelizable
- Not in-place

### Use Cases
- Large datasets requiring guaranteed O(n log n)
- External sorting (sorting data larger than RAM)
- Linked lists
- When stability is required

---

## 5. Quick Sort

**Time Complexity:** O(n log n) average, O(n²) worst  
**Space Complexity:** O(log n)  
**Stable:** No

### Description
Quick sort picks a pivot element and partitions the array around it such that elements smaller than pivot come before it and elements larger come after it. Then recursively sorts the partitions.

### Characteristics
- Very fast in practice
- In-place algorithm
- Cache-friendly
- Worst case O(n²) can be avoided with good pivot selection

### Use Cases
- General-purpose sorting
- When average case performance matters
- Systems programming
- Default in many standard libraries

---

## 6. Heap Sort

**Time Complexity:** O(n log n)  
**Space Complexity:** O(1)  
**Stable:** No

### Description
Heap sort builds a max heap from the data, then repeatedly extracts the maximum element and rebuilds the heap.

### Characteristics
- Guaranteed O(n log n) performance
- In-place algorithm
- Not stable
- Not as cache-friendly as quick sort

### Use Cases
- When guaranteed O(n log n) and O(1) space is required
- Real-time systems requiring predictable performance
- Priority queue implementation

---

## 7. Tim Sort

**Time Complexity:** O(n log n)  
**Space Complexity:** O(n)  
**Stable:** Yes

### Description
Tim sort is a hybrid stable sorting algorithm derived from merge sort and insertion sort. It divides the array into small chunks (runs), sorts them using insertion sort, and merges them using merge sort.

### Characteristics
- Highly optimized for real-world data
- Takes advantage of existing order in data
- Best case: O(n) for already sorted data
- Used in Python and Java standard libraries

### Use Cases
- General-purpose sorting in production
- Real-world data with patterns
- When stability is required
- Default sort in many modern languages

---

## 8. Radix Sort

**Time Complexity:** O(nk) where k is number of digits  
**Space Complexity:** O(n + k)  
**Stable:** Yes

### Description
Radix sort is a non-comparison sorting algorithm that sorts integers by processing individual digits. It sorts from least significant digit to most significant digit.

### Characteristics
- Linear time complexity for fixed-length integers
- Not comparison-based
- Requires counting sort as subroutine
- Works best with integers

### Use Cases
- Sorting integers with limited range
- Large datasets of fixed-length keys
- When comparison cost is high
- Parallel processing applications

---

## 9. Bucket Sort

**Time Complexity:** O(n + k) average  
**Space Complexity:** O(n + k)  
**Stable:** Yes

### Description
Bucket sort distributes elements into buckets, sorts each bucket individually (using another sorting algorithm), and concatenates the sorted buckets.

### Characteristics
- Efficient when input is uniformly distributed
- Not comparison-based
- Requires knowledge of input distribution
- Can be very fast for appropriate data

### Use Cases
- Uniformly distributed data
- Floating-point numbers in known range
- External sorting with limited RAM
- When input distribution is known

---

## 10. Shell Sort

**Time Complexity:** O(n log²n) to O(n^(3/2))  
**Space Complexity:** O(1)  
**Stable:** No

### Description
Shell sort is a generalization of insertion sort that allows the exchange of items that are far apart. The algorithm sorts elements at specific intervals (gaps), progressively reducing the gap.

### Characteristics
- Better than O(n²) algorithms
- In-place algorithm
- Gap sequence affects performance
- Relatively simple to implement

### Use Cases
- Medium-sized datasets
- Embedded systems with memory constraints
- When simplicity and reasonable performance are needed
- Legacy systems

---

## Performance Comparison

### For our benchmark (100 random numbers, range 1-1000):

**Expected Rankings (fastest to slowest):**

1. **Radix Sort / Bucket Sort** - O(n) behavior for this range
2. **Tim Sort** - Highly optimized hybrid
3. **Quick Sort** - Excellent average case
4. **Heap Sort** - Guaranteed O(n log n)
5. **Merge Sort** - Stable O(n log n)
6. **Shell Sort** - Sub-quadratic
7. **Insertion Sort** - O(n²) but efficient for small n
8. **Selection Sort** - Always O(n²)
9. **Bubble Sort** - O(n²) with overhead

**Note:** Actual performance varies based on:
- CPU architecture and cache
- Random number distribution
- Compiler optimizations
- System load

---

## Algorithm Selection Guide

| Requirement | Algorithm |
|------------|-----------|
| Guaranteed O(n log n) + O(1) space | Heap Sort |
| Guaranteed O(n log n) + stability | Merge Sort |
| Best average performance | Quick Sort |
| Small dataset (< 50 items) | Insertion Sort |
| Production general-purpose | Tim Sort |
| Integer sorting | Radix Sort |
| Uniform distribution | Bucket Sort |
| Memory constrained | Shell Sort / Heap Sort |
| Educational | Bubble Sort |

---

## Implementation Notes

All algorithms are implemented in Rust with:
- Zero-cost abstractions
- Memory safety guarantees
- No runtime overhead
- Compile-time optimizations

The benchmark uses `std::time::Instant` for microsecond-precision timing, ensuring accurate performance measurements across all algorithms.

---

**References:**
- Knuth, D. E. (1998). The Art of Computer Programming, Volume 3: Sorting and Searching
- Cormen, T. H., et al. (2009). Introduction to Algorithms (3rd ed.)
- Rust Standard Library Documentation

**Last Updated:** December 2024
