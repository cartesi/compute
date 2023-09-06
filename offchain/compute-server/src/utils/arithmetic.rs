pub const fn max_uint(k: u32) -> i64 {
    assert!(k <= 64);
    let shifting = (1 as u64).checked_shl(k);
    let result: i64 = match shifting {
        Some(sh) => (sh - 1) as i64,
        None => -1,
    };
    result
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

pub fn clz(mut x: u64) -> u32 {
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

pub fn ctz(mut x: u64) -> u32 {
    x &= !x + 1;
    63 - clz(x)
}

pub fn semi_sum(a: u64, b: u64) -> u64 {
    assert!(ulte(a, b));
    a + (b - a) / 2
}
