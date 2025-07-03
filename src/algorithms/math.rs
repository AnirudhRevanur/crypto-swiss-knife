pub fn euclid_algo(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    euclid_algo(b % a, b)
}

pub fn extended_euclid_algo(a: i32, b: i32) -> Option<i32> {
    let (mut r1, mut r2) = (a, b);
    let (mut t1, mut t2) = (1, 0);

    while r2 != 0 {
        let q = r1 / r2;
        let r = r1 % r2;
        r1 = r2;
        r2 = r;

        let t = t1 - q * t2;
        t1 = t2;
        t2 = t;
    }

    if r1 != 1 {
        return None;
    }

    Some((t1 % b + b) % b)
}
