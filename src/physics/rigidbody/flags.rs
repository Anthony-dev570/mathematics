use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RigidbodyFlags {
    UseKinematics = 1,
    UseGravity = 2
}

impl RigidbodyFlags {
    pub const DEFAULT: u8 = 1 | 2;
}

impl BitOr<RigidbodyFlags> for RigidbodyFlags {
    type Output = u8;

    fn bitor(self, rhs: RigidbodyFlags) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitAnd<RigidbodyFlags> for RigidbodyFlags {
    type Output = u8;

    fn bitand(self, rhs: RigidbodyFlags) -> Self::Output {
        self as u8 & rhs as u8
    }
}

impl BitAnd<RigidbodyFlags> for u8 {
    type Output = u8;

    fn bitand(self, rhs: RigidbodyFlags) -> Self::Output {
        self & rhs as u8
    }
}

impl BitOr<RigidbodyFlags> for u8 {
    type Output = u8;

    fn bitor(self, rhs: RigidbodyFlags) -> Self::Output {
        self | rhs as u8
    }
}

impl BitAnd<u8> for RigidbodyFlags {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self as u8 & rhs
    }
}

impl BitOr<u8> for RigidbodyFlags {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self as u8 | rhs
    }
}