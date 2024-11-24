use atomics::atomic_enum;

fn main() {}

#[atomic_enum(u8)]
pub enum BaseU8 {
    A,
    B,
}

#[atomic_enum(u16)]
pub enum BaseU16 {
    A,
    B,
}

#[atomic_enum(u32)]
pub enum BaseU32 {
    A,
    B,
}

#[atomic_enum(usize)]
pub enum BaseUsize {
    A,
    B,
}

#[atomic_enum(u8; flags)]
pub enum Flags {
    A,
    B,
}

#[atomic_enum(u8; ordered)]
pub enum Ordered {
    A,
    B,
}

#[atomic_enum(u8; flags, ordered)]
pub enum FlagsOrdered {
    A,
    B,
}
