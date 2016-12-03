use time::{self, SteadyTime, Duration};
use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};

const TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";

/// We need to create this so we can manually serialize a SteadyTime
///
/// This function only exists to allow a VrCtx to be serialized for VrAdminRpy. However, this type
/// will never actually be encoded or shipped across nodes, since the variant is never used that
/// way. Because of this, users should never trust the decoded value.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodableSteadyTime(pub SteadyTime);

impl EncodableSteadyTime {
    pub fn now() -> EncodableSteadyTime {
        EncodableSteadyTime(SteadyTime::now())
    }
}

impl Encodable for EncodableSteadyTime {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let EncodableSteadyTime(time) = *self;
        s.emit_tuple_struct("EncodableSteadyTime", 1, |s| {
            try!(s.emit_tuple_arg(0, |s| {
                s.emit_str("")
            }));
            Ok(())
        })
    }
}

/// Caution: The value can not be relied upon, since we just populate with the current time.
/// SteadyTime types should really never be encoded, because they are only valid locally.
/// This metod
impl Decodable for EncodableSteadyTime {
    fn decode<D: Decoder>(d: &mut D) -> Result<EncodableSteadyTime, D::Error> {
        d.read_tuple_struct("EncodableSteadyTime", 1, |d| {
            let s = try!(d.read_tuple_arg(0, |d| { d.read_str() }));
            Ok(EncodableSteadyTime(SteadyTime::now()))
        })
    }
}

/// We need to create this so we can manually serialize a Duration
///
/// This function only exists to allow a VrCtx to be serialized for VrAdminRpy. However, this type
/// will never actually be encoded or shipped across nodes, since the variant is never used that
/// way. Because of this, users should never trust the decoded value.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodableDuration(pub Duration);

impl EncodableDuration {
    pub fn milliseconds(ms: i64) -> EncodableDuration {
        EncodableDuration(Duration::milliseconds(ms))
    }
}

impl Encodable for EncodableDuration {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let EncodableDuration(time) = *self;
        s.emit_tuple_struct("EncodableDuration", 1, |s| {
            try!(s.emit_tuple_arg(0, |s| {
                s.emit_str("")
            }));
            Ok(())
        })
    }
}

/// Caution: The value can not be relied upon, since we just populate with the current time.
/// SteadyTime types should really never be encoded, because they are only valid locally.
/// This metod
impl Decodable for EncodableDuration {
    fn decode<D: Decoder>(d: &mut D) -> Result<EncodableDuration, D::Error> {
        d.read_tuple_struct("EncodableDuration", 1, |d| {
            let s = try!(d.read_tuple_arg(0, |d| { d.read_str() }));
            Ok(EncodableDuration(Duration::milliseconds(0)))
        })
    }
}

