use std::io::{Result, Error, ErrorKind, Read, Write};
use std::str;
use libc::EAGAIN;

pub struct Writer {
    pos: (usize, usize),
    data: Vec<Vec<u8>>,
    ready: bool
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            pos: (0, 0),
            data: Vec::new(),
            ready: false
        }
    }

    pub fn encode<T: Format>(msg: T) -> Vec<Vec<u8>>{
        msg.format().value()
    }

    pub fn set_data(&mut self, data: Vec<Vec<u8>>) {
        self.data = data;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn write<T: Write>(&mut self, sock: &mut T) -> Result<Option<()>> {
        loop {
            if self.complete() {
                self.pos = (0, 0);
                self.data = Vec::new();
                return Ok(Some(()))
            }
            let ref mut inner = self.data[self.pos.0];
            let ref mut slice = &inner[self.pos.1..];
            match sock.write(slice) {
                Ok(0) => return Ok(None),
                Ok(n) => {
                    self.pos.1 += n;
                    if self.pos.1 == inner.len() {
                        self.pos.0 += 1;
                        self.pos.1 = 0;
                    }
                },
                Err(err) => return Err(err)
            }
        }
    }

    fn complete(&self) -> bool {
        if self.pos.0 != (self.data.len()) {
            false
        } else {
            true
        }
    }
}

pub trait Format {
    fn format(&self) -> Combinator;
}

pub struct Combinator {
    output: Vec<Vec<u8>>,
    // Values to be filled in later; mainly array sizes
    open: Vec<(usize, usize)>,
}

/// Contains combinators that build up the formatter
impl Combinator {
    pub fn new() -> Combinator {
        Combinator {
            output: Vec::new(),
            open: Vec::new()
        }
    }

    pub fn value(self) -> Vec<Vec<u8>> {
        if !self.open.is_empty() {
            panic!("Tried to realize an incomplete io string: {:?}", self.output)
        }
        self.output
    }

    pub fn array(mut self) -> Combinator {
        self.output.push(vec!['*' as u8]);
        self.open.push((self.output.len() - 1, 0));
        self
    }

    pub fn simple(mut self, simple: &str) -> Combinator {
        let mut string = String::with_capacity(3 + simple.len());
        string.push('+');
        string.push_str(simple);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.bump_count();
        self
    }

    pub fn int(mut self, int: usize) -> Combinator {
        let intstr = int.to_string();
        let mut string = String::with_capacity(3 + intstr.len());
        string.push(':');
        string.push_str(&intstr);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.bump_count();
        self
    }

    pub fn bulk_s(mut self, bulk: &str) -> Combinator {
        let lenstr = bulk.len().to_string();
        let mut string = String::with_capacity(3 + lenstr.len());
        string.push('$');
        string.push_str(&lenstr);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.output.push(bulk.to_string().into_bytes());
        self.output.push("\r\n".to_string().into_bytes());
        self.bump_count();
        self
    }

    pub fn bulk(mut self, bulk: Vec<u8>) -> Combinator {
        let lenstr = bulk.len().to_string();
        let mut string = String::with_capacity(3 + lenstr.len());
        string.push('$');
        string.push_str(&lenstr);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.output.push(bulk);
        self.output.push("\r\n".to_string().into_bytes());
        self.bump_count();
        self
    }

    pub fn error(mut self, err: &str) -> Combinator {
        let mut string = String::with_capacity(3 + err.len());
        string.push('-');
        string.push_str(err);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.bump_count();
        self
    }

    /// Closes an array
    pub fn end(mut self) -> Combinator {
        // fail fast
        let (index, count) = self.open.pop().unwrap();
        // Change of scope so we can end the borrow on array before we return self
        {
            let mut array = self.output.get_mut(index).unwrap();
            for byte in count.to_string().into_bytes() {
                array.push(byte)
            }
            array.push('\r' as u8);
            array.push('\n' as u8);
        }
        self
    }

