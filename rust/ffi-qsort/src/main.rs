
use std::cmp::Ordering;
use std::os::raw::c_void;
use std::mem;

#[cfg(target_os = "macos")]
mod internal {
    use std::os::raw::c_void;
    use std::mem;
    use std::cmp::Ordering;

    extern {
        fn qsort_r(data: *mut c_void,
            num: usize,
            size: usize,
            state: *const c_void,
            comp: extern fn(*const c_void, *const c_void, *const c_void) -> i32);
    }

    extern "C" fn comparator<T>(state: *const c_void, a: *const c_void, b: *const c_void) -> i32 {
        ::compare_with::<T>(state, a, b)
    }

    pub fn sort<T>(data: &mut [T], comp: fn(a: &T, b: &T) -> Ordering) {
        unsafe {
            let state: *const c_void = mem::transmute(comp);
            qsort_r(data.as_mut_ptr() as *mut c_void,
                data.len(),
                mem::size_of::<T>(),
                state,
                comparator::<T>);
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod internal {
    use std::os::raw::c_void;
    use std::mem;
    use std::cmp::Ordering;

    extern {
        fn qsort_r(data: *mut c_void,
            num: usize,
            size: usize,
            comp: extern fn(*const c_void, *const c_void, *const c_void) -> i32,
            state: *const c_void);
    }

    extern "C" fn comparator<T>(a: *const c_void, b: *const c_void, state: *const c_void) -> i32 {
        ::compare_with::<T>(state, a, b)
    }

    pub fn sort<T>(data: &mut [T], comp: fn(a: &T, b: &T) -> Ordering) {
        unsafe {
            let state: *const c_void = mem::transmute(comp);
            qsort_r(data.as_mut_ptr() as *mut c_void,
                data.len(),
                mem::size_of::<T>(),
                comparator::<T>,
                state);
        }
    }
}

fn compare_with<T>(state: *const c_void, a: *const c_void, b: *const c_void) -> i32 {
    unsafe {
        let a: &T = mem::transmute(a);
        let b: &T = mem::transmute(b);
        let f: fn(&T, &T) -> Ordering = mem::transmute(state);
        match f(a, b) {
            Ordering::Equal => 0,
            Ordering::Less => -1,
            Ordering::Greater => 1,
        }
    }
}

pub fn sort<T>(data: &mut [T], comp: fn(a: &T, b: &T) -> Ordering) {
    internal::sort::<T>(data, comp);
}

fn main() {
    let mut data = [5i32, 2, 1, 7];
    sort(&mut data, |a, b| a.cmp(b));
    println!("sorted: {:?}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Thing {
        val: i32,
    }

    #[test]
    fn sorts_ints() {
        let mut data = [1i32, 277, -33, 5, 4, 8, 0, 3, 2, 1, 4];
        sort(&mut data, |a, b| a.cmp(b));
        assert_eq!(data, [-33, 0, 1, 1, 2, 3, 4, 4, 5, 8, 277]);
    }

    #[test]
    fn sorts_strings() {
        let mut data = ["z", "f", "a", "g"];
        sort(&mut data, |a, b| a.cmp(b));
        assert_eq!(data, ["a", "f", "g", "z"]);
    }

    #[test]
    fn sorts_structs() {
        let mut data = [
            Thing {val: 101},
            Thing {val: 900},
            Thing {val: 100},
            Thing {val: 600},
            Thing {val: 102}
        ];
        sort(&mut data, |a, b| a.val.cmp(&b.val));
        let vals: Vec<i32> = data.iter().map(|t| t.val).collect();
        assert_eq!(vals, [100, 101, 102, 600, 900]);
    }

    #[test]
    fn sorts_empty() {
        let mut data: [i32; 0] = [];
        sort(&mut data, |a, b| a.cmp(b));
        assert_eq!(data, []);
    }

    #[test]
    fn sorts_slice_of_vec() {
        let mut data: Vec<u64> = vec![7, 0, 2, 0, 0, 9, 3, 9];
        sort(&mut data[1..], |a, b| a.cmp(b));
        assert_eq!(data, [7, 0, 0, 0, 2, 3, 9, 9]);
    }
}
