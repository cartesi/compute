use crate::constants;
use ruint::Uint;
use utils::arithmetic;
#[derive(Clone, Debug)]
pub struct StrideCounter {
    interval: Interval,
    total: i64,
    value: i32,
}

impl StrideCounter {
    pub fn new(interval: Interval, total: i64, v: Option<i32>) -> StrideCounter {
        let value = match v {
            Some(value) => value,
            None => -1,
        };
        StrideCounter {
            interval,
            total,
            value,
        }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    /*pub fn cycle(&self) -> u32 {
        let cycle = (self.value << (self.interval.log2_stride - constants::A)) as u32;
        cycle
    }*/

    fn ucycle(&self) -> u32 {
        let ucycle = (self.value << self.interval.log2_stride) as u32;
        ucycle
    }

    pub fn remaining_strides(&self) -> i64 {
        self.total - self.value as i64
    }
}
#[derive(Clone, Debug)]

pub struct Interval {
    pub base_meta_counter: Uint<256, 4>,
    pub log2_stride: u32,
    pub log2_stride_count: u32,
}

impl Interval {
    pub fn new(
        base_meta_counter: Uint<256, 4>,
        log2_stride: u32,
        log2_stride_count: u32,
    ) -> Interval {
        Interval {
            base_meta_counter,
            log2_stride,
            log2_stride_count,
        }
    }

    /*pub fn iter(&self, log2_total_strides: i32) -> IntervalIterator {
        IntervalIterator::new(self.clone(), log2_total_strides)
    }*/

    pub fn _build_iter(&self, log2_total_strides: u32) -> (u64, StrideCounter) {
        let total_strides = arithmetic::max_uint(log2_total_strides);
        let stride = StrideCounter::new(self.clone(), total_strides.try_into().unwrap(), None);
        (total_strides as u64, stride)
    }

    /*pub fn big_strides(&self) -> (u64, StrideCounter) {
        let bid_strides_in_interval = if self.log2_stride_count >= constants::A {
            self.log2_stride_count - constants::A
        } else {
            0
        };

        self._build_iter(bid_strides_in_interval)
    }

    pub fn big_strides_iter(&self) -> BigStridesIter {
        BigStridesIter {
            interval: self,
            remaining_big_strides: self.log2_stride_count as u64,
        }
    }*/
    

    /*fn strides(&self) -> impl Iterator<Item = (u64, u64, u64)> {
        let (total_strides, mut stride) = self._build_iter(self.log2_stride_count);
        (0..total_strides).map(move |_| {
            stride.increment();
            (stride.value as u64, stride.cycle() as u64, stride.remaining_strides() as u64)
        })
    }*/

    /*pub fn total_ucycles_in_cycle(&self) -> i32 {
        let ucycles = std::cmp::min(constants::A, self.log2_stride_count);
        arithmetic::max_uint(ucycles) as i32
    }*/

    /*pub fn ucycles_in_cycle(&self) -> IntervalIterator {
        //println!("call total_ucycles_in_cycleeeeee");
        println!("call total_ucycles_in_cycle from there");

        let total_strides = self.total_ucycles_in_cycle();
        println!("total strides {:?}", total_strides);
        IntervalIterator::new(self.clone(), total_strides)
    }*/

    /*pub fn ucycles_in_cycle(&self) -> impl Iterator<Item = (u64, u64, u64)> {
        let total_strides = self.total_ucycles_in_cycle();
        let mut stride = StrideCounter::new(self.clone(), total_strides as i64, None);
        (0..total_strides).map(move |_| {
            stride.increment();
            (stride.value as u64, stride.ucycle() as u64, stride.remaining_strides() as u64)
        })
    }

    pub fn ucycles_in_cycle_iter(&self) -> UCyclesInCycleIter {
        let total_ucycles_in_cycle = self.total_ucycles_in_cycle();

        UCyclesInCycleIter {
            interval: self,
            remaining_ucycles_in_cycle: total_ucycles_in_cycle as u64,
            remaining_strides: (total_ucycles_in_cycle << self.log2_stride) as u64,
        }
    }*/
}
/*
pub struct BigStridesIter<'a> {
    interval: &'a Interval,
    remaining_big_strides: u64,
}

impl<'a> Iterator for BigStridesIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_big_strides > 0 {
            self.remaining_big_strides -= 1;
            Some(self.remaining_big_strides)
        } else {
            None
        }
    }
}

pub struct UCyclesInCycleIter<'a> {
    interval: &'a Interval,
    remaining_ucycles_in_cycle: u64,
    remaining_strides: u64,

}

impl<'a> Iterator for UCyclesInCycleIter<'a> {
    type Item = (i32, u64); // Return a tuple containing ucycle and remaining_strides

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_ucycles_in_cycle > 0 {
            let ucycle = self.interval.total_ucycles_in_cycle() - self.remaining_ucycles_in_cycle as i32;
            self.remaining_ucycles_in_cycle -= 1;
            let remaining_strides = self.remaining_strides;
            self.remaining_strides -= 1;
            Some((ucycle, remaining_strides))
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct IntervalIterator {
    counter: StrideCounter,
}

impl IntervalIterator {
    fn new(interval: Interval, log2_total_strides: i32) -> IntervalIterator {
        let counter = StrideCounter::new( interval,  log2_total_strides as i64, None );
        IntervalIterator { counter }
    }
}

impl<'a> Iterator for IntervalIterator {
    type Item = StrideCounter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter.remaining_strides() > 0 {
            self.counter.increment();
            Some(self.counter.clone())
        } else {
            None
        }
    }
}
*/
