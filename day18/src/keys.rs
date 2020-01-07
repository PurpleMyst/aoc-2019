pub const KEYS: u32 = 26;

// We store the keys as a sort of bitset in a 32-bit wide integer to make `State` `Copy`.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Keys(u32);

impl std::fmt::Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Keys")
            .field(
                &(b'a'..=b'z')
                    .filter(|&c| self.has(c))
                    .map(|c| c as char)
                    .collect::<String>(),
            )
            .finish()
    }
}

fn key_to_idx(key: u8) -> u8 {
    debug_assert!(key.is_ascii_lowercase());
    key - b'a'
}

impl Keys {
    pub fn all() -> Self {
        Self((1 << KEYS) - 1)
    }

    pub fn none() -> Self {
        Self(0)
    }

    pub fn add(self, key: u8) -> Self {
        Self(self.0 | (1 << key_to_idx(key)))
    }

    pub fn add_all(self, keys: Keys) -> Self {
        Self(self.0 | keys.0)
    }

    pub fn remove(self, key: u8) -> Self {
        Self(self.0 ^ (self.0 & (1 << key_to_idx(key))))
    }

    pub fn has(self, key: u8) -> bool {
        self.0 & (1 << key_to_idx(key)) != 0
    }

    pub fn has_all(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}