    // Increment the number of elements in the inner most array
    fn bump_count(&mut self) {
        if let Some(&mut (_, ref mut count)) = self.open.last_mut() {
            *count += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub enum RespType{
    Simple(String),
    Error(String),
    Int(usize),
    Bulk(Vec<u8>),
    Array(usize)
}

enum SizeOp {
    Bulk,
    Array
}

pub struct Reader<T: Parse> {
    // Complete resp types. These are to be parsed with Parser<T>.
    data: Vec<RespType>,
    // Current buffer being read into
    buf: Vec<u8>,
    // Index in the vector marking the start of an unread RespType
    start_byte: usize,
    // The number of bytes actually read into buf.
    // Buf must be initialized to be read into, so we can't use buf.len()
    read_bytes: usize,
    parsers: Vec<Parser<T>>,
    // Indexes of matching parsers
    matching: Vec<usize>
}

impl<T: Parse> Reader<T> {
    pub fn new() -> Reader<T> {
        let parsers = T::parsers();
        let mut matching: Vec<_> = (0..parsers.len()).collect();
        matching.reverse();
        Reader {
            data: Vec::new(),
            // Somewhat arbitrary size
            buf: vec![0; 128],
            start_byte: 0,
            read_bytes: 0,
            matching: matching,
            parsers: parsers

        }
    }

    fn parse_complete(&mut self, r: RespType) -> Result<Option<T>> {
        let index = self.data.len();
        self.data.push(r.clone());
        let mut matching = Vec::new();
        while let Some(i) = self.matching.pop() {
            if let Some(p) = self.parsers.get(i) {
                if let Some(f) = p.expected.get(index) {
                    if f(&r) {
                        if p.expected.len() == index + 1 {
                            // We have a complete message!
                            let ref f = p.construct;
                            match f(&mut self.data) {
                                Ok(msg) => return Ok(Some(msg)),
                                Err(e) => return Err(e)
                            }
                        }
                        matching.push(i);
                    }
                }
            }
        }
        matching.reverse();
        self.matching = matching;
        if self.matching.len() == 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "No such message"))
        }
        Ok(None)
    }

