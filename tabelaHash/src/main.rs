fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for _ in 0..n {
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
fn main() {
    let mut data = [5, 6, 9, 0, 7, 2, 12];
    bubble_sort(&mut data);
    println!("{:?}", data);
}