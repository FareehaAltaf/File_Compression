// mod tests;
pub mod random{
    use rand::Rng;
    pub fn gen_random_ab() -> Vec<(u32, u32)> { // genereates random numbers, these numbers will eventually be the number of as and bs in the file
        let mut list = vec![];
        for _ in 0..10 { // make 10 random numbers
            let a = rand::thread_rng().gen_range(1..=20); // gen random no of a
            let b = rand::thread_rng().gen_range(1..=20); // gen random no of b
            list.push((a, b)); // keep them in list as pairs
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_random_ab() {
        let list = random::gen_random_ab();
        assert_eq!(list.len(), 10);
    }
}