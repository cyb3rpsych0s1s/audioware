/// see [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Hashing/FNV1a.hpp#L7)
pub const fn fnv1a32(str: &str) -> u32 {
    const PRIME: u32 = 0x0100_0193;
    const SEED: u32 = 0x811C_9DC5;

    let mut tail = str.as_bytes();
    let mut hash = SEED;
    loop {
        match tail.split_first() {
            Some((head, rem)) => {
                hash ^= *head as u32;
                hash = hash.wrapping_mul(PRIME);
                tail = rem;
            }
            None => break hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fnv1a32;

    #[test]
    fn hash() {
        let id: &str = "ono_hhuh";
        let hash: u32 = fnv1a32(id);
        assert_eq!(hash, 1820247331);

        let id: &str = "get_this_damn_UI_out_of_my_face";
        let hash: u32 = fnv1a32(id);
        assert_eq!(hash, 2677247932);
    }
}
