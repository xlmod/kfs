

use core::ops::BitOr;

/// Represent a modifier key.
///
/// Can be bitwise together with KeyModifier struct.
#[repr(u8)]
pub enum Modifier {
    LeftCtrl = 1,
    LeftShift = (1 << 1),
    RightShift = (1 << 2),
    LeftAlt = (1 << 3),
    CapsLock = (1 << 4),
    RightCtrl = (1 << 5),
    RightAlt = (1 << 6),
}

pub static mut KEYMODIFIER: KeyModifier = KeyModifier::new();

/// Can store different Modifier to send along ascii_character in Key struct.
#[derive(Debug, Clone, Copy)]
pub struct KeyModifier(u8);

impl KeyModifier {
    /// Return a KeyModifier set to 0.
    pub const fn new () -> Self {
        Self(0)
    }

    /// Set the modifier.
    pub fn set(&mut self, modifier: Modifier) -> &mut Self {
        self.0 |= modifier as u8;
        self
    }

    /// Reset the modifier.
    #[allow(dead_code)]
    pub fn reset(&mut self, modifier: Modifier) -> &mut Self{
        self.0 &= !(modifier as u8);
        self
    }

    /// Check if modifier is set.
    pub fn is_set_one(&self, modifier: Modifier) -> bool {
        self.0 & (modifier as u8) != 0
    }

    /// Check if modifier is set.
    pub fn is_set(&self, keymodifier: Self) -> bool {
        self.0 & (keymodifier.0) != 0
    }


}

/// Implementation for individual modifier set.
impl KeyModifier {

    /// Return KeyModifier with LeftCtrl bit set.
    pub fn left_ctrl() -> Self {
        Self::new().set(Modifier::LeftCtrl).clone()
    }

    /// Return KeyModifier with LeftShift bit set.
    pub fn left_shit() -> Self {
        Self::new().set(Modifier::LeftShift).clone()
    }

    /// Return KeyModifier with LeftAlt bit set.
    pub fn left_alt() -> Self {
        Self::new().set(Modifier::LeftAlt).clone()
    }

    /// Return KeyModifier with RightCtrl bit set.
    pub fn right_shift() -> Self {
        Self::new().set(Modifier::RightShift).clone()
    }

    /// Return KeyModifier with RightCtrl bit set.
    pub fn right_ctrl() -> Self {
        Self::new().set(Modifier::RightCtrl).clone()
    }

    /// Return KeyModifier with RightAlt bit set.
    pub fn right_alt() -> Self {
        Self::new().set(Modifier::RightAlt).clone()
    }

    /// Return KeyModifier with CapsLock bit set.
    pub fn caps_lock() -> Self {
        Self::new().set(Modifier::CapsLock).clone()
    }

}

impl BitOr for KeyModifier {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
