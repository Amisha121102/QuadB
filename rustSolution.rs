// 1. Check for Palindrome

fn is_palindrome(str: &str) -> bool {
    let mut reversed = str.to_string();
    reversed.chars().rev().collect::<String>() == str
}

// 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurrence(arr: &[i32], target: i32) -> i32 {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = -1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            result = mid as i32;
            high = mid - 1;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

// 3. Given a string of words, implement a function that returns the shortest word in the string.

use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

fn shortest_word(str: &str) -> String {
    let mut reader = BufReader::new(Cursor::new(str));
    let mut shortest = String::new();
    reader.read_line(&mut shortest).unwrap();

    let mut word = String::new();
    while reader.read_line(&mut word).unwrap() > 0 {
        if word.trim().len() < shortest.trim().len() {
            shortest = word.clone();
        }
        word.clear();
    }

    shortest.trim().to_string()
}


// 4. Check for prime number

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(f64::sqrt(n as f64) as i32) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

// 5. Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median_sorted_array(nums: &Vec<i32>) -> f64 {
    let n = nums.len();
    let mid = n / 2;

    if n % 2 == 0 {
        // If the number of elements is even
        (nums[mid - 1] + nums[mid]) as f64 / 2.0
    } else {
        // If the number of elements is odd
        nums[mid] as f64
    }
}


// 6. Implement a function that finds the longest common prefix of a given set of strings.

use std::cmp::min;

fn find_longest_common_prefix(arr: &Vec<String>) -> String {
    let mut length = std::i32::MAX;
    for s in arr {
        length = min(length, s.len() as i32);
    }

    let mut low = 0;
    let mut high = length - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mut all_equal = true;
        for s in arr {
            if s.get(..mid + 1).unwrap() != arr[0].get(..mid + 1).unwrap() {
                all_equal = false;
                break;
            }
        }
        if all_equal {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    arr[0].get(..low as usize).unwrap().to_string()
}



// 7. Implement a function that returns the kth smallest element in a given array.

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = nums[high];
    let mut i = low as i32 - 1;

    for j in low..high {
        if nums[j] <= pivot {
            i += 1;
            nums.swap(i as usize, j);
        }
    }

    nums.swap((i + 1) as usize, high);
    (i + 1) as usize
}

fn quickselect(nums: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    if low <= high {
        let pivot_index = partition(nums, low, high);

        if pivot_index == k - 1 {
            return nums[pivot_index];
        } else if pivot_index > k - 1 {
            return quickselect(nums, low, pivot_index - 1, k);
        } else {
            return quickselect(nums, pivot_index + 1, high, k);
        }
    }

    // If k is out of range or array is empty, return -1 (or any other appropriate value)
    -1
}

fn find_kth_smallest(nums: &mut [i32], k: usize) -> i32 {
    let n = nums.len();

    // Check if k is within range
    if k <= 0 || k > n {
        return -1; // Return -1 if k is out of range
    }

    quickselect(nums, 0, n - 1, k)
}

// 8. Given a binary tree, implement a function that returns the maximum depth of the tree.

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}


// 9. Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 10. Check if a number is prime

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 11. Merge two sorted arrays

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

// 12. Find the maximum subarray sum

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    
    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }
    
    max_sum
}
