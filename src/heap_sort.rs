/*
- It is simpler to use rust's built-in methods
- It could be written like so:
    - https://www.hackertouch.com/heap-sort-in-rust.html

 */

use std::collections::BinaryHeap;

pub fn heap_sort(list: &[i32]) -> Vec<i32> {
    // let mut heap = create_heap(list);
    // let mut new_list = vec![];
    // while let Some(x) = heap.pop() {
    //     new_list.insert(0, x);
    // }
    // new_list

    create_heap(list).into_sorted_vec()
}

// this could be used, but it's a bit more manual than necessary
pub fn create_heap(list: &[i32]) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::new();
    for x in list {
        heap.push(*x);
    }
    heap
}

// NOTE: this is the in-place version, if needed
// fn create_max_heap(data: &mut Vec<i32>, length: usize, index: usize) {
//     let mut largest = index;
//     let mut left = 2 * index + 1;
//     let mut right = 2 * index + 2;

//     if left < right && data[index] < data[left] {
//         largest = left
//     }

//     if right < length && data[largest] < data[right] {
//         largest = right
//     }

//     if largest != index {
//         (data[index], data[largest]) = (data[largest], data[index]);

//         create_max_heap(data, length, index);
//     }
// }

// NEXT: Create ACTUAL heap with rust
