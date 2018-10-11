fn add_to_factors(facs: &mut Vec<(u32, u32)>, f: u32) {
    if let Some(e) = facs.last_mut() {
        if e.0 == f {
            e.1 += 1;
            return;
        }
    }
    facs.push((f, 1));
}

fn find_prime_factors(s: u32) -> Vec<(u32, u32)> {
    let mut facs: Vec<(u32, u32)> = Vec::new();
    let mut d = 2;
    let mut s = s;
    while s > 1 {
        if s % d == 0 {
            add_to_factors(&mut facs, d);
            s /= d;
        } else {
            d += 1;
        }
    }
    facs
}

fn expand_divisors(
    divs: &mut Vec<u32>,
    facs: &[(u32, u32)],
    f: u32) {
    match facs.first() {
        Some((x, c)) => for i in 0..(c+1) {
            expand_divisors(divs, &facs[1..], x.pow(i) * f);
        },
        None => divs.push(f),
    }
}

pub fn find_divisors(s: u32) -> Vec<u32> {
    let facs = find_prime_factors(s);
    let mut divs: Vec<u32> = Vec::new();
    expand_divisors(&mut divs, &facs, 1);
    divs.sort();
    divs
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::*;

    struct Timer {
        st: u64,
    }

    impl Timer {
        fn new() -> Timer {
            Timer { st: time::precise_time_ns() }
        }

        fn elapsed_ns(&self) -> u64 {
            time::precise_time_ns() - self.st
        }

        fn elapsed(&self) -> String {
            let secs = self.elapsed_ns() as f64 / 1e9f64;
            format!("{:.5} secs", secs)
        }
    }

    #[test]
    fn it_works() {
        let data = [
            (1000u32, vec![1u32, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 200, 250, 500, 1000]),
            (13u32, vec![1u32, 13]),
            (1u32, vec![1u32]),
            (25u32, vec![1u32, 5, 25]),
        ];

        for (t, e) in data.iter() {
            assert_eq!(find_divisors(*t), *e, "find_divisors({})", *t);
        }
    }

    #[test]
    fn large_number() {
        let t = Timer::new();
        let ans = find_divisors(1_000_000_000);
        let t = t.elapsed();

        assert_eq!(
            ans,
            vec![1, 2, 4, 5, 8, 10, 16,
                 20, 25, 32, 40, 50, 64,
                 80, 100, 125, 128, 160,
                 200, 250, 256, 320, 400,
                 500, 512, 625, 640, 800,
                 1000, 1250, 1280, 1600,
                 2000, 2500, 2560, 3125,
                 3200, 4000, 5000, 6250,
                 6400, 8000, 10000, 12500,
                 12800, 15625, 16000, 20000,
                 25000, 31250, 32000, 40000,
                 50000, 62500, 64000, 78125,
                 80000, 100000, 125000, 156250,
                 160000, 200000, 250000, 312500,
                 320000, 390625, 400000, 500000,
                 625000, 781250, 800000, 1000000,
                 1250000, 1562500, 1600000, 1953125,
                 2000000, 2500000, 3125000, 3906250,
                 4000000, 5000000, 6250000, 7812500,
                 8000000, 10000000, 12500000, 15625000,
                 20000000, 25000000, 31250000, 40000000,
                 50000000, 62500000, 100000000, 125000000,
                 200000000, 250000000, 500000000, 1000000000]);

        println!("large number took {}", t);
    }
}
