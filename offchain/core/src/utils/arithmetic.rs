pub const fn max_uint(k: u64) -> u64 {
    assert!(k <= 64);
    (1 << k) - 1
}

pub fn ulte(x: u64, y: u64) -> bool {
    x == y || x < y
}

pub fn ult(x: u64, y: u64) -> bool {
    x < y
}

pub fn is_pow2(x: u64) -> bool {
    (x & (x - 1)) == 0
}

// Returns number of leading zeroes of x
pub fn clz(mut x: u64) -> u64 {
    if x == 0 {
        return 64;
    }
    let mut n = 0;
    if (x & 0xFFFFFFFF00000000) == 0 {
        n += 32;
        x <<= 32;
    }
    if (x & 0xFFFF000000000000) == 0 {
        n += 16;
        x <<= 16;
    }
    if (x & 0xFF00000000000000) == 0 {
        n += 8;
        x <<= 8;
    }
    if (x & 0xF000000000000000) == 0 {
        n += 4;
        x <<= 4;
    }
    if (x & 0xC000000000000000) == 0 {
        n += 2;
        x <<= 2;
    }
    if (x & 0x8000000000000000) == 0 {
        n += 1;
    }
    n
}

// Returns number of trailing zeroes of x
pub fn ctz(mut x: u64) -> u64 {
    x &= !x + 1;
    63 - clz(x)
}

pub fn semi_sum(a: u64, b: u64) -> u64 {
    assert!(ulte(a, b));
    a + (b - a) / 2
}
