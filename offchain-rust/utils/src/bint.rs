/*use std::ops::{Mul, Neg, Not};

const BINT_SIZE: usize = 4;
const BINT_BITS: i64 = 64;
const BINT_WORDBITS: i64 = 32;
const BINT_WORDMAX: u64 = u64::MAX;
const BINT_WORDMSB: u64 = 0x8000_0000;
const BINT_MININTEGER: u64 = Bint::bint_mininteger();
#[derive(Debug, Clone, PartialEq, Default)]
struct Bint {
    value: [u64; BINT_SIZE],
}

impl Bint {
    fn zero() -> Self {
        Self {
            value: [0; BINT_SIZE],
        }
    }

    fn one() -> Self {
        let mut value = [0; BINT_SIZE];
        value[0] = 1;
        Self { value }
    }

    fn is_zero(&self) -> bool {
        self.value.iter().all(|&x| x == 0)
    }

    fn inc(&mut self) -> Result<(), &'static str> {
        let mut carry = 1;
        for i in 0..BINT_SIZE {
            let (result, overflow) = self.value[i].overflowing_add(carry);
            self.value[i] = result;
            if !overflow {
                break;
            }
            carry = 1;
        }
        if carry != 0 {
            Err("increment overflow")
        } else {
            Ok(())
        }
    }

    fn shl(&self, shift: i64) -> Result<Self, &'static str> {
        if shift == std::i64::MIN || shift.abs() >= BINT_BITS {
            return Ok(Self::zero());
        }
        if shift < 0 {
            return self.shr(-shift);
        }
        let nvals = shift / BINT_WORDBITS;
        let mut result = self.shl_words(nvals)?;
        let mut shift = shift - nvals * BINT_WORDBITS;
        if shift != 0 {
            let wordbitsmy = BINT_WORDBITS - shift;
            for i in 0..BINT_SIZE - 1 {
                result.value[i] = ((result.value[i] << shift)
                    | (result.value[i + 1] >> wordbitsmy))
                    & BINT_WORDMAX;
            }
            result.value[BINT_SIZE - 1] = (result.value[BINT_SIZE - 1] << shift) & BINT_WORDMAX;
        }
        Ok(result)
    }

    fn shr(&self, shift: i64) -> Result<Self, &'static str> {
        if shift == std::i64::MIN || shift.abs() >= BINT_BITS {
            return Ok(Self::zero());
        }
        if shift < 0 {
            return self.shl(-shift);
        }
        let nvals = shift / BINT_WORDBITS;
        let mut result = self.shr_words(nvals)?;
        let mut shift = shift - nvals * BINT_WORDBITS;
        if shift != 0 {
            let wordbitsmy = BINT_WORDBITS - shift;
            for i in 0..BINT_SIZE - 1 {
                result.value[i] = ((result.value[i] >> shift)
                    | (result.value[i + 1] << wordbitsmy))
                    & BINT_WORDMAX;
            }
            result.value[BINT_SIZE - 1] = result.value[BINT_SIZE - 1] >> shift;
        }
        Ok(result)
    }

    fn shl_words(&self, nvals: i64) -> Result<Self, &'static str> {
        let mut result = Self::zero();
        if nvals > BINT_SIZE as i64 {
            return Ok(result);
        }
        for i in nvals as usize..BINT_SIZE {
            result.value[i] = self.value[i - nvals as usize];
        }
        Ok(result)
    }

    fn shr_words(&self, nvals: i64) -> Result<Self, &'static str> {
        let mut result = Self::zero();
        if nvals > BINT_SIZE as i64 {
            return Ok(result);
        }
        for i in 0..(BINT_SIZE - nvals as usize) {
            result.value[i] = self.value[i + nvals as usize];
        }
        Ok(result)
    }

    fn bint_mininteger() -> Bint {
        let mut x = Bint::default();
        for i in 0..BINT_SIZE - 1 {
            x.value[i] = 0;
        }
        x.value[BINT_SIZE - 1] = BINT_WORDMSB;
        x
    }

    fn isneg(x: &Bint) -> bool {
        if let Some(x) = tobint(x) {
            return x.value[BINT_SIZE - 1] & BINT_WORDMSB != 0;
        }
        if let Some(x) = bint_tonumber(x) {
            return x < 0;
        }
        false
    }

    fn is_one(x: &Bint) -> bool {
        if let Some(x) = tobint(x) {
            if x.value[0] != 1 {
                return false;
            }
            for i in 1..BINT_SIZE {
                if x.value[i] != 0 {
                    return false;
                }
            }
            return true;
        }
        if let Some(x) = bint_tonumber(x) {
            return x == 1;
        }
        false
    }

    fn is_odd(x: &Bint) -> bool {
        if let Some(x) = tobint(x) {
            return x.value[0] & 1 == 1;
        }
        if let Some(x) = bint_tonumber(x) {
            return x.abs() % 2 == 1;
        }
        false
    }

    fn shrone(&mut self) {
        let wordbitsm1 = BINT_WORDBITS - 1;
        for i in 0..(BINT_SIZE - 1) {
            self.value[i] = ((self.value[i] >> 1) | (self.value[i + 1] << wordbitsm1)) & BINT_WORDMAX;
        }
        self.value[BINT_SIZE - 1] >>= 1;
    }

    fn dec(&mut self) {
        for i in 0..BINT_SIZE {
            let tmp = self.value[i];
            let v = (tmp - 1) & BINT_WORDMAX;
            self.value[i] = v;
            if !(v > tmp) {
                break;
            }
        }
    }

    fn tointeger(&self) -> Option<i64> {
        if self.is_bint() {
            let mut n: i64 = 0;
            let neg = self.is_neg();
            let mut x = self.clone();
            if neg {
                x = -x;
            }
            for i in 1..=BINT_SIZE {
                n |= (x[i] << (BINT_WORDBITS * (i - 1))) as i64;
            }
            if neg {
                n = -n;
            }
            Some(n)
        } else {
            to_integer(x)
        }
    }

    fn unm(&mut self) -> &mut Self {
        self.bnot().inc();
        self
    }

    fn abs(&mut self) {
        if self.isneg() {
            self._unm();
        }
    }

    fn is_even(x: &Bint) -> bool {
        if let Some(bint) = x.downcast_ref::<Bint>() {
            bint.value[0] & 1 == 0
        } else if let Some(num) = x.downcast_ref::<i64>() {
            num.abs() % 2 == 0
        } else {
            false
        }
    }

    fn tobase(x: &Bint, base: u32, unsigned: Option<bool>) -> Option<String> {
        if let Some(bint) = x.downcast_ref::<Bint>() {
            if unsigned.is_none() {
                unsigned = Some(base != 10);
            }
            let is_x_neg = Bint:isneg(bint);
            if (base == 10 && unsigned.unwrap()) || (base == 16 && unsigned.unwrap() && !is_x_neg) {
                if bint <= BINT_MATHMAXINTEGER && bint >= BINT_MATHMININTEGER {
                    let n = bint.tointeger().unwrap();
                    if base == 10 {
                        return Some(n.to_string());
                    } else if unsigned.unwrap() {
                        return Some(format!("{:x}", n));
                    }
                }
            }
            let mut ss = Vec::new();
            let neg = !unsigned.unwrap_or(base == 10) && is_x_neg;
            let mut x = if neg { 
                bint.abs();
                bint 
            } else {
                &bint_new(x).unwrap() 
            };
            let x_is_zero = x.is_zero();
            if x_is_zero {
                return Some("0".to_string());
            }
            let step = get_base_step(base);
            let basepow = ipow(1, base as u64, step.try_into().unwrap());
            let mut size = BINT_SIZE;
            let mut xd;
            let mut carry;
            let mut d;
            loop {
                carry = 0;
                let mut x_is_zero = true;
                for i in (0..size).rev() {
                    carry |= x.value[i];
                    d = carry / basepow;
                    xd = carry % basepow
                    if x_is_zero && d != 0 {
                        size = i;
                        x_is_zero = false;
                    }
                    x.value[i] = d;
                    carry = xd << BINT_WORDBITS;
                }
                for _ in 0..step {
                    let div = xd.divmod(base);
                    xd = div.0;
                    d = div.1;
                    if x_is_zero && xd == 0 && d == 0 {
                        break;
                    }
                    ss.insert(0, BASE_LETTERS[d as usize]);
                }
                if x_is_zero {
                    break;
                }
            }
            if neg {
                ss.insert(0, '-');
            }
            Some(ss.iter().collect())
        } else {
            None
        }
    }
  
}

impl Not for Bint {
    type Output = Bint;

    fn not(self) -> Bint {
        let mut y = Bint{value:[0; BINT_SIZE]};
        for i in 0..BINT_SIZE {
            y.0[i] = !self.0[i] & BINT_WORDMAX;
        }
        y
    }
}

impl Mul for Bint {
    type Output = Bint;

    fn mul(self, other: Bint) -> Bint {
        let ix = tobint(&self);
        let iy = tobint(&other);

        if let (Some(ix), Some(iy)) = (ix, iy) {
            let mut z = Bint::zero();
            let sizep1 = BINT_SIZE + 1;
            let mut s = sizep1;
            let mut e = 0;

            for i in 1..=BINT_SIZE {
                if ix[i] != 0 || iy[i] != 0 {
                    e = e.max(i);
                    s = s.min(i);
                }
            }

            for i in s..=e {
                for j in s..=(sizep1 - i).min(e) {
                    let a = ix[i] * iy[j];

                    if a != 0 {
                        let mut carry = 0;

                        for k in (i + j - 1)..=BINT_SIZE {
                            let tmp = z[k] + (a & BINT_WORDMAX) + carry;
                            carry = tmp >> BINT_WORDBITS;
                            z[k] = tmp & BINT_WORDMAX;
                            a >>= BINT_WORDBITS;
                        }
                    }
                }
            }

            z
        } else {
            Bint::from(bint_tonumber(&self) * bint_tonumber(&other))
        }
    }
}

impl Neg for Bint {
    type Output = Bint;

    fn neg(self) -> Bint {
        !self.inc()
    }
}

fn to_integer(x: f64) -> Option<i64> {
    let mut x = x;
    let ty = x.classify();
    if ty == FpCategory::Normal || ty == FpCategory::Zero {
        let floor_x = x.floor();
        if floor_x == x {
            x = floor_x;
        }
    }
    if x.is_finite() && x.fract() == 0.0 {
        Some(x as i64)
    } else {
        None
    }
}

fn tobint(x: &Bint) -> Option<[u64; BINT_SIZE]> {
    x.downcast_ref::<Bint>().map(|b| b.value)
}

fn is_bint(x: &Bint) -> bool {
    x.is::<Bint>()
}


fn bint_assert_convert(x: &Bint) -> Result<Bint, &'static str> {
    if let Some(b) = tobint(x) {
        Ok(Bint { value: b })
    } else {
        Err("invalid conversion to bint")
    }
}

fn bint_assert_tointeger(x: &Bint) -> Result<i64, &'static str> {
    if let Some(b) = x.downcast_ref::<i64>() {
        Ok(*b)
    } else {
        Err("invalid conversion to integer")
    }
}

fn bint_new(x: &Bint) -> Result<Bint, &'static str> {
    if let Some(b) = tobint(x) {
        Ok(Bint { value: b })
    } else if let Some(n) = x.downcast_ref::<i64>() {
        Ok(Bint::new(*n))
    } else {
        Err("invalid conversion to bint")
    }
}

fn bint_tonumber(x: &Bint) -> f64 {
    if let Some(b) = tobint(x) {
        let mut result: u64 = 0;
        for i in (0..BINT_SIZE).rev() {
            result = result.wrapping_mul(2u64.pow(32));
            result = result.wrapping_add(b[i]);
        }
        if b[BINT_SIZE - 1] & BINT_WORDMSB != 0 {
            result as i64 as f64
        } else {
            result as f64
        }
    } else if let Some(n) = x.downcast_ref::<i64>() {
        *n as f64
    } else {
        0.0
    }
}

fn bint_div(x: &Bint, y: &Bint) -> f64 {
    bint_tonumber(x) / bint_tonumber(y)
}

fn bint_mod(x: &Bint, y: &Bint) -> f64 {
    let nx = bint_tonumber(x);
    let ny = bint_tonumber(y);
    nx % ny
}

fn bint_eq(x: &Bint, y: &Bint) -> bool {
    bint_tonumber(x) == bint_tonumber(y)
}

fn bint_ne(x: &Bint, y: &Bint) -> bool {
    bint_tonumber(x) != bint_tonumber(y)
}

fn bint_abs(x: &Bint) -> Bint {
    let mut result = bint_new(x).unwrap();
    result.value[BINT_SIZE - 1] &= !BINT_WORDMSB;
    result
}

fn udivmod(x: &Bint, y: &Bint) -> (Bint, Bint) {
    let mut nume = bint_new(x);
    let deno = bint_assert_convert(y);

    let mut ishighzero = true;
    for i in 2..BINT_SIZE {
        if deno[i] != 0 {
            ishighzero = false;
            break;
        }
    }

    if ishighzero {
        let low = deno[1];
        assert!(low != 0, "attempt to divide by zero");
        if low == 1 {
            return (nume, bint_zero());
        } else if low <= (BINT_WORDMSB - 1) {
            let rema = sudivmod(&mut nume, low);
            return (nume.unwrap(), bint_fromuinteger(rema));
        }
    }

    if nume.ult(&deno) {
        return (bint_zero(), nume);
    }

    let denolbit = findleftbit(&deno);
    let (numelbit, numesize) = findleftbit(&nume);
    let bit = numelbit - denolbit;
    let mut deno = deno << bit;
    let wordmaxp1 = BINT_WORDMAX + 1;
    let wordbitsm1 = BINT_WORDBITS - 1;
    let mut denosize = numesize;
    let mut quot = bint_zero();

    while bit >= 0 {
        let mut le = true;
        let size = numesize.max(denosize);
        for i in (1..=size).rev() {
            let a = deno[i];
            let b = nume[i];
            if a != b {
                le = a < b;
                break;
            }
        }

        if le {
            let mut borrow = 0;
            for i in 1..=size {
                let res = nume[i] + wordmaxp1 - deno[i] - borrow;
                nume[i] = res & BINT_WORDMAX;
                borrow = (res >> BINT_WORDBITS) ^ 1;
            }

            let i = (bit / BINT_WORDBITS) + 1;
            quot[i] |= 1 << (bit % BINT_WORDBITS);
        }

        for i in 1..denosize {
            deno[i] = ((deno[i] >> 1) | (deno[i + 1] << wordbitsm1)) & BINT_WORDMAX;
        }
        let lastdenoword = deno[denosize] >> 1;
        deno[denosize] = lastdenoword;

        if lastdenoword == 0 {
            while deno[denosize] == 0 {
                denosize -= 1;
            }
            if denosize == 0 {
                break;
            }
        }

        bit -= 1;
    }

    (quot, nume)
}


fn bint_umod(x: &Bint, y: &Bint) -> Bint {
    let (_, rema) = bint_udivmod(x, y);
    rema
}

fn bint_tdivmod(x: &Bint, y: &Bint) -> (Bint, Bint) {
    let ax = bint_abs(x);
    let ay = bint_abs(y);
    let mut quot = Bint::zero();
    let mut rema = ax.clone();
    if bint_eq(x, &BINT_MININTEGER) && bint_eq(y, &bint_minus_one()) {
        panic!("division overflow");
    }
    while !rema.is_zero() && bint_ult(&ay, &rema) {
        quot.inc().unwrap();
        rema -= ay.clone();
    }
    let isxneg = x.is_neg();
    let isyneg = y.is_neg();
    if isxneg != isyneg {
        quot = -quot;
    }
    if isxneg {
        rema = -rema;
    }
    (quot, rema)
}

fn bint_tdiv(x: &Bint, y: &Bint) -> Bint {
    let (quot, _) = bint_tdivmod(x, y);
    quot
}

fn bint_tmod(x: &Bint, y: &Bint) -> Bint {
    let (_, rema) = bint_tdivmod(x, y);
    rema
}

fn bint_ipow(x: &Bint, y: &Bint) -> Bint {
    let y = bint_assert_convert(y).unwrap();
    if y.is_zero() {
        return Bint::one();
    } else if y == Bint::one() {
        return bint_new(x).unwrap();
    }
    let mut x = bint_new(x).unwrap();
    let mut y = y.clone();
    let mut z = Bint::one();
    while Bint::is_one(&!y) {
        if Bint::is_even(&y) {
            x = x.clone() * x.clone();
            y.shrone();
        } else {
            z = x.clone() * z.clone();
            x = x.clone() * x.clone();
            y.dec();
            y.shrone();
        }
    }
    x = x * z;
    x
}
fn bint_upowmod(x: &Bint, y: &Bint, m: &Bint) -> Bint {
    let m = bint_assert_convert(m).unwrap();
    if Bint::is_one(&m) {
        return Bint::zero();
    }
    let mut x = bint_new(x).unwrap();
    let mut y = bint_new(y).unwrap();
    let mut z = Bint::one();
    x = bint_umod(&x, &m)
    while !y.is_zero() {
        if Bint::is_odd(&y) {
            z = bint_umod(&(z*x), &m)
        }
        y.shrone();
        x = bint_umod(&(x*x), &m)
    }
    z
}

fn bint_umod(x: &Bint, y: &Bint) -> Bint {
    let (_, rema) = bint_udivmod(x, y);
    rema
}

fn get_base_step(base: u32) -> usize {
    let mut step = 0;
    let mut dmax = 1;
    let limit = std::i64::MAX / base as i64;
    loop {
        step += 1;
        dmax *= base;
        if i64::from(dmax) >= limit {
            break;
        }
    }
    step
}

fn bint_shl(x: &Bint, y: &Bint) -> Result<Bint, &'static str> {
    let x = bint_new(x)?;
    let y = bint_assert_tointeger(y)?;
    if y == std::i64::MIN || y.abs() >= BINT_BITS {
        Ok(Bint::zero())
    } else if y < 0 {
        Ok(x.shr(-y)?)
    } else {
        let nvals = y / BINT_WORDBITS;
        if nvals != 0 {
            Ok(x.shl_words(nvals)?.shl(y - nvals * BINT_WORDBITS)?)
        } else {
            Ok(x.shl(y)?)
        }
    }
}

fn bint_band(x: &Bint, y: &Bint) -> Result<Bint, &'static str> {
    let x = bint_new(x)?;
    let mut y = bint_assert_convert(y)?;
    for i in 0..BINT_SIZE {
        y.value[i] &= x.value[i];
    }
    Ok(y)
}

fn bint_bor(x: &Bint, y: &Bint) -> Result<Bint, &'static str> {
    let x = bint_new(x)?;
    let mut y = bint_assert_convert(y)?;
    for i in 0..BINT_SIZE {
        y.value[i] |= x.value[i];
    }
    Ok(y)
}

fn bint_bxor(x: &Bint, y: &Bint) -> Result<Bint, &'static str> {
    let x = bint_new(x)?;
    let mut y = bint_assert_convert(y)?;
    for i in 0..BINT_SIZE {
        y.value[i] ^= x.value[i];
    }
    Ok(y)
}

fn bint_bnot(x: &Bint) -> Result<Bint, &'static str> {
    let mut y = bint_assert_convert(x)?;
    for i in 0..BINT_SIZE {
        y.value[i] = (!y.value[i]) & BINT_WORDMAX;
    }
    Ok(y)
}

fn bint_unm(x: &Bint) -> Result<Bint, &'static str> {
    let mut x = bint_assert_convert(x)?;
    x = !x;
    x.inc();
    Ok(x)
}

fn bint_ult(x: &Bint, y: &Bint) -> bool {
    let x = bint_assert_convert(x).unwrap();
    let y = bint_assert_convert(y).unwrap();
    for i in (0..BINT_SIZE).rev() {
        if x.value[i] != y.value[i] {
            return x.value[i] < y.value[i];
        }
    }
    false
}

fn bint_ule(x: &Bint, y: &Bint) -> bool {
    let x = bint_assert_convert(x).unwrap();
    let y = bint_assert_convert(y).unwrap();
    for i in (0..BINT_SIZE).rev() {
        if x.value[i] != y.value[i] {
            return x.value[i] < y.value[i];
        }
    }
    true
}

fn bint_lt(x: &Bint, y: &Bint) -> bool {
    let ix = tobint(x);
    let iy = tobint(y);
    if let (Some(ix), Some(iy)) = (ix, iy) {
        let xneg = ix[BINT_SIZE - 1] & BINT_WORDMSB != 0;
        let yneg = iy[BINT_SIZE - 1] & BINT_WORDMSB != 0;
        if xneg == yneg {
            for i in (0..BINT_SIZE).rev() {
                if ix[i] != iy[i] {
                    return ix[i] < iy[i];
                }
            }
            false
        } else {
            xneg && !yneg
        }
    } else {
        bint_tonumber(x) < bint_tonumber(y)
    }
}

fn bint_le(x: &Bint, y: &Bint) -> bool {
    let ix = tobint(x);
    let iy = tobint(y);
    if let (Some(ix), Some(iy)) = (ix, iy) {
        let xneg = ix[BINT_SIZE - 1] & BINT_WORDMSB != 0;
        let yneg = iy[BINT_SIZE - 1] & BINT_WORDMSB != 0;
        if xneg == yneg {
            for i in (0..BINT_SIZE).rev() {
                if ix[i] != iy[i] {
                    return ix[i] < iy[i];
                }
            }
            true
        } else {
            xneg && !yneg
        }
    } else {
        bint_tonumber(x) <= bint_tonumber(y)
    }
}

fn ipow(y: u64, x: u64, n: u32) -> u64 {
    if n == 1 {
        return y * x;
    } else if n & 1 == 0 {
        return ipow(y, x * x, n / 2);
    }
    ipow(x * y, x * x, (n - 1) / 2)
}

impl std::fmt::Display for Bint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Bint::tobase(self, 10, None).unwrap())
    }
}

fn newmodule() -> Result<Bint, &'static str> {
    let bint = Bint::zero();
    Ok(bint)
}

fn main() {
    let bint = newmodule().unwrap();
    println!("{:?}", bint);
}
*/