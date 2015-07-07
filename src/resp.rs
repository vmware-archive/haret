use mio::tcp::TcpStream;
use std::io::{Result, Error, ErrorKind, Read, Write};
use std::str;

pub enum WriteCompletion {
    Done,
    Incomplete
}

pub struct Writer {
    pos: (usize, usize),
    data: Vec<Vec<u8>>,
}

impl Writer {
    fn new() -> Writer {
        Writer {
            pos: (0, 0),
            data: Vec::new()
        }
    }

    fn format<T: Format>(&mut self, msg: T) {
        self.data = msg.format().value();
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn write<T: Write>(&mut self, sock: &mut T) -> Result<WriteCompletion> {
        let mut done = false;
        loop {
            if (self.complete()) {
                self.pos = (0, 0);
                self.data = Vec::new();
                return Ok(WriteCompletion::Done)
            }
            let ref mut inner = self.data[self.pos.0];
            let ref mut slice = &inner[self.pos.1..];
            match sock.write(slice) {
                Ok(0) => return Ok(WriteCompletion::Incomplete),
                Ok(n) => {
                    self.pos.1 += n;
                    if self.pos.1 == inner.len() {
                        self.pos.0 += 1
                    }
                },
                Err(err) => return Err(err)
            }
        }
    }

    fn complete(&self) -> bool {
        if self.pos.0 != (self.data.len()) { return false}
        let ref inner = self.data[self.pos.0];
        if self.pos.1 == (inner.len()) {
            true
        } else {
            false
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
    fn new() -> Combinator {
        Combinator {
            output: Vec::new(),
            open: Vec::new()
        }
    }

    fn value(self) -> Vec<Vec<u8>> {
        if !self.open.is_empty() {
            panic!("Tried to realize an incomplete io string: {:?}", self.output)
        }
        self.output
    }

    fn array(mut self) -> Combinator {
        self.output.push(vec!['*' as u8]);
        self.open.push((self.output.len() - 1, 0));
        self
    }

    fn simple(mut self, simple: &str) -> Combinator {
        let mut string = String::with_capacity(3 + simple.len());
        string.push('+');
        string.push_str(simple);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.bump_count();
        self
    }

    fn int(mut self, int: usize) -> Combinator {
        let intstr = int.to_string();
        let mut string = String::with_capacity(3 + intstr.len());
        string.push(':');
        string.push_str(&intstr);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.bump_count();
        self
    }

    fn bulk(mut self, bulk: Vec<u8>) -> Combinator {
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

    fn error(mut self, err: &str) -> Combinator {
        let mut string = String::with_capacity(3 + err.len());
        string.push('-');
        string.push_str(err);
        string.push_str("\r\n");
        self.output.push(string.into_bytes());
        self.bump_count();
        self
    }

    /// Closes an array
    fn end(mut self) -> Combinator {
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
        if let Some(&mut (_, mut count)) = self.open.last_mut() {
            count += 1;
        }
    }
}

#[derive(Clone)]
pub enum RespType{
    Simple(String),
    Error(String),
    Int(usize),
    Bulk(Vec<u8>),
    Array(usize)
}

pub enum ReadCompletion<T: Parse> {
    Done(T),
    Incomplete
}

pub struct Reader<T: Parse> {
    // Complete resp types. These are to be parsed with Parser<T>.
    data: Vec<RespType>,
    // Current buffer being read into
    buf: Vec<u8>,
    // Location in the vector marking the start of an unread RespType
    start_byte: usize,
    parsers: Vec<Parser<T>>
}

enum RespTypeCompletion {
    Done(RespType),
    Incomplete
}

impl<T: Parse> Reader<T> {
    fn new() -> Reader<T> {
        let mut parsers = T::parsers();
        for parser in &mut parsers {
            parser.expected.reverse()
        }
        Reader {
            data: Vec::new(),
            // Somewhat arbitrary size
            buf: Vec::with_capacity(128),
            start_byte: 0,
            parsers: parsers
        }
    }

    fn parse_complete(&mut self, r: RespType) -> Result<ReadCompletion<T>> {
        self.data.push(r.clone());
        let it = self.parsers.iter_mut().filter_map(move |p| {
            if let Some(f) = p.expected.pop() {
                if f(&r) { Some(p) } else { None }
            } else {
                // We should never get here
                assert!(false);
                None
            }
        });
        let mut count = 0;
        for parser in it {
            count += 1;
            if parser.expected.is_empty() {
                // We have a complete message!
                let ref f = parser.construct;
                let msg = f(&self.data);
                return Ok(ReadCompletion::Done(msg))
            }
        }
        if count == 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "No such message"))
        }
        Ok(ReadCompletion::Incomplete)
    }

    fn read<R: Read>(&mut self, sock: &mut R) -> Result<ReadCompletion<T>> {
        loop {
            self.maybe_shift_buffer();
            self.maybe_double_capacity();
            let start = self.buf.len();
            match sock.read(&mut self.buf[start..]) {
                Ok(0) => return Ok(ReadCompletion::Incomplete),
                Ok(n) => (),
                Err(e) => return Err(e)
            }
            let completion = match self.buf[self.start_byte] as char {
                '*' => self.read_array(),
                '+' => self.read_simple(),
                ':' => self.read_int(),
                '$' => self.read_bulk(),
                '-' => self.read_error(),
                _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid Lead Byte"))
            };
            match completion {
                Ok(RespTypeCompletion::Done(resp_type)) => {
                    match self.parse_complete(resp_type) {
                        Ok(ReadCompletion::Incomplete) => (),
                        val => return val
                    }
                },
                Ok(RespTypeCompletion::Incomplete) => return Ok(ReadCompletion::Incomplete),
                Err(e) => return Err(e)
            }
        }
    }

    fn read_array(&mut self) -> Result<RespTypeCompletion> {
        match self.read_size() {
            Ok(None) => Ok(RespTypeCompletion::Incomplete),
            Err(e) => Err(e),
            Ok(Some((size, end))) => {
                self.start_byte = end;
                Ok(RespTypeCompletion::Done(RespType::Array(size)))
            }
        }
    }

    fn read_bulk(&mut self) -> Result<RespTypeCompletion> {
        match self.read_size() {
            Ok(None) => Ok(RespTypeCompletion::Incomplete),
            Err(e) => Err(e),
            Ok(Some((size, end))) => {
                self.resize_buffer(size, end);
                Ok(RespTypeCompletion::Incomplete)
            }
        }
    }

    // TODO: Do we want to try to shift the buffer left when starting past a certain point
    // instead of allocating more space?
    fn resize_buffer(&mut self, size: usize, start: usize) {
        let capacity = self.buf.capacity();
        if capacity > (start+1) {
            let remaining = capacity - (start + 1);
            self.buf.reserve(size - remaining);
        } else {
            self.buf.reserve(size)
        }
    }

    /// If we have already parsed 50% of the buffer, shift the rest of the read bytes all the way
    /// to the beginning. Note that 50% is just a random ass guess here. We probably want some
    /// dynamic system to make this optimal. It should probably also be combined in feedback with
    /// resize_buffer().
    fn maybe_shift_buffer(&mut self) {
        if self.start_byte as f64 >= 0.5 as f64 * self.buf.capacity() as f64 {
            for i in 0..self.start_byte {
                self.buf.remove(i);
            }
        }
    }

    #[inline]
    fn maybe_double_capacity(&mut self) {
        let capacity = self.buf.capacity();
        if self.buf.len() == capacity {
            self.buf.reserve(capacity);
        }
    }

    fn read_simple(&mut self) -> Result<RespTypeCompletion> {
        let start = self.start_byte + 1;
        let slice = &self.buf[start..];
        let mut it = slice.iter().skip_while(|&a| *a != LF);
        let end = start + it.count() + 1;
        if self.buf.len() <= end { return Ok(RespTypeCompletion::Incomplete) }
        if !(self.buf[end-1] == CR && self.buf[end] == LF) {
            return Err(Error::new(ErrorKind::InvalidInput, "Arrays Require CRLF after size"))
        }
        self.start_byte = end;
        match str::from_utf8(&self.buf[start..end-1]) {
            Ok(string) => Ok(RespTypeCompletion::Done(RespType::Simple(string.to_string()))),
            Err(err) => Err(Error::new(ErrorKind::InvalidInput, err))
        }
    }

    fn read_error(&mut self) -> Result<RespTypeCompletion> {
        match self.read_simple() {
            Ok(RespTypeCompletion::Done(RespType::Simple(string))) =>
                Ok(RespTypeCompletion::Done(RespType::Error(string))),
            val => val
        }
    }

    fn read_int(&mut self) -> Result<RespTypeCompletion> {
        match self.read_array() {
            Ok(RespTypeCompletion::Done(RespType::Array(int))) =>
                Ok(RespTypeCompletion::Done(RespType::Int(int))),
            val => val
        }
    }

    fn read_size(&self) -> Result<Option<(usize, usize)>> {
        let slice = &self.buf[self.start_byte+1..];
        let mut it = slice.iter().take_while(|&i| is_digit(*i));
        let mut size: usize = 0;
        let mut count = 0;
        for i in it {
            size = size*10 + to_decimal(*i);
            count += 1;
        }
        let start = self.start_byte + 1 + count;
        let end = start + 3;
        if self.buf.len() <= end { return Ok(None) }
        if !(self.buf[end-1] == CR && self.buf[end] == LF) {
            return Err(Error::new(ErrorKind::InvalidInput, "Arrays Require CRLF after size"))
        }
        Ok(Some((size, end)))
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

pub struct Parser<T: Parse> {
    // TODO: new() should return a result
    construct: Box<Fn(&Vec<RespType>) -> T>,
    expected: Vec<Box<Fn(&RespType) -> bool>>,
    open: Vec<(usize, usize)>
}

impl<T: Parse> Parser<T> {
    fn new(constructor: Box<Fn(&Vec<RespType>) -> T>) -> Parser<T> {
        Parser {
            construct: constructor,
            expected: Vec::new(),
            open: Vec::new()
        }
    }

    fn array(mut self) -> Parser<T> {
        // dummy closure
        self.expected.push(Box::new(|_| true));
        self.open.push((self.expected.len() -1, 0));
        self
    }

    fn end(mut self) -> Parser<T> {
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
    fn vararray(mut self) -> Parser<T> {
        self.expected.push(Box::new(|a| {
            match a {
                &RespType::Array(_) => true,
                _ => false
            }
        }));
        self
    }

    fn simple(mut self, val: Option<&str>) -> Parser<T> {
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

    fn error(mut self, val: Option<&str>) -> Parser<T> {
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

    fn int(mut self, val: Option<usize>) -> Parser<T> {
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
    fn bulk(mut self, val: Option<&str>) -> Parser<T> {
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
