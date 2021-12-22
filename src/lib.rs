pub struct RandStat {
    cells: [u8; 100],
}

impl Default for RandStat {
    fn default() -> Self {
        Self::new()
    }
}

impl RandStat {

    pub fn new() -> Self {
        RandStat{
            cells: [0; 100],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let rs = RandStat::new();
    }
}
