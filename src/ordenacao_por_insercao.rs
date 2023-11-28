use std::usize;

pub fn insertion_sort(array: &mut [usize]) {
    for i in 1..array.len()  {
        let mut j = i;

        while j > 0 && array[j] < array[j-1] {
            array.swap(j, j - 1);
            j = j - 1;

        }

    }
    for ele in array {
        println!("{}", ele)
    }
}
