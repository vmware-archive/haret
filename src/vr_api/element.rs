use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
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
