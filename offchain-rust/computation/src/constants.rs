use utils::arithmetic;

pub const LEVELS: u32 = 4;
pub const MAX_CYCLE: u32 = 63;

pub const LOG2STEP: [u32; 4] = [24, 14, 7, 0];
pub const HEIGHTS: [u32; 4] = [39, 10, 7, 7];

pub const LOG2_UARCH_SPAN: u32 = 64;
pub const UARCH_SPAN: i64 = arithmetic::max_uint(LOG2_UARCH_SPAN);

pub const LOG2_EMULATOR_SPAN: u32 = 63;
pub const EMULATOR_SPAN: i64 = arithmetic::max_uint(LOG2_EMULATOR_SPAN);
