
pub fn get_random_hash() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
        const HASH_LENGTH: usize = 15;
        let mut rng = rand::thread_rng();

        let hash: String = (0..HASH_LENGTH)
            .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
            })
            .collect();
        return hash;
}