    pub fn read<R: Read>(&mut self, sock: &mut R) -> Result<Option<T>> {
        loop {
            // TODO: Do we ever want to shrink the buffer after we've grown it?
            self.maybe_shift_buffer();
            self.maybe_double_capacity();
            let start = self.read_bytes;
            match sock.read(&mut self.buf[start..]) {
                Ok(0) => return Err(Error::new(ErrorKind::ConnectionAborted,
                                               "Client closed connection")),
                Ok(n) => self.read_bytes += n,
                Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        if code == EAGAIN { return Ok(None) }
                    }
                    return Err(e)
                }
            };
            while self.start_byte != self.read_bytes {
                let completion = match self.buf[self.start_byte] as char {
                    '*' => self.read_array(),
                    '+' => self.read_simple(),
                    ':' => self.read_int(),
                    '$' => self.read_bulk(),
                    '-' => self.read_error(),
                    c => {
                        return Err(Error::new(ErrorKind::InvalidInput,
                                               format!("Invalid Lead Byte: {}", c)))
                    }
                };
                match completion {
                    Ok(Some(resp_type)) => {
                        match self.parse_complete(resp_type) {
                            Ok(None) => (),
                            val => {
                                // Reset for the next message
                                self.data = Vec::new();
                                self.matching = (0..self.parsers.len()).collect();
                                self.matching.reverse();
                                return val
                            }
                        }
                    },
                    Ok(None) => return Ok(None),
                    Err(e) => return Err(e)
                }
            }
        }
    }

    fn read_array(&mut self) -> Result<Option<RespType>> {
        match self.read_size(SizeOp::Array) {
            Ok(None) => Ok(None),
            Err(e) => Err(e),
            Ok(Some((size, start))) => {
                self.start_byte = start;
                Ok(Some(RespType::Array(size)))
            }
        }
    }

    fn read_bulk(&mut self) -> Result<Option<RespType>> {
        match self.read_size(SizeOp::Bulk) {
            Ok(None) => Ok(None),
            Err(e) => Err(e),
            Ok(Some((size, start))) => {
                // Reserve enough space for the blob + CRLF
                self.resize_buffer(size+2, start);
                match self.read_blob(size, start) {
                    Ok(Some(blob)) => Ok(Some(RespType::Bulk(blob))),
                    Ok(None) => Ok(None),
                    Err(err) => Err(err)
                }
            }
        }
    }

    fn read_blob(&mut self, size: usize, start: usize) -> Result<Option<Vec<u8>>> {
        let end = start + size + 2;
        if self.read_bytes < end { return Ok(None) }
        if !(self.buf[end-2] == CR && self.buf[end-1] == LF) {
            let string = "Bulk strings require CRLF after blob";
            return Err(Error::new(ErrorKind::InvalidInput, string))
        }
        self.start_byte = end;
        let mut vec = Vec::with_capacity(size);
        for i in start..end-2 {
            vec.push(self.buf[i])
        }
        Ok(Some(vec))
    }

    // TODO: Do we want to try to shift the buffer left when starting past a certain point
    // instead of allocating more space?
    fn resize_buffer(&mut self, size: usize, start: usize) {
        let capacity = self.buf.capacity();
        if capacity > start {
            let remaining = capacity - start;
            if remaining < size { self.buf.reserve(size - remaining) }
        } else {
            self.buf.reserve(size)
        }
        self.initialize_buffer();
    }

    fn initialize_buffer(&mut self) {
        for _ in self.buf.len()..self.buf.capacity() {
            self.buf.push(0);
        }
    }

    /// If we have already parsed 50% of the buffer, shift the rest of the read bytes all the way
    /// to the beginning. Note that 50% is just a random ass guess here. We probably want some
    /// dynamic system to make this optimal. It should probably also be combined in feedback with
    /// resize_buffer().
    fn maybe_shift_buffer(&mut self) {
        if self.start_byte as f64 >= 0.5 as f64 * self.buf.capacity() as f64 {
            for _ in 0..self.start_byte {
                self.buf.remove(0);
                self.read_bytes -= 1;
            }
            self.start_byte = 0;
            self.initialize_buffer();
        }
    }

    fn maybe_double_capacity(&mut self) {
        let capacity = self.buf.capacity();
        if self.read_bytes == capacity {
            self.buf.reserve(capacity);
            self.initialize_buffer();
        }
    }

    fn read_simple(&mut self) -> Result<Option<RespType>> {
        let start = self.start_byte + 1;
        let slice = &self.buf[start..];
        let strlen = slice.iter().take_while(|&a| *a != CR).count();
        let end = start + strlen + 2;
        if self.read_bytes < end { return Ok(None) }
        if !(self.buf[end-2] == CR && self.buf[end-1] == LF) {
            return Err(Error::new(ErrorKind::InvalidInput, "Simple strings require CRLF after size"))
        }
        self.start_byte = end;
        match str::from_utf8(&self.buf[start..end-2]) {
            Ok(string) => Ok(Some(RespType::Simple(string.to_string()))),
            Err(err) => Err(Error::new(ErrorKind::InvalidInput, err))
        }
    }

    fn read_error(&mut self) -> Result<Option<RespType>> {
        match self.read_simple() {
            Ok(Some(RespType::Simple(string))) =>
                Ok(Some(RespType::Error(string))),
            val => val
        }
    }

    fn read_int(&mut self) -> Result<Option<RespType>> {
        match self.read_array() {
            Ok(Some(RespType::Array(int))) =>
                Ok(Some(RespType::Int(int))),
            val => val
        }
    }

    fn read_size(&self, op: SizeOp) -> Result<Option<(usize, usize)>> {
        let slice = &self.buf[self.start_byte+1..];
        let it = slice.iter().take_while(|&i| is_digit(*i));
        let mut size: usize = 0;
        let mut count = 0;
        for i in it {
            size = size*10 + to_decimal(*i);
            count += 1;
        }
        let end = self.start_byte + count + 2;
        if self.read_bytes <= end { return Ok(None) }
        if !(self.buf[end-1] == CR && self.buf[end] == LF) {
            let string = match op {
                SizeOp::Bulk => "Bulk strings require CRLF after size",
                SizeOp::Array => "Arrays require CRLF after size"
            };
            return Err(Error::new(ErrorKind::InvalidInput, string))
        }
        Ok(Some((size, end+1)))
    }
}

const CR:u8 = 13;
const LF: u8 = 10;

#[inline]
// Subtract '0'
fn to_decimal(i: u8) -> usize {
    (i - 48) as usize
}

// Is this an ascii digit in decimal?
#[inline]
fn is_digit(i: u8) -> bool {
    i > 47 && i < 58
}

pub trait Parse {
    fn parsers() -> Vec<Parser<Self>>;
}

