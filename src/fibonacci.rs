pub fn fib_match(n: i32) -> i32 {
    match n {
        0 | 1 => n,
        _ => fib_match(n-2) + fib_match(n-1),
    }
}

pub fn fib_dp_simple(n: i32) -> i32 {
    let mut f1 = 0;
    let mut f2 = 1;
    let mut tmp = 0;
    for _ in 0..n-1 {
        tmp = f1 + f2;
        f1 = f2;
        f2 = tmp;
    }
    tmp
}

const MAX: usize = 1500;
struct FibMemo {
    memo: [usize; MAX],
}

impl FibMemo {
    fn new() -> Self {
        FibMemo { memo: [0; MAX] }
    }

    fn calc(&mut self, n: usize) -> usize {
        if n < 2 { return n; }
        if self.memo[n] != 0 { return self.memo[n]; }
        self.memo[n] = self.calc(n-2) + self.calc(n-1);
        self.memo[n]
    }
}

pub fn fib_memo(n: usize) -> usize {
    let mut f = FibMemo::new();
    f.calc(n)
}

pub fn fib_one(n: i32) -> i32 {
    fn func(a: i32, b: i32, c: i32) -> i32 {
        if c < 2 { return a; }
        func(a+b, a, c-1)
    }
    func(1, 0, n)
}

fn mul(a: &[i32; 4], b: &[i32; 4]) -> [i32; 4] {
    let mut c = [0; 4];
    for i in 0..2 {
        for k in 0..2 {
            for j in 0..2 {
                c[i*2+j] = c[i*2+j] + a[i*2+k] * b[k*2+j];
            }
        }
    }
    c
}

fn pow(mut a: [i32; 4], mut n: i32) -> [i32; 4] {
    let mut b = [1, 0, 0, 1];
    while n > 0 {
        if n & 1 != 0 {
            b = mul(&b, &a);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    b
}

pub fn fib_repeat(n: i32) -> i32 {
    // a[0] = A[0][0]
    // a[1] = A[0][1]
    // a[2] = A[1][0]
    // a[3] = A[1][1]
    let mut a = [1, 1, 1, 0];
    a = pow(a, n);
    a[2]
}

pub fn fib_formulas(n: i32) -> i32 {
    ( ( ( ( 1f64 + 5f64.sqrt() ) / 2f64 ).powi(n) - ( ( 1f64 - 5f64.sqrt() ) / 2f64 ).powi(n) ) / 5f64.sqrt() ).round() as i32
}

#[cfg(test)]
mod tests_fibonacci {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_fib_match() {
        assert_eq!(55, fib_match(10));
    }

    #[test]
    fn test_fib_dp_simple() {
        assert_eq!(55, fib_dp_simple(10));
    }

    #[test]
    fn test_fib_memo() {
        assert_eq!(55, fib_memo(10));
    }

    #[test]
    fn test_fib_one() {
        assert_eq!(55, fib_one(10));
    }

    #[test]
    fn test_fib_formulas() {
        assert_eq!(55, fib_formulas(10));
    }

    #[test]
    fn test_fib_repeat() {
        assert_eq!(55, fib_repeat(10));
    }

    #[bench]
    fn bench_fib_match(b: &mut Bencher) {
        b.iter(|| fib_match(30));
    }

    #[bench]
    fn bench_fib_dp_simple(b: &mut Bencher) {
        b.iter(|| fib_dp_simple(1000));
    }

    #[bench]
    fn bench_fib_memo(b: &mut Bencher) {
        b.iter(|| fib_memo(1000));
    }

    #[bench]
    fn bench_fib_one(b: &mut Bencher) {
        b.iter(|| fib_one(1000));
    }

    #[bench]
    fn bench_fib_repeat(b: &mut Bencher) {
        b.iter(|| fib_repeat(1000));
    }

    #[bench]
    fn bench_fib_formulas(b: &mut Bencher) {
        b.iter(|| fib_formulas(1000));
    }
}
