pub fn sort(vals: &mut Vec<u8>) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate time;

    use ::sort;
    use self::rand::Rng;
    use ::std::collections::HashMap;

    fn create_rand_vec(n: usize) -> (Vec<u8>, u64) {
        let st = time::precise_time_ns();
        let mut v: Vec<u8> = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            v.push(rng.gen());
        }
        (v, time::precise_time_ns() - st)
    }

    fn is_sorted(vals: &Vec<u8>) -> bool {
        if vals.is_empty() {
            return true;
        }

        for i in 1..vals.len() {
            if vals[i-1] > vals[i] {
                return false;
            }
        }

        true
    }

    fn build_idx(vals: &Vec<u8>) -> HashMap<u8, u32> {
        let mut idx: HashMap<u8, u32> = HashMap::new();
        for val in vals.iter() {
            idx.entry(*val)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(0);
        }
        idx
    }

    fn indexes_are_same(a: &HashMap<u8, u32>, b: &HashMap<u8, u32>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for (va, ca) in a {
            match b.get(va) {
                Some(cb) => {
                    if cb != ca {
                        return false;
                    }
                },
                None => {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn basic() {
        let mut a = vec![10u8, 10, 1, 56, 10, 56];
        sort(&mut a);
        assert_eq!(a, vec![1u8, 10, 10, 10, 56, 56]);
    }

    #[test]
    fn empty() {
        let mut a: Vec<u8> = Vec::new();
        sort(&mut a);
        assert_eq!(a, Vec::new());
    }

    #[test]
    fn one_billion() {
        let (mut v, t) = create_rand_vec(1_000_000_000);

        let idx = build_idx(&v);

        let start_time = time::precise_time_ns();
        sort(&mut v);
        let elapsed = time::precise_time_ns() - start_time;

        // must be sorted
        assert!(is_sorted(&v), "not sorted");

        // must be the same contents
        assert!(indexes_are_same(&idx, &build_idx(&v)), "contents of v were changed");

        // must be fast enough
        assert!(elapsed < t, "not efficient");
    }
}