// TODO: Instead of using closures should we use pointers to named functions?
pub struct Parser<T: Parse> {
    construct: Box<Fn(&mut Vec<RespType>) -> Result<T>>,
    expected: Vec<Box<Fn(&RespType) -> bool>>,
    open: Vec<(usize, usize)>
}

impl<T: Parse> Parser<T> {
    pub fn new(constructor: Box<Fn(&mut Vec<RespType>) -> Result<T>>) -> Parser<T> {
        Parser {
            construct: constructor,
            expected: Vec::new(),
            open: Vec::new()
        }
    }

    pub fn array(mut self) -> Parser<T> {
        // dummy closure
        self.expected.push(Box::new(|_| true));
        self.open.push((self.expected.len() -1, 0));
        self
    }

    pub fn end(mut self) -> Parser<T> {
        let (index, count) = self.open.pop().unwrap();
        self.expected[index] = Box::new(move |a| {
            match a {
                &RespType::Array(int) => int == count,
                _ => false
            }
        });
        self
    }

    /// Match a variable length array
    pub fn vararray(mut self) -> Parser<T> {
        self.expected.push(Box::new(|a| {
            match a {
                &RespType::Array(_) => true,
                _ => false
            }
        }));
        self
    }

    pub fn simple(mut self, val: Option<&str>) -> Parser<T> {
        match val {
            None => self.expected.push(Box::new(|s| {
                match s {
                    &RespType::Simple(_) => true,
                    _ => false
                }
            })),
            Some(string) => {
                let string = string.to_string();
                self.expected.push(Box::new(move |s| {
                    match s {
                        &RespType::Simple(ref string2) => string == *string2,
                        _ => false
                    }
                }))
            }
        }
        self.bump_count();
        self
    }

    pub fn error(mut self, val: Option<&str>) -> Parser<T> {
        match val {
            None => self.expected.push(Box::new(|s| {
                match s {
                    &RespType::Error(_) => true,
                    _ => false
                }
            })),
            Some(string) => {
                let string = string.to_string();
                self.expected.push(Box::new(move |s| {
                    match s {
                        &RespType::Error(ref string2) => string == *string2,
                        _ => false
                    }
                }))
            }
        }
        self.bump_count();
        self
    }

    pub fn int(mut self, val: Option<usize>) -> Parser<T> {
        match val {
            None => self.expected.push(Box::new(|i| {
                match i {
                    &RespType::Int(_) => true,
                    _ => false
                }
            })),
            Some(int) => self.expected.push(Box::new(move |i| {
                match i {
                    &RespType::Int(int2) => int == int2,
                    _ => false
                }
            }))
        }
        self.bump_count();
        self
    }

    // Basically the only time we want to match is for a human readable string
    // denoting a command or response. Make the api nicer by not requiring
    // conversion to a vec for this.
    pub fn bulk(mut self, val: Option<&str>) -> Parser<T> {
        match val {
            None => self.expected.push(Box::new(|v| {
                match v {
                    &RespType::Bulk(_) => true,
                    _ => false
                }
            })),
            Some(string) => {
                let string = string.to_string();
                self.expected.push(Box::new(move |v| {
                    match v {
                        &RespType::Bulk(ref v) => {
                            match String::from_utf8(v.clone()) {
                                Ok(s) => string == s,
                                _ => false
                            }
                        },
                        _ => false
                    }
                }))
            }
        }
        self.bump_count();
        self
    }

