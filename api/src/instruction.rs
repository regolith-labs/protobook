use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ProtobookInstruction {
    /// Makers
    Cancel = 0,
    Claim = 1,
    Close = 2,
    Expire = 3,
    Open = 4,

    /// Takers
    Collect = 100,
    Fill = 101,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Cancel {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Claim {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Expire {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {
    pub amount_a: u64,
    pub amount_b: u64,
    pub commission: u64,
    pub expires_at: i64,
    pub threshold: u64,
    pub seed: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Collect {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Fill {}


instruction!(ProtobookInstruction, Cancel);
instruction!(ProtobookInstruction, Claim);
instruction!(ProtobookInstruction, Close);
instruction!(ProtobookInstruction, Expire);
instruction!(ProtobookInstruction, Open);
instruction!(ProtobookInstruction, Collect);
instruction!(ProtobookInstruction, Fill);
