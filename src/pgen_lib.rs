use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn gen_pwd(length: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|_| rng.sample(Alphanumeric))
        .take(length)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwd_gen_ok() -> Result<(), String> {
        let valid_size = 10;
        let pwd = gen_pwd(valid_size);
        assert_eq!(pwd.chars().count(), valid_size);
        Ok(())
    }
}
