use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum ElementType {
    Binary,
    List,
    Set,
    Queue
}

impl FromStr for ElementType {
    type Err = ();
    fn from_str(string: &str) -> Result<ElementType, ()> {
        match string {
            "binary" => Ok(ElementType::Binary),
            "list"  => Ok(ElementType::List),
            "set" => Ok(ElementType::Set),
            "queue" => Ok(ElementType::Queue),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Version {
    epoch: usize,
    vsn: usize,
    lsn: usize,
}

impl Version {
    pub fn new() -> Version {
        Version {
            epoch: 0,
            vsn: 0,
            lsn: 0
        }
    }

    pub fn inc(&mut self) -> Version {
        self.lsn = self.lsn + 1;
        self.clone()
    }
}

impl FromStr for Version {
    type Err = ();
    fn from_str(string: &str) -> Result<Version, ()> {
        let v: Vec<&str> = string.split(':').collect();
        if v.len() != 3 { return Err(()); }
        let epoch = try!(usize::from_str(v[0]).or(Err(())));
        let vsn = try!(usize::from_str(v[1]).or(Err(())));
        let lsn = try!(usize::from_str(v[2]).or(Err(())));
        Ok(Version { epoch: epoch, vsn: vsn, lsn: lsn })
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.epoch, self.vsn, self.lsn)
    }
}
