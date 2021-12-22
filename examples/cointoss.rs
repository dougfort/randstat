#[derive(Debug)]
#[repr(u8)]
enum Coin {
    Tails = 0,
    Heads = 1,
}

impl From<u8> for Coin {

    fn from(i: u8) -> Self {
        match  i {
            0x00 => Coin::Tails,
            0x01 => Coin::Heads,
            _ => panic!("unknown Coin {}", i),
        }
    }
}

fn main() {
    let init = vec![randstat::StatInit{percentage:  50, value: Coin::Heads as u8}];
    let rs: randstat::RandStat = randstat::RandStat::new(&init).unwrap();
    for c in rs.take(100).map(Coin::from) {
        println!("{:?}", c);
    }
}
