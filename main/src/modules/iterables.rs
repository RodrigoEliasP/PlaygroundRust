use std::time::Instant;


pub fn searches_test() {
    let mut v = (0..(1e9 as i64)).collect();
    fn test_binary_search(vec: Vec<i64>) -> Vec<i64>{
        let timer = Instant::now();
        let position = vec.binary_search(&(1e6 as i64)).unwrap();
        println!("binary: the position of 1m is {} found in {:?}", position, timer.elapsed());
        vec
    }
    fn test_linear_search(vec: Vec<i64>) -> Vec<i64> {
        let timer = Instant::now();
        let position = vec.iter().position(|&x| x == 1e6 as i64).unwrap();
        println!("linear: the position of 1m is {} found in {:?}", position, timer.elapsed());
        vec
    }
    v = test_binary_search(v);
    test_linear_search(v);
}
