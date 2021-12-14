#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum CallingConv {
    C = 0,
    SystemV64 = 78,
    Uefi = 79,
}
