use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ProtobookInstruction {
    Cancel = 0,
    Close = 1,
    Collect = 2,
    Fill = 3,
    Open = 4,
    Redeem = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Cancel {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Collect {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Expire {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {
    pub amount_a: [u8; 8],
    pub amount_b: [u8; 8],
    pub expires_at: [u8; 8],
    pub fee: [u8; 8],
    pub id: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Fill {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Redeem {}

instruction!(ProtobookInstruction, Cancel);
instruction!(ProtobookInstruction, Close);
instruction!(ProtobookInstruction, Collect);
instruction!(ProtobookInstruction, Fill);
instruction!(ProtobookInstruction, Open);
instruction!(ProtobookInstruction, Redeem);
