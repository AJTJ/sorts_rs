fn heap_sort(list: &[i32]) -> &[i32] {
    &[22, 11, 4]
}

// NOTE: this is a JS version
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