    // Increment the number of elements in the inner most array
    fn bump_count(&mut self) {
        if let Some(&mut (_, ref mut count)) = self.open.last_mut() {
            *count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write, Result, Error, ErrorKind};
    use std::fs::{File, OpenOptions};
    use std::fmt::Debug;
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Msg {
        ClusterSetName(String),
        ClusterGetName,
        NumClients(usize),
        ClusterName(String),
        Error(String)
    }

    impl Format for Msg {
        fn format(&self) -> Combinator {
            let c = Combinator::new();
            match *self {
                Msg::ClusterSetName(ref name) =>
                    c.array().bulk_s("cluster").bulk_s("setname").bulk_s(&name).end(),
                Msg::ClusterGetName =>
                    c.array().bulk_s("cluster").bulk_s("getname").end(),
                Msg::NumClients(num) => c.int(num),
                Msg::ClusterName(ref name) => c.simple(name),
                Msg::Error(ref string) => c.error(string)
            }
        }
    }

    impl Parse for Msg {
        fn parsers() -> Vec<Parser<Msg>> {
            // The constructor takes the chain of matched types from parsing
            // and constructs a Msg::ClusterSetName(...)
            let cluster_set_name_constructor = Box::new(|types: &Vec<RespType>| {
                if let RespType::Bulk(ref val) = types[3] {
                    match String::from_utf8(val.clone()) {
                        Ok(string) => return Ok(Msg::ClusterSetName(string)),
                        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
                    }
                } else {
                    // We should never get here
                    assert!(false);
                    Err(Error::new(ErrorKind::InvalidInput,
                                   "Failed to construct Msg::ClusterSetName"))
                }
            });

            vec![Parser::new(cluster_set_name_constructor).array()
                .bulk(Some("cluster")).bulk(Some("setname")).bulk(None).end(),

                 Parser::new(Box::new(|_| Ok(Msg::ClusterGetName)))
                     .array().bulk(Some("cluster")).bulk(Some("getname")).end(),

                 Parser::new(Box::new(|types| {
                    if let Some(&RespType::Int(i)) = types.first() {
                        Ok(Msg::NumClients(i))
                    } else {
                        // We should never get here
                        assert!(false);
                        Err(Error::new(ErrorKind::InvalidInput,
                                       "Failed to construct Msg::NumClients"))
                    }
                 })).int(None),

                 Parser::new(Box::new(|types| {
                     if let Some(&RespType::Simple(ref s)) = types.first() {
                         return Ok(Msg::ClusterName(s.clone()))
                     } else {
                        // We should never get here
                        assert!(false);
                        Err(Error::new(ErrorKind::InvalidInput,
                                       "Failed to construct Msg::ClusterName"))
                     }
                 })).simple(None),

                 Parser::new(Box::new(|types| {
                     if let Some(&RespType::Error(ref s)) = types.first() {
                         return Ok(Msg::Error(s.clone()))
                     } else {
                        // We should never get here
                        assert!(false);
                        Err(Error::new(ErrorKind::InvalidInput,
                                       "Failed to construct Msg::Error"))
                     }
                 })).error(None)

                ]
        }
    }

    fn write_msg<T: Format + Clone>(file: &mut File, msg: &T) {
        let mut writer = Writer::new();
        writer.format((*msg).clone());
        match writer.write(file) {
            Ok(Some(())) => assert!(true),
            _ => assert!(false)
        }
    }

    fn read_msg<T>(file: &mut File, reader: &mut Reader<T>) -> Option<T>
      where T: Parse + Format + Clone + Debug {
        match reader.read(file) {
            Ok(Some(msg)) => Some(msg),
            Ok(None) => None,
            Err(e) => {
                println!("file len = {}", file.metadata().unwrap().len());
                println!("Error in read_msg: {:?}", e);
                assert!(false);
                None
            }
        }
    }

    #[test]
    fn basic() {
        let msg = Msg::ClusterSetName("cluster1".to_string());
        let msg1 = Msg::Error("Some Error".to_string());
        let msg2 = Msg::NumClients(1000);
        let msg3 = Msg::ClusterName("cluster2".to_string());
        let path = "/tmp/resp_test";
        let mut reader = Reader::new();
        let mut file1 = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .truncate(true)
                                    .open(path).unwrap();
        let mut file2 = OpenOptions::new()
                                    .read(true)
                                    .open(path).unwrap();
        write_msg(&mut file1, &msg);
        if let Some(msg4) = read_msg(&mut file2, &mut reader) {
            println!("readmsg1");
            assert_eq!(msg, msg4);
        } else { assert!(false) }
        write_msg(&mut file1, &msg1);
        if let Some(msg4) = read_msg(&mut file2, &mut reader) {
            println!("readmsg2");
            assert_eq!(msg1, msg4);
        } else { assert!(false) }
        write_msg(&mut file1, &msg2);
        if let Some(msg4) = read_msg(&mut file2, &mut reader) {
            println!("readmsg3");
            assert_eq!(msg2, msg4);
        } else { assert!(false) }
        write_msg(&mut file1, &msg3);
        if let Some(msg4) = read_msg(&mut file2, &mut reader) {
            println!("readmsg4");
            assert_eq!(msg3, msg4);
        } else { assert!(false) }
    }
}
