use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ProtobookInstruction {
    Cancel = 0,
    Claim = 1,
    Close = 2,
    Fill = 3,
    Open = 4,
    Redeem = 5,
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
    pub expires_at: i64,
    pub fee: u64,
    pub id: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Fill {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Redeem {}

instruction!(ProtobookInstruction, Cancel);
instruction!(ProtobookInstruction, Claim);
instruction!(ProtobookInstruction, Close);
instruction!(ProtobookInstruction, Fill);
instruction!(ProtobookInstruction, Open);
instruction!(ProtobookInstruction, Redeem);
