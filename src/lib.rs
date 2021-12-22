//! Random Status Selector. 
//! 
//! This object implements Iterator to returns a stream of status bytes.
//! 

use rand::Rng;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RandStatOverflowError {}

impl fmt::Display for RandStatOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RandStat overflow")
    }
}

impl Error for RandStatOverflowError {}

pub struct RandStat {
    cells: [u8; 100],
}

pub struct StatInit {
    pub percentage: usize,
    pub value:  u8,
}

impl RandStat {

    pub fn new(init_vec: &[StatInit]) -> Result<Self, RandStatOverflowError> {
        let mut cells = [0; 100];
        let mut index: usize = 0;
        for init in init_vec {
            for _ in 0..init.percentage {
                if index >= cells.len() {
                    return Err(RandStatOverflowError{});
                }
                cells[index] = init.value;
                index += 1;
            }
        }
        Ok(RandStat{ cells })
    }
}

impl Default for RandStat {
    fn default() -> Self {
        RandStat{
            cells: [0; 100],
        }
    }
}

/// Returns a stream of status bytes
impl Iterator for RandStat {

    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen::<usize>() % self.cells.len();
        Some(self.cells[index])
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_default() {
        let rs: RandStat = Default::default();
        for i in rs.take(1000) {
            assert_eq!(i, 0);
        } 
    }

    #[test]
    fn can_create_single_value() {
        let init = vec![StatInit{percentage:  100, value: 0xff}];
        let rs: RandStat = RandStat::new(&init).unwrap();
        for i in rs.take(1000) {
            assert_eq!(i, 0xff);
        } 
    }

    #[test]
    fn can_catch_overflow() {
        let init = vec![
            StatInit{percentage:  100, value: 0x01},
            StatInit{percentage:  1, value: 0x02},
        ];
        let rs = RandStat::new(&init);
        assert!(rs.is_err());
    }

    #[test]
    fn can_create_multi_value() {
        let init = vec![
            StatInit{percentage:  10, value: 0x01},
            StatInit{percentage:  10, value: 0x02},
            StatInit{percentage:  10, value: 0x03},
        ];
        let rs: RandStat = RandStat::new(&init).unwrap();
        let test_vec: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
        for i in rs.take(1000) {
            assert!(test_vec.contains(&i));
        } 
    }
}
