

fn bubblesort(mut array: Vec<isize>) -> Vec<isize> {
    let mut swapped: bool;

    loop {
        swapped = false;

        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                let temp = array[i];
                array[i] = array[i + 1];
                array[i + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    array
}

pub fn run() {
    let unsorted_array = vec![3, 0, 2, 5, -1, 4, 1];
    let expected_array = vec![-1, 0, 1, 2, 3, 4, 5];

    let sorted_array = bubblesort(unsorted_array);

    assert_eq!(sorted_array, expected_array);
}