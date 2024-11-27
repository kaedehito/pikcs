use crate::version;
#[allow(warnings)]
#[derive(Debug, Clone, Copy)]
pub struct Version {
    pub first: u64,
    pub second: u64,
    pub three: u64,
}

#[allow(warnings)]
impl Version {
    pub fn to_tuple(&self) -> (u64, u64, u64) {
        (self.first, self.second, self.three)
    }
}

#[allow(warnings)]
pub trait ToVersion {
    fn to_version(&self) -> Option<version::Version>;
}

impl ToVersion for String {
    fn to_version(&self) -> Option<version::Version> {
        let s: Vec<&str> = self.split(".").collect();
        if s.len() >= 3 {
            let ver = Version {
                first: s.first().unwrap().parse::<u64>().unwrap(),
                second: s[1].parse::<u64>().unwrap(),
                three: s[2].parse::<u64>().unwrap(),
            };
            return Some(ver);
        }
        None
    }
}
