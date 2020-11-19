use rand::distributions::{Distribution, Uniform};

const ALPHABET: &[u8] = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

/// generate_id returns a string between 15 and 24 characters long with the characters chosen
/// from the base-58 alphabet.
pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();

    let uniform_len: Uniform<u32> = Uniform::new_inclusive(15, 24);
    let len = uniform_len.sample(&mut rng);
    let mut s = String::with_capacity(len as usize);

    let uniform_chars: Uniform<u32> = Uniform::new_inclusive(0, 57);
    for _ in 0..len {
        s.push(ALPHABET[uniform_chars.sample(&mut rng) as usize] as char);
    }

    s
}

#[cfg(test)]
mod tests {
    use crate::{generate_id, ALPHABET};

    #[test]
    fn test_generate_id() {
        let a = generate_id();
        let b = generate_id();

        assert!(a.len() >= 15 && a.len() <= 24);
        assert!(b.len() >= 15 && b.len() <= 24);

        for c in a.as_bytes().iter() {
            assert!(ALPHABET.contains(c));
        }
        for c in b.as_bytes().iter() {
            assert!(ALPHABET.contains(&c as &u8));
        }

        assert_ne!(a, b);
    }
}
