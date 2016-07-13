use membership::Member;
use orset::ORSet;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum ClusterMsg {
    Members(String, ORSet<Member>),
    Ping
}
