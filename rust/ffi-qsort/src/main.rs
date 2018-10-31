fn main() {
    let mut data = [5i32, 2, 1, 7];
    unsafe {
        // qsort(...);
    }
    println!("sorted: {:?}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_qsorts() {
        let mut data = [1i32, 277, -33, 5, 4, 8, 0, 3, 2, 1, 4];
        unsafe {
            // qsort(...);
        }
        assert_eq!(data, [-33, 0, 1, 1, 2, 3, 4, 4, 5, 8, 277]);
    }
}
