use crate::utils::arithmetic;

pub const LEVELS: u64 = 3;
pub const MAX_CYCLE: u64 = 63;

pub const LOG2_STEP: [u64; 3] = [31, 16, 0];
pub const HEIGHTS: [u64; 3] = [32, 15, 16];

pub const LOG2_UARCH_SPAN: u64 = 16;
pub const UARCH_SPAN: u64 = arithmetic::max_uint(LOG2_UARCH_SPAN);

pub const LOG2_EMULATOR_SPAN: u64 = 47;
pub const EMULATOR_SPAN: u64 = arithmetic::max_uint(LOG2_EMULATOR_SPAN);
