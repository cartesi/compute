use crate::utils::arithmetic;

pub const LEVELS: usize = 3;
pub const MAX_CYCLE: u8 = 63;

pub const LOG2STEP: [u32; 3] = [31, 16, 0];
pub const HEIGHTS: [u8; 3] = [32, 15, 16];

pub const LOG2_UARCH_SPAN: u32 = 16;
pub const UARCH_SPAN: i64 = arithmetic::max_uint(LOG2_UARCH_SPAN);

pub const LOG2_EMULATOR_SPAN: u32 = 47;
pub const EMULATOR_SPAN: i64 = arithmetic::max_uint(LOG2_EMULATOR_SPAN);
