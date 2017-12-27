mod js {
    extern {
        pub fn keyboard_is_pressed(key: u32) -> u32;
    }
}

#[allow(dead_code)]
pub enum Key {
    KeyA = 65,
    KeyK = 75,
    KeyL = 76,
    KeyP = 80,
    KeyQ = 81,
    KeyR = 82,
    KeyS = 83,
}

pub fn is_pressed(key: Key) -> bool {
  unsafe { js::keyboard_is_pressed(key as u32) != 0 }
}
