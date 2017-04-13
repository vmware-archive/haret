// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Msg {
    // message oneof groups
    msg: ::std::option::Option<Msg_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Msg {}

#[derive(Clone,PartialEq)]
pub enum Msg_oneof_msg {
    vr(VrMsg),
    namespace(NamespaceMsg),
    admin_req(AdminReq),
    admin_rpy(AdminRpy),
    api_rpy(ApiRpy),
    error(::std::string::String),
}

impl Msg {
    pub fn new() -> Msg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Msg {
        static mut instance: ::protobuf::lazy::Lazy<Msg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Msg,
        };
        unsafe {
            instance.get(|| {
                Msg {
                    msg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .VrMsg vr = 1;

    pub fn clear_vr(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_vr(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::vr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_vr(&mut self, v: VrMsg) {
        self.msg = ::std::option::Option::Some(Msg_oneof_msg::vr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vr(&mut self) -> &mut VrMsg {
        if let ::std::option::Option::Some(Msg_oneof_msg::vr(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Msg_oneof_msg::vr(VrMsg::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::vr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_vr(&mut self) -> VrMsg {
        if self.has_vr() {
            match self.msg.take() {
                ::std::option::Option::Some(Msg_oneof_msg::vr(v)) => v,
                _ => panic!(),
            }
        } else {
            VrMsg::new()
        }
    }

    pub fn get_vr(&self) -> &VrMsg {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::vr(ref v)) => v,
            _ => VrMsg::default_instance(),
        }
    }

    // optional .NamespaceMsg namespace = 2;

    pub fn clear_namespace(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_namespace(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::namespace(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: NamespaceMsg) {
        self.msg = ::std::option::Option::Some(Msg_oneof_msg::namespace(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut NamespaceMsg {
        if let ::std::option::Option::Some(Msg_oneof_msg::namespace(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Msg_oneof_msg::namespace(NamespaceMsg::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::namespace(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_namespace(&mut self) -> NamespaceMsg {
        if self.has_namespace() {
            match self.msg.take() {
                ::std::option::Option::Some(Msg_oneof_msg::namespace(v)) => v,
                _ => panic!(),
            }
        } else {
            NamespaceMsg::new()
        }
    }

    pub fn get_namespace(&self) -> &NamespaceMsg {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::namespace(ref v)) => v,
            _ => NamespaceMsg::default_instance(),
        }
    }

    // optional .AdminReq admin_req = 3;

    pub fn clear_admin_req(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_admin_req(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::admin_req(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_admin_req(&mut self, v: AdminReq) {
        self.msg = ::std::option::Option::Some(Msg_oneof_msg::admin_req(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin_req(&mut self) -> &mut AdminReq {
        if let ::std::option::Option::Some(Msg_oneof_msg::admin_req(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Msg_oneof_msg::admin_req(AdminReq::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::admin_req(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_admin_req(&mut self) -> AdminReq {
        if self.has_admin_req() {
            match self.msg.take() {
                ::std::option::Option::Some(Msg_oneof_msg::admin_req(v)) => v,
                _ => panic!(),
            }
        } else {
            AdminReq::new()
        }
    }

    pub fn get_admin_req(&self) -> &AdminReq {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::admin_req(ref v)) => v,
            _ => AdminReq::default_instance(),
        }
    }

    // optional .AdminRpy admin_rpy = 4;

    pub fn clear_admin_rpy(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_admin_rpy(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_admin_rpy(&mut self, v: AdminRpy) {
        self.msg = ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin_rpy(&mut self) -> &mut AdminRpy {
        if let ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(AdminRpy::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_admin_rpy(&mut self) -> AdminRpy {
        if self.has_admin_rpy() {
            match self.msg.take() {
                ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(v)) => v,
                _ => panic!(),
            }
        } else {
            AdminRpy::new()
        }
    }

    pub fn get_admin_rpy(&self) -> &AdminRpy {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(ref v)) => v,
            _ => AdminRpy::default_instance(),
        }
    }

    // optional .ApiRpy api_rpy = 5;

    pub fn clear_api_rpy(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_api_rpy(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::api_rpy(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_api_rpy(&mut self, v: ApiRpy) {
        self.msg = ::std::option::Option::Some(Msg_oneof_msg::api_rpy(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_rpy(&mut self) -> &mut ApiRpy {
        if let ::std::option::Option::Some(Msg_oneof_msg::api_rpy(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Msg_oneof_msg::api_rpy(ApiRpy::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::api_rpy(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_api_rpy(&mut self) -> ApiRpy {
        if self.has_api_rpy() {
            match self.msg.take() {
                ::std::option::Option::Some(Msg_oneof_msg::api_rpy(v)) => v,
                _ => panic!(),
            }
        } else {
            ApiRpy::new()
        }
    }

    pub fn get_api_rpy(&self) -> &ApiRpy {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::api_rpy(ref v)) => v,
            _ => ApiRpy::default_instance(),
        }
    }

    // optional string error = 6;

    pub fn clear_error(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.msg = ::std::option::Option::Some(Msg_oneof_msg::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Msg_oneof_msg::error(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(Msg_oneof_msg::error(::std::string::String::new()));
        }
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.msg.take() {
                ::std::option::Option::Some(Msg_oneof_msg::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.msg {
            ::std::option::Option::Some(Msg_oneof_msg::error(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for Msg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Msg_oneof_msg::vr(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Msg_oneof_msg::namespace(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Msg_oneof_msg::admin_req(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Msg_oneof_msg::admin_rpy(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Msg_oneof_msg::api_rpy(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(Msg_oneof_msg::error(try!(is.read_string())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Msg_oneof_msg::vr(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Msg_oneof_msg::namespace(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Msg_oneof_msg::admin_req(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Msg_oneof_msg::admin_rpy(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Msg_oneof_msg::api_rpy(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Msg_oneof_msg::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &Msg_oneof_msg::vr(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Msg_oneof_msg::namespace(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Msg_oneof_msg::admin_req(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Msg_oneof_msg::admin_rpy(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Msg_oneof_msg::api_rpy(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &Msg_oneof_msg::error(ref v) => {
                    try!(os.write_string(6, v));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Msg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Msg {
    fn new() -> Msg {
        Msg::new()
    }

    fn descriptor_static(_: ::std::option::Option<Msg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "vr",
                    Msg::has_vr,
                    Msg::get_vr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "namespace",
                    Msg::has_namespace,
                    Msg::get_namespace,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "admin_req",
                    Msg::has_admin_req,
                    Msg::get_admin_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "admin_rpy",
                    Msg::has_admin_rpy,
                    Msg::get_admin_rpy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "api_rpy",
                    Msg::has_api_rpy,
                    Msg::get_api_rpy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    Msg::has_error,
                    Msg::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Msg>(
                    "Msg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Msg {
    fn clear(&mut self) {
        self.clear_vr();
        self.clear_namespace();
        self.clear_admin_req();
        self.clear_admin_rpy();
        self.clear_api_rpy();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, other: &Msg) -> bool {
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VrMsg {
    // message oneof groups
    msg: ::std::option::Option<VrMsg_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VrMsg {}

#[derive(Clone,PartialEq)]
pub enum VrMsg_oneof_msg {
    tick(bool),
    client_request(ClientRequest),
    reconfiguration(VrReconfiguration),
    client_reply(ClientReply),
    start_view_change(StartViewChange),
    do_view_change(DoViewChange),
    start_view(StartView),
    prepare(Prepare),
    prepare_ok(PrepareOk),
    commit(Commit),
    get_state(GetState),
    new_state(NewState),
    recovery(Recovery),
    recovery_response(RecoveryResponse),
    start_epoch(StartEpoch),
    epoch_started(EpochStarted),
}

impl VrMsg {
    pub fn new() -> VrMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VrMsg {
        static mut instance: ::protobuf::lazy::Lazy<VrMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VrMsg,
        };
        unsafe {
            instance.get(|| {
                VrMsg {
                    msg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool tick = 1;

    pub fn clear_tick(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::tick(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: bool) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::tick(v))
    }

    pub fn get_tick(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::tick(v)) => v,
            _ => false,
        }
    }

    // optional .ClientRequest client_request = 2;

    pub fn clear_client_request(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_client_request(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::client_request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_client_request(&mut self, v: ClientRequest) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::client_request(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_request(&mut self) -> &mut ClientRequest {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::client_request(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::client_request(ClientRequest::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::client_request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_client_request(&mut self) -> ClientRequest {
        if self.has_client_request() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::client_request(v)) => v,
                _ => panic!(),
            }
        } else {
            ClientRequest::new()
        }
    }

    pub fn get_client_request(&self) -> &ClientRequest {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::client_request(ref v)) => v,
            _ => ClientRequest::default_instance(),
        }
    }

    // optional .VrReconfiguration reconfiguration = 3;

    pub fn clear_reconfiguration(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_reconfiguration(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_reconfiguration(&mut self, v: VrReconfiguration) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reconfiguration(&mut self) -> &mut VrReconfiguration {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(VrReconfiguration::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_reconfiguration(&mut self) -> VrReconfiguration {
        if self.has_reconfiguration() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(v)) => v,
                _ => panic!(),
            }
        } else {
            VrReconfiguration::new()
        }
    }

    pub fn get_reconfiguration(&self) -> &VrReconfiguration {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(ref v)) => v,
            _ => VrReconfiguration::default_instance(),
        }
    }

    // optional .ClientReply client_reply = 4;

    pub fn clear_client_reply(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_client_reply(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_client_reply(&mut self, v: ClientReply) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_reply(&mut self) -> &mut ClientReply {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(ClientReply::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_client_reply(&mut self) -> ClientReply {
        if self.has_client_reply() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(v)) => v,
                _ => panic!(),
            }
        } else {
            ClientReply::new()
        }
    }

    pub fn get_client_reply(&self) -> &ClientReply {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(ref v)) => v,
            _ => ClientReply::default_instance(),
        }
    }

    // optional .StartViewChange start_view_change = 5;

    pub fn clear_start_view_change(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_start_view_change(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_view_change(&mut self, v: StartViewChange) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_view_change(&mut self) -> &mut StartViewChange {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(StartViewChange::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_view_change(&mut self) -> StartViewChange {
        if self.has_start_view_change() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(v)) => v,
                _ => panic!(),
            }
        } else {
            StartViewChange::new()
        }
    }

    pub fn get_start_view_change(&self) -> &StartViewChange {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(ref v)) => v,
            _ => StartViewChange::default_instance(),
        }
    }

    // optional .DoViewChange do_view_change = 6;

    pub fn clear_do_view_change(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_do_view_change(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_do_view_change(&mut self, v: DoViewChange) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_do_view_change(&mut self) -> &mut DoViewChange {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(DoViewChange::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_do_view_change(&mut self) -> DoViewChange {
        if self.has_do_view_change() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(v)) => v,
                _ => panic!(),
            }
        } else {
            DoViewChange::new()
        }
    }

    pub fn get_do_view_change(&self) -> &DoViewChange {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(ref v)) => v,
            _ => DoViewChange::default_instance(),
        }
    }

    // optional .StartView start_view = 7;

    pub fn clear_start_view(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_start_view(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_view(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_view(&mut self, v: StartView) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_view(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_view(&mut self) -> &mut StartView {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::start_view(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_view(StartView::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_view(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_view(&mut self) -> StartView {
        if self.has_start_view() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::start_view(v)) => v,
                _ => panic!(),
            }
        } else {
            StartView::new()
        }
    }

    pub fn get_start_view(&self) -> &StartView {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_view(ref v)) => v,
            _ => StartView::default_instance(),
        }
    }

    // optional .Prepare prepare = 8;

    pub fn clear_prepare(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_prepare(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::prepare(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_prepare(&mut self, v: Prepare) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::prepare(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prepare(&mut self) -> &mut Prepare {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::prepare(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::prepare(Prepare::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::prepare(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_prepare(&mut self) -> Prepare {
        if self.has_prepare() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::prepare(v)) => v,
                _ => panic!(),
            }
        } else {
            Prepare::new()
        }
    }

    pub fn get_prepare(&self) -> &Prepare {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::prepare(ref v)) => v,
            _ => Prepare::default_instance(),
        }
    }

    // optional .PrepareOk prepare_ok = 9;

    pub fn clear_prepare_ok(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_prepare_ok(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_prepare_ok(&mut self, v: PrepareOk) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prepare_ok(&mut self) -> &mut PrepareOk {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(PrepareOk::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_prepare_ok(&mut self) -> PrepareOk {
        if self.has_prepare_ok() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(v)) => v,
                _ => panic!(),
            }
        } else {
            PrepareOk::new()
        }
    }

    pub fn get_prepare_ok(&self) -> &PrepareOk {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(ref v)) => v,
            _ => PrepareOk::default_instance(),
        }
    }

    // optional .Commit commit = 10;

    pub fn clear_commit(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::commit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: Commit) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::commit(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit(&mut self) -> &mut Commit {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::commit(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::commit(Commit::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::commit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_commit(&mut self) -> Commit {
        if self.has_commit() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::commit(v)) => v,
                _ => panic!(),
            }
        } else {
            Commit::new()
        }
    }

    pub fn get_commit(&self) -> &Commit {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::commit(ref v)) => v,
            _ => Commit::default_instance(),
        }
    }

    // optional .GetState get_state = 11;

    pub fn clear_get_state(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_get_state(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::get_state(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_state(&mut self, v: GetState) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::get_state(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_state(&mut self) -> &mut GetState {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::get_state(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::get_state(GetState::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::get_state(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_state(&mut self) -> GetState {
        if self.has_get_state() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::get_state(v)) => v,
                _ => panic!(),
            }
        } else {
            GetState::new()
        }
    }

    pub fn get_get_state(&self) -> &GetState {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::get_state(ref v)) => v,
            _ => GetState::default_instance(),
        }
    }

    // optional .NewState new_state = 12;

    pub fn clear_new_state(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_new_state(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::new_state(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_new_state(&mut self, v: NewState) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::new_state(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_state(&mut self) -> &mut NewState {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::new_state(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::new_state(NewState::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::new_state(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_new_state(&mut self) -> NewState {
        if self.has_new_state() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::new_state(v)) => v,
                _ => panic!(),
            }
        } else {
            NewState::new()
        }
    }

    pub fn get_new_state(&self) -> &NewState {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::new_state(ref v)) => v,
            _ => NewState::default_instance(),
        }
    }

    // optional .Recovery recovery = 13;

    pub fn clear_recovery(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_recovery(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::recovery(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_recovery(&mut self, v: Recovery) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::recovery(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_recovery(&mut self) -> &mut Recovery {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::recovery(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::recovery(Recovery::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::recovery(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_recovery(&mut self) -> Recovery {
        if self.has_recovery() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::recovery(v)) => v,
                _ => panic!(),
            }
        } else {
            Recovery::new()
        }
    }

    pub fn get_recovery(&self) -> &Recovery {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::recovery(ref v)) => v,
            _ => Recovery::default_instance(),
        }
    }

    // optional .RecoveryResponse recovery_response = 14;

    pub fn clear_recovery_response(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_recovery_response(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_recovery_response(&mut self, v: RecoveryResponse) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_recovery_response(&mut self) -> &mut RecoveryResponse {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(RecoveryResponse::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_recovery_response(&mut self) -> RecoveryResponse {
        if self.has_recovery_response() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(v)) => v,
                _ => panic!(),
            }
        } else {
            RecoveryResponse::new()
        }
    }

    pub fn get_recovery_response(&self) -> &RecoveryResponse {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(ref v)) => v,
            _ => RecoveryResponse::default_instance(),
        }
    }

    // optional .StartEpoch start_epoch = 15;

    pub fn clear_start_epoch(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_start_epoch(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_epoch(&mut self, v: StartEpoch) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_epoch(&mut self) -> &mut StartEpoch {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(StartEpoch::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_epoch(&mut self) -> StartEpoch {
        if self.has_start_epoch() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(v)) => v,
                _ => panic!(),
            }
        } else {
            StartEpoch::new()
        }
    }

    pub fn get_start_epoch(&self) -> &StartEpoch {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(ref v)) => v,
            _ => StartEpoch::default_instance(),
        }
    }

    // optional .EpochStarted epoch_started = 16;

    pub fn clear_epoch_started(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_epoch_started(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_epoch_started(&mut self, v: EpochStarted) {
        self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_epoch_started(&mut self) -> &mut EpochStarted {
        if let ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(EpochStarted::new()));
        }
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_epoch_started(&mut self) -> EpochStarted {
        if self.has_epoch_started() {
            match self.msg.take() {
                ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(v)) => v,
                _ => panic!(),
            }
        } else {
            EpochStarted::new()
        }
    }

    pub fn get_epoch_started(&self) -> &EpochStarted {
        match self.msg {
            ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(ref v)) => v,
            _ => EpochStarted::default_instance(),
        }
    }
}

impl ::protobuf::Message for VrMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::tick(try!(is.read_bool())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::client_request(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::reconfiguration(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::client_reply(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_view_change(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::do_view_change(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_view(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::prepare(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::prepare_ok(try!(is.read_message())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::commit(try!(is.read_message())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::get_state(try!(is.read_message())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::new_state(try!(is.read_message())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::recovery(try!(is.read_message())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::recovery_response(try!(is.read_message())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::start_epoch(try!(is.read_message())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(VrMsg_oneof_msg::epoch_started(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &VrMsg_oneof_msg::tick(v) => {
                    my_size += 2;
                },
                &VrMsg_oneof_msg::client_request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::reconfiguration(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::client_reply(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::start_view_change(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::do_view_change(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::start_view(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::prepare(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::prepare_ok(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::commit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::get_state(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::new_state(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::recovery(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::recovery_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::start_epoch(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrMsg_oneof_msg::epoch_started(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &VrMsg_oneof_msg::tick(v) => {
                    try!(os.write_bool(1, v));
                },
                &VrMsg_oneof_msg::client_request(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::reconfiguration(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::client_reply(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::start_view_change(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::do_view_change(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::start_view(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::prepare(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::prepare_ok(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::commit(ref v) => {
                    try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::get_state(ref v) => {
                    try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::new_state(ref v) => {
                    try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::recovery(ref v) => {
                    try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::recovery_response(ref v) => {
                    try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::start_epoch(ref v) => {
                    try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrMsg_oneof_msg::epoch_started(ref v) => {
                    try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VrMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VrMsg {
    fn new() -> VrMsg {
        VrMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<VrMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "tick",
                    VrMsg::has_tick,
                    VrMsg::get_tick,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "client_request",
                    VrMsg::has_client_request,
                    VrMsg::get_client_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "reconfiguration",
                    VrMsg::has_reconfiguration,
                    VrMsg::get_reconfiguration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "client_reply",
                    VrMsg::has_client_reply,
                    VrMsg::get_client_reply,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "start_view_change",
                    VrMsg::has_start_view_change,
                    VrMsg::get_start_view_change,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "do_view_change",
                    VrMsg::has_do_view_change,
                    VrMsg::get_do_view_change,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "start_view",
                    VrMsg::has_start_view,
                    VrMsg::get_start_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "prepare",
                    VrMsg::has_prepare,
                    VrMsg::get_prepare,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "prepare_ok",
                    VrMsg::has_prepare_ok,
                    VrMsg::get_prepare_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "commit",
                    VrMsg::has_commit,
                    VrMsg::get_commit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_state",
                    VrMsg::has_get_state,
                    VrMsg::get_get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new_state",
                    VrMsg::has_new_state,
                    VrMsg::get_new_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "recovery",
                    VrMsg::has_recovery,
                    VrMsg::get_recovery,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "recovery_response",
                    VrMsg::has_recovery_response,
                    VrMsg::get_recovery_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "start_epoch",
                    VrMsg::has_start_epoch,
                    VrMsg::get_start_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "epoch_started",
                    VrMsg::has_epoch_started,
                    VrMsg::get_epoch_started,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VrMsg>(
                    "VrMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VrMsg {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_client_request();
        self.clear_reconfiguration();
        self.clear_client_reply();
        self.clear_start_view_change();
        self.clear_do_view_change();
        self.clear_start_view();
        self.clear_prepare();
        self.clear_prepare_ok();
        self.clear_commit();
        self.clear_get_state();
        self.clear_new_state();
        self.clear_recovery();
        self.clear_recovery_response();
        self.clear_start_epoch();
        self.clear_epoch_started();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VrMsg {
    fn eq(&self, other: &VrMsg) -> bool {
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VrMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientRequest {
    // message fields
    op: ::protobuf::SingularPtrField<VrApiReq>,
    client_id: ::protobuf::SingularField<::std::string::String>,
    request_num: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientRequest {}

impl ClientRequest {
    pub fn new() -> ClientRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientRequest {
        static mut instance: ::protobuf::lazy::Lazy<ClientRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientRequest,
        };
        unsafe {
            instance.get(|| {
                ClientRequest {
                    op: ::protobuf::SingularPtrField::none(),
                    client_id: ::protobuf::SingularField::none(),
                    request_num: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .VrApiReq op = 1;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: VrApiReq) {
        self.op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_op(&mut self) -> &mut VrApiReq {
        if self.op.is_none() {
            self.op.set_default();
        };
        self.op.as_mut().unwrap()
    }

    // Take field
    pub fn take_op(&mut self) -> VrApiReq {
        self.op.take().unwrap_or_else(|| VrApiReq::new())
    }

    pub fn get_op(&self) -> &VrApiReq {
        self.op.as_ref().unwrap_or_else(|| VrApiReq::default_instance())
    }

    // optional string client_id = 2;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::string::String) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::string::String {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::string::String {
        self.client_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_client_id(&self) -> &str {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 request_num = 3;

    pub fn clear_request_num(&mut self) {
        self.request_num = ::std::option::Option::None;
    }

    pub fn has_request_num(&self) -> bool {
        self.request_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_num(&mut self, v: u64) {
        self.request_num = ::std::option::Option::Some(v);
    }

    pub fn get_request_num(&self) -> u64 {
        self.request_num.unwrap_or(0)
    }
}

impl ::protobuf::Message for ClientRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.op));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.request_num = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.op {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.client_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.request_num {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.op.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.client_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.request_num {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientRequest {
    fn new() -> ClientRequest {
        ClientRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "op",
                    ClientRequest::has_op,
                    ClientRequest::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "client_id",
                    ClientRequest::has_client_id,
                    ClientRequest::get_client_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "request_num",
                    ClientRequest::has_request_num,
                    ClientRequest::get_request_num,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientRequest>(
                    "ClientRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientRequest {
    fn clear(&mut self) {
        self.clear_op();
        self.clear_client_id();
        self.clear_request_num();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientRequest {
    fn eq(&self, other: &ClientRequest) -> bool {
        self.op == other.op &&
        self.client_id == other.client_id &&
        self.request_num == other.request_num &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VrReconfiguration {
    // message fields
    client_req_num: ::std::option::Option<u64>,
    epoch: ::std::option::Option<u64>,
    replicas: ::protobuf::RepeatedField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VrReconfiguration {}

impl VrReconfiguration {
    pub fn new() -> VrReconfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VrReconfiguration {
        static mut instance: ::protobuf::lazy::Lazy<VrReconfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VrReconfiguration,
        };
        unsafe {
            instance.get(|| {
                VrReconfiguration {
                    client_req_num: ::std::option::Option::None,
                    epoch: ::std::option::Option::None,
                    replicas: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 client_req_num = 1;

    pub fn clear_client_req_num(&mut self) {
        self.client_req_num = ::std::option::Option::None;
    }

    pub fn has_client_req_num(&self) -> bool {
        self.client_req_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_req_num(&mut self, v: u64) {
        self.client_req_num = ::std::option::Option::Some(v);
    }

    pub fn get_client_req_num(&self) -> u64 {
        self.client_req_num.unwrap_or(0)
    }

    // optional uint64 epoch = 2;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // repeated .Pid replicas = 3;

    pub fn clear_replicas(&mut self) {
        self.replicas.clear();
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: ::protobuf::RepeatedField<super::rabble_messages::Pid>) {
        self.replicas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_replicas(&mut self) -> &mut ::protobuf::RepeatedField<super::rabble_messages::Pid> {
        &mut self.replicas
    }

    // Take field
    pub fn take_replicas(&mut self) -> ::protobuf::RepeatedField<super::rabble_messages::Pid> {
        ::std::mem::replace(&mut self.replicas, ::protobuf::RepeatedField::new())
    }

    pub fn get_replicas(&self) -> &[super::rabble_messages::Pid] {
        &self.replicas
    }
}

impl ::protobuf::Message for VrReconfiguration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.client_req_num = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replicas));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.client_req_num {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.replicas {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_req_num {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.epoch {
            try!(os.write_uint64(2, v));
        };
        for v in &self.replicas {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VrReconfiguration>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VrReconfiguration {
    fn new() -> VrReconfiguration {
        VrReconfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<VrReconfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "client_req_num",
                    VrReconfiguration::has_client_req_num,
                    VrReconfiguration::get_client_req_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    VrReconfiguration::has_epoch,
                    VrReconfiguration::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "replicas",
                    VrReconfiguration::get_replicas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VrReconfiguration>(
                    "VrReconfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VrReconfiguration {
    fn clear(&mut self) {
        self.clear_client_req_num();
        self.clear_epoch();
        self.clear_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VrReconfiguration {
    fn eq(&self, other: &VrReconfiguration) -> bool {
        self.client_req_num == other.client_req_num &&
        self.epoch == other.epoch &&
        self.replicas == other.replicas &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VrReconfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientReply {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    request_num: ::std::option::Option<u64>,
    value: ::protobuf::SingularPtrField<VrApiRsp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientReply {}

impl ClientReply {
    pub fn new() -> ClientReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientReply {
        static mut instance: ::protobuf::lazy::Lazy<ClientReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientReply,
        };
        unsafe {
            instance.get(|| {
                ClientReply {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    request_num: ::std::option::Option::None,
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 request_num = 3;

    pub fn clear_request_num(&mut self) {
        self.request_num = ::std::option::Option::None;
    }

    pub fn has_request_num(&self) -> bool {
        self.request_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_num(&mut self, v: u64) {
        self.request_num = ::std::option::Option::Some(v);
    }

    pub fn get_request_num(&self) -> u64 {
        self.request_num.unwrap_or(0)
    }

    // optional .VrApiRsp value = 4;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: VrApiRsp) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut VrApiRsp {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> VrApiRsp {
        self.value.take().unwrap_or_else(|| VrApiRsp::new())
    }

    pub fn get_value(&self) -> &VrApiRsp {
        self.value.as_ref().unwrap_or_else(|| VrApiRsp::default_instance())
    }
}

impl ::protobuf::Message for ClientReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.request_num = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.request_num {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.request_num {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientReply {
    fn new() -> ClientReply {
        ClientReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    ClientReply::has_epoch,
                    ClientReply::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    ClientReply::has_view,
                    ClientReply::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "request_num",
                    ClientReply::has_request_num,
                    ClientReply::get_request_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    ClientReply::has_value,
                    ClientReply::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientReply>(
                    "ClientReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientReply {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_request_num();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientReply {
    fn eq(&self, other: &ClientReply) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.request_num == other.request_num &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StartViewChange {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartViewChange {}

impl StartViewChange {
    pub fn new() -> StartViewChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartViewChange {
        static mut instance: ::protobuf::lazy::Lazy<StartViewChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartViewChange,
        };
        unsafe {
            instance.get(|| {
                StartViewChange {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    from: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional .Pid from = 4;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }
}

impl ::protobuf::Message for StartViewChange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StartViewChange>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartViewChange {
    fn new() -> StartViewChange {
        StartViewChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartViewChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    StartViewChange::has_epoch,
                    StartViewChange::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    StartViewChange::has_view,
                    StartViewChange::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    StartViewChange::has_op,
                    StartViewChange::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    StartViewChange::has_from,
                    StartViewChange::get_from,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartViewChange>(
                    "StartViewChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartViewChange {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_from();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartViewChange {
    fn eq(&self, other: &StartViewChange) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.from == other.from &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartViewChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DoViewChange {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    last_normal_view: ::std::option::Option<u64>,
    log: ::protobuf::RepeatedField<VrMsg>,
    commit_num: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoViewChange {}

impl DoViewChange {
    pub fn new() -> DoViewChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoViewChange {
        static mut instance: ::protobuf::lazy::Lazy<DoViewChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoViewChange,
        };
        unsafe {
            instance.get(|| {
                DoViewChange {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    from: ::protobuf::SingularPtrField::none(),
                    last_normal_view: ::std::option::Option::None,
                    log: ::protobuf::RepeatedField::new(),
                    commit_num: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional .Pid from = 4;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional uint64 last_normal_view = 5;

    pub fn clear_last_normal_view(&mut self) {
        self.last_normal_view = ::std::option::Option::None;
    }

    pub fn has_last_normal_view(&self) -> bool {
        self.last_normal_view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_normal_view(&mut self, v: u64) {
        self.last_normal_view = ::std::option::Option::Some(v);
    }

    pub fn get_last_normal_view(&self) -> u64 {
        self.last_normal_view.unwrap_or(0)
    }

    // repeated .VrMsg log = 6;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::protobuf::RepeatedField<VrMsg>) {
        self.log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log(&mut self) -> &mut ::protobuf::RepeatedField<VrMsg> {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::protobuf::RepeatedField<VrMsg> {
        ::std::mem::replace(&mut self.log, ::protobuf::RepeatedField::new())
    }

    pub fn get_log(&self) -> &[VrMsg] {
        &self.log
    }

    // optional uint64 commit_num = 7;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }
}

impl ::protobuf::Message for DoViewChange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_normal_view = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.last_normal_view {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.log {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.last_normal_view {
            try!(os.write_uint64(5, v));
        };
        for v in &self.log {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(7, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DoViewChange>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DoViewChange {
    fn new() -> DoViewChange {
        DoViewChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoViewChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    DoViewChange::has_epoch,
                    DoViewChange::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    DoViewChange::has_view,
                    DoViewChange::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    DoViewChange::has_op,
                    DoViewChange::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    DoViewChange::has_from,
                    DoViewChange::get_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_normal_view",
                    DoViewChange::has_last_normal_view,
                    DoViewChange::get_last_normal_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "log",
                    DoViewChange::get_log,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    DoViewChange::has_commit_num,
                    DoViewChange::get_commit_num,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoViewChange>(
                    "DoViewChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoViewChange {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_from();
        self.clear_last_normal_view();
        self.clear_log();
        self.clear_commit_num();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DoViewChange {
    fn eq(&self, other: &DoViewChange) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.from == other.from &&
        self.last_normal_view == other.last_normal_view &&
        self.log == other.log &&
        self.commit_num == other.commit_num &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DoViewChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StartView {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    log: ::protobuf::RepeatedField<VrMsg>,
    commit_num: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartView {}

impl StartView {
    pub fn new() -> StartView {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartView {
        static mut instance: ::protobuf::lazy::Lazy<StartView> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartView,
        };
        unsafe {
            instance.get(|| {
                StartView {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    log: ::protobuf::RepeatedField::new(),
                    commit_num: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // repeated .VrMsg log = 4;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::protobuf::RepeatedField<VrMsg>) {
        self.log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log(&mut self) -> &mut ::protobuf::RepeatedField<VrMsg> {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::protobuf::RepeatedField<VrMsg> {
        ::std::mem::replace(&mut self.log, ::protobuf::RepeatedField::new())
    }

    pub fn get_log(&self) -> &[VrMsg] {
        &self.log
    }

    // optional uint64 commit_num = 5;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }
}

impl ::protobuf::Message for StartView {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.log {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        for v in &self.log {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(5, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StartView>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartView {
    fn new() -> StartView {
        StartView::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartView>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    StartView::has_epoch,
                    StartView::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    StartView::has_view,
                    StartView::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    StartView::has_op,
                    StartView::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "log",
                    StartView::get_log,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    StartView::has_commit_num,
                    StartView::get_commit_num,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartView>(
                    "StartView",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartView {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_log();
        self.clear_commit_num();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartView {
    fn eq(&self, other: &StartView) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.log == other.log &&
        self.commit_num == other.commit_num &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Prepare {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    commit_num: ::std::option::Option<u64>,
    msg: ::protobuf::SingularPtrField<VrMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Prepare {}

impl Prepare {
    pub fn new() -> Prepare {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Prepare {
        static mut instance: ::protobuf::lazy::Lazy<Prepare> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Prepare,
        };
        unsafe {
            instance.get(|| {
                Prepare {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    commit_num: ::std::option::Option::None,
                    msg: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional uint64 commit_num = 4;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }

    // optional .VrMsg msg = 5;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: VrMsg) {
        self.msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut VrMsg {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> VrMsg {
        self.msg.take().unwrap_or_else(|| VrMsg::new())
    }

    pub fn get_msg(&self) -> &VrMsg {
        self.msg.as_ref().unwrap_or_else(|| VrMsg::default_instance())
    }
}

impl ::protobuf::Message for Prepare {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.msg));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.msg {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.msg.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Prepare>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Prepare {
    fn new() -> Prepare {
        Prepare::new()
    }

    fn descriptor_static(_: ::std::option::Option<Prepare>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    Prepare::has_epoch,
                    Prepare::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    Prepare::has_view,
                    Prepare::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    Prepare::has_op,
                    Prepare::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    Prepare::has_commit_num,
                    Prepare::get_commit_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "msg",
                    Prepare::has_msg,
                    Prepare::get_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Prepare>(
                    "Prepare",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Prepare {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_commit_num();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Prepare {
    fn eq(&self, other: &Prepare) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.commit_num == other.commit_num &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Prepare {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PrepareOk {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrepareOk {}

impl PrepareOk {
    pub fn new() -> PrepareOk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrepareOk {
        static mut instance: ::protobuf::lazy::Lazy<PrepareOk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrepareOk,
        };
        unsafe {
            instance.get(|| {
                PrepareOk {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    from: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional .Pid from = 4;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }
}

impl ::protobuf::Message for PrepareOk {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PrepareOk>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PrepareOk {
    fn new() -> PrepareOk {
        PrepareOk::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrepareOk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    PrepareOk::has_epoch,
                    PrepareOk::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    PrepareOk::has_view,
                    PrepareOk::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    PrepareOk::has_op,
                    PrepareOk::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    PrepareOk::has_from,
                    PrepareOk::get_from,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrepareOk>(
                    "PrepareOk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrepareOk {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_from();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PrepareOk {
    fn eq(&self, other: &PrepareOk) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.from == other.from &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PrepareOk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Commit {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    commit_num: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Commit {}

impl Commit {
    pub fn new() -> Commit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Commit {
        static mut instance: ::protobuf::lazy::Lazy<Commit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Commit,
        };
        unsafe {
            instance.get(|| {
                Commit {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    commit_num: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 commit_num = 3;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }
}

impl ::protobuf::Message for Commit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Commit>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Commit {
    fn new() -> Commit {
        Commit::new()
    }

    fn descriptor_static(_: ::std::option::Option<Commit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    Commit::has_epoch,
                    Commit::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    Commit::has_view,
                    Commit::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    Commit::has_commit_num,
                    Commit::get_commit_num,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Commit>(
                    "Commit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Commit {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_commit_num();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Commit {
    fn eq(&self, other: &Commit) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.commit_num == other.commit_num &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Commit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetState {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetState {}

impl GetState {
    pub fn new() -> GetState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetState {
        static mut instance: ::protobuf::lazy::Lazy<GetState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetState,
        };
        unsafe {
            instance.get(|| {
                GetState {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    from: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional .Pid from = 4;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }
}

impl ::protobuf::Message for GetState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetState {
    fn new() -> GetState {
        GetState::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    GetState::has_epoch,
                    GetState::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    GetState::has_view,
                    GetState::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    GetState::has_op,
                    GetState::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    GetState::has_from,
                    GetState::get_from,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetState>(
                    "GetState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetState {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_from();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetState {
    fn eq(&self, other: &GetState) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.from == other.from &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NewState {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    primary: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    commit_num: ::std::option::Option<u64>,
    log_tail: ::protobuf::RepeatedField<VrMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewState {}

impl NewState {
    pub fn new() -> NewState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewState {
        static mut instance: ::protobuf::lazy::Lazy<NewState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewState,
        };
        unsafe {
            instance.get(|| {
                NewState {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    primary: ::protobuf::SingularPtrField::none(),
                    commit_num: ::std::option::Option::None,
                    log_tail: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 3;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional .Pid primary = 4;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: super::rabble_messages::Pid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut super::rabble_messages::Pid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> super::rabble_messages::Pid {
        self.primary.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_primary(&self) -> &super::rabble_messages::Pid {
        self.primary.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional uint64 commit_num = 5;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }

    // repeated .VrMsg log_tail = 6;

    pub fn clear_log_tail(&mut self) {
        self.log_tail.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_tail(&mut self, v: ::protobuf::RepeatedField<VrMsg>) {
        self.log_tail = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log_tail(&mut self) -> &mut ::protobuf::RepeatedField<VrMsg> {
        &mut self.log_tail
    }

    // Take field
    pub fn take_log_tail(&mut self) -> ::protobuf::RepeatedField<VrMsg> {
        ::std::mem::replace(&mut self.log_tail, ::protobuf::RepeatedField::new())
    }

    pub fn get_log_tail(&self) -> &[VrMsg] {
        &self.log_tail
    }
}

impl ::protobuf::Message for NewState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log_tail));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.primary {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.log_tail {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(5, v));
        };
        for v in &self.log_tail {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<NewState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NewState {
    fn new() -> NewState {
        NewState::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    NewState::has_epoch,
                    NewState::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    NewState::has_view,
                    NewState::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    NewState::has_op,
                    NewState::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "primary",
                    NewState::has_primary,
                    NewState::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    NewState::has_commit_num,
                    NewState::get_commit_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "log_tail",
                    NewState::get_log_tail,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewState>(
                    "NewState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewState {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_primary();
        self.clear_commit_num();
        self.clear_log_tail();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NewState {
    fn eq(&self, other: &NewState) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.primary == other.primary &&
        self.commit_num == other.commit_num &&
        self.log_tail == other.log_tail &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NewState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Recovery {
    // message fields
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    nonce: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Recovery {}

impl Recovery {
    pub fn new() -> Recovery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Recovery {
        static mut instance: ::protobuf::lazy::Lazy<Recovery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Recovery,
        };
        unsafe {
            instance.get(|| {
                Recovery {
                    from: ::protobuf::SingularPtrField::none(),
                    nonce: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Pid from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional string nonce = 2;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        if self.nonce.is_none() {
            self.nonce.set_default();
        };
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        self.nonce.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Recovery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nonce));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.nonce {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.nonce.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Recovery>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Recovery {
    fn new() -> Recovery {
        Recovery::new()
    }

    fn descriptor_static(_: ::std::option::Option<Recovery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    Recovery::has_from,
                    Recovery::get_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "nonce",
                    Recovery::has_nonce,
                    Recovery::get_nonce,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Recovery>(
                    "Recovery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Recovery {
    fn clear(&mut self) {
        self.clear_from();
        self.clear_nonce();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Recovery {
    fn eq(&self, other: &Recovery) -> bool {
        self.from == other.from &&
        self.nonce == other.nonce &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Recovery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RecoveryResponse {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    nonce: ::protobuf::SingularField<::std::string::String>,
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    op: ::std::option::Option<u64>,
    commit_num: ::std::option::Option<u64>,
    log: ::protobuf::RepeatedField<VrMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecoveryResponse {}

impl RecoveryResponse {
    pub fn new() -> RecoveryResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecoveryResponse {
        static mut instance: ::protobuf::lazy::Lazy<RecoveryResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecoveryResponse,
        };
        unsafe {
            instance.get(|| {
                RecoveryResponse {
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    nonce: ::protobuf::SingularField::none(),
                    from: ::protobuf::SingularPtrField::none(),
                    op: ::std::option::Option::None,
                    commit_num: ::std::option::Option::None,
                    log: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 2;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional string nonce = 3;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        if self.nonce.is_none() {
            self.nonce.set_default();
        };
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        self.nonce.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Pid from = 4;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional uint64 op = 5;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional uint64 commit_num = 6;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }

    // repeated .VrMsg log = 7;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::protobuf::RepeatedField<VrMsg>) {
        self.log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log(&mut self) -> &mut ::protobuf::RepeatedField<VrMsg> {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::protobuf::RepeatedField<VrMsg> {
        ::std::mem::replace(&mut self.log, ::protobuf::RepeatedField::new())
    }

    pub fn get_log(&self) -> &[VrMsg] {
        &self.log
    }
}

impl ::protobuf::Message for RecoveryResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nonce));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.nonce {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.log {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.nonce.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(6, v));
        };
        for v in &self.log {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RecoveryResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RecoveryResponse {
    fn new() -> RecoveryResponse {
        RecoveryResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecoveryResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    RecoveryResponse::has_epoch,
                    RecoveryResponse::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    RecoveryResponse::has_view,
                    RecoveryResponse::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "nonce",
                    RecoveryResponse::has_nonce,
                    RecoveryResponse::get_nonce,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    RecoveryResponse::has_from,
                    RecoveryResponse::get_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    RecoveryResponse::has_op,
                    RecoveryResponse::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    RecoveryResponse::has_commit_num,
                    RecoveryResponse::get_commit_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "log",
                    RecoveryResponse::get_log,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecoveryResponse>(
                    "RecoveryResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecoveryResponse {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_nonce();
        self.clear_from();
        self.clear_op();
        self.clear_commit_num();
        self.clear_log();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RecoveryResponse {
    fn eq(&self, other: &RecoveryResponse) -> bool {
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.nonce == other.nonce &&
        self.from == other.from &&
        self.op == other.op &&
        self.commit_num == other.commit_num &&
        self.log == other.log &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RecoveryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StartEpoch {
    // message fields
    epoch: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    old_config: ::protobuf::SingularPtrField<VersionedReplicas>,
    new_config: ::protobuf::SingularPtrField<VersionedReplicas>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartEpoch {}

impl StartEpoch {
    pub fn new() -> StartEpoch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartEpoch {
        static mut instance: ::protobuf::lazy::Lazy<StartEpoch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartEpoch,
        };
        unsafe {
            instance.get(|| {
                StartEpoch {
                    epoch: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    old_config: ::protobuf::SingularPtrField::none(),
                    new_config: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 op = 2;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional .VersionedReplicas old_config = 3;

    pub fn clear_old_config(&mut self) {
        self.old_config.clear();
    }

    pub fn has_old_config(&self) -> bool {
        self.old_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_config(&mut self, v: VersionedReplicas) {
        self.old_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_config(&mut self) -> &mut VersionedReplicas {
        if self.old_config.is_none() {
            self.old_config.set_default();
        };
        self.old_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_config(&mut self) -> VersionedReplicas {
        self.old_config.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_old_config(&self) -> &VersionedReplicas {
        self.old_config.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }

    // optional .VersionedReplicas new_config = 4;

    pub fn clear_new_config(&mut self) {
        self.new_config.clear();
    }

    pub fn has_new_config(&self) -> bool {
        self.new_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_config(&mut self, v: VersionedReplicas) {
        self.new_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_config(&mut self) -> &mut VersionedReplicas {
        if self.new_config.is_none() {
            self.new_config.set_default();
        };
        self.new_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_config(&mut self) -> VersionedReplicas {
        self.new_config.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_new_config(&self) -> &VersionedReplicas {
        self.new_config.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }
}

impl ::protobuf::Message for StartEpoch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_config));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new_config));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.old_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.new_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.old_config.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.new_config.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StartEpoch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartEpoch {
    fn new() -> StartEpoch {
        StartEpoch::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartEpoch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    StartEpoch::has_epoch,
                    StartEpoch::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    StartEpoch::has_op,
                    StartEpoch::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "old_config",
                    StartEpoch::has_old_config,
                    StartEpoch::get_old_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new_config",
                    StartEpoch::has_new_config,
                    StartEpoch::get_new_config,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartEpoch>(
                    "StartEpoch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartEpoch {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_op();
        self.clear_old_config();
        self.clear_new_config();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartEpoch {
    fn eq(&self, other: &StartEpoch) -> bool {
        self.epoch == other.epoch &&
        self.op == other.op &&
        self.old_config == other.old_config &&
        self.new_config == other.new_config &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartEpoch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EpochStarted {
    // message fields
    epoch: ::std::option::Option<u64>,
    from: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EpochStarted {}

impl EpochStarted {
    pub fn new() -> EpochStarted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EpochStarted {
        static mut instance: ::protobuf::lazy::Lazy<EpochStarted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EpochStarted,
        };
        unsafe {
            instance.get(|| {
                EpochStarted {
                    epoch: ::std::option::Option::None,
                    from: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional .Pid from = 2;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: super::rabble_messages::Pid) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut super::rabble_messages::Pid {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> super::rabble_messages::Pid {
        self.from.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_from(&self) -> &super::rabble_messages::Pid {
        self.from.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }
}

impl ::protobuf::Message for EpochStarted {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<EpochStarted>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EpochStarted {
    fn new() -> EpochStarted {
        EpochStarted::new()
    }

    fn descriptor_static(_: ::std::option::Option<EpochStarted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    EpochStarted::has_epoch,
                    EpochStarted::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    EpochStarted::has_from,
                    EpochStarted::get_from,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EpochStarted>(
                    "EpochStarted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EpochStarted {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_from();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EpochStarted {
    fn eq(&self, other: &EpochStarted) -> bool {
        self.epoch == other.epoch &&
        self.from == other.from &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EpochStarted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VrApiReq {
    // message oneof groups
    req: ::std::option::Option<VrApiReq_oneof_req>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VrApiReq {}

#[derive(Clone,PartialEq)]
pub enum VrApiReq_oneof_req {
    tree_op(TreeOp),
    tree_cas(TreeCas),
}

impl VrApiReq {
    pub fn new() -> VrApiReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VrApiReq {
        static mut instance: ::protobuf::lazy::Lazy<VrApiReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VrApiReq,
        };
        unsafe {
            instance.get(|| {
                VrApiReq {
                    req: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .TreeOp tree_op = 1;

    pub fn clear_tree_op(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_tree_op(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_op(&mut self, v: TreeOp) {
        self.req = ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree_op(&mut self) -> &mut TreeOp {
        if let ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(TreeOp::new()));
        }
        match self.req {
            ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_op(&mut self) -> TreeOp {
        if self.has_tree_op() {
            match self.req.take() {
                ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeOp::new()
        }
    }

    pub fn get_tree_op(&self) -> &TreeOp {
        match self.req {
            ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(ref v)) => v,
            _ => TreeOp::default_instance(),
        }
    }

    // optional .TreeCas tree_cas = 2;

    pub fn clear_tree_cas(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_tree_cas(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_cas(&mut self, v: TreeCas) {
        self.req = ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree_cas(&mut self) -> &mut TreeCas {
        if let ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(TreeCas::new()));
        }
        match self.req {
            ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_cas(&mut self) -> TreeCas {
        if self.has_tree_cas() {
            match self.req.take() {
                ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeCas::new()
        }
    }

    pub fn get_tree_cas(&self) -> &TreeCas {
        match self.req {
            ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(ref v)) => v,
            _ => TreeCas::default_instance(),
        }
    }
}

impl ::protobuf::Message for VrApiReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(VrApiReq_oneof_req::tree_op(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(VrApiReq_oneof_req::tree_cas(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.req {
            match v {
                &VrApiReq_oneof_req::tree_op(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrApiReq_oneof_req::tree_cas(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.req {
            match v {
                &VrApiReq_oneof_req::tree_op(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrApiReq_oneof_req::tree_cas(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VrApiReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VrApiReq {
    fn new() -> VrApiReq {
        VrApiReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<VrApiReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tree_op",
                    VrApiReq::has_tree_op,
                    VrApiReq::get_tree_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tree_cas",
                    VrApiReq::has_tree_cas,
                    VrApiReq::get_tree_cas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VrApiReq>(
                    "VrApiReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VrApiReq {
    fn clear(&mut self) {
        self.clear_tree_op();
        self.clear_tree_cas();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VrApiReq {
    fn eq(&self, other: &VrApiReq) -> bool {
        self.req == other.req &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VrApiReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TreeCas {
    // message fields
    guards: ::protobuf::RepeatedField<Guard>,
    ops: ::protobuf::RepeatedField<TreeOp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TreeCas {}

impl TreeCas {
    pub fn new() -> TreeCas {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TreeCas {
        static mut instance: ::protobuf::lazy::Lazy<TreeCas> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TreeCas,
        };
        unsafe {
            instance.get(|| {
                TreeCas {
                    guards: ::protobuf::RepeatedField::new(),
                    ops: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Guard guards = 1;

    pub fn clear_guards(&mut self) {
        self.guards.clear();
    }

    // Param is passed by value, moved
    pub fn set_guards(&mut self, v: ::protobuf::RepeatedField<Guard>) {
        self.guards = v;
    }

    // Mutable pointer to the field.
    pub fn mut_guards(&mut self) -> &mut ::protobuf::RepeatedField<Guard> {
        &mut self.guards
    }

    // Take field
    pub fn take_guards(&mut self) -> ::protobuf::RepeatedField<Guard> {
        ::std::mem::replace(&mut self.guards, ::protobuf::RepeatedField::new())
    }

    pub fn get_guards(&self) -> &[Guard] {
        &self.guards
    }

    // repeated .TreeOp ops = 2;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::protobuf::RepeatedField<TreeOp>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::protobuf::RepeatedField<TreeOp> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::protobuf::RepeatedField<TreeOp> {
        ::std::mem::replace(&mut self.ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_ops(&self) -> &[TreeOp] {
        &self.ops
    }
}

impl ::protobuf::Message for TreeCas {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.guards));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.guards {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.guards {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.ops {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TreeCas>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TreeCas {
    fn new() -> TreeCas {
        TreeCas::new()
    }

    fn descriptor_static(_: ::std::option::Option<TreeCas>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "guards",
                    TreeCas::get_guards,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ops",
                    TreeCas::get_ops,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TreeCas>(
                    "TreeCas",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TreeCas {
    fn clear(&mut self) {
        self.clear_guards();
        self.clear_ops();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TreeCas {
    fn eq(&self, other: &TreeCas) -> bool {
        self.guards == other.guards &&
        self.ops == other.ops &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TreeCas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Guard {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Guard {}

impl Guard {
    pub fn new() -> Guard {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Guard {
        static mut instance: ::protobuf::lazy::Lazy<Guard> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Guard,
        };
        unsafe {
            instance.get(|| {
                Guard {
                    path: ::protobuf::SingularField::none(),
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }
}

impl ::protobuf::Message for Guard {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.version {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.version {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Guard>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Guard {
    fn new() -> Guard {
        Guard::new()
    }

    fn descriptor_static(_: ::std::option::Option<Guard>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    Guard::has_path,
                    Guard::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "version",
                    Guard::has_version,
                    Guard::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Guard>(
                    "Guard",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Guard {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Guard {
    fn eq(&self, other: &Guard) -> bool {
        self.path == other.path &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TreeOp {
    // message oneof groups
    op: ::std::option::Option<TreeOp_oneof_op>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TreeOp {}

#[derive(Clone,PartialEq)]
pub enum TreeOp_oneof_op {
    snapshot(::std::string::String),
    create_node(CreateNode),
    delete_node(::std::string::String),
    list_keys(::std::string::String),
    blob_put(BlobPut),
    blob_get(::std::string::String),
    blob_size(::std::string::String),
    queue_push(QueuePush),
    queue_pop(::std::string::String),
    queue_front(::std::string::String),
    queue_back(::std::string::String),
    queue_len(::std::string::String),
    set_insert(SetInsert),
    set_remove(SetRemove),
    set_contains(SetContains),
    set_union(SetUnion),
    set_intersection(SetIntersection),
    set_difference(SetDifference),
    set_symmetric_difference(SetSymmetricDifference),
    set_subset_path(SetSubsetPath),
    set_subset_set(SetSubsetSet),
    set_superset_path(SetSupersetPath),
    set_superset_set(SetSupersetSet),
}

impl TreeOp {
    pub fn new() -> TreeOp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TreeOp {
        static mut instance: ::protobuf::lazy::Lazy<TreeOp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TreeOp,
        };
        unsafe {
            instance.get(|| {
                TreeOp {
                    op: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string snapshot = 1;

    pub fn clear_snapshot(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_snapshot(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::snapshot(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_snapshot(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::snapshot(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshot(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::snapshot(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::snapshot(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::snapshot(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_snapshot(&mut self) -> ::std::string::String {
        if self.has_snapshot() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::snapshot(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_snapshot(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::snapshot(ref v)) => v,
            _ => "",
        }
    }

    // optional .CreateNode create_node = 2;

    pub fn clear_create_node(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_create_node(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::create_node(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_node(&mut self, v: CreateNode) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::create_node(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_node(&mut self) -> &mut CreateNode {
        if let ::std::option::Option::Some(TreeOp_oneof_op::create_node(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::create_node(CreateNode::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::create_node(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_node(&mut self) -> CreateNode {
        if self.has_create_node() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::create_node(v)) => v,
                _ => panic!(),
            }
        } else {
            CreateNode::new()
        }
    }

    pub fn get_create_node(&self) -> &CreateNode {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::create_node(ref v)) => v,
            _ => CreateNode::default_instance(),
        }
    }

    // optional string delete_node = 3;

    pub fn clear_delete_node(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_delete_node(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::delete_node(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_node(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::delete_node(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_node(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::delete_node(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::delete_node(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::delete_node(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_node(&mut self) -> ::std::string::String {
        if self.has_delete_node() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::delete_node(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_delete_node(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::delete_node(ref v)) => v,
            _ => "",
        }
    }

    // optional string list_keys = 4;

    pub fn clear_list_keys(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_list_keys(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::list_keys(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_list_keys(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::list_keys(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_keys(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::list_keys(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::list_keys(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::list_keys(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_list_keys(&mut self) -> ::std::string::String {
        if self.has_list_keys() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::list_keys(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_list_keys(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::list_keys(ref v)) => v,
            _ => "",
        }
    }

    // optional .BlobPut blob_put = 5;

    pub fn clear_blob_put(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_blob_put(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_blob_put(&mut self, v: BlobPut) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob_put(&mut self) -> &mut BlobPut {
        if let ::std::option::Option::Some(TreeOp_oneof_op::blob_put(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_put(BlobPut::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_blob_put(&mut self) -> BlobPut {
        if self.has_blob_put() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::blob_put(v)) => v,
                _ => panic!(),
            }
        } else {
            BlobPut::new()
        }
    }

    pub fn get_blob_put(&self) -> &BlobPut {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_put(ref v)) => v,
            _ => BlobPut::default_instance(),
        }
    }

    // optional string blob_get = 6;

    pub fn clear_blob_get(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_blob_get(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_get(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_blob_get(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_get(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob_get(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::blob_get(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_get(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_blob_get(&mut self) -> ::std::string::String {
        if self.has_blob_get() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::blob_get(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_blob_get(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_get(ref v)) => v,
            _ => "",
        }
    }

    // optional string blob_size = 7;

    pub fn clear_blob_size(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_blob_size(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_size(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_blob_size(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_size(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob_size(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::blob_size(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_size(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_size(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_blob_size(&mut self) -> ::std::string::String {
        if self.has_blob_size() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::blob_size(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_blob_size(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_size(ref v)) => v,
            _ => "",
        }
    }

    // optional .QueuePush queue_push = 8;

    pub fn clear_queue_push(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_queue_push(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_push(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_queue_push(&mut self, v: QueuePush) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_push(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_queue_push(&mut self) -> &mut QueuePush {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_push(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_push(QueuePush::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_push(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_push(&mut self) -> QueuePush {
        if self.has_queue_push() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_push(v)) => v,
                _ => panic!(),
            }
        } else {
            QueuePush::new()
        }
    }

    pub fn get_queue_push(&self) -> &QueuePush {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_push(ref v)) => v,
            _ => QueuePush::default_instance(),
        }
    }

    // optional string queue_pop = 9;

    pub fn clear_queue_pop(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_queue_pop(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_queue_pop(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_queue_pop(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_pop(&mut self) -> ::std::string::String {
        if self.has_queue_pop() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_queue_pop(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(ref v)) => v,
            _ => "",
        }
    }

    // optional string queue_front = 10;

    pub fn clear_queue_front(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_queue_front(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_front(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_queue_front(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_front(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_queue_front(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_front(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_front(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_front(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_front(&mut self) -> ::std::string::String {
        if self.has_queue_front() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_front(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_queue_front(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_front(ref v)) => v,
            _ => "",
        }
    }

    // optional string queue_back = 11;

    pub fn clear_queue_back(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_queue_back(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_back(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_queue_back(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_back(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_queue_back(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_back(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_back(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_back(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_back(&mut self) -> ::std::string::String {
        if self.has_queue_back() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_back(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_queue_back(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_back(ref v)) => v,
            _ => "",
        }
    }

    // optional string queue_len = 12;

    pub fn clear_queue_len(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_queue_len(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_len(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_queue_len(&mut self, v: ::std::string::String) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_len(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_queue_len(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_len(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_len(::std::string::String::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_len(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_len(&mut self) -> ::std::string::String {
        if self.has_queue_len() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_len(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_queue_len(&self) -> &str {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_len(ref v)) => v,
            _ => "",
        }
    }

    // optional .SetInsert set_insert = 13;

    pub fn clear_set_insert(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_insert(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_insert(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_insert(&mut self, v: SetInsert) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_insert(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_insert(&mut self) -> &mut SetInsert {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_insert(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_insert(SetInsert::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_insert(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_insert(&mut self) -> SetInsert {
        if self.has_set_insert() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_insert(v)) => v,
                _ => panic!(),
            }
        } else {
            SetInsert::new()
        }
    }

    pub fn get_set_insert(&self) -> &SetInsert {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_insert(ref v)) => v,
            _ => SetInsert::default_instance(),
        }
    }

    // optional .SetRemove set_remove = 14;

    pub fn clear_set_remove(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_remove(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_remove(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_remove(&mut self, v: SetRemove) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_remove(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_remove(&mut self) -> &mut SetRemove {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_remove(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_remove(SetRemove::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_remove(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_remove(&mut self) -> SetRemove {
        if self.has_set_remove() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_remove(v)) => v,
                _ => panic!(),
            }
        } else {
            SetRemove::new()
        }
    }

    pub fn get_set_remove(&self) -> &SetRemove {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_remove(ref v)) => v,
            _ => SetRemove::default_instance(),
        }
    }

    // optional .SetContains set_contains = 15;

    pub fn clear_set_contains(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_contains(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_contains(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_contains(&mut self, v: SetContains) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_contains(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_contains(&mut self) -> &mut SetContains {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_contains(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_contains(SetContains::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_contains(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_contains(&mut self) -> SetContains {
        if self.has_set_contains() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_contains(v)) => v,
                _ => panic!(),
            }
        } else {
            SetContains::new()
        }
    }

    pub fn get_set_contains(&self) -> &SetContains {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_contains(ref v)) => v,
            _ => SetContains::default_instance(),
        }
    }

    // optional .SetUnion set_union = 16;

    pub fn clear_set_union(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_union(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_union(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_union(&mut self, v: SetUnion) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_union(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_union(&mut self) -> &mut SetUnion {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_union(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_union(SetUnion::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_union(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_union(&mut self) -> SetUnion {
        if self.has_set_union() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_union(v)) => v,
                _ => panic!(),
            }
        } else {
            SetUnion::new()
        }
    }

    pub fn get_set_union(&self) -> &SetUnion {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_union(ref v)) => v,
            _ => SetUnion::default_instance(),
        }
    }

    // optional .SetIntersection set_intersection = 17;

    pub fn clear_set_intersection(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_intersection(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_intersection(&mut self, v: SetIntersection) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_intersection(&mut self) -> &mut SetIntersection {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(SetIntersection::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_intersection(&mut self) -> SetIntersection {
        if self.has_set_intersection() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(v)) => v,
                _ => panic!(),
            }
        } else {
            SetIntersection::new()
        }
    }

    pub fn get_set_intersection(&self) -> &SetIntersection {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(ref v)) => v,
            _ => SetIntersection::default_instance(),
        }
    }

    // optional .SetDifference set_difference = 18;

    pub fn clear_set_difference(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_difference(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_difference(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_difference(&mut self, v: SetDifference) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_difference(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_difference(&mut self) -> &mut SetDifference {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_difference(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_difference(SetDifference::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_difference(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_difference(&mut self) -> SetDifference {
        if self.has_set_difference() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_difference(v)) => v,
                _ => panic!(),
            }
        } else {
            SetDifference::new()
        }
    }

    pub fn get_set_difference(&self) -> &SetDifference {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_difference(ref v)) => v,
            _ => SetDifference::default_instance(),
        }
    }

    // optional .SetSymmetricDifference set_symmetric_difference = 19;

    pub fn clear_set_symmetric_difference(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_symmetric_difference(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_symmetric_difference(&mut self, v: SetSymmetricDifference) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_symmetric_difference(&mut self) -> &mut SetSymmetricDifference {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(SetSymmetricDifference::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_symmetric_difference(&mut self) -> SetSymmetricDifference {
        if self.has_set_symmetric_difference() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(v)) => v,
                _ => panic!(),
            }
        } else {
            SetSymmetricDifference::new()
        }
    }

    pub fn get_set_symmetric_difference(&self) -> &SetSymmetricDifference {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(ref v)) => v,
            _ => SetSymmetricDifference::default_instance(),
        }
    }

    // optional .SetSubsetPath set_subset_path = 20;

    pub fn clear_set_subset_path(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_subset_path(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_subset_path(&mut self, v: SetSubsetPath) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_subset_path(&mut self) -> &mut SetSubsetPath {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(SetSubsetPath::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_subset_path(&mut self) -> SetSubsetPath {
        if self.has_set_subset_path() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(v)) => v,
                _ => panic!(),
            }
        } else {
            SetSubsetPath::new()
        }
    }

    pub fn get_set_subset_path(&self) -> &SetSubsetPath {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(ref v)) => v,
            _ => SetSubsetPath::default_instance(),
        }
    }

    // optional .SetSubsetSet set_subset_set = 21;

    pub fn clear_set_subset_set(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_subset_set(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_subset_set(&mut self, v: SetSubsetSet) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_subset_set(&mut self) -> &mut SetSubsetSet {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(SetSubsetSet::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_subset_set(&mut self) -> SetSubsetSet {
        if self.has_set_subset_set() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(v)) => v,
                _ => panic!(),
            }
        } else {
            SetSubsetSet::new()
        }
    }

    pub fn get_set_subset_set(&self) -> &SetSubsetSet {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(ref v)) => v,
            _ => SetSubsetSet::default_instance(),
        }
    }

    // optional .SetSupersetPath set_superset_path = 22;

    pub fn clear_set_superset_path(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_superset_path(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_superset_path(&mut self, v: SetSupersetPath) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_superset_path(&mut self) -> &mut SetSupersetPath {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(SetSupersetPath::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_superset_path(&mut self) -> SetSupersetPath {
        if self.has_set_superset_path() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(v)) => v,
                _ => panic!(),
            }
        } else {
            SetSupersetPath::new()
        }
    }

    pub fn get_set_superset_path(&self) -> &SetSupersetPath {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(ref v)) => v,
            _ => SetSupersetPath::default_instance(),
        }
    }

    // optional .SetSupersetSet set_superset_set = 23;

    pub fn clear_set_superset_set(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_set_superset_set(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_superset_set(&mut self, v: SetSupersetSet) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set_superset_set(&mut self) -> &mut SetSupersetSet {
        if let ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(SetSupersetSet::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_superset_set(&mut self) -> SetSupersetSet {
        if self.has_set_superset_set() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(v)) => v,
                _ => panic!(),
            }
        } else {
            SetSupersetSet::new()
        }
    }

    pub fn get_set_superset_set(&self) -> &SetSupersetSet {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(ref v)) => v,
            _ => SetSupersetSet::default_instance(),
        }
    }
}

impl ::protobuf::Message for TreeOp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::snapshot(try!(is.read_string())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::create_node(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::delete_node(try!(is.read_string())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::list_keys(try!(is.read_string())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_put(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_get(try!(is.read_string())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_size(try!(is.read_string())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_push(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(try!(is.read_string())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_front(try!(is.read_string())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_back(try!(is.read_string())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_len(try!(is.read_string())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_insert(try!(is.read_message())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_remove(try!(is.read_message())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_contains(try!(is.read_message())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_union(try!(is.read_message())));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(try!(is.read_message())));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_difference(try!(is.read_message())));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(try!(is.read_message())));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(try!(is.read_message())));
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(try!(is.read_message())));
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(try!(is.read_message())));
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.op {
            match v {
                &TreeOp_oneof_op::snapshot(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &TreeOp_oneof_op::create_node(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::delete_node(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &TreeOp_oneof_op::list_keys(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
                &TreeOp_oneof_op::blob_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::blob_get(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
                &TreeOp_oneof_op::blob_size(ref v) => {
                    my_size += ::protobuf::rt::string_size(7, &v);
                },
                &TreeOp_oneof_op::queue_push(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::queue_pop(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
                &TreeOp_oneof_op::queue_front(ref v) => {
                    my_size += ::protobuf::rt::string_size(10, &v);
                },
                &TreeOp_oneof_op::queue_back(ref v) => {
                    my_size += ::protobuf::rt::string_size(11, &v);
                },
                &TreeOp_oneof_op::queue_len(ref v) => {
                    my_size += ::protobuf::rt::string_size(12, &v);
                },
                &TreeOp_oneof_op::set_insert(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_remove(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_contains(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_union(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_intersection(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_difference(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_symmetric_difference(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_subset_path(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_subset_set(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_superset_path(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::set_superset_set(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.op {
            match v {
                &TreeOp_oneof_op::snapshot(ref v) => {
                    try!(os.write_string(1, v));
                },
                &TreeOp_oneof_op::create_node(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::delete_node(ref v) => {
                    try!(os.write_string(3, v));
                },
                &TreeOp_oneof_op::list_keys(ref v) => {
                    try!(os.write_string(4, v));
                },
                &TreeOp_oneof_op::blob_put(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::blob_get(ref v) => {
                    try!(os.write_string(6, v));
                },
                &TreeOp_oneof_op::blob_size(ref v) => {
                    try!(os.write_string(7, v));
                },
                &TreeOp_oneof_op::queue_push(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::queue_pop(ref v) => {
                    try!(os.write_string(9, v));
                },
                &TreeOp_oneof_op::queue_front(ref v) => {
                    try!(os.write_string(10, v));
                },
                &TreeOp_oneof_op::queue_back(ref v) => {
                    try!(os.write_string(11, v));
                },
                &TreeOp_oneof_op::queue_len(ref v) => {
                    try!(os.write_string(12, v));
                },
                &TreeOp_oneof_op::set_insert(ref v) => {
                    try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_remove(ref v) => {
                    try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_contains(ref v) => {
                    try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_union(ref v) => {
                    try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_intersection(ref v) => {
                    try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_difference(ref v) => {
                    try!(os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_symmetric_difference(ref v) => {
                    try!(os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_subset_path(ref v) => {
                    try!(os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_subset_set(ref v) => {
                    try!(os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_superset_path(ref v) => {
                    try!(os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOp_oneof_op::set_superset_set(ref v) => {
                    try!(os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TreeOp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TreeOp {
    fn new() -> TreeOp {
        TreeOp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TreeOp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "snapshot",
                    TreeOp::has_snapshot,
                    TreeOp::get_snapshot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "create_node",
                    TreeOp::has_create_node,
                    TreeOp::get_create_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "delete_node",
                    TreeOp::has_delete_node,
                    TreeOp::get_delete_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "list_keys",
                    TreeOp::has_list_keys,
                    TreeOp::get_list_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "blob_put",
                    TreeOp::has_blob_put,
                    TreeOp::get_blob_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blob_get",
                    TreeOp::has_blob_get,
                    TreeOp::get_blob_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blob_size",
                    TreeOp::has_blob_size,
                    TreeOp::get_blob_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "queue_push",
                    TreeOp::has_queue_push,
                    TreeOp::get_queue_push,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "queue_pop",
                    TreeOp::has_queue_pop,
                    TreeOp::get_queue_pop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "queue_front",
                    TreeOp::has_queue_front,
                    TreeOp::get_queue_front,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "queue_back",
                    TreeOp::has_queue_back,
                    TreeOp::get_queue_back,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "queue_len",
                    TreeOp::has_queue_len,
                    TreeOp::get_queue_len,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_insert",
                    TreeOp::has_set_insert,
                    TreeOp::get_set_insert,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_remove",
                    TreeOp::has_set_remove,
                    TreeOp::get_set_remove,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_contains",
                    TreeOp::has_set_contains,
                    TreeOp::get_set_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_union",
                    TreeOp::has_set_union,
                    TreeOp::get_set_union,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_intersection",
                    TreeOp::has_set_intersection,
                    TreeOp::get_set_intersection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_difference",
                    TreeOp::has_set_difference,
                    TreeOp::get_set_difference,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_symmetric_difference",
                    TreeOp::has_set_symmetric_difference,
                    TreeOp::get_set_symmetric_difference,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_subset_path",
                    TreeOp::has_set_subset_path,
                    TreeOp::get_set_subset_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_subset_set",
                    TreeOp::has_set_subset_set,
                    TreeOp::get_set_subset_set,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_superset_path",
                    TreeOp::has_set_superset_path,
                    TreeOp::get_set_superset_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set_superset_set",
                    TreeOp::has_set_superset_set,
                    TreeOp::get_set_superset_set,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TreeOp>(
                    "TreeOp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TreeOp {
    fn clear(&mut self) {
        self.clear_snapshot();
        self.clear_create_node();
        self.clear_delete_node();
        self.clear_list_keys();
        self.clear_blob_put();
        self.clear_blob_get();
        self.clear_blob_size();
        self.clear_queue_push();
        self.clear_queue_pop();
        self.clear_queue_front();
        self.clear_queue_back();
        self.clear_queue_len();
        self.clear_set_insert();
        self.clear_set_remove();
        self.clear_set_contains();
        self.clear_set_union();
        self.clear_set_intersection();
        self.clear_set_difference();
        self.clear_set_symmetric_difference();
        self.clear_set_subset_path();
        self.clear_set_subset_set();
        self.clear_set_superset_path();
        self.clear_set_superset_set();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TreeOp {
    fn eq(&self, other: &TreeOp) -> bool {
        self.op == other.op &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TreeOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateNode {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    node_type: ::std::option::Option<NodeType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateNode {}

impl CreateNode {
    pub fn new() -> CreateNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateNode {
        static mut instance: ::protobuf::lazy::Lazy<CreateNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateNode,
        };
        unsafe {
            instance.get(|| {
                CreateNode {
                    path: ::protobuf::SingularField::none(),
                    node_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .NodeType node_type = 2;

    pub fn clear_node_type(&mut self) {
        self.node_type = ::std::option::Option::None;
    }

    pub fn has_node_type(&self) -> bool {
        self.node_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_type(&mut self, v: NodeType) {
        self.node_type = ::std::option::Option::Some(v);
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type.unwrap_or(NodeType::BlobType)
    }
}

impl ::protobuf::Message for CreateNode {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.node_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.node_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.node_type {
            try!(os.write_enum(2, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateNode>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateNode {
    fn new() -> CreateNode {
        CreateNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    CreateNode::has_path,
                    CreateNode::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "node_type",
                    CreateNode::has_node_type,
                    CreateNode::get_node_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateNode>(
                    "CreateNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateNode {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_node_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateNode {
    fn eq(&self, other: &CreateNode) -> bool {
        self.path == other.path &&
        self.node_type == other.node_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlobPut {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlobPut {}

impl BlobPut {
    pub fn new() -> BlobPut {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlobPut {
        static mut instance: ::protobuf::lazy::Lazy<BlobPut> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlobPut,
        };
        unsafe {
            instance.get(|| {
                BlobPut {
                    path: ::protobuf::SingularField::none(),
                    val: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes val = 2;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        };
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for BlobPut {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.val {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.val.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlobPut>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlobPut {
    fn new() -> BlobPut {
        BlobPut::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlobPut>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    BlobPut::has_path,
                    BlobPut::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "val",
                    BlobPut::has_val,
                    BlobPut::get_val,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlobPut>(
                    "BlobPut",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlobPut {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlobPut {
    fn eq(&self, other: &BlobPut) -> bool {
        self.path == other.path &&
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlobPut {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct QueuePush {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueuePush {}

impl QueuePush {
    pub fn new() -> QueuePush {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueuePush {
        static mut instance: ::protobuf::lazy::Lazy<QueuePush> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueuePush,
        };
        unsafe {
            instance.get(|| {
                QueuePush {
                    path: ::protobuf::SingularField::none(),
                    val: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes val = 2;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        };
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for QueuePush {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.val {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.val.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<QueuePush>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueuePush {
    fn new() -> QueuePush {
        QueuePush::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueuePush>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    QueuePush::has_path,
                    QueuePush::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "val",
                    QueuePush::has_val,
                    QueuePush::get_val,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueuePush>(
                    "QueuePush",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueuePush {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for QueuePush {
    fn eq(&self, other: &QueuePush) -> bool {
        self.path == other.path &&
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for QueuePush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetInsert {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetInsert {}

impl SetInsert {
    pub fn new() -> SetInsert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetInsert {
        static mut instance: ::protobuf::lazy::Lazy<SetInsert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetInsert,
        };
        unsafe {
            instance.get(|| {
                SetInsert {
                    path: ::protobuf::SingularField::none(),
                    val: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes val = 2;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        };
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SetInsert {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.val {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.val.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetInsert>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetInsert {
    fn new() -> SetInsert {
        SetInsert::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetInsert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    SetInsert::has_path,
                    SetInsert::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "val",
                    SetInsert::has_val,
                    SetInsert::get_val,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetInsert>(
                    "SetInsert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetInsert {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetInsert {
    fn eq(&self, other: &SetInsert) -> bool {
        self.path == other.path &&
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetInsert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetRemove {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetRemove {}

impl SetRemove {
    pub fn new() -> SetRemove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetRemove {
        static mut instance: ::protobuf::lazy::Lazy<SetRemove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetRemove,
        };
        unsafe {
            instance.get(|| {
                SetRemove {
                    path: ::protobuf::SingularField::none(),
                    val: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes val = 2;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        };
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SetRemove {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.val {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.val.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetRemove>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetRemove {
    fn new() -> SetRemove {
        SetRemove::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetRemove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    SetRemove::has_path,
                    SetRemove::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "val",
                    SetRemove::has_val,
                    SetRemove::get_val,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetRemove>(
                    "SetRemove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetRemove {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetRemove {
    fn eq(&self, other: &SetRemove) -> bool {
        self.path == other.path &&
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetContains {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetContains {}

impl SetContains {
    pub fn new() -> SetContains {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetContains {
        static mut instance: ::protobuf::lazy::Lazy<SetContains> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetContains,
        };
        unsafe {
            instance.get(|| {
                SetContains {
                    path: ::protobuf::SingularField::none(),
                    val: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes val = 2;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    pub fn has_val(&self) -> bool {
        self.val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.val = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val.is_none() {
            self.val.set_default();
        };
        self.val.as_mut().unwrap()
    }

    // Take field
    pub fn take_val(&mut self) -> ::std::vec::Vec<u8> {
        self.val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val(&self) -> &[u8] {
        match self.val.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SetContains {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.val {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.val.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetContains>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetContains {
    fn new() -> SetContains {
        SetContains::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetContains>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    SetContains::has_path,
                    SetContains::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "val",
                    SetContains::has_val,
                    SetContains::get_val,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetContains>(
                    "SetContains",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetContains {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetContains {
    fn eq(&self, other: &SetContains) -> bool {
        self.path == other.path &&
        self.val == other.val &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetContains {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Set {
    // message fields
    values: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Set {}

impl Set {
    pub fn new() -> Set {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Set {
        static mut instance: ::protobuf::lazy::Lazy<Set> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Set,
        };
        unsafe {
            instance.get(|| {
                Set {
                    values: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[::std::vec::Vec<u8>] {
        &self.values
    }
}

impl ::protobuf::Message for Set {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.values));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.values {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Set>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Set {
    fn new() -> Set {
        Set::new()
    }

    fn descriptor_static(_: ::std::option::Option<Set>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "values",
                    Set::get_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Set>(
                    "Set",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Set {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Set {
    fn eq(&self, other: &Set) -> bool {
        self.values == other.values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Set {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetUnion {
    // message fields
    paths: ::protobuf::RepeatedField<::std::string::String>,
    sets: ::protobuf::RepeatedField<Set>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetUnion {}

impl SetUnion {
    pub fn new() -> SetUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetUnion {
        static mut instance: ::protobuf::lazy::Lazy<SetUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetUnion,
        };
        unsafe {
            instance.get(|| {
                SetUnion {
                    paths: ::protobuf::RepeatedField::new(),
                    sets: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string paths = 1;

    pub fn clear_paths(&mut self) {
        self.paths.clear();
    }

    // Param is passed by value, moved
    pub fn set_paths(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.paths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_paths(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.paths
    }

    // Take field
    pub fn take_paths(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.paths, ::protobuf::RepeatedField::new())
    }

    pub fn get_paths(&self) -> &[::std::string::String] {
        &self.paths
    }

    // repeated .Set sets = 2;

    pub fn clear_sets(&mut self) {
        self.sets.clear();
    }

    // Param is passed by value, moved
    pub fn set_sets(&mut self, v: ::protobuf::RepeatedField<Set>) {
        self.sets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sets(&mut self) -> &mut ::protobuf::RepeatedField<Set> {
        &mut self.sets
    }

    // Take field
    pub fn take_sets(&mut self) -> ::protobuf::RepeatedField<Set> {
        ::std::mem::replace(&mut self.sets, ::protobuf::RepeatedField::new())
    }

    pub fn get_sets(&self) -> &[Set] {
        &self.sets
    }
}

impl ::protobuf::Message for SetUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.paths));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sets));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.paths {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.sets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.paths {
            try!(os.write_string(1, &v));
        };
        for v in &self.sets {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetUnion {
    fn new() -> SetUnion {
        SetUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "paths",
                    SetUnion::get_paths,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "sets",
                    SetUnion::get_sets,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetUnion>(
                    "SetUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetUnion {
    fn clear(&mut self) {
        self.clear_paths();
        self.clear_sets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetUnion {
    fn eq(&self, other: &SetUnion) -> bool {
        self.paths == other.paths &&
        self.sets == other.sets &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetIntersection {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetIntersection {}

impl SetIntersection {
    pub fn new() -> SetIntersection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetIntersection {
        static mut instance: ::protobuf::lazy::Lazy<SetIntersection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetIntersection,
        };
        unsafe {
            instance.get(|| {
                SetIntersection {
                    path1: ::protobuf::SingularField::none(),
                    path2: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path1 = 1;

    pub fn clear_path1(&mut self) {
        self.path1.clear();
    }

    pub fn has_path1(&self) -> bool {
        self.path1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path1(&mut self, v: ::std::string::String) {
        self.path1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path1(&mut self) -> &mut ::std::string::String {
        if self.path1.is_none() {
            self.path1.set_default();
        };
        self.path1.as_mut().unwrap()
    }

    // Take field
    pub fn take_path1(&mut self) -> ::std::string::String {
        self.path1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path1(&self) -> &str {
        match self.path1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string path2 = 2;

    pub fn clear_path2(&mut self) {
        self.path2.clear();
    }

    pub fn has_path2(&self) -> bool {
        self.path2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path2(&mut self, v: ::std::string::String) {
        self.path2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path2(&mut self) -> &mut ::std::string::String {
        if self.path2.is_none() {
            self.path2.set_default();
        };
        self.path2.as_mut().unwrap()
    }

    // Take field
    pub fn take_path2(&mut self) -> ::std::string::String {
        self.path2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path2(&self) -> &str {
        match self.path2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetIntersection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path1 {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.path2 {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.path2.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetIntersection>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetIntersection {
    fn new() -> SetIntersection {
        SetIntersection::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetIntersection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path1",
                    SetIntersection::has_path1,
                    SetIntersection::get_path1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path2",
                    SetIntersection::has_path2,
                    SetIntersection::get_path2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetIntersection>(
                    "SetIntersection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetIntersection {
    fn clear(&mut self) {
        self.clear_path1();
        self.clear_path2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetIntersection {
    fn eq(&self, other: &SetIntersection) -> bool {
        self.path1 == other.path1 &&
        self.path2 == other.path2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetIntersection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetDifference {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetDifference {}

impl SetDifference {
    pub fn new() -> SetDifference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetDifference {
        static mut instance: ::protobuf::lazy::Lazy<SetDifference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetDifference,
        };
        unsafe {
            instance.get(|| {
                SetDifference {
                    path1: ::protobuf::SingularField::none(),
                    path2: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path1 = 1;

    pub fn clear_path1(&mut self) {
        self.path1.clear();
    }

    pub fn has_path1(&self) -> bool {
        self.path1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path1(&mut self, v: ::std::string::String) {
        self.path1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path1(&mut self) -> &mut ::std::string::String {
        if self.path1.is_none() {
            self.path1.set_default();
        };
        self.path1.as_mut().unwrap()
    }

    // Take field
    pub fn take_path1(&mut self) -> ::std::string::String {
        self.path1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path1(&self) -> &str {
        match self.path1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string path2 = 2;

    pub fn clear_path2(&mut self) {
        self.path2.clear();
    }

    pub fn has_path2(&self) -> bool {
        self.path2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path2(&mut self, v: ::std::string::String) {
        self.path2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path2(&mut self) -> &mut ::std::string::String {
        if self.path2.is_none() {
            self.path2.set_default();
        };
        self.path2.as_mut().unwrap()
    }

    // Take field
    pub fn take_path2(&mut self) -> ::std::string::String {
        self.path2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path2(&self) -> &str {
        match self.path2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetDifference {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path1 {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.path2 {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.path2.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetDifference>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetDifference {
    fn new() -> SetDifference {
        SetDifference::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetDifference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path1",
                    SetDifference::has_path1,
                    SetDifference::get_path1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path2",
                    SetDifference::has_path2,
                    SetDifference::get_path2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetDifference>(
                    "SetDifference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetDifference {
    fn clear(&mut self) {
        self.clear_path1();
        self.clear_path2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetDifference {
    fn eq(&self, other: &SetDifference) -> bool {
        self.path1 == other.path1 &&
        self.path2 == other.path2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetDifference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetSymmetricDifference {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetSymmetricDifference {}

impl SetSymmetricDifference {
    pub fn new() -> SetSymmetricDifference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetSymmetricDifference {
        static mut instance: ::protobuf::lazy::Lazy<SetSymmetricDifference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetSymmetricDifference,
        };
        unsafe {
            instance.get(|| {
                SetSymmetricDifference {
                    path1: ::protobuf::SingularField::none(),
                    path2: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path1 = 1;

    pub fn clear_path1(&mut self) {
        self.path1.clear();
    }

    pub fn has_path1(&self) -> bool {
        self.path1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path1(&mut self, v: ::std::string::String) {
        self.path1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path1(&mut self) -> &mut ::std::string::String {
        if self.path1.is_none() {
            self.path1.set_default();
        };
        self.path1.as_mut().unwrap()
    }

    // Take field
    pub fn take_path1(&mut self) -> ::std::string::String {
        self.path1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path1(&self) -> &str {
        match self.path1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string path2 = 2;

    pub fn clear_path2(&mut self) {
        self.path2.clear();
    }

    pub fn has_path2(&self) -> bool {
        self.path2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path2(&mut self, v: ::std::string::String) {
        self.path2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path2(&mut self) -> &mut ::std::string::String {
        if self.path2.is_none() {
            self.path2.set_default();
        };
        self.path2.as_mut().unwrap()
    }

    // Take field
    pub fn take_path2(&mut self) -> ::std::string::String {
        self.path2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path2(&self) -> &str {
        match self.path2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetSymmetricDifference {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path1 {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.path2 {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.path2.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetSymmetricDifference>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetSymmetricDifference {
    fn new() -> SetSymmetricDifference {
        SetSymmetricDifference::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetSymmetricDifference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path1",
                    SetSymmetricDifference::has_path1,
                    SetSymmetricDifference::get_path1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path2",
                    SetSymmetricDifference::has_path2,
                    SetSymmetricDifference::get_path2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetSymmetricDifference>(
                    "SetSymmetricDifference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetSymmetricDifference {
    fn clear(&mut self) {
        self.clear_path1();
        self.clear_path2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetSymmetricDifference {
    fn eq(&self, other: &SetSymmetricDifference) -> bool {
        self.path1 == other.path1 &&
        self.path2 == other.path2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetSymmetricDifference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetSubsetPath {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetSubsetPath {}

impl SetSubsetPath {
    pub fn new() -> SetSubsetPath {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetSubsetPath {
        static mut instance: ::protobuf::lazy::Lazy<SetSubsetPath> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetSubsetPath,
        };
        unsafe {
            instance.get(|| {
                SetSubsetPath {
                    path1: ::protobuf::SingularField::none(),
                    path2: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path1 = 1;

    pub fn clear_path1(&mut self) {
        self.path1.clear();
    }

    pub fn has_path1(&self) -> bool {
        self.path1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path1(&mut self, v: ::std::string::String) {
        self.path1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path1(&mut self) -> &mut ::std::string::String {
        if self.path1.is_none() {
            self.path1.set_default();
        };
        self.path1.as_mut().unwrap()
    }

    // Take field
    pub fn take_path1(&mut self) -> ::std::string::String {
        self.path1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path1(&self) -> &str {
        match self.path1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string path2 = 2;

    pub fn clear_path2(&mut self) {
        self.path2.clear();
    }

    pub fn has_path2(&self) -> bool {
        self.path2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path2(&mut self, v: ::std::string::String) {
        self.path2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path2(&mut self) -> &mut ::std::string::String {
        if self.path2.is_none() {
            self.path2.set_default();
        };
        self.path2.as_mut().unwrap()
    }

    // Take field
    pub fn take_path2(&mut self) -> ::std::string::String {
        self.path2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path2(&self) -> &str {
        match self.path2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetSubsetPath {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path1 {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.path2 {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.path2.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetSubsetPath>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetSubsetPath {
    fn new() -> SetSubsetPath {
        SetSubsetPath::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetSubsetPath>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path1",
                    SetSubsetPath::has_path1,
                    SetSubsetPath::get_path1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path2",
                    SetSubsetPath::has_path2,
                    SetSubsetPath::get_path2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetSubsetPath>(
                    "SetSubsetPath",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetSubsetPath {
    fn clear(&mut self) {
        self.clear_path1();
        self.clear_path2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetSubsetPath {
    fn eq(&self, other: &SetSubsetPath) -> bool {
        self.path1 == other.path1 &&
        self.path2 == other.path2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetSubsetPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetSubsetSet {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    set: ::protobuf::SingularPtrField<Set>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetSubsetSet {}

impl SetSubsetSet {
    pub fn new() -> SetSubsetSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetSubsetSet {
        static mut instance: ::protobuf::lazy::Lazy<SetSubsetSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetSubsetSet,
        };
        unsafe {
            instance.get(|| {
                SetSubsetSet {
                    path: ::protobuf::SingularField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Set set = 2;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: Set) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set(&mut self) -> &mut Set {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    // Take field
    pub fn take_set(&mut self) -> Set {
        self.set.take().unwrap_or_else(|| Set::new())
    }

    pub fn get_set(&self) -> &Set {
        self.set.as_ref().unwrap_or_else(|| Set::default_instance())
    }
}

impl ::protobuf::Message for SetSubsetSet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.set));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.set {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.set.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetSubsetSet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetSubsetSet {
    fn new() -> SetSubsetSet {
        SetSubsetSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetSubsetSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    SetSubsetSet::has_path,
                    SetSubsetSet::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set",
                    SetSubsetSet::has_set,
                    SetSubsetSet::get_set,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetSubsetSet>(
                    "SetSubsetSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetSubsetSet {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_set();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetSubsetSet {
    fn eq(&self, other: &SetSubsetSet) -> bool {
        self.path == other.path &&
        self.set == other.set &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetSubsetSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetSupersetPath {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetSupersetPath {}

impl SetSupersetPath {
    pub fn new() -> SetSupersetPath {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetSupersetPath {
        static mut instance: ::protobuf::lazy::Lazy<SetSupersetPath> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetSupersetPath,
        };
        unsafe {
            instance.get(|| {
                SetSupersetPath {
                    path1: ::protobuf::SingularField::none(),
                    path2: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path1 = 1;

    pub fn clear_path1(&mut self) {
        self.path1.clear();
    }

    pub fn has_path1(&self) -> bool {
        self.path1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path1(&mut self, v: ::std::string::String) {
        self.path1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path1(&mut self) -> &mut ::std::string::String {
        if self.path1.is_none() {
            self.path1.set_default();
        };
        self.path1.as_mut().unwrap()
    }

    // Take field
    pub fn take_path1(&mut self) -> ::std::string::String {
        self.path1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path1(&self) -> &str {
        match self.path1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string path2 = 2;

    pub fn clear_path2(&mut self) {
        self.path2.clear();
    }

    pub fn has_path2(&self) -> bool {
        self.path2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path2(&mut self, v: ::std::string::String) {
        self.path2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path2(&mut self) -> &mut ::std::string::String {
        if self.path2.is_none() {
            self.path2.set_default();
        };
        self.path2.as_mut().unwrap()
    }

    // Take field
    pub fn take_path2(&mut self) -> ::std::string::String {
        self.path2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path2(&self) -> &str {
        match self.path2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetSupersetPath {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path1 {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.path2 {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.path2.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetSupersetPath>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetSupersetPath {
    fn new() -> SetSupersetPath {
        SetSupersetPath::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetSupersetPath>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path1",
                    SetSupersetPath::has_path1,
                    SetSupersetPath::get_path1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path2",
                    SetSupersetPath::has_path2,
                    SetSupersetPath::get_path2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetSupersetPath>(
                    "SetSupersetPath",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetSupersetPath {
    fn clear(&mut self) {
        self.clear_path1();
        self.clear_path2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetSupersetPath {
    fn eq(&self, other: &SetSupersetPath) -> bool {
        self.path1 == other.path1 &&
        self.path2 == other.path2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetSupersetPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetSupersetSet {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    set: ::protobuf::SingularPtrField<Set>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetSupersetSet {}

impl SetSupersetSet {
    pub fn new() -> SetSupersetSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetSupersetSet {
        static mut instance: ::protobuf::lazy::Lazy<SetSupersetSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetSupersetSet,
        };
        unsafe {
            instance.get(|| {
                SetSupersetSet {
                    path: ::protobuf::SingularField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Set set = 2;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: Set) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set(&mut self) -> &mut Set {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    // Take field
    pub fn take_set(&mut self) -> Set {
        self.set.take().unwrap_or_else(|| Set::new())
    }

    pub fn get_set(&self) -> &Set {
        self.set.as_ref().unwrap_or_else(|| Set::default_instance())
    }
}

impl ::protobuf::Message for SetSupersetSet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.set));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.set {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.set.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetSupersetSet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetSupersetSet {
    fn new() -> SetSupersetSet {
        SetSupersetSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetSupersetSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    SetSupersetSet::has_path,
                    SetSupersetSet::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set",
                    SetSupersetSet::has_set,
                    SetSupersetSet::get_set,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetSupersetSet>(
                    "SetSupersetSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetSupersetSet {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_set();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetSupersetSet {
    fn eq(&self, other: &SetSupersetSet) -> bool {
        self.path == other.path &&
        self.set == other.set &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetSupersetSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VrApiRsp {
    // message oneof groups
    rsp: ::std::option::Option<VrApiRsp_oneof_rsp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VrApiRsp {}

#[derive(Clone,PartialEq)]
pub enum VrApiRsp_oneof_rsp {
    ok(bool),
    tree_op_result(TreeOpResult),
    tree_cas_result(TreeCasResult),
    path(::std::string::String),
    error(VrApiError),
}

impl VrApiRsp {
    pub fn new() -> VrApiRsp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VrApiRsp {
        static mut instance: ::protobuf::lazy::Lazy<VrApiRsp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VrApiRsp,
        };
        unsafe {
            instance.get(|| {
                VrApiRsp {
                    rsp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool ok = 1;

    pub fn clear_ok(&mut self) {
        self.rsp = ::std::option::Option::None;
    }

    pub fn has_ok(&self) -> bool {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::ok(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::ok(v))
    }

    pub fn get_ok(&self) -> bool {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::ok(v)) => v,
            _ => false,
        }
    }

    // optional .TreeOpResult tree_op_result = 2;

    pub fn clear_tree_op_result(&mut self) {
        self.rsp = ::std::option::Option::None;
    }

    pub fn has_tree_op_result(&self) -> bool {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_op_result(&mut self, v: TreeOpResult) {
        self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree_op_result(&mut self) -> &mut TreeOpResult {
        if let ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(_)) = self.rsp {
        } else {
            self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(TreeOpResult::new()));
        }
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_op_result(&mut self) -> TreeOpResult {
        if self.has_tree_op_result() {
            match self.rsp.take() {
                ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeOpResult::new()
        }
    }

    pub fn get_tree_op_result(&self) -> &TreeOpResult {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(ref v)) => v,
            _ => TreeOpResult::default_instance(),
        }
    }

    // optional .TreeCasResult tree_cas_result = 3;

    pub fn clear_tree_cas_result(&mut self) {
        self.rsp = ::std::option::Option::None;
    }

    pub fn has_tree_cas_result(&self) -> bool {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_cas_result(&mut self, v: TreeCasResult) {
        self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree_cas_result(&mut self) -> &mut TreeCasResult {
        if let ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(_)) = self.rsp {
        } else {
            self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(TreeCasResult::new()));
        }
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_cas_result(&mut self) -> TreeCasResult {
        if self.has_tree_cas_result() {
            match self.rsp.take() {
                ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeCasResult::new()
        }
    }

    pub fn get_tree_cas_result(&self) -> &TreeCasResult {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(ref v)) => v,
            _ => TreeCasResult::default_instance(),
        }
    }

    // optional string path = 4;

    pub fn clear_path(&mut self) {
        self.rsp = ::std::option::Option::None;
    }

    pub fn has_path(&self) -> bool {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(_)) = self.rsp {
        } else {
            self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(::std::string::String::new()));
        }
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        if self.has_path() {
            match self.rsp.take() {
                ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_path(&self) -> &str {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(ref v)) => v,
            _ => "",
        }
    }

    // optional .VrApiError error = 5;

    pub fn clear_error(&mut self) {
        self.rsp = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: VrApiError) {
        self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut VrApiError {
        if let ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(_)) = self.rsp {
        } else {
            self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(VrApiError::new()));
        }
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> VrApiError {
        if self.has_error() {
            match self.rsp.take() {
                ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(v)) => v,
                _ => panic!(),
            }
        } else {
            VrApiError::new()
        }
    }

    pub fn get_error(&self) -> &VrApiError {
        match self.rsp {
            ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(ref v)) => v,
            _ => VrApiError::default_instance(),
        }
    }
}

impl ::protobuf::Message for VrApiRsp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::ok(try!(is.read_bool())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_op_result(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::tree_cas_result(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::path(try!(is.read_string())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rsp = ::std::option::Option::Some(VrApiRsp_oneof_rsp::error(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.rsp {
            match v {
                &VrApiRsp_oneof_rsp::ok(v) => {
                    my_size += 2;
                },
                &VrApiRsp_oneof_rsp::tree_op_result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrApiRsp_oneof_rsp::tree_cas_result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrApiRsp_oneof_rsp::path(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
                &VrApiRsp_oneof_rsp::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.rsp {
            match v {
                &VrApiRsp_oneof_rsp::ok(v) => {
                    try!(os.write_bool(1, v));
                },
                &VrApiRsp_oneof_rsp::tree_op_result(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrApiRsp_oneof_rsp::tree_cas_result(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrApiRsp_oneof_rsp::path(ref v) => {
                    try!(os.write_string(4, v));
                },
                &VrApiRsp_oneof_rsp::error(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VrApiRsp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VrApiRsp {
    fn new() -> VrApiRsp {
        VrApiRsp::new()
    }

    fn descriptor_static(_: ::std::option::Option<VrApiRsp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "ok",
                    VrApiRsp::has_ok,
                    VrApiRsp::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tree_op_result",
                    VrApiRsp::has_tree_op_result,
                    VrApiRsp::get_tree_op_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tree_cas_result",
                    VrApiRsp::has_tree_cas_result,
                    VrApiRsp::get_tree_cas_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    VrApiRsp::has_path,
                    VrApiRsp::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    VrApiRsp::has_error,
                    VrApiRsp::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VrApiRsp>(
                    "VrApiRsp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VrApiRsp {
    fn clear(&mut self) {
        self.clear_ok();
        self.clear_tree_op_result();
        self.clear_tree_cas_result();
        self.clear_path();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VrApiRsp {
    fn eq(&self, other: &VrApiRsp) -> bool {
        self.rsp == other.rsp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VrApiRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VrApiError {
    // message oneof groups
    error: ::std::option::Option<VrApiError_oneof_error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VrApiError {}

#[derive(Clone,PartialEq)]
pub enum VrApiError_oneof_error {
    not_found(::std::string::String),
    already_exists(::std::string::String),
    does_not_exist(::std::string::String),
    wrong_type(WrongType),
    path_must_end_in_directory(::std::string::String),
    path_must_be_absolute(::std::string::String),
    cas_failed(CasFailed),
    bad_format(::std::string::String),
    io(::std::string::String),
    encoding_error(::std::string::String),
    invalid_cas(::std::string::String),
    msg(::std::string::String),
    cannot_delete_root(bool),
    invalid_msg(bool),
    timeout(bool),
    not_enough_replicas(bool),
    bad_epoch(bool),
}

impl VrApiError {
    pub fn new() -> VrApiError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VrApiError {
        static mut instance: ::protobuf::lazy::Lazy<VrApiError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VrApiError,
        };
        unsafe {
            instance.get(|| {
                VrApiError {
                    error: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string not_found = 1;

    pub fn clear_not_found(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_not_found(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::not_found(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_not_found(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::not_found(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_not_found(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::not_found(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::not_found(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::not_found(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_not_found(&mut self) -> ::std::string::String {
        if self.has_not_found() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::not_found(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_not_found(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::not_found(ref v)) => v,
            _ => "",
        }
    }

    // optional string already_exists = 2;

    pub fn clear_already_exists(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_already_exists(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::already_exists(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_already_exists(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::already_exists(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_already_exists(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::already_exists(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::already_exists(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::already_exists(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_already_exists(&mut self) -> ::std::string::String {
        if self.has_already_exists() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::already_exists(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_already_exists(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::already_exists(ref v)) => v,
            _ => "",
        }
    }

    // optional string does_not_exist = 3;

    pub fn clear_does_not_exist(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_does_not_exist(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_does_not_exist(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_does_not_exist(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_does_not_exist(&mut self) -> ::std::string::String {
        if self.has_does_not_exist() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_does_not_exist(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(ref v)) => v,
            _ => "",
        }
    }

    // optional .WrongType wrong_type = 4;

    pub fn clear_wrong_type(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_wrong_type(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_wrong_type(&mut self, v: WrongType) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wrong_type(&mut self) -> &mut WrongType {
        if let ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(WrongType::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_wrong_type(&mut self) -> WrongType {
        if self.has_wrong_type() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(v)) => v,
                _ => panic!(),
            }
        } else {
            WrongType::new()
        }
    }

    pub fn get_wrong_type(&self) -> &WrongType {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(ref v)) => v,
            _ => WrongType::default_instance(),
        }
    }

    // optional string path_must_end_in_directory = 5;

    pub fn clear_path_must_end_in_directory(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_path_must_end_in_directory(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path_must_end_in_directory(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path_must_end_in_directory(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path_must_end_in_directory(&mut self) -> ::std::string::String {
        if self.has_path_must_end_in_directory() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_path_must_end_in_directory(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(ref v)) => v,
            _ => "",
        }
    }

    // optional string path_must_be_absolute = 6;

    pub fn clear_path_must_be_absolute(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_path_must_be_absolute(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path_must_be_absolute(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path_must_be_absolute(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path_must_be_absolute(&mut self) -> ::std::string::String {
        if self.has_path_must_be_absolute() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_path_must_be_absolute(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(ref v)) => v,
            _ => "",
        }
    }

    // optional .CasFailed cas_failed = 7;

    pub fn clear_cas_failed(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_cas_failed(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cas_failed(&mut self, v: CasFailed) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cas_failed(&mut self) -> &mut CasFailed {
        if let ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(CasFailed::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cas_failed(&mut self) -> CasFailed {
        if self.has_cas_failed() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(v)) => v,
                _ => panic!(),
            }
        } else {
            CasFailed::new()
        }
    }

    pub fn get_cas_failed(&self) -> &CasFailed {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(ref v)) => v,
            _ => CasFailed::default_instance(),
        }
    }

    // optional string bad_format = 8;

    pub fn clear_bad_format(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_bad_format(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::bad_format(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bad_format(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::bad_format(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bad_format(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::bad_format(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::bad_format(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::bad_format(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bad_format(&mut self) -> ::std::string::String {
        if self.has_bad_format() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::bad_format(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_bad_format(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::bad_format(ref v)) => v,
            _ => "",
        }
    }

    // optional string io = 9;

    pub fn clear_io(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_io(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::io(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_io(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::io(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_io(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::io(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::io(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::io(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_io(&mut self) -> ::std::string::String {
        if self.has_io() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::io(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_io(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::io(ref v)) => v,
            _ => "",
        }
    }

    // optional string encoding_error = 10;

    pub fn clear_encoding_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_encoding_error(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_encoding_error(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encoding_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_encoding_error(&mut self) -> ::std::string::String {
        if self.has_encoding_error() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_encoding_error(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(ref v)) => v,
            _ => "",
        }
    }

    // optional string invalid_cas = 11;

    pub fn clear_invalid_cas(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_invalid_cas(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_invalid_cas(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_invalid_cas(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_invalid_cas(&mut self) -> ::std::string::String {
        if self.has_invalid_cas() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_invalid_cas(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(ref v)) => v,
            _ => "",
        }
    }

    // optional string msg = 12;

    pub fn clear_msg(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_msg(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::msg(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::msg(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(VrApiError_oneof_error::msg(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(VrApiError_oneof_error::msg(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::msg(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        if self.has_msg() {
            match self.error.take() {
                ::std::option::Option::Some(VrApiError_oneof_error::msg(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_msg(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::msg(ref v)) => v,
            _ => "",
        }
    }

    // optional bool cannot_delete_root = 13;

    pub fn clear_cannot_delete_root(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_cannot_delete_root(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::cannot_delete_root(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cannot_delete_root(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::cannot_delete_root(v))
    }

    pub fn get_cannot_delete_root(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::cannot_delete_root(v)) => v,
            _ => false,
        }
    }

    // optional bool invalid_msg = 14;

    pub fn clear_invalid_msg(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_invalid_msg(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::invalid_msg(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_invalid_msg(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::invalid_msg(v))
    }

    pub fn get_invalid_msg(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::invalid_msg(v)) => v,
            _ => false,
        }
    }

    // optional bool timeout = 15;

    pub fn clear_timeout(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::timeout(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::timeout(v))
    }

    pub fn get_timeout(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::timeout(v)) => v,
            _ => false,
        }
    }

    // optional bool not_enough_replicas = 16;

    pub fn clear_not_enough_replicas(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_not_enough_replicas(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::not_enough_replicas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_not_enough_replicas(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::not_enough_replicas(v))
    }

    pub fn get_not_enough_replicas(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::not_enough_replicas(v)) => v,
            _ => false,
        }
    }

    // optional bool bad_epoch = 17;

    pub fn clear_bad_epoch(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_bad_epoch(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::bad_epoch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bad_epoch(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(VrApiError_oneof_error::bad_epoch(v))
    }

    pub fn get_bad_epoch(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(VrApiError_oneof_error::bad_epoch(v)) => v,
            _ => false,
        }
    }
}

impl ::protobuf::Message for VrApiError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::not_found(try!(is.read_string())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::already_exists(try!(is.read_string())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::does_not_exist(try!(is.read_string())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::wrong_type(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::path_must_end_in_directory(try!(is.read_string())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::path_must_be_absolute(try!(is.read_string())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::cas_failed(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::bad_format(try!(is.read_string())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::io(try!(is.read_string())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::encoding_error(try!(is.read_string())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::invalid_cas(try!(is.read_string())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::msg(try!(is.read_string())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::cannot_delete_root(try!(is.read_bool())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::invalid_msg(try!(is.read_bool())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::timeout(try!(is.read_bool())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::not_enough_replicas(try!(is.read_bool())));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(VrApiError_oneof_error::bad_epoch(try!(is.read_bool())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.error {
            match v {
                &VrApiError_oneof_error::not_found(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &VrApiError_oneof_error::already_exists(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &VrApiError_oneof_error::does_not_exist(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &VrApiError_oneof_error::wrong_type(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrApiError_oneof_error::path_must_end_in_directory(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
                &VrApiError_oneof_error::path_must_be_absolute(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
                &VrApiError_oneof_error::cas_failed(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &VrApiError_oneof_error::bad_format(ref v) => {
                    my_size += ::protobuf::rt::string_size(8, &v);
                },
                &VrApiError_oneof_error::io(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
                &VrApiError_oneof_error::encoding_error(ref v) => {
                    my_size += ::protobuf::rt::string_size(10, &v);
                },
                &VrApiError_oneof_error::invalid_cas(ref v) => {
                    my_size += ::protobuf::rt::string_size(11, &v);
                },
                &VrApiError_oneof_error::msg(ref v) => {
                    my_size += ::protobuf::rt::string_size(12, &v);
                },
                &VrApiError_oneof_error::cannot_delete_root(v) => {
                    my_size += 2;
                },
                &VrApiError_oneof_error::invalid_msg(v) => {
                    my_size += 2;
                },
                &VrApiError_oneof_error::timeout(v) => {
                    my_size += 2;
                },
                &VrApiError_oneof_error::not_enough_replicas(v) => {
                    my_size += 3;
                },
                &VrApiError_oneof_error::bad_epoch(v) => {
                    my_size += 3;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.error {
            match v {
                &VrApiError_oneof_error::not_found(ref v) => {
                    try!(os.write_string(1, v));
                },
                &VrApiError_oneof_error::already_exists(ref v) => {
                    try!(os.write_string(2, v));
                },
                &VrApiError_oneof_error::does_not_exist(ref v) => {
                    try!(os.write_string(3, v));
                },
                &VrApiError_oneof_error::wrong_type(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrApiError_oneof_error::path_must_end_in_directory(ref v) => {
                    try!(os.write_string(5, v));
                },
                &VrApiError_oneof_error::path_must_be_absolute(ref v) => {
                    try!(os.write_string(6, v));
                },
                &VrApiError_oneof_error::cas_failed(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &VrApiError_oneof_error::bad_format(ref v) => {
                    try!(os.write_string(8, v));
                },
                &VrApiError_oneof_error::io(ref v) => {
                    try!(os.write_string(9, v));
                },
                &VrApiError_oneof_error::encoding_error(ref v) => {
                    try!(os.write_string(10, v));
                },
                &VrApiError_oneof_error::invalid_cas(ref v) => {
                    try!(os.write_string(11, v));
                },
                &VrApiError_oneof_error::msg(ref v) => {
                    try!(os.write_string(12, v));
                },
                &VrApiError_oneof_error::cannot_delete_root(v) => {
                    try!(os.write_bool(13, v));
                },
                &VrApiError_oneof_error::invalid_msg(v) => {
                    try!(os.write_bool(14, v));
                },
                &VrApiError_oneof_error::timeout(v) => {
                    try!(os.write_bool(15, v));
                },
                &VrApiError_oneof_error::not_enough_replicas(v) => {
                    try!(os.write_bool(16, v));
                },
                &VrApiError_oneof_error::bad_epoch(v) => {
                    try!(os.write_bool(17, v));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VrApiError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VrApiError {
    fn new() -> VrApiError {
        VrApiError::new()
    }

    fn descriptor_static(_: ::std::option::Option<VrApiError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "not_found",
                    VrApiError::has_not_found,
                    VrApiError::get_not_found,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "already_exists",
                    VrApiError::has_already_exists,
                    VrApiError::get_already_exists,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "does_not_exist",
                    VrApiError::has_does_not_exist,
                    VrApiError::get_does_not_exist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "wrong_type",
                    VrApiError::has_wrong_type,
                    VrApiError::get_wrong_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path_must_end_in_directory",
                    VrApiError::has_path_must_end_in_directory,
                    VrApiError::get_path_must_end_in_directory,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path_must_be_absolute",
                    VrApiError::has_path_must_be_absolute,
                    VrApiError::get_path_must_be_absolute,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cas_failed",
                    VrApiError::has_cas_failed,
                    VrApiError::get_cas_failed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "bad_format",
                    VrApiError::has_bad_format,
                    VrApiError::get_bad_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "io",
                    VrApiError::has_io,
                    VrApiError::get_io,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "encoding_error",
                    VrApiError::has_encoding_error,
                    VrApiError::get_encoding_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "invalid_cas",
                    VrApiError::has_invalid_cas,
                    VrApiError::get_invalid_cas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg",
                    VrApiError::has_msg,
                    VrApiError::get_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "cannot_delete_root",
                    VrApiError::has_cannot_delete_root,
                    VrApiError::get_cannot_delete_root,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "invalid_msg",
                    VrApiError::has_invalid_msg,
                    VrApiError::get_invalid_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "timeout",
                    VrApiError::has_timeout,
                    VrApiError::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "not_enough_replicas",
                    VrApiError::has_not_enough_replicas,
                    VrApiError::get_not_enough_replicas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bad_epoch",
                    VrApiError::has_bad_epoch,
                    VrApiError::get_bad_epoch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VrApiError>(
                    "VrApiError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VrApiError {
    fn clear(&mut self) {
        self.clear_not_found();
        self.clear_already_exists();
        self.clear_does_not_exist();
        self.clear_wrong_type();
        self.clear_path_must_end_in_directory();
        self.clear_path_must_be_absolute();
        self.clear_cas_failed();
        self.clear_bad_format();
        self.clear_io();
        self.clear_encoding_error();
        self.clear_invalid_cas();
        self.clear_msg();
        self.clear_cannot_delete_root();
        self.clear_invalid_msg();
        self.clear_timeout();
        self.clear_not_enough_replicas();
        self.clear_bad_epoch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VrApiError {
    fn eq(&self, other: &VrApiError) -> bool {
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VrApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WrongType {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    node_type: ::std::option::Option<NodeType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WrongType {}

impl WrongType {
    pub fn new() -> WrongType {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WrongType {
        static mut instance: ::protobuf::lazy::Lazy<WrongType> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WrongType,
        };
        unsafe {
            instance.get(|| {
                WrongType {
                    path: ::protobuf::SingularField::none(),
                    node_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .NodeType node_type = 2;

    pub fn clear_node_type(&mut self) {
        self.node_type = ::std::option::Option::None;
    }

    pub fn has_node_type(&self) -> bool {
        self.node_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_type(&mut self, v: NodeType) {
        self.node_type = ::std::option::Option::Some(v);
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type.unwrap_or(NodeType::BlobType)
    }
}

impl ::protobuf::Message for WrongType {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.node_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.node_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.node_type {
            try!(os.write_enum(2, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WrongType>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WrongType {
    fn new() -> WrongType {
        WrongType::new()
    }

    fn descriptor_static(_: ::std::option::Option<WrongType>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    WrongType::has_path,
                    WrongType::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "node_type",
                    WrongType::has_node_type,
                    WrongType::get_node_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WrongType>(
                    "WrongType",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WrongType {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_node_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WrongType {
    fn eq(&self, other: &WrongType) -> bool {
        self.path == other.path &&
        self.node_type == other.node_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WrongType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CasFailed {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    expected: ::std::option::Option<u64>,
    actual: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CasFailed {}

impl CasFailed {
    pub fn new() -> CasFailed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CasFailed {
        static mut instance: ::protobuf::lazy::Lazy<CasFailed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CasFailed,
        };
        unsafe {
            instance.get(|| {
                CasFailed {
                    path: ::protobuf::SingularField::none(),
                    expected: ::std::option::Option::None,
                    actual: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 expected = 2;

    pub fn clear_expected(&mut self) {
        self.expected = ::std::option::Option::None;
    }

    pub fn has_expected(&self) -> bool {
        self.expected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expected(&mut self, v: u64) {
        self.expected = ::std::option::Option::Some(v);
    }

    pub fn get_expected(&self) -> u64 {
        self.expected.unwrap_or(0)
    }

    // optional uint64 actual = 3;

    pub fn clear_actual(&mut self) {
        self.actual = ::std::option::Option::None;
    }

    pub fn has_actual(&self) -> bool {
        self.actual.is_some()
    }

    // Param is passed by value, moved
    pub fn set_actual(&mut self, v: u64) {
        self.actual = ::std::option::Option::Some(v);
    }

    pub fn get_actual(&self) -> u64 {
        self.actual.unwrap_or(0)
    }
}

impl ::protobuf::Message for CasFailed {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.expected = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.actual = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.expected {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.actual {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.expected {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.actual {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CasFailed>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CasFailed {
    fn new() -> CasFailed {
        CasFailed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CasFailed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    CasFailed::has_path,
                    CasFailed::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "expected",
                    CasFailed::has_expected,
                    CasFailed::get_expected,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "actual",
                    CasFailed::has_actual,
                    CasFailed::get_actual,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CasFailed>(
                    "CasFailed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CasFailed {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_expected();
        self.clear_actual();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CasFailed {
    fn eq(&self, other: &CasFailed) -> bool {
        self.path == other.path &&
        self.expected == other.expected &&
        self.actual == other.actual &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CasFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TreeOpResult {
    // message fields
    version: ::std::option::Option<u64>,
    // message oneof groups
    result: ::std::option::Option<TreeOpResult_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TreeOpResult {}

#[derive(Clone,PartialEq)]
pub enum TreeOpResult_oneof_result {
    ok(bool),
    empty(bool),
    bool(bool),
    blob(::std::vec::Vec<u8>),
    int(u64),
    set(Set),
    keys(Keys),
}

impl TreeOpResult {
    pub fn new() -> TreeOpResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TreeOpResult {
        static mut instance: ::protobuf::lazy::Lazy<TreeOpResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TreeOpResult,
        };
        unsafe {
            instance.get(|| {
                TreeOpResult {
                    version: ::std::option::Option::None,
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    // optional bool ok = 2;

    pub fn clear_ok(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_ok(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::ok(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::ok(v))
    }

    pub fn get_ok(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::ok(v)) => v,
            _ => false,
        }
    }

    // optional bool empty = 3;

    pub fn clear_empty(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_empty(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::empty(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_empty(&mut self, v: bool) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::empty(v))
    }

    pub fn get_empty(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::empty(v)) => v,
            _ => false,
        }
    }

    // optional bool bool = 4;

    pub fn clear_bool(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_bool(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::bool(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool(&mut self, v: bool) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::bool(v))
    }

    pub fn get_bool(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::bool(v)) => v,
            _ => false,
        }
    }

    // optional bytes blob = 5;

    pub fn clear_blob(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_blob(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::blob(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_blob(&mut self, v: ::std::vec::Vec<u8>) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::blob(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(TreeOpResult_oneof_result::blob(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::blob(::std::vec::Vec::new()));
        }
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::blob(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_blob(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_blob() {
            match self.result.take() {
                ::std::option::Option::Some(TreeOpResult_oneof_result::blob(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_blob(&self) -> &[u8] {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::blob(ref v)) => v,
            _ => &[],
        }
    }

    // optional uint64 int = 6;

    pub fn clear_int(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_int(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::int(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int(&mut self, v: u64) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::int(v))
    }

    pub fn get_int(&self) -> u64 {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::int(v)) => v,
            _ => 0,
        }
    }

    // optional .Set set = 7;

    pub fn clear_set(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_set(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::set(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: Set) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::set(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set(&mut self) -> &mut Set {
        if let ::std::option::Option::Some(TreeOpResult_oneof_result::set(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::set(Set::new()));
        }
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::set(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set(&mut self) -> Set {
        if self.has_set() {
            match self.result.take() {
                ::std::option::Option::Some(TreeOpResult_oneof_result::set(v)) => v,
                _ => panic!(),
            }
        } else {
            Set::new()
        }
    }

    pub fn get_set(&self) -> &Set {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::set(ref v)) => v,
            _ => Set::default_instance(),
        }
    }

    // optional .Keys keys = 8;

    pub fn clear_keys(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_keys(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::keys(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: Keys) {
        self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::keys(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keys(&mut self) -> &mut Keys {
        if let ::std::option::Option::Some(TreeOpResult_oneof_result::keys(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::keys(Keys::new()));
        }
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::keys(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_keys(&mut self) -> Keys {
        if self.has_keys() {
            match self.result.take() {
                ::std::option::Option::Some(TreeOpResult_oneof_result::keys(v)) => v,
                _ => panic!(),
            }
        } else {
            Keys::new()
        }
    }

    pub fn get_keys(&self) -> &Keys {
        match self.result {
            ::std::option::Option::Some(TreeOpResult_oneof_result::keys(ref v)) => v,
            _ => Keys::default_instance(),
        }
    }
}

impl ::protobuf::Message for TreeOpResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::ok(try!(is.read_bool())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::empty(try!(is.read_bool())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::bool(try!(is.read_bool())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::blob(try!(is.read_bytes())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::int(try!(is.read_uint64())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::set(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::keys(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.version {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &TreeOpResult_oneof_result::ok(v) => {
                    my_size += 2;
                },
                &TreeOpResult_oneof_result::empty(v) => {
                    my_size += 2;
                },
                &TreeOpResult_oneof_result::bool(v) => {
                    my_size += 2;
                },
                &TreeOpResult_oneof_result::blob(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(5, &v);
                },
                &TreeOpResult_oneof_result::int(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TreeOpResult_oneof_result::set(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOpResult_oneof_result::keys(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_uint64(1, v));
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &TreeOpResult_oneof_result::ok(v) => {
                    try!(os.write_bool(2, v));
                },
                &TreeOpResult_oneof_result::empty(v) => {
                    try!(os.write_bool(3, v));
                },
                &TreeOpResult_oneof_result::bool(v) => {
                    try!(os.write_bool(4, v));
                },
                &TreeOpResult_oneof_result::blob(ref v) => {
                    try!(os.write_bytes(5, v));
                },
                &TreeOpResult_oneof_result::int(v) => {
                    try!(os.write_uint64(6, v));
                },
                &TreeOpResult_oneof_result::set(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &TreeOpResult_oneof_result::keys(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TreeOpResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TreeOpResult {
    fn new() -> TreeOpResult {
        TreeOpResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<TreeOpResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "version",
                    TreeOpResult::has_version,
                    TreeOpResult::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "ok",
                    TreeOpResult::has_ok,
                    TreeOpResult::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "empty",
                    TreeOpResult::has_empty,
                    TreeOpResult::get_empty,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool",
                    TreeOpResult::has_bool,
                    TreeOpResult::get_bool,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "blob",
                    TreeOpResult::has_blob,
                    TreeOpResult::get_blob,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "int",
                    TreeOpResult::has_int,
                    TreeOpResult::get_int,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set",
                    TreeOpResult::has_set,
                    TreeOpResult::get_set,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "keys",
                    TreeOpResult::has_keys,
                    TreeOpResult::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TreeOpResult>(
                    "TreeOpResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TreeOpResult {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_ok();
        self.clear_empty();
        self.clear_bool();
        self.clear_blob();
        self.clear_int();
        self.clear_set();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TreeOpResult {
    fn eq(&self, other: &TreeOpResult) -> bool {
        self.version == other.version &&
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TreeOpResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TreeCasResult {
    // message fields
    result: ::protobuf::RepeatedField<TreeOpResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TreeCasResult {}

impl TreeCasResult {
    pub fn new() -> TreeCasResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TreeCasResult {
        static mut instance: ::protobuf::lazy::Lazy<TreeCasResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TreeCasResult,
        };
        unsafe {
            instance.get(|| {
                TreeCasResult {
                    result: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TreeOpResult result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ::protobuf::RepeatedField<TreeOpResult>) {
        self.result = v;
    }

    // Mutable pointer to the field.
    pub fn mut_result(&mut self) -> &mut ::protobuf::RepeatedField<TreeOpResult> {
        &mut self.result
    }

    // Take field
    pub fn take_result(&mut self) -> ::protobuf::RepeatedField<TreeOpResult> {
        ::std::mem::replace(&mut self.result, ::protobuf::RepeatedField::new())
    }

    pub fn get_result(&self) -> &[TreeOpResult] {
        &self.result
    }
}

impl ::protobuf::Message for TreeCasResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.result));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.result {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.result {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TreeCasResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TreeCasResult {
    fn new() -> TreeCasResult {
        TreeCasResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<TreeCasResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "result",
                    TreeCasResult::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TreeCasResult>(
                    "TreeCasResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TreeCasResult {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TreeCasResult {
    fn eq(&self, other: &TreeCasResult) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TreeCasResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Keys {
    // message fields
    keys: ::protobuf::RepeatedField<Keys_KeysEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Keys {}

impl Keys {
    pub fn new() -> Keys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Keys {
        static mut instance: ::protobuf::lazy::Lazy<Keys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Keys,
        };
        unsafe {
            instance.get(|| {
                Keys {
                    keys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Keys.KeysEntry keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<Keys_KeysEntry>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<Keys_KeysEntry> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<Keys_KeysEntry> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[Keys_KeysEntry] {
        &self.keys
    }
}

impl ::protobuf::Message for Keys {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Keys>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Keys {
    fn new() -> Keys {
        Keys::new()
    }

    fn descriptor_static(_: ::std::option::Option<Keys>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    Keys::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Keys>(
                    "Keys",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Keys {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Keys {
    fn eq(&self, other: &Keys) -> bool {
        self.keys == other.keys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Keys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Keys_KeysEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Keys_KeysEntry {}

impl Keys_KeysEntry {
    pub fn new() -> Keys_KeysEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Keys_KeysEntry {
        static mut instance: ::protobuf::lazy::Lazy<Keys_KeysEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Keys_KeysEntry,
        };
        unsafe {
            instance.get(|| {
                Keys_KeysEntry {
                    key: ::protobuf::SingularField::none(),
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> u64 {
        self.value.unwrap_or(0)
    }
}

impl ::protobuf::Message for Keys_KeysEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.key {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Keys_KeysEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Keys_KeysEntry {
    fn new() -> Keys_KeysEntry {
        Keys_KeysEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Keys_KeysEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Keys_KeysEntry::has_key,
                    Keys_KeysEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "value",
                    Keys_KeysEntry::has_value,
                    Keys_KeysEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Keys_KeysEntry>(
                    "Keys_KeysEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Keys_KeysEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Keys_KeysEntry {
    fn eq(&self, other: &Keys_KeysEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Keys_KeysEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NamespaceMsg {
    // message oneof groups
    msg: ::std::option::Option<NamespaceMsg_oneof_msg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NamespaceMsg {}

#[derive(Clone,PartialEq)]
pub enum NamespaceMsg_oneof_msg {
    namespaces(Namespaces),
    register_client(RegisterClient),
    api_addr(::std::string::String),
    reconfiguration(NamespaceReconfiguration),
    stop(super::rabble_messages::Pid),
    new_primary(super::rabble_messages::Pid),
    clear_primary(::std::string::String),
}

impl NamespaceMsg {
    pub fn new() -> NamespaceMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamespaceMsg {
        static mut instance: ::protobuf::lazy::Lazy<NamespaceMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamespaceMsg,
        };
        unsafe {
            instance.get(|| {
                NamespaceMsg {
                    msg: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Namespaces namespaces = 1;

    pub fn clear_namespaces(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_namespaces(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_namespaces(&mut self, v: Namespaces) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespaces(&mut self) -> &mut Namespaces {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(Namespaces::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_namespaces(&mut self) -> Namespaces {
        if self.has_namespaces() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(v)) => v,
                _ => panic!(),
            }
        } else {
            Namespaces::new()
        }
    }

    pub fn get_namespaces(&self) -> &Namespaces {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(ref v)) => v,
            _ => Namespaces::default_instance(),
        }
    }

    // optional .RegisterClient register_client = 2;

    pub fn clear_register_client(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_register_client(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_register_client(&mut self, v: RegisterClient) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_register_client(&mut self) -> &mut RegisterClient {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(RegisterClient::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_register_client(&mut self) -> RegisterClient {
        if self.has_register_client() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(v)) => v,
                _ => panic!(),
            }
        } else {
            RegisterClient::new()
        }
    }

    pub fn get_register_client(&self) -> &RegisterClient {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(ref v)) => v,
            _ => RegisterClient::default_instance(),
        }
    }

    // optional string api_addr = 3;

    pub fn clear_api_addr(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_api_addr(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_api_addr(&mut self, v: ::std::string::String) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_addr(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(::std::string::String::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_api_addr(&mut self) -> ::std::string::String {
        if self.has_api_addr() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_api_addr(&self) -> &str {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(ref v)) => v,
            _ => "",
        }
    }

    // optional .NamespaceReconfiguration reconfiguration = 4;

    pub fn clear_reconfiguration(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_reconfiguration(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_reconfiguration(&mut self, v: NamespaceReconfiguration) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reconfiguration(&mut self) -> &mut NamespaceReconfiguration {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(NamespaceReconfiguration::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_reconfiguration(&mut self) -> NamespaceReconfiguration {
        if self.has_reconfiguration() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(v)) => v,
                _ => panic!(),
            }
        } else {
            NamespaceReconfiguration::new()
        }
    }

    pub fn get_reconfiguration(&self) -> &NamespaceReconfiguration {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(ref v)) => v,
            _ => NamespaceReconfiguration::default_instance(),
        }
    }

    // optional .Pid stop = 5;

    pub fn clear_stop(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: super::rabble_messages::Pid) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stop(&mut self) -> &mut super::rabble_messages::Pid {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(super::rabble_messages::Pid::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_stop(&mut self) -> super::rabble_messages::Pid {
        if self.has_stop() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Pid::new()
        }
    }

    pub fn get_stop(&self) -> &super::rabble_messages::Pid {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(ref v)) => v,
            _ => super::rabble_messages::Pid::default_instance(),
        }
    }

    // optional .Pid new_primary = 6;

    pub fn clear_new_primary(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_new_primary(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_new_primary(&mut self, v: super::rabble_messages::Pid) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_primary(&mut self) -> &mut super::rabble_messages::Pid {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(super::rabble_messages::Pid::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_new_primary(&mut self) -> super::rabble_messages::Pid {
        if self.has_new_primary() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Pid::new()
        }
    }

    pub fn get_new_primary(&self) -> &super::rabble_messages::Pid {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(ref v)) => v,
            _ => super::rabble_messages::Pid::default_instance(),
        }
    }

    // optional string clear_primary = 7;

    pub fn clear_clear_primary(&mut self) {
        self.msg = ::std::option::Option::None;
    }

    pub fn has_clear_primary(&self) -> bool {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_clear_primary(&mut self, v: ::std::string::String) {
        self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clear_primary(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(_)) = self.msg {
        } else {
            self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(::std::string::String::new()));
        }
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_clear_primary(&mut self) -> ::std::string::String {
        if self.has_clear_primary() {
            match self.msg.take() {
                ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_clear_primary(&self) -> &str {
        match self.msg {
            ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for NamespaceMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::namespaces(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::register_client(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::api_addr(try!(is.read_string())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::reconfiguration(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::stop(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::new_primary(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.msg = ::std::option::Option::Some(NamespaceMsg_oneof_msg::clear_primary(try!(is.read_string())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &NamespaceMsg_oneof_msg::namespaces(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NamespaceMsg_oneof_msg::register_client(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NamespaceMsg_oneof_msg::api_addr(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &NamespaceMsg_oneof_msg::reconfiguration(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NamespaceMsg_oneof_msg::stop(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NamespaceMsg_oneof_msg::new_primary(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NamespaceMsg_oneof_msg::clear_primary(ref v) => {
                    my_size += ::protobuf::rt::string_size(7, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.msg {
            match v {
                &NamespaceMsg_oneof_msg::namespaces(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NamespaceMsg_oneof_msg::register_client(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NamespaceMsg_oneof_msg::api_addr(ref v) => {
                    try!(os.write_string(3, v));
                },
                &NamespaceMsg_oneof_msg::reconfiguration(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NamespaceMsg_oneof_msg::stop(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NamespaceMsg_oneof_msg::new_primary(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NamespaceMsg_oneof_msg::clear_primary(ref v) => {
                    try!(os.write_string(7, v));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<NamespaceMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NamespaceMsg {
    fn new() -> NamespaceMsg {
        NamespaceMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamespaceMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "namespaces",
                    NamespaceMsg::has_namespaces,
                    NamespaceMsg::get_namespaces,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "register_client",
                    NamespaceMsg::has_register_client,
                    NamespaceMsg::get_register_client,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "api_addr",
                    NamespaceMsg::has_api_addr,
                    NamespaceMsg::get_api_addr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "reconfiguration",
                    NamespaceMsg::has_reconfiguration,
                    NamespaceMsg::get_reconfiguration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stop",
                    NamespaceMsg::has_stop,
                    NamespaceMsg::get_stop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new_primary",
                    NamespaceMsg::has_new_primary,
                    NamespaceMsg::get_new_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "clear_primary",
                    NamespaceMsg::has_clear_primary,
                    NamespaceMsg::get_clear_primary,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamespaceMsg>(
                    "NamespaceMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamespaceMsg {
    fn clear(&mut self) {
        self.clear_namespaces();
        self.clear_register_client();
        self.clear_api_addr();
        self.clear_reconfiguration();
        self.clear_stop();
        self.clear_new_primary();
        self.clear_clear_primary();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NamespaceMsg {
    fn eq(&self, other: &NamespaceMsg) -> bool {
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NamespaceMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Namespaces {
    // message fields
    map: ::protobuf::RepeatedField<Namespaces_MapEntry>,
    primaries: ::protobuf::RepeatedField<Namespaces_PrimariesEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Namespaces {}

impl Namespaces {
    pub fn new() -> Namespaces {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Namespaces {
        static mut instance: ::protobuf::lazy::Lazy<Namespaces> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Namespaces,
        };
        unsafe {
            instance.get(|| {
                Namespaces {
                    map: ::protobuf::RepeatedField::new(),
                    primaries: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Namespaces.MapEntry map = 1;

    pub fn clear_map(&mut self) {
        self.map.clear();
    }

    // Param is passed by value, moved
    pub fn set_map(&mut self, v: ::protobuf::RepeatedField<Namespaces_MapEntry>) {
        self.map = v;
    }

    // Mutable pointer to the field.
    pub fn mut_map(&mut self) -> &mut ::protobuf::RepeatedField<Namespaces_MapEntry> {
        &mut self.map
    }

    // Take field
    pub fn take_map(&mut self) -> ::protobuf::RepeatedField<Namespaces_MapEntry> {
        ::std::mem::replace(&mut self.map, ::protobuf::RepeatedField::new())
    }

    pub fn get_map(&self) -> &[Namespaces_MapEntry] {
        &self.map
    }

    // repeated .Namespaces.PrimariesEntry primaries = 2;

    pub fn clear_primaries(&mut self) {
        self.primaries.clear();
    }

    // Param is passed by value, moved
    pub fn set_primaries(&mut self, v: ::protobuf::RepeatedField<Namespaces_PrimariesEntry>) {
        self.primaries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_primaries(&mut self) -> &mut ::protobuf::RepeatedField<Namespaces_PrimariesEntry> {
        &mut self.primaries
    }

    // Take field
    pub fn take_primaries(&mut self) -> ::protobuf::RepeatedField<Namespaces_PrimariesEntry> {
        ::std::mem::replace(&mut self.primaries, ::protobuf::RepeatedField::new())
    }

    pub fn get_primaries(&self) -> &[Namespaces_PrimariesEntry] {
        &self.primaries
    }
}

impl ::protobuf::Message for Namespaces {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.map));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.primaries));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.map {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.primaries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.map {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.primaries {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Namespaces>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Namespaces {
    fn new() -> Namespaces {
        Namespaces::new()
    }

    fn descriptor_static(_: ::std::option::Option<Namespaces>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "map",
                    Namespaces::get_map,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "primaries",
                    Namespaces::get_primaries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Namespaces>(
                    "Namespaces",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Namespaces {
    fn clear(&mut self) {
        self.clear_map();
        self.clear_primaries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Namespaces {
    fn eq(&self, other: &Namespaces) -> bool {
        self.map == other.map &&
        self.primaries == other.primaries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Namespaces {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Namespaces_MapEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<ReplicaConfig>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Namespaces_MapEntry {}

impl Namespaces_MapEntry {
    pub fn new() -> Namespaces_MapEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Namespaces_MapEntry {
        static mut instance: ::protobuf::lazy::Lazy<Namespaces_MapEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Namespaces_MapEntry,
        };
        unsafe {
            instance.get(|| {
                Namespaces_MapEntry {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .ReplicaConfig value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ReplicaConfig) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ReplicaConfig {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ReplicaConfig {
        self.value.take().unwrap_or_else(|| ReplicaConfig::new())
    }

    pub fn get_value(&self) -> &ReplicaConfig {
        self.value.as_ref().unwrap_or_else(|| ReplicaConfig::default_instance())
    }
}

impl ::protobuf::Message for Namespaces_MapEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.key {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Namespaces_MapEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Namespaces_MapEntry {
    fn new() -> Namespaces_MapEntry {
        Namespaces_MapEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Namespaces_MapEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Namespaces_MapEntry::has_key,
                    Namespaces_MapEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    Namespaces_MapEntry::has_value,
                    Namespaces_MapEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Namespaces_MapEntry>(
                    "Namespaces_MapEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Namespaces_MapEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Namespaces_MapEntry {
    fn eq(&self, other: &Namespaces_MapEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Namespaces_MapEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Namespaces_PrimariesEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Namespaces_PrimariesEntry {}

impl Namespaces_PrimariesEntry {
    pub fn new() -> Namespaces_PrimariesEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Namespaces_PrimariesEntry {
        static mut instance: ::protobuf::lazy::Lazy<Namespaces_PrimariesEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Namespaces_PrimariesEntry,
        };
        unsafe {
            instance.get(|| {
                Namespaces_PrimariesEntry {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Pid value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::rabble_messages::Pid) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut super::rabble_messages::Pid {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> super::rabble_messages::Pid {
        self.value.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_value(&self) -> &super::rabble_messages::Pid {
        self.value.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }
}

impl ::protobuf::Message for Namespaces_PrimariesEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.key {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Namespaces_PrimariesEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Namespaces_PrimariesEntry {
    fn new() -> Namespaces_PrimariesEntry {
        Namespaces_PrimariesEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Namespaces_PrimariesEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Namespaces_PrimariesEntry::has_key,
                    Namespaces_PrimariesEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    Namespaces_PrimariesEntry::has_value,
                    Namespaces_PrimariesEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Namespaces_PrimariesEntry>(
                    "Namespaces_PrimariesEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Namespaces_PrimariesEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Namespaces_PrimariesEntry {
    fn eq(&self, other: &Namespaces_PrimariesEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Namespaces_PrimariesEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReplicaConfig {
    // message fields
    old: ::protobuf::SingularPtrField<VersionedReplicas>,
    new: ::protobuf::SingularPtrField<VersionedReplicas>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicaConfig {}

impl ReplicaConfig {
    pub fn new() -> ReplicaConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaConfig {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaConfig,
        };
        unsafe {
            instance.get(|| {
                ReplicaConfig {
                    old: ::protobuf::SingularPtrField::none(),
                    new: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .VersionedReplicas old = 1;

    pub fn clear_old(&mut self) {
        self.old.clear();
    }

    pub fn has_old(&self) -> bool {
        self.old.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old(&mut self, v: VersionedReplicas) {
        self.old = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old(&mut self) -> &mut VersionedReplicas {
        if self.old.is_none() {
            self.old.set_default();
        };
        self.old.as_mut().unwrap()
    }

    // Take field
    pub fn take_old(&mut self) -> VersionedReplicas {
        self.old.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_old(&self) -> &VersionedReplicas {
        self.old.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }

    // optional .VersionedReplicas new = 2;

    pub fn clear_new(&mut self) {
        self.new.clear();
    }

    pub fn has_new(&self) -> bool {
        self.new.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new(&mut self, v: VersionedReplicas) {
        self.new = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new(&mut self) -> &mut VersionedReplicas {
        if self.new.is_none() {
            self.new.set_default();
        };
        self.new.as_mut().unwrap()
    }

    // Take field
    pub fn take_new(&mut self) -> VersionedReplicas {
        self.new.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_new(&self) -> &VersionedReplicas {
        self.new.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }
}

impl ::protobuf::Message for ReplicaConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.old {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.new {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.old.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.new.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReplicaConfig>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReplicaConfig {
    fn new() -> ReplicaConfig {
        ReplicaConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "old",
                    ReplicaConfig::has_old,
                    ReplicaConfig::get_old,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new",
                    ReplicaConfig::has_new,
                    ReplicaConfig::get_new,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaConfig>(
                    "ReplicaConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaConfig {
    fn clear(&mut self) {
        self.clear_old();
        self.clear_new();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReplicaConfig {
    fn eq(&self, other: &ReplicaConfig) -> bool {
        self.old == other.old &&
        self.new == other.new &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReplicaConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegisterClient {
    // message fields
    client_id: ::protobuf::SingularField<::std::string::String>,
    namespace_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterClient {}

impl RegisterClient {
    pub fn new() -> RegisterClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterClient {
        static mut instance: ::protobuf::lazy::Lazy<RegisterClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterClient,
        };
        unsafe {
            instance.get(|| {
                RegisterClient {
                    client_id: ::protobuf::SingularField::none(),
                    namespace_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::string::String) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::string::String {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::string::String {
        self.client_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_client_id(&self) -> &str {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string namespace_id = 2;

    pub fn clear_namespace_id(&mut self) {
        self.namespace_id.clear();
    }

    pub fn has_namespace_id(&self) -> bool {
        self.namespace_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespace_id(&mut self, v: ::std::string::String) {
        self.namespace_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace_id(&mut self) -> &mut ::std::string::String {
        if self.namespace_id.is_none() {
            self.namespace_id.set_default();
        };
        self.namespace_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_namespace_id(&mut self) -> ::std::string::String {
        self.namespace_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_namespace_id(&self) -> &str {
        match self.namespace_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for RegisterClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namespace_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.client_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.namespace_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.namespace_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RegisterClient>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegisterClient {
    fn new() -> RegisterClient {
        RegisterClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "client_id",
                    RegisterClient::has_client_id,
                    RegisterClient::get_client_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "namespace_id",
                    RegisterClient::has_namespace_id,
                    RegisterClient::get_namespace_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterClient>(
                    "RegisterClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterClient {
    fn clear(&mut self) {
        self.clear_client_id();
        self.clear_namespace_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegisterClient {
    fn eq(&self, other: &RegisterClient) -> bool {
        self.client_id == other.client_id &&
        self.namespace_id == other.namespace_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegisterClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NamespaceReconfiguration {
    // message fields
    namespace_id: ::protobuf::SingularField<::std::string::String>,
    old_config: ::protobuf::SingularPtrField<VersionedReplicas>,
    new_config: ::protobuf::SingularPtrField<VersionedReplicas>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NamespaceReconfiguration {}

impl NamespaceReconfiguration {
    pub fn new() -> NamespaceReconfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamespaceReconfiguration {
        static mut instance: ::protobuf::lazy::Lazy<NamespaceReconfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamespaceReconfiguration,
        };
        unsafe {
            instance.get(|| {
                NamespaceReconfiguration {
                    namespace_id: ::protobuf::SingularField::none(),
                    old_config: ::protobuf::SingularPtrField::none(),
                    new_config: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string namespace_id = 1;

    pub fn clear_namespace_id(&mut self) {
        self.namespace_id.clear();
    }

    pub fn has_namespace_id(&self) -> bool {
        self.namespace_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespace_id(&mut self, v: ::std::string::String) {
        self.namespace_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace_id(&mut self) -> &mut ::std::string::String {
        if self.namespace_id.is_none() {
            self.namespace_id.set_default();
        };
        self.namespace_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_namespace_id(&mut self) -> ::std::string::String {
        self.namespace_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_namespace_id(&self) -> &str {
        match self.namespace_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .VersionedReplicas old_config = 2;

    pub fn clear_old_config(&mut self) {
        self.old_config.clear();
    }

    pub fn has_old_config(&self) -> bool {
        self.old_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_config(&mut self, v: VersionedReplicas) {
        self.old_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_config(&mut self) -> &mut VersionedReplicas {
        if self.old_config.is_none() {
            self.old_config.set_default();
        };
        self.old_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_config(&mut self) -> VersionedReplicas {
        self.old_config.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_old_config(&self) -> &VersionedReplicas {
        self.old_config.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }

    // optional .VersionedReplicas new_config = 3;

    pub fn clear_new_config(&mut self) {
        self.new_config.clear();
    }

    pub fn has_new_config(&self) -> bool {
        self.new_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_config(&mut self, v: VersionedReplicas) {
        self.new_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_config(&mut self) -> &mut VersionedReplicas {
        if self.new_config.is_none() {
            self.new_config.set_default();
        };
        self.new_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_config(&mut self) -> VersionedReplicas {
        self.new_config.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_new_config(&self) -> &VersionedReplicas {
        self.new_config.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }
}

impl ::protobuf::Message for NamespaceReconfiguration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namespace_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_config));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new_config));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.namespace_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.old_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.new_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.namespace_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.old_config.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.new_config.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<NamespaceReconfiguration>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NamespaceReconfiguration {
    fn new() -> NamespaceReconfiguration {
        NamespaceReconfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamespaceReconfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "namespace_id",
                    NamespaceReconfiguration::has_namespace_id,
                    NamespaceReconfiguration::get_namespace_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "old_config",
                    NamespaceReconfiguration::has_old_config,
                    NamespaceReconfiguration::get_old_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new_config",
                    NamespaceReconfiguration::has_new_config,
                    NamespaceReconfiguration::get_new_config,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamespaceReconfiguration>(
                    "NamespaceReconfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamespaceReconfiguration {
    fn clear(&mut self) {
        self.clear_namespace_id();
        self.clear_old_config();
        self.clear_new_config();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NamespaceReconfiguration {
    fn eq(&self, other: &NamespaceReconfiguration) -> bool {
        self.namespace_id == other.namespace_id &&
        self.old_config == other.old_config &&
        self.new_config == other.new_config &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NamespaceReconfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VersionedReplicas {
    // message fields
    epoch: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    replicas: ::protobuf::RepeatedField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VersionedReplicas {}

impl VersionedReplicas {
    pub fn new() -> VersionedReplicas {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionedReplicas {
        static mut instance: ::protobuf::lazy::Lazy<VersionedReplicas> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionedReplicas,
        };
        unsafe {
            instance.get(|| {
                VersionedReplicas {
                    epoch: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    replicas: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 epoch = 1;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 op = 2;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // repeated .Pid replicas = 3;

    pub fn clear_replicas(&mut self) {
        self.replicas.clear();
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: ::protobuf::RepeatedField<super::rabble_messages::Pid>) {
        self.replicas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_replicas(&mut self) -> &mut ::protobuf::RepeatedField<super::rabble_messages::Pid> {
        &mut self.replicas
    }

    // Take field
    pub fn take_replicas(&mut self) -> ::protobuf::RepeatedField<super::rabble_messages::Pid> {
        ::std::mem::replace(&mut self.replicas, ::protobuf::RepeatedField::new())
    }

    pub fn get_replicas(&self) -> &[super::rabble_messages::Pid] {
        &self.replicas
    }
}

impl ::protobuf::Message for VersionedReplicas {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replicas));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.replicas {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.epoch {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(2, v));
        };
        for v in &self.replicas {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VersionedReplicas>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VersionedReplicas {
    fn new() -> VersionedReplicas {
        VersionedReplicas::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionedReplicas>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    VersionedReplicas::has_epoch,
                    VersionedReplicas::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    VersionedReplicas::has_op,
                    VersionedReplicas::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "replicas",
                    VersionedReplicas::get_replicas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionedReplicas>(
                    "VersionedReplicas",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionedReplicas {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_op();
        self.clear_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VersionedReplicas {
    fn eq(&self, other: &VersionedReplicas) -> bool {
        self.epoch == other.epoch &&
        self.op == other.op &&
        self.replicas == other.replicas &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VersionedReplicas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ApiRpy {
    // message oneof groups
    rpy: ::std::option::Option<ApiRpy_oneof_rpy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiRpy {}

#[derive(Clone,PartialEq)]
pub enum ApiRpy_oneof_rpy {
    client_registration(ClientRegistration),
    redirect(Redirect),
    retry(u64),
    unknown_namespace(bool),
}

impl ApiRpy {
    pub fn new() -> ApiRpy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiRpy {
        static mut instance: ::protobuf::lazy::Lazy<ApiRpy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiRpy,
        };
        unsafe {
            instance.get(|| {
                ApiRpy {
                    rpy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ClientRegistration client_registration = 1;

    pub fn clear_client_registration(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_client_registration(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_client_registration(&mut self, v: ClientRegistration) {
        self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_registration(&mut self) -> &mut ClientRegistration {
        if let ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(ClientRegistration::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_client_registration(&mut self) -> ClientRegistration {
        if self.has_client_registration() {
            match self.rpy.take() {
                ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(v)) => v,
                _ => panic!(),
            }
        } else {
            ClientRegistration::new()
        }
    }

    pub fn get_client_registration(&self) -> &ClientRegistration {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(ref v)) => v,
            _ => ClientRegistration::default_instance(),
        }
    }

    // optional .Redirect redirect = 2;

    pub fn clear_redirect(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_redirect(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_redirect(&mut self, v: Redirect) {
        self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect(&mut self) -> &mut Redirect {
        if let ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(Redirect::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_redirect(&mut self) -> Redirect {
        if self.has_redirect() {
            match self.rpy.take() {
                ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(v)) => v,
                _ => panic!(),
            }
        } else {
            Redirect::new()
        }
    }

    pub fn get_redirect(&self) -> &Redirect {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(ref v)) => v,
            _ => Redirect::default_instance(),
        }
    }

    // optional uint64 retry = 3;

    pub fn clear_retry(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_retry(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::retry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_retry(&mut self, v: u64) {
        self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::retry(v))
    }

    pub fn get_retry(&self) -> u64 {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::retry(v)) => v,
            _ => 0,
        }
    }

    // optional bool unknown_namespace = 4;

    pub fn clear_unknown_namespace(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_unknown_namespace(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::unknown_namespace(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unknown_namespace(&mut self, v: bool) {
        self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::unknown_namespace(v))
    }

    pub fn get_unknown_namespace(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(ApiRpy_oneof_rpy::unknown_namespace(v)) => v,
            _ => false,
        }
    }
}

impl ::protobuf::Message for ApiRpy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::client_registration(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::redirect(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::retry(try!(is.read_uint64())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(ApiRpy_oneof_rpy::unknown_namespace(try!(is.read_bool())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.rpy {
            match v {
                &ApiRpy_oneof_rpy::client_registration(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiRpy_oneof_rpy::redirect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiRpy_oneof_rpy::retry(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &ApiRpy_oneof_rpy::unknown_namespace(v) => {
                    my_size += 2;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.rpy {
            match v {
                &ApiRpy_oneof_rpy::client_registration(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ApiRpy_oneof_rpy::redirect(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ApiRpy_oneof_rpy::retry(v) => {
                    try!(os.write_uint64(3, v));
                },
                &ApiRpy_oneof_rpy::unknown_namespace(v) => {
                    try!(os.write_bool(4, v));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ApiRpy>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiRpy {
    fn new() -> ApiRpy {
        ApiRpy::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiRpy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "client_registration",
                    ApiRpy::has_client_registration,
                    ApiRpy::get_client_registration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "redirect",
                    ApiRpy::has_redirect,
                    ApiRpy::get_redirect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "retry",
                    ApiRpy::has_retry,
                    ApiRpy::get_retry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "unknown_namespace",
                    ApiRpy::has_unknown_namespace,
                    ApiRpy::get_unknown_namespace,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiRpy>(
                    "ApiRpy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiRpy {
    fn clear(&mut self) {
        self.clear_client_registration();
        self.clear_redirect();
        self.clear_retry();
        self.clear_unknown_namespace();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ApiRpy {
    fn eq(&self, other: &ApiRpy) -> bool {
        self.rpy == other.rpy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ApiRpy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientRegistration {
    // message fields
    primary: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    new_registration: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientRegistration {}

impl ClientRegistration {
    pub fn new() -> ClientRegistration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientRegistration {
        static mut instance: ::protobuf::lazy::Lazy<ClientRegistration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientRegistration,
        };
        unsafe {
            instance.get(|| {
                ClientRegistration {
                    primary: ::protobuf::SingularPtrField::none(),
                    new_registration: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Pid primary = 1;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: super::rabble_messages::Pid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut super::rabble_messages::Pid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> super::rabble_messages::Pid {
        self.primary.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_primary(&self) -> &super::rabble_messages::Pid {
        self.primary.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional bool new_registration = 2;

    pub fn clear_new_registration(&mut self) {
        self.new_registration = ::std::option::Option::None;
    }

    pub fn has_new_registration(&self) -> bool {
        self.new_registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_registration(&mut self, v: bool) {
        self.new_registration = ::std::option::Option::Some(v);
    }

    pub fn get_new_registration(&self) -> bool {
        self.new_registration.unwrap_or(false)
    }
}

impl ::protobuf::Message for ClientRegistration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.new_registration = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.primary {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.new_registration.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.new_registration {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientRegistration>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientRegistration {
    fn new() -> ClientRegistration {
        ClientRegistration::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientRegistration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "primary",
                    ClientRegistration::has_primary,
                    ClientRegistration::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "new_registration",
                    ClientRegistration::has_new_registration,
                    ClientRegistration::get_new_registration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientRegistration>(
                    "ClientRegistration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientRegistration {
    fn clear(&mut self) {
        self.clear_primary();
        self.clear_new_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientRegistration {
    fn eq(&self, other: &ClientRegistration) -> bool {
        self.primary == other.primary &&
        self.new_registration == other.new_registration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientRegistration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Redirect {
    // message fields
    primary: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    api_addr: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Redirect {}

impl Redirect {
    pub fn new() -> Redirect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Redirect {
        static mut instance: ::protobuf::lazy::Lazy<Redirect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Redirect,
        };
        unsafe {
            instance.get(|| {
                Redirect {
                    primary: ::protobuf::SingularPtrField::none(),
                    api_addr: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Pid primary = 1;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: super::rabble_messages::Pid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut super::rabble_messages::Pid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> super::rabble_messages::Pid {
        self.primary.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_primary(&self) -> &super::rabble_messages::Pid {
        self.primary.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional string api_addr = 2;

    pub fn clear_api_addr(&mut self) {
        self.api_addr.clear();
    }

    pub fn has_api_addr(&self) -> bool {
        self.api_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_api_addr(&mut self, v: ::std::string::String) {
        self.api_addr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_addr(&mut self) -> &mut ::std::string::String {
        if self.api_addr.is_none() {
            self.api_addr.set_default();
        };
        self.api_addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_api_addr(&mut self) -> ::std::string::String {
        self.api_addr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_api_addr(&self) -> &str {
        match self.api_addr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Redirect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.api_addr));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.primary {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.api_addr {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.api_addr.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Redirect>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Redirect {
    fn new() -> Redirect {
        Redirect::new()
    }

    fn descriptor_static(_: ::std::option::Option<Redirect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "primary",
                    Redirect::has_primary,
                    Redirect::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "api_addr",
                    Redirect::has_api_addr,
                    Redirect::get_api_addr,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Redirect>(
                    "Redirect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Redirect {
    fn clear(&mut self) {
        self.clear_primary();
        self.clear_api_addr();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Redirect {
    fn eq(&self, other: &Redirect) -> bool {
        self.primary == other.primary &&
        self.api_addr == other.api_addr &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Redirect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AdminReq {
    // message oneof groups
    req: ::std::option::Option<AdminReq_oneof_req>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminReq {}

#[derive(Clone,PartialEq)]
pub enum AdminReq_oneof_req {
    get_config(bool),
    join(super::rabble_messages::NodeId),
    create_namespace(super::rabble_messages::Pids),
    get_namespaces(bool),
    get_replica_state(super::rabble_messages::Pid),
    get_primary(::std::string::String),
    get_cluster_status(bool),
    get_metrics(super::rabble_messages::Pid),
}

impl AdminReq {
    pub fn new() -> AdminReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminReq {
        static mut instance: ::protobuf::lazy::Lazy<AdminReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminReq,
        };
        unsafe {
            instance.get(|| {
                AdminReq {
                    req: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool get_config = 1;

    pub fn clear_get_config(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_get_config(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_config(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_config(&mut self, v: bool) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_config(v))
    }

    pub fn get_get_config(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_config(v)) => v,
            _ => false,
        }
    }

    // optional .NodeId join = 2;

    pub fn clear_join(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_join(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::join(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_join(&mut self, v: super::rabble_messages::NodeId) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::join(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_join(&mut self) -> &mut super::rabble_messages::NodeId {
        if let ::std::option::Option::Some(AdminReq_oneof_req::join(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(AdminReq_oneof_req::join(super::rabble_messages::NodeId::new()));
        }
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::join(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_join(&mut self) -> super::rabble_messages::NodeId {
        if self.has_join() {
            match self.req.take() {
                ::std::option::Option::Some(AdminReq_oneof_req::join(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::NodeId::new()
        }
    }

    pub fn get_join(&self) -> &super::rabble_messages::NodeId {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::join(ref v)) => v,
            _ => super::rabble_messages::NodeId::default_instance(),
        }
    }

    // optional .Pids create_namespace = 3;

    pub fn clear_create_namespace(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_create_namespace(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_namespace(&mut self, v: super::rabble_messages::Pids) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_namespace(&mut self) -> &mut super::rabble_messages::Pids {
        if let ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(super::rabble_messages::Pids::new()));
        }
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_namespace(&mut self) -> super::rabble_messages::Pids {
        if self.has_create_namespace() {
            match self.req.take() {
                ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Pids::new()
        }
    }

    pub fn get_create_namespace(&self) -> &super::rabble_messages::Pids {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(ref v)) => v,
            _ => super::rabble_messages::Pids::default_instance(),
        }
    }

    // optional bool get_namespaces = 4;

    pub fn clear_get_namespaces(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_get_namespaces(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_namespaces(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_namespaces(&mut self, v: bool) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_namespaces(v))
    }

    pub fn get_get_namespaces(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_namespaces(v)) => v,
            _ => false,
        }
    }

    // optional .Pid get_replica_state = 5;

    pub fn clear_get_replica_state(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_get_replica_state(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_replica_state(&mut self, v: super::rabble_messages::Pid) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_replica_state(&mut self) -> &mut super::rabble_messages::Pid {
        if let ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(super::rabble_messages::Pid::new()));
        }
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_replica_state(&mut self) -> super::rabble_messages::Pid {
        if self.has_get_replica_state() {
            match self.req.take() {
                ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Pid::new()
        }
    }

    pub fn get_get_replica_state(&self) -> &super::rabble_messages::Pid {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(ref v)) => v,
            _ => super::rabble_messages::Pid::default_instance(),
        }
    }

    // optional string get_primary = 6;

    pub fn clear_get_primary(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_get_primary(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_primary(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_primary(&mut self, v: ::std::string::String) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_primary(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_primary(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(AdminReq_oneof_req::get_primary(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_primary(::std::string::String::new()));
        }
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_primary(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_primary(&mut self) -> ::std::string::String {
        if self.has_get_primary() {
            match self.req.take() {
                ::std::option::Option::Some(AdminReq_oneof_req::get_primary(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_get_primary(&self) -> &str {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_primary(ref v)) => v,
            _ => "",
        }
    }

    // optional bool get_cluster_status = 7;

    pub fn clear_get_cluster_status(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_get_cluster_status(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_cluster_status(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_cluster_status(&mut self, v: bool) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_cluster_status(v))
    }

    pub fn get_get_cluster_status(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_cluster_status(v)) => v,
            _ => false,
        }
    }

    // optional .Pid get_metrics = 8;

    pub fn clear_get_metrics(&mut self) {
        self.req = ::std::option::Option::None;
    }

    pub fn has_get_metrics(&self) -> bool {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_metrics(&mut self, v: super::rabble_messages::Pid) {
        self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_metrics(&mut self) -> &mut super::rabble_messages::Pid {
        if let ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(_)) = self.req {
        } else {
            self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(super::rabble_messages::Pid::new()));
        }
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_metrics(&mut self) -> super::rabble_messages::Pid {
        if self.has_get_metrics() {
            match self.req.take() {
                ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Pid::new()
        }
    }

    pub fn get_get_metrics(&self) -> &super::rabble_messages::Pid {
        match self.req {
            ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(ref v)) => v,
            _ => super::rabble_messages::Pid::default_instance(),
        }
    }
}

impl ::protobuf::Message for AdminReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_config(try!(is.read_bool())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::join(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::create_namespace(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_namespaces(try!(is.read_bool())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_replica_state(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_primary(try!(is.read_string())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_cluster_status(try!(is.read_bool())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req = ::std::option::Option::Some(AdminReq_oneof_req::get_metrics(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.req {
            match v {
                &AdminReq_oneof_req::get_config(v) => {
                    my_size += 2;
                },
                &AdminReq_oneof_req::join(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminReq_oneof_req::create_namespace(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminReq_oneof_req::get_namespaces(v) => {
                    my_size += 2;
                },
                &AdminReq_oneof_req::get_replica_state(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminReq_oneof_req::get_primary(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
                &AdminReq_oneof_req::get_cluster_status(v) => {
                    my_size += 2;
                },
                &AdminReq_oneof_req::get_metrics(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.req {
            match v {
                &AdminReq_oneof_req::get_config(v) => {
                    try!(os.write_bool(1, v));
                },
                &AdminReq_oneof_req::join(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminReq_oneof_req::create_namespace(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminReq_oneof_req::get_namespaces(v) => {
                    try!(os.write_bool(4, v));
                },
                &AdminReq_oneof_req::get_replica_state(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminReq_oneof_req::get_primary(ref v) => {
                    try!(os.write_string(6, v));
                },
                &AdminReq_oneof_req::get_cluster_status(v) => {
                    try!(os.write_bool(7, v));
                },
                &AdminReq_oneof_req::get_metrics(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AdminReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminReq {
    fn new() -> AdminReq {
        AdminReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "get_config",
                    AdminReq::has_get_config,
                    AdminReq::get_get_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "join",
                    AdminReq::has_join,
                    AdminReq::get_join,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "create_namespace",
                    AdminReq::has_create_namespace,
                    AdminReq::get_create_namespace,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "get_namespaces",
                    AdminReq::has_get_namespaces,
                    AdminReq::get_get_namespaces,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_replica_state",
                    AdminReq::has_get_replica_state,
                    AdminReq::get_get_replica_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "get_primary",
                    AdminReq::has_get_primary,
                    AdminReq::get_get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "get_cluster_status",
                    AdminReq::has_get_cluster_status,
                    AdminReq::get_get_cluster_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_metrics",
                    AdminReq::has_get_metrics,
                    AdminReq::get_get_metrics,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminReq>(
                    "AdminReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminReq {
    fn clear(&mut self) {
        self.clear_get_config();
        self.clear_join();
        self.clear_create_namespace();
        self.clear_get_namespaces();
        self.clear_get_replica_state();
        self.clear_get_primary();
        self.clear_get_cluster_status();
        self.clear_get_metrics();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AdminReq {
    fn eq(&self, other: &AdminReq) -> bool {
        self.req == other.req &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AdminReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AdminRpy {
    // message oneof groups
    rpy: ::std::option::Option<AdminRpy_oneof_rpy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminRpy {}

#[derive(Clone,PartialEq)]
pub enum AdminRpy_oneof_rpy {
    ok(bool),
    timeout(bool),
    error(::std::string::String),
    config(Config),
    namespace_id(::std::string::String),
    namespaces(Namespaces),
    replica_state(VrCtxSummary),
    replica_not_found(super::rabble_messages::Pid),
    primary(Primary),
    metrics(super::rabble_messages::Metrics),
}

impl AdminRpy {
    pub fn new() -> AdminRpy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminRpy {
        static mut instance: ::protobuf::lazy::Lazy<AdminRpy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminRpy,
        };
        unsafe {
            instance.get(|| {
                AdminRpy {
                    rpy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool ok = 1;

    pub fn clear_ok(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_ok(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::ok(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::ok(v))
    }

    pub fn get_ok(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::ok(v)) => v,
            _ => false,
        }
    }

    // optional bool timeout = 2;

    pub fn clear_timeout(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::timeout(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: bool) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::timeout(v))
    }

    pub fn get_timeout(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::timeout(v)) => v,
            _ => false,
        }
    }

    // optional string error = 3;

    pub fn clear_error(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::error(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::error(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::error(::std::string::String::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        if self.has_error() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_error(&self) -> &str {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::error(ref v)) => v,
            _ => "",
        }
    }

    // optional .Config config = 4;

    pub fn clear_config(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_config(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::config(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: Config) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::config(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut Config {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::config(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::config(Config::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::config(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_config(&mut self) -> Config {
        if self.has_config() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::config(v)) => v,
                _ => panic!(),
            }
        } else {
            Config::new()
        }
    }

    pub fn get_config(&self) -> &Config {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::config(ref v)) => v,
            _ => Config::default_instance(),
        }
    }

    // optional string namespace_id = 5;

    pub fn clear_namespace_id(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_namespace_id(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_namespace_id(&mut self, v: ::std::string::String) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace_id(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(::std::string::String::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_namespace_id(&mut self) -> ::std::string::String {
        if self.has_namespace_id() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_namespace_id(&self) -> &str {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(ref v)) => v,
            _ => "",
        }
    }

    // optional .Namespaces namespaces = 6;

    pub fn clear_namespaces(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_namespaces(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_namespaces(&mut self, v: Namespaces) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespaces(&mut self) -> &mut Namespaces {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(Namespaces::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_namespaces(&mut self) -> Namespaces {
        if self.has_namespaces() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(v)) => v,
                _ => panic!(),
            }
        } else {
            Namespaces::new()
        }
    }

    pub fn get_namespaces(&self) -> &Namespaces {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(ref v)) => v,
            _ => Namespaces::default_instance(),
        }
    }

    // optional .VrCtxSummary replica_state = 7;

    pub fn clear_replica_state(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_replica_state(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replica_state(&mut self, v: VrCtxSummary) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replica_state(&mut self) -> &mut VrCtxSummary {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(VrCtxSummary::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replica_state(&mut self) -> VrCtxSummary {
        if self.has_replica_state() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(v)) => v,
                _ => panic!(),
            }
        } else {
            VrCtxSummary::new()
        }
    }

    pub fn get_replica_state(&self) -> &VrCtxSummary {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(ref v)) => v,
            _ => VrCtxSummary::default_instance(),
        }
    }

    // optional .Pid replica_not_found = 8;

    pub fn clear_replica_not_found(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_replica_not_found(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replica_not_found(&mut self, v: super::rabble_messages::Pid) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replica_not_found(&mut self) -> &mut super::rabble_messages::Pid {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(super::rabble_messages::Pid::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replica_not_found(&mut self) -> super::rabble_messages::Pid {
        if self.has_replica_not_found() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Pid::new()
        }
    }

    pub fn get_replica_not_found(&self) -> &super::rabble_messages::Pid {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(ref v)) => v,
            _ => super::rabble_messages::Pid::default_instance(),
        }
    }

    // optional .Primary primary = 9;

    pub fn clear_primary(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_primary(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: Primary) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut Primary {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(Primary::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_primary(&mut self) -> Primary {
        if self.has_primary() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(v)) => v,
                _ => panic!(),
            }
        } else {
            Primary::new()
        }
    }

    pub fn get_primary(&self) -> &Primary {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(ref v)) => v,
            _ => Primary::default_instance(),
        }
    }

    // optional .Metrics metrics = 10;

    pub fn clear_metrics(&mut self) {
        self.rpy = ::std::option::Option::None;
    }

    pub fn has_metrics(&self) -> bool {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metrics(&mut self, v: super::rabble_messages::Metrics) {
        self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metrics(&mut self) -> &mut super::rabble_messages::Metrics {
        if let ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(_)) = self.rpy {
        } else {
            self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(super::rabble_messages::Metrics::new()));
        }
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metrics(&mut self) -> super::rabble_messages::Metrics {
        if self.has_metrics() {
            match self.rpy.take() {
                ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(v)) => v,
                _ => panic!(),
            }
        } else {
            super::rabble_messages::Metrics::new()
        }
    }

    pub fn get_metrics(&self) -> &super::rabble_messages::Metrics {
        match self.rpy {
            ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(ref v)) => v,
            _ => super::rabble_messages::Metrics::default_instance(),
        }
    }
}

impl ::protobuf::Message for AdminRpy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::ok(try!(is.read_bool())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::timeout(try!(is.read_bool())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::error(try!(is.read_string())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::config(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::namespace_id(try!(is.read_string())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::namespaces(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_state(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::replica_not_found(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::primary(try!(is.read_message())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.rpy = ::std::option::Option::Some(AdminRpy_oneof_rpy::metrics(try!(is.read_message())));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.rpy {
            match v {
                &AdminRpy_oneof_rpy::ok(v) => {
                    my_size += 2;
                },
                &AdminRpy_oneof_rpy::timeout(v) => {
                    my_size += 2;
                },
                &AdminRpy_oneof_rpy::error(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &AdminRpy_oneof_rpy::config(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminRpy_oneof_rpy::namespace_id(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
                &AdminRpy_oneof_rpy::namespaces(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminRpy_oneof_rpy::replica_state(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminRpy_oneof_rpy::replica_not_found(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminRpy_oneof_rpy::primary(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AdminRpy_oneof_rpy::metrics(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.rpy {
            match v {
                &AdminRpy_oneof_rpy::ok(v) => {
                    try!(os.write_bool(1, v));
                },
                &AdminRpy_oneof_rpy::timeout(v) => {
                    try!(os.write_bool(2, v));
                },
                &AdminRpy_oneof_rpy::error(ref v) => {
                    try!(os.write_string(3, v));
                },
                &AdminRpy_oneof_rpy::config(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminRpy_oneof_rpy::namespace_id(ref v) => {
                    try!(os.write_string(5, v));
                },
                &AdminRpy_oneof_rpy::namespaces(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminRpy_oneof_rpy::replica_state(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminRpy_oneof_rpy::replica_not_found(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminRpy_oneof_rpy::primary(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &AdminRpy_oneof_rpy::metrics(ref v) => {
                    try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AdminRpy>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminRpy {
    fn new() -> AdminRpy {
        AdminRpy::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminRpy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "ok",
                    AdminRpy::has_ok,
                    AdminRpy::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "timeout",
                    AdminRpy::has_timeout,
                    AdminRpy::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    AdminRpy::has_error,
                    AdminRpy::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "config",
                    AdminRpy::has_config,
                    AdminRpy::get_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "namespace_id",
                    AdminRpy::has_namespace_id,
                    AdminRpy::get_namespace_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "namespaces",
                    AdminRpy::has_namespaces,
                    AdminRpy::get_namespaces,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "replica_state",
                    AdminRpy::has_replica_state,
                    AdminRpy::get_replica_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "replica_not_found",
                    AdminRpy::has_replica_not_found,
                    AdminRpy::get_replica_not_found,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "primary",
                    AdminRpy::has_primary,
                    AdminRpy::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "metrics",
                    AdminRpy::has_metrics,
                    AdminRpy::get_metrics,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminRpy>(
                    "AdminRpy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminRpy {
    fn clear(&mut self) {
        self.clear_ok();
        self.clear_timeout();
        self.clear_error();
        self.clear_config();
        self.clear_namespace_id();
        self.clear_namespaces();
        self.clear_replica_state();
        self.clear_replica_not_found();
        self.clear_primary();
        self.clear_metrics();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AdminRpy {
    fn eq(&self, other: &AdminRpy) -> bool {
        self.rpy == other.rpy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AdminRpy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Config {
    // message fields
    node_name: ::protobuf::SingularField<::std::string::String>,
    cluster_host: ::protobuf::SingularField<::std::string::String>,
    admin_host: ::protobuf::SingularField<::std::string::String>,
    api_host: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Config {}

impl Config {
    pub fn new() -> Config {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Config {
        static mut instance: ::protobuf::lazy::Lazy<Config> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Config,
        };
        unsafe {
            instance.get(|| {
                Config {
                    node_name: ::protobuf::SingularField::none(),
                    cluster_host: ::protobuf::SingularField::none(),
                    admin_host: ::protobuf::SingularField::none(),
                    api_host: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string node_name = 1;

    pub fn clear_node_name(&mut self) {
        self.node_name.clear();
    }

    pub fn has_node_name(&self) -> bool {
        self.node_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_name(&mut self, v: ::std::string::String) {
        self.node_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_name(&mut self) -> &mut ::std::string::String {
        if self.node_name.is_none() {
            self.node_name.set_default();
        };
        self.node_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_node_name(&mut self) -> ::std::string::String {
        self.node_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_node_name(&self) -> &str {
        match self.node_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string cluster_host = 2;

    pub fn clear_cluster_host(&mut self) {
        self.cluster_host.clear();
    }

    pub fn has_cluster_host(&self) -> bool {
        self.cluster_host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_host(&mut self, v: ::std::string::String) {
        self.cluster_host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_host(&mut self) -> &mut ::std::string::String {
        if self.cluster_host.is_none() {
            self.cluster_host.set_default();
        };
        self.cluster_host.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_host(&mut self) -> ::std::string::String {
        self.cluster_host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cluster_host(&self) -> &str {
        match self.cluster_host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string admin_host = 3;

    pub fn clear_admin_host(&mut self) {
        self.admin_host.clear();
    }

    pub fn has_admin_host(&self) -> bool {
        self.admin_host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin_host(&mut self, v: ::std::string::String) {
        self.admin_host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin_host(&mut self) -> &mut ::std::string::String {
        if self.admin_host.is_none() {
            self.admin_host.set_default();
        };
        self.admin_host.as_mut().unwrap()
    }

    // Take field
    pub fn take_admin_host(&mut self) -> ::std::string::String {
        self.admin_host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_admin_host(&self) -> &str {
        match self.admin_host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string api_host = 4;

    pub fn clear_api_host(&mut self) {
        self.api_host.clear();
    }

    pub fn has_api_host(&self) -> bool {
        self.api_host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_api_host(&mut self, v: ::std::string::String) {
        self.api_host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_host(&mut self) -> &mut ::std::string::String {
        if self.api_host.is_none() {
            self.api_host.set_default();
        };
        self.api_host.as_mut().unwrap()
    }

    // Take field
    pub fn take_api_host(&mut self) -> ::std::string::String {
        self.api_host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_api_host(&self) -> &str {
        match self.api_host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Config {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.node_name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cluster_host));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.admin_host));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.api_host));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.node_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.cluster_host {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.admin_host {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.api_host {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.cluster_host.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.admin_host.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.api_host.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Config>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Config {
    fn new() -> Config {
        Config::new()
    }

    fn descriptor_static(_: ::std::option::Option<Config>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "node_name",
                    Config::has_node_name,
                    Config::get_node_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cluster_host",
                    Config::has_cluster_host,
                    Config::get_cluster_host,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "admin_host",
                    Config::has_admin_host,
                    Config::get_admin_host,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "api_host",
                    Config::has_api_host,
                    Config::get_api_host,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Config>(
                    "Config",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Config {
    fn clear(&mut self) {
        self.clear_node_name();
        self.clear_cluster_host();
        self.clear_admin_host();
        self.clear_api_host();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Config {
    fn eq(&self, other: &Config) -> bool {
        self.node_name == other.node_name &&
        self.cluster_host == other.cluster_host &&
        self.admin_host == other.admin_host &&
        self.api_host == other.api_host &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Config {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Primary {
    // message fields
    primary: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Primary {}

impl Primary {
    pub fn new() -> Primary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Primary {
        static mut instance: ::protobuf::lazy::Lazy<Primary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Primary,
        };
        unsafe {
            instance.get(|| {
                Primary {
                    primary: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Pid primary = 1;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: super::rabble_messages::Pid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut super::rabble_messages::Pid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> super::rabble_messages::Pid {
        self.primary.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_primary(&self) -> &super::rabble_messages::Pid {
        self.primary.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }
}

impl ::protobuf::Message for Primary {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.primary {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Primary>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Primary {
    fn new() -> Primary {
        Primary::new()
    }

    fn descriptor_static(_: ::std::option::Option<Primary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "primary",
                    Primary::has_primary,
                    Primary::get_primary,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Primary>(
                    "Primary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Primary {
    fn clear(&mut self) {
        self.clear_primary();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Primary {
    fn eq(&self, other: &Primary) -> bool {
        self.primary == other.primary &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Primary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VrCtxSummary {
    // message fields
    state: ::protobuf::SingularField<::std::string::String>,
    pid: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    primary: ::protobuf::SingularPtrField<super::rabble_messages::Pid>,
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    op: ::std::option::Option<u64>,
    commit_num: ::std::option::Option<u64>,
    last_received_time: ::protobuf::SingularField<::std::string::String>,
    last_normal_view: ::protobuf::SingularField<::std::string::String>,
    old_config: ::protobuf::SingularPtrField<VersionedReplicas>,
    new_config: ::protobuf::SingularPtrField<VersionedReplicas>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VrCtxSummary {}

impl VrCtxSummary {
    pub fn new() -> VrCtxSummary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VrCtxSummary {
        static mut instance: ::protobuf::lazy::Lazy<VrCtxSummary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VrCtxSummary,
        };
        unsafe {
            instance.get(|| {
                VrCtxSummary {
                    state: ::protobuf::SingularField::none(),
                    pid: ::protobuf::SingularPtrField::none(),
                    primary: ::protobuf::SingularPtrField::none(),
                    epoch: ::std::option::Option::None,
                    view: ::std::option::Option::None,
                    op: ::std::option::Option::None,
                    commit_num: ::std::option::Option::None,
                    last_received_time: ::protobuf::SingularField::none(),
                    last_normal_view: ::protobuf::SingularField::none(),
                    old_config: ::protobuf::SingularPtrField::none(),
                    new_config: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string state = 1;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state(&mut self) -> &mut ::std::string::String {
        if self.state.is_none() {
            self.state.set_default();
        };
        self.state.as_mut().unwrap()
    }

    // Take field
    pub fn take_state(&mut self) -> ::std::string::String {
        self.state.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_state(&self) -> &str {
        match self.state.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Pid pid = 2;

    pub fn clear_pid(&mut self) {
        self.pid.clear();
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: super::rabble_messages::Pid) {
        self.pid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pid(&mut self) -> &mut super::rabble_messages::Pid {
        if self.pid.is_none() {
            self.pid.set_default();
        };
        self.pid.as_mut().unwrap()
    }

    // Take field
    pub fn take_pid(&mut self) -> super::rabble_messages::Pid {
        self.pid.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_pid(&self) -> &super::rabble_messages::Pid {
        self.pid.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional .Pid primary = 3;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: super::rabble_messages::Pid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut super::rabble_messages::Pid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> super::rabble_messages::Pid {
        self.primary.take().unwrap_or_else(|| super::rabble_messages::Pid::new())
    }

    pub fn get_primary(&self) -> &super::rabble_messages::Pid {
        self.primary.as_ref().unwrap_or_else(|| super::rabble_messages::Pid::default_instance())
    }

    // optional uint64 epoch = 4;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    // optional uint64 view = 5;

    pub fn clear_view(&mut self) {
        self.view = ::std::option::Option::None;
    }

    pub fn has_view(&self) -> bool {
        self.view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view(&mut self, v: u64) {
        self.view = ::std::option::Option::Some(v);
    }

    pub fn get_view(&self) -> u64 {
        self.view.unwrap_or(0)
    }

    // optional uint64 op = 6;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: u64) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> u64 {
        self.op.unwrap_or(0)
    }

    // optional uint64 commit_num = 7;

    pub fn clear_commit_num(&mut self) {
        self.commit_num = ::std::option::Option::None;
    }

    pub fn has_commit_num(&self) -> bool {
        self.commit_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_num(&mut self, v: u64) {
        self.commit_num = ::std::option::Option::Some(v);
    }

    pub fn get_commit_num(&self) -> u64 {
        self.commit_num.unwrap_or(0)
    }

    // optional string last_received_time = 8;

    pub fn clear_last_received_time(&mut self) {
        self.last_received_time.clear();
    }

    pub fn has_last_received_time(&self) -> bool {
        self.last_received_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_received_time(&mut self, v: ::std::string::String) {
        self.last_received_time = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_received_time(&mut self) -> &mut ::std::string::String {
        if self.last_received_time.is_none() {
            self.last_received_time.set_default();
        };
        self.last_received_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_received_time(&mut self) -> ::std::string::String {
        self.last_received_time.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_last_received_time(&self) -> &str {
        match self.last_received_time.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string last_normal_view = 9;

    pub fn clear_last_normal_view(&mut self) {
        self.last_normal_view.clear();
    }

    pub fn has_last_normal_view(&self) -> bool {
        self.last_normal_view.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_normal_view(&mut self, v: ::std::string::String) {
        self.last_normal_view = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_normal_view(&mut self) -> &mut ::std::string::String {
        if self.last_normal_view.is_none() {
            self.last_normal_view.set_default();
        };
        self.last_normal_view.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_normal_view(&mut self) -> ::std::string::String {
        self.last_normal_view.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_last_normal_view(&self) -> &str {
        match self.last_normal_view.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .VersionedReplicas old_config = 10;

    pub fn clear_old_config(&mut self) {
        self.old_config.clear();
    }

    pub fn has_old_config(&self) -> bool {
        self.old_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_config(&mut self, v: VersionedReplicas) {
        self.old_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_config(&mut self) -> &mut VersionedReplicas {
        if self.old_config.is_none() {
            self.old_config.set_default();
        };
        self.old_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_config(&mut self) -> VersionedReplicas {
        self.old_config.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_old_config(&self) -> &VersionedReplicas {
        self.old_config.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }

    // optional .VersionedReplicas new_config = 11;

    pub fn clear_new_config(&mut self) {
        self.new_config.clear();
    }

    pub fn has_new_config(&self) -> bool {
        self.new_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_config(&mut self, v: VersionedReplicas) {
        self.new_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_config(&mut self) -> &mut VersionedReplicas {
        if self.new_config.is_none() {
            self.new_config.set_default();
        };
        self.new_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_config(&mut self) -> VersionedReplicas {
        self.new_config.take().unwrap_or_else(|| VersionedReplicas::new())
    }

    pub fn get_new_config(&self) -> &VersionedReplicas {
        self.new_config.as_ref().unwrap_or_else(|| VersionedReplicas::default_instance())
    }
}

impl ::protobuf::Message for VrCtxSummary {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.state));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pid));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.view = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.op = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_num = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.last_received_time));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.last_normal_view));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_config));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new_config));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.state {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.pid {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.primary {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.epoch {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.view {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.commit_num {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.last_received_time {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in &self.last_normal_view {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        for value in &self.old_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.new_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.state.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.pid.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.epoch {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.view {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.op {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.commit_num {
            try!(os.write_uint64(7, v));
        };
        if let Some(v) = self.last_received_time.as_ref() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.last_normal_view.as_ref() {
            try!(os.write_string(9, &v));
        };
        if let Some(v) = self.old_config.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.new_config.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VrCtxSummary>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VrCtxSummary {
    fn new() -> VrCtxSummary {
        VrCtxSummary::new()
    }

    fn descriptor_static(_: ::std::option::Option<VrCtxSummary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "state",
                    VrCtxSummary::has_state,
                    VrCtxSummary::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pid",
                    VrCtxSummary::has_pid,
                    VrCtxSummary::get_pid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "primary",
                    VrCtxSummary::has_primary,
                    VrCtxSummary::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "epoch",
                    VrCtxSummary::has_epoch,
                    VrCtxSummary::get_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "view",
                    VrCtxSummary::has_view,
                    VrCtxSummary::get_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "op",
                    VrCtxSummary::has_op,
                    VrCtxSummary::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_num",
                    VrCtxSummary::has_commit_num,
                    VrCtxSummary::get_commit_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "last_received_time",
                    VrCtxSummary::has_last_received_time,
                    VrCtxSummary::get_last_received_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "last_normal_view",
                    VrCtxSummary::has_last_normal_view,
                    VrCtxSummary::get_last_normal_view,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "old_config",
                    VrCtxSummary::has_old_config,
                    VrCtxSummary::get_old_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new_config",
                    VrCtxSummary::has_new_config,
                    VrCtxSummary::get_new_config,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VrCtxSummary>(
                    "VrCtxSummary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VrCtxSummary {
    fn clear(&mut self) {
        self.clear_state();
        self.clear_pid();
        self.clear_primary();
        self.clear_epoch();
        self.clear_view();
        self.clear_op();
        self.clear_commit_num();
        self.clear_last_received_time();
        self.clear_last_normal_view();
        self.clear_old_config();
        self.clear_new_config();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VrCtxSummary {
    fn eq(&self, other: &VrCtxSummary) -> bool {
        self.state == other.state &&
        self.pid == other.pid &&
        self.primary == other.primary &&
        self.epoch == other.epoch &&
        self.view == other.view &&
        self.op == other.op &&
        self.commit_num == other.commit_num &&
        self.last_received_time == other.last_received_time &&
        self.last_normal_view == other.last_normal_view &&
        self.old_config == other.old_config &&
        self.new_config == other.new_config &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VrCtxSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NodeType {
    BlobType = 0,
    QueueType = 1,
    SetType = 2,
    DirectoryType = 3,
}

impl ::protobuf::ProtobufEnum for NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NodeType> {
        match value {
            0 => ::std::option::Option::Some(NodeType::BlobType),
            1 => ::std::option::Option::Some(NodeType::QueueType),
            2 => ::std::option::Option::Some(NodeType::SetType),
            3 => ::std::option::Option::Some(NodeType::DirectoryType),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NodeType] = &[
            NodeType::BlobType,
            NodeType::QueueType,
            NodeType::SetType,
            NodeType::DirectoryType,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<NodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NodeType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x70, 0x62, 0x5f, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15,
    0x72, 0x61, 0x62, 0x62, 0x6c, 0x65, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xe5, 0x01, 0x0a, 0x03, 0x4d, 0x73, 0x67, 0x12, 0x18, 0x0a,
    0x02, 0x76, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x72, 0x4d, 0x73,
    0x67, 0x48, 0x00, 0x52, 0x02, 0x76, 0x72, 0x12, 0x2d, 0x0a, 0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73,
    0x70, 0x61, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x4d, 0x73, 0x67, 0x48, 0x00, 0x52, 0x09, 0x6e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x12, 0x28, 0x0a, 0x09, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f,
    0x72, 0x65, 0x71, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x41, 0x64, 0x6d, 0x69,
    0x6e, 0x52, 0x65, 0x71, 0x48, 0x00, 0x52, 0x08, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x71,
    0x12, 0x28, 0x0a, 0x09, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f, 0x72, 0x70, 0x79, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x70, 0x79, 0x48, 0x00,
    0x52, 0x08, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x70, 0x79, 0x12, 0x22, 0x0a, 0x07, 0x61, 0x70,
    0x69, 0x5f, 0x72, 0x70, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x41, 0x70,
    0x69, 0x52, 0x70, 0x79, 0x48, 0x00, 0x52, 0x06, 0x61, 0x70, 0x69, 0x52, 0x70, 0x79, 0x12, 0x16,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x8f, 0x06,
    0x0a, 0x05, 0x56, 0x72, 0x4d, 0x73, 0x67, 0x12, 0x14, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x12, 0x37, 0x0a,
    0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3e, 0x0a, 0x0f, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x56, 0x72, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0f, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x31, 0x0a, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x48, 0x00, 0x52, 0x0b, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x3e, 0x0a, 0x11, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x76, 0x69, 0x65, 0x77, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x56, 0x69, 0x65, 0x77,
    0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x48, 0x00, 0x52, 0x0f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56,
    0x69, 0x65, 0x77, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x35, 0x0a, 0x0e, 0x64, 0x6f, 0x5f,
    0x76, 0x69, 0x65, 0x77, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0d, 0x2e, 0x44, 0x6f, 0x56, 0x69, 0x65, 0x77, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x48, 0x00, 0x52, 0x0c, 0x64, 0x6f, 0x56, 0x69, 0x65, 0x77, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x12, 0x2b, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x69, 0x65, 0x77, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x56, 0x69, 0x65, 0x77,
    0x48, 0x00, 0x52, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x69, 0x65, 0x77, 0x12, 0x24, 0x0a,
    0x07, 0x70, 0x72, 0x65, 0x70, 0x61, 0x72, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08,
    0x2e, 0x50, 0x72, 0x65, 0x70, 0x61, 0x72, 0x65, 0x48, 0x00, 0x52, 0x07, 0x70, 0x72, 0x65, 0x70,
    0x61, 0x72, 0x65, 0x12, 0x2b, 0x0a, 0x0a, 0x70, 0x72, 0x65, 0x70, 0x61, 0x72, 0x65, 0x5f, 0x6f,
    0x6b, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x50, 0x72, 0x65, 0x70, 0x61, 0x72,
    0x65, 0x4f, 0x6b, 0x48, 0x00, 0x52, 0x09, 0x70, 0x72, 0x65, 0x70, 0x61, 0x72, 0x65, 0x4f, 0x6b,
    0x12, 0x21, 0x0a, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x07, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x48, 0x00, 0x52, 0x06, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x12, 0x28, 0x0a, 0x09, 0x67, 0x65, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x48, 0x00, 0x52, 0x08, 0x67, 0x65, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x28, 0x0a,
    0x09, 0x6e, 0x65, 0x77, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x09, 0x2e, 0x4e, 0x65, 0x77, 0x53, 0x74, 0x61, 0x74, 0x65, 0x48, 0x00, 0x52, 0x08, 0x6e,
    0x65, 0x77, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x27, 0x0a, 0x08, 0x72, 0x65, 0x63, 0x6f, 0x76,
    0x65, 0x72, 0x79, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x52, 0x65, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x79, 0x48, 0x00, 0x52, 0x08, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x12, 0x40, 0x0a, 0x11, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x52, 0x65,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00,
    0x52, 0x10, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x2e, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x65, 0x70, 0x6f, 0x63,
    0x68, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x45,
    0x70, 0x6f, 0x63, 0x68, 0x48, 0x00, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x45, 0x70, 0x6f,
    0x63, 0x68, 0x12, 0x34, 0x0a, 0x0d, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x5f, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x65, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x45, 0x70, 0x6f, 0x63,
    0x68, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x48, 0x00, 0x52, 0x0c, 0x65, 0x70, 0x6f, 0x63,
    0x68, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22,
    0x68, 0x0a, 0x0d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x19, 0x0a, 0x02, 0x6f, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x56,
    0x72, 0x41, 0x70, 0x69, 0x52, 0x65, 0x71, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x1b, 0x0a, 0x09, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x4e, 0x75, 0x6d, 0x22, 0x71, 0x0a, 0x11, 0x56, 0x72, 0x52,
    0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x24,
    0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65,
    0x71, 0x4e, 0x75, 0x6d, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x20, 0x0a, 0x08, 0x72, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50,
    0x69, 0x64, 0x52, 0x08, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x22, 0x79, 0x0a, 0x0b,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x65,
    0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63,
    0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x1f, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x56, 0x72, 0x41, 0x70, 0x69, 0x52, 0x73, 0x70,
    0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x65, 0x0a, 0x0f, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x56, 0x69, 0x65, 0x77, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70,
    0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68,
    0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04,
    0x76, 0x69, 0x65, 0x77, 0x12, 0x0e, 0x0a, 0x02, 0x6f, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x02, 0x6f, 0x70, 0x12, 0x18, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x22, 0xc5,
    0x01, 0x0a, 0x0c, 0x44, 0x6f, 0x56, 0x69, 0x65, 0x77, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12,
    0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05,
    0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x0e, 0x0a, 0x02, 0x6f, 0x70, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x18, 0x0a, 0x04, 0x66, 0x72, 0x6f,
    0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x04, 0x66,
    0x72, 0x6f, 0x6d, 0x12, 0x28, 0x0a, 0x10, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x6e, 0x6f, 0x72, 0x6d,
    0x61, 0x6c, 0x5f, 0x76, 0x69, 0x65, 0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0e, 0x6c,
    0x61, 0x73, 0x74, 0x4e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x56, 0x69, 0x65, 0x77, 0x12, 0x18, 0x0a,
    0x03, 0x6c, 0x6f, 0x67, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x72, 0x4d,
    0x73, 0x67, 0x52, 0x03, 0x6c, 0x6f, 0x67, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x22, 0x7e, 0x0a, 0x09, 0x53, 0x74, 0x61, 0x72, 0x74, 0x56,
    0x69, 0x65, 0x77, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65,
    0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x0e, 0x0a,
    0x02, 0x6f, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x18, 0x0a,
    0x03, 0x6c, 0x6f, 0x67, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x72, 0x4d,
    0x73, 0x67, 0x52, 0x03, 0x6c, 0x6f, 0x67, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x22, 0x7c, 0x0a, 0x07, 0x50, 0x72, 0x65, 0x70, 0x61, 0x72,
    0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x0e, 0x0a, 0x02, 0x6f,
    0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x1d, 0x0a, 0x0a, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x18, 0x0a, 0x03, 0x6d, 0x73,
    0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x72, 0x4d, 0x73, 0x67, 0x52,
    0x03, 0x6d, 0x73, 0x67, 0x22, 0x5f, 0x0a, 0x09, 0x50, 0x72, 0x65, 0x70, 0x61, 0x72, 0x65, 0x4f,
    0x6b, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x0e, 0x0a, 0x02, 0x6f,
    0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x18, 0x0a, 0x04, 0x66,
    0x72, 0x6f, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52,
    0x04, 0x66, 0x72, 0x6f, 0x6d, 0x22, 0x51, 0x0a, 0x06, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x12,
    0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05,
    0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x22, 0x5e, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69,
    0x65, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x0e,
    0x0a, 0x02, 0x6f, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x18,
    0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50,
    0x69, 0x64, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x22, 0xa6, 0x01, 0x0a, 0x08, 0x4e, 0x65, 0x77,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76,
    0x69, 0x65, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12,
    0x0e, 0x0a, 0x02, 0x6f, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12,
    0x1e, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12,
    0x1d, 0x0a, 0x0a, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x21,
    0x0a, 0x08, 0x6c, 0x6f, 0x67, 0x5f, 0x74, 0x61, 0x69, 0x6c, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x06, 0x2e, 0x56, 0x72, 0x4d, 0x73, 0x67, 0x52, 0x07, 0x6c, 0x6f, 0x67, 0x54, 0x61, 0x69,
    0x6c, 0x22, 0x3a, 0x0a, 0x08, 0x52, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x12, 0x18, 0x0a,
    0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69,
    0x64, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x14, 0x0a, 0x05, 0x6e, 0x6f, 0x6e, 0x63, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x22, 0xb5, 0x01,
    0x0a, 0x10, 0x52, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x14, 0x0a, 0x05,
    0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x6e, 0x6f, 0x6e,
    0x63, 0x65, 0x12, 0x18, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x0e, 0x0a, 0x02,
    0x6f, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x1d, 0x0a, 0x0a,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x18, 0x0a, 0x03, 0x6c,
    0x6f, 0x67, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x56, 0x72, 0x4d, 0x73, 0x67,
    0x52, 0x03, 0x6c, 0x6f, 0x67, 0x22, 0x98, 0x01, 0x0a, 0x0a, 0x53, 0x74, 0x61, 0x72, 0x74, 0x45,
    0x70, 0x6f, 0x63, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x0e, 0x0a, 0x02, 0x6f, 0x70,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x31, 0x0a, 0x0a, 0x6f, 0x6c,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x73, 0x52, 0x09, 0x6f, 0x6c, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x31, 0x0a,
    0x0a, 0x6e, 0x65, 0x77, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70,
    0x6c, 0x69, 0x63, 0x61, 0x73, 0x52, 0x09, 0x6e, 0x65, 0x77, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x22, 0x3e, 0x0a, 0x0c, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64,
    0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x18, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d,
    0x22, 0x5c, 0x0a, 0x08, 0x56, 0x72, 0x41, 0x70, 0x69, 0x52, 0x65, 0x71, 0x12, 0x22, 0x0a, 0x07,
    0x74, 0x72, 0x65, 0x65, 0x5f, 0x6f, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e,
    0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x48, 0x00, 0x52, 0x06, 0x74, 0x72, 0x65, 0x65, 0x4f, 0x70,
    0x12, 0x25, 0x0a, 0x08, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x63, 0x61, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x08, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x43, 0x61, 0x73, 0x48, 0x00, 0x52, 0x07,
    0x74, 0x72, 0x65, 0x65, 0x43, 0x61, 0x73, 0x42, 0x05, 0x0a, 0x03, 0x72, 0x65, 0x71, 0x22, 0x44,
    0x0a, 0x07, 0x54, 0x72, 0x65, 0x65, 0x43, 0x61, 0x73, 0x12, 0x1e, 0x0a, 0x06, 0x67, 0x75, 0x61,
    0x72, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x47, 0x75, 0x61, 0x72,
    0x64, 0x52, 0x06, 0x67, 0x75, 0x61, 0x72, 0x64, 0x73, 0x12, 0x19, 0x0a, 0x03, 0x6f, 0x70, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52,
    0x03, 0x6f, 0x70, 0x73, 0x22, 0x35, 0x0a, 0x05, 0x47, 0x75, 0x61, 0x72, 0x64, 0x12, 0x12, 0x0a,
    0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74,
    0x68, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xa2, 0x08, 0x0a, 0x06,
    0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x12, 0x1c, 0x0a, 0x08, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68,
    0x6f, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x73, 0x6e, 0x61, 0x70,
    0x73, 0x68, 0x6f, 0x74, 0x12, 0x2e, 0x0a, 0x0b, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x6e,
    0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x4e, 0x6f, 0x64, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x6e,
    0x6f, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0a, 0x64, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x1d, 0x0a, 0x09, 0x6c, 0x69, 0x73, 0x74, 0x5f,
    0x6b, 0x65, 0x79, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x6c, 0x69,
    0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x12, 0x25, 0x0a, 0x08, 0x62, 0x6c, 0x6f, 0x62, 0x5f, 0x70,
    0x75, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x42, 0x6c, 0x6f, 0x62, 0x50,
    0x75, 0x74, 0x48, 0x00, 0x52, 0x07, 0x62, 0x6c, 0x6f, 0x62, 0x50, 0x75, 0x74, 0x12, 0x1b, 0x0a,
    0x08, 0x62, 0x6c, 0x6f, 0x62, 0x5f, 0x67, 0x65, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x48,
    0x00, 0x52, 0x07, 0x62, 0x6c, 0x6f, 0x62, 0x47, 0x65, 0x74, 0x12, 0x1d, 0x0a, 0x09, 0x62, 0x6c,
    0x6f, 0x62, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52,
    0x08, 0x62, 0x6c, 0x6f, 0x62, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x2b, 0x0a, 0x0a, 0x71, 0x75, 0x65,
    0x75, 0x65, 0x5f, 0x70, 0x75, 0x73, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e,
    0x51, 0x75, 0x65, 0x75, 0x65, 0x50, 0x75, 0x73, 0x68, 0x48, 0x00, 0x52, 0x09, 0x71, 0x75, 0x65,
    0x75, 0x65, 0x50, 0x75, 0x73, 0x68, 0x12, 0x1d, 0x0a, 0x09, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f,
    0x70, 0x6f, 0x70, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x71, 0x75, 0x65,
    0x75, 0x65, 0x50, 0x6f, 0x70, 0x12, 0x21, 0x0a, 0x0b, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x66,
    0x72, 0x6f, 0x6e, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0a, 0x71, 0x75,
    0x65, 0x75, 0x65, 0x46, 0x72, 0x6f, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x0a, 0x71, 0x75, 0x65, 0x75,
    0x65, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x09,
    0x71, 0x75, 0x65, 0x75, 0x65, 0x42, 0x61, 0x63, 0x6b, 0x12, 0x1d, 0x0a, 0x09, 0x71, 0x75, 0x65,
    0x75, 0x65, 0x5f, 0x6c, 0x65, 0x6e, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08,
    0x71, 0x75, 0x65, 0x75, 0x65, 0x4c, 0x65, 0x6e, 0x12, 0x2b, 0x0a, 0x0a, 0x73, 0x65, 0x74, 0x5f,
    0x69, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x53,
    0x65, 0x74, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x48, 0x00, 0x52, 0x09, 0x73, 0x65, 0x74, 0x49,
    0x6e, 0x73, 0x65, 0x72, 0x74, 0x12, 0x2b, 0x0a, 0x0a, 0x73, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x6d,
    0x6f, 0x76, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x53, 0x65, 0x74, 0x52,
    0x65, 0x6d, 0x6f, 0x76, 0x65, 0x48, 0x00, 0x52, 0x09, 0x73, 0x65, 0x74, 0x52, 0x65, 0x6d, 0x6f,
    0x76, 0x65, 0x12, 0x31, 0x0a, 0x0c, 0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x53, 0x65, 0x74, 0x43, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x48, 0x00, 0x52, 0x0b, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x28, 0x0a, 0x09, 0x73, 0x65, 0x74, 0x5f, 0x75, 0x6e, 0x69,
    0x6f, 0x6e, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x53, 0x65, 0x74, 0x55, 0x6e,
    0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x08, 0x73, 0x65, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12,
    0x3d, 0x0a, 0x10, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x53, 0x65, 0x74, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0f, 0x73,
    0x65, 0x74, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x37,
    0x0a, 0x0e, 0x73, 0x65, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65,
    0x18, 0x12, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x53, 0x65, 0x74, 0x44, 0x69, 0x66, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x65, 0x74, 0x44, 0x69, 0x66,
    0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x53, 0x0a, 0x18, 0x73, 0x65, 0x74, 0x5f, 0x73,
    0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65,
    0x6e, 0x63, 0x65, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x53, 0x65, 0x74, 0x53,
    0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x63, 0x65, 0x48, 0x00, 0x52, 0x16, 0x73, 0x65, 0x74, 0x53, 0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x38, 0x0a, 0x0f,
    0x73, 0x65, 0x74, 0x5f, 0x73, 0x75, 0x62, 0x73, 0x65, 0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18,
    0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65,
    0x74, 0x50, 0x61, 0x74, 0x68, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73,
    0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x12, 0x35, 0x0a, 0x0e, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x75,
    0x62, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x65, 0x74, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x48, 0x00, 0x52,
    0x0c, 0x73, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x12, 0x3e, 0x0a,
    0x11, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x5f, 0x70, 0x61,
    0x74, 0x68, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75,
    0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x48, 0x00, 0x52, 0x0f, 0x73, 0x65,
    0x74, 0x53, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x12, 0x3b, 0x0a,
    0x10, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x65,
    0x74, 0x18, 0x17, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70,
    0x65, 0x72, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x48, 0x00, 0x52, 0x0e, 0x73, 0x65, 0x74, 0x53,
    0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x42, 0x04, 0x0a, 0x02, 0x6f, 0x70,
    0x22, 0x48, 0x0a, 0x0a, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x12,
    0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61,
    0x74, 0x68, 0x12, 0x26, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0x2f, 0x0a, 0x07, 0x42, 0x6c,
    0x6f, 0x62, 0x50, 0x75, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x31, 0x0a, 0x09, 0x51,
    0x75, 0x65, 0x75, 0x65, 0x50, 0x75, 0x73, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03,
    0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x31,
    0x0a, 0x09, 0x53, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
    0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61,
    0x6c, 0x22, 0x31, 0x0a, 0x09, 0x53, 0x65, 0x74, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x12, 0x12,
    0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61,
    0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x03, 0x76, 0x61, 0x6c, 0x22, 0x33, 0x0a, 0x0b, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x1d, 0x0a, 0x03, 0x53, 0x65, 0x74,
    0x12, 0x16, 0x0a, 0x06, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c,
    0x52, 0x06, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x22, 0x3a, 0x0a, 0x08, 0x53, 0x65, 0x74, 0x55,
    0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x73, 0x12, 0x18, 0x0a, 0x04, 0x73, 0x65,
    0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x04,
    0x73, 0x65, 0x74, 0x73, 0x22, 0x3d, 0x0a, 0x0f, 0x53, 0x65, 0x74, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x12, 0x14, 0x0a,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x32, 0x22, 0x3b, 0x0a, 0x0d, 0x53, 0x65, 0x74, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72,
    0x65, 0x6e, 0x63, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32,
    0x22, 0x44, 0x0a, 0x16, 0x53, 0x65, 0x74, 0x53, 0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63,
    0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x31, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31,
    0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x22, 0x3b, 0x0a, 0x0d, 0x53, 0x65, 0x74, 0x53, 0x75, 0x62,
    0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x12, 0x14, 0x0a,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x32, 0x22, 0x3a, 0x0a, 0x0c, 0x53, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65, 0x74,
    0x53, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x16, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x03, 0x73, 0x65, 0x74, 0x22,
    0x3d, 0x0a, 0x0f, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x50, 0x61,
    0x74, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68,
    0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x22, 0x3c,
    0x0a, 0x0e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74,
    0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x12, 0x16, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x03, 0x73, 0x65, 0x74, 0x22, 0xcf, 0x01, 0x0a,
    0x08, 0x56, 0x72, 0x41, 0x70, 0x69, 0x52, 0x73, 0x70, 0x12, 0x10, 0x0a, 0x02, 0x6f, 0x6b, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x02, 0x6f, 0x6b, 0x12, 0x35, 0x0a, 0x0e, 0x74,
    0x72, 0x65, 0x65, 0x5f, 0x6f, 0x70, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x48, 0x00, 0x52, 0x0c, 0x74, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x38, 0x0a, 0x0f, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x63, 0x61, 0x73, 0x5f, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x54, 0x72,
    0x65, 0x65, 0x43, 0x61, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x74,
    0x72, 0x65, 0x65, 0x43, 0x61, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x14, 0x0a, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x70, 0x61,
    0x74, 0x68, 0x12, 0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0b, 0x2e, 0x56, 0x72, 0x41, 0x70, 0x69, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00,
    0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x42, 0x05, 0x0a, 0x03, 0x72, 0x73, 0x70, 0x22, 0xa5,
    0x05, 0x0a, 0x0a, 0x56, 0x72, 0x41, 0x70, 0x69, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x1d, 0x0a,
    0x09, 0x6e, 0x6f, 0x74, 0x5f, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x00, 0x52, 0x08, 0x6e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x27, 0x0a, 0x0e,
    0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x5f, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0d, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x45,
    0x78, 0x69, 0x73, 0x74, 0x73, 0x12, 0x26, 0x0a, 0x0e, 0x64, 0x6f, 0x65, 0x73, 0x5f, 0x6e, 0x6f,
    0x74, 0x5f, 0x65, 0x78, 0x69, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52,
    0x0c, 0x64, 0x6f, 0x65, 0x73, 0x4e, 0x6f, 0x74, 0x45, 0x78, 0x69, 0x73, 0x74, 0x12, 0x2b, 0x0a,
    0x0a, 0x77, 0x72, 0x6f, 0x6e, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0a, 0x2e, 0x57, 0x72, 0x6f, 0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x48, 0x00, 0x52,
    0x09, 0x77, 0x72, 0x6f, 0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x3c, 0x0a, 0x1a, 0x70, 0x61,
    0x74, 0x68, 0x5f, 0x6d, 0x75, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x5f, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x52, 0x16, 0x70, 0x61, 0x74, 0x68, 0x4d, 0x75, 0x73, 0x74, 0x45, 0x6e, 0x64, 0x49, 0x6e, 0x44,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x12, 0x33, 0x0a, 0x15, 0x70, 0x61, 0x74, 0x68,
    0x5f, 0x6d, 0x75, 0x73, 0x74, 0x5f, 0x62, 0x65, 0x5f, 0x61, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74,
    0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x12, 0x70, 0x61, 0x74, 0x68, 0x4d,
    0x75, 0x73, 0x74, 0x42, 0x65, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x12, 0x2b, 0x0a,
    0x0a, 0x63, 0x61, 0x73, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0a, 0x2e, 0x43, 0x61, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x48, 0x00, 0x52,
    0x09, 0x63, 0x61, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x12, 0x1f, 0x0a, 0x0a, 0x62, 0x61,
    0x64, 0x5f, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x52, 0x09, 0x62, 0x61, 0x64, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x10, 0x0a, 0x02, 0x69,
    0x6f, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x02, 0x69, 0x6f, 0x12, 0x27, 0x0a,
    0x0e, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0d, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e,
    0x67, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x21, 0x0a, 0x0b, 0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x5f, 0x63, 0x61, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0a, 0x69,
    0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x43, 0x61, 0x73, 0x12, 0x12, 0x0a, 0x03, 0x6d, 0x73, 0x67,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x12, 0x2e, 0x0a,
    0x12, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72,
    0x6f, 0x6f, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x10, 0x63, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x6f, 0x6f, 0x74, 0x12, 0x21, 0x0a,
    0x0b, 0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x6d, 0x73, 0x67, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x0a, 0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x4d, 0x73, 0x67,
    0x12, 0x1a, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28,
    0x08, 0x48, 0x00, 0x52, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12, 0x30, 0x0a, 0x13,
    0x6e, 0x6f, 0x74, 0x5f, 0x65, 0x6e, 0x6f, 0x75, 0x67, 0x68, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x11, 0x6e, 0x6f, 0x74,
    0x45, 0x6e, 0x6f, 0x75, 0x67, 0x68, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x12, 0x1d,
    0x0a, 0x09, 0x62, 0x61, 0x64, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x11, 0x20, 0x01, 0x28,
    0x08, 0x48, 0x00, 0x52, 0x08, 0x62, 0x61, 0x64, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x42, 0x07, 0x0a,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x47, 0x0a, 0x09, 0x57, 0x72, 0x6f, 0x6e, 0x67, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x26, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x4e, 0x6f, 0x64,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22,
    0x53, 0x0a, 0x09, 0x43, 0x61, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68,
    0x12, 0x1a, 0x0a, 0x08, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x08, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x12, 0x16, 0x0a, 0x06,
    0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x61, 0x63,
    0x74, 0x75, 0x61, 0x6c, 0x22, 0xd3, 0x01, 0x0a, 0x0c, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x10, 0x0a, 0x02, 0x6f, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x02, 0x6f,
    0x6b, 0x12, 0x16, 0x0a, 0x05, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x48, 0x00, 0x52, 0x05, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x14, 0x0a, 0x04, 0x62, 0x6f, 0x6f,
    0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x12,
    0x14, 0x0a, 0x04, 0x62, 0x6c, 0x6f, 0x62, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52,
    0x04, 0x62, 0x6c, 0x6f, 0x62, 0x12, 0x12, 0x0a, 0x03, 0x69, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x04, 0x48, 0x00, 0x52, 0x03, 0x69, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x03, 0x73, 0x65, 0x74,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74, 0x48, 0x00, 0x52, 0x03,
    0x73, 0x65, 0x74, 0x12, 0x1b, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x05, 0x2e, 0x4b, 0x65, 0x79, 0x73, 0x48, 0x00, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73,
    0x42, 0x08, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x36, 0x0a, 0x0d, 0x54, 0x72,
    0x65, 0x65, 0x43, 0x61, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x25, 0x0a, 0x06, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x54, 0x72,
    0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x22, 0x64, 0x0a, 0x04, 0x4b, 0x65, 0x79, 0x73, 0x12, 0x23, 0x0a, 0x04, 0x6b, 0x65,
    0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x4b, 0x65, 0x79, 0x73, 0x2e,
    0x4b, 0x65, 0x79, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x1a,
    0x37, 0x0a, 0x09, 0x4b, 0x65, 0x79, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22, 0xd0, 0x02, 0x0a, 0x0c, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x4d, 0x73, 0x67, 0x12, 0x2d, 0x0a, 0x0a, 0x6e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e,
    0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x48, 0x00, 0x52, 0x0a, 0x6e, 0x61,
    0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x12, 0x3a, 0x0a, 0x0f, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x12, 0x1b, 0x0a, 0x08, 0x61, 0x70, 0x69, 0x5f, 0x61, 0x64, 0x64, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x07, 0x61, 0x70, 0x69, 0x41, 0x64, 0x64,
    0x72, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0f, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1a, 0x0a, 0x04, 0x73, 0x74, 0x6f, 0x70,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x48, 0x00, 0x52, 0x04,
    0x73, 0x74, 0x6f, 0x70, 0x12, 0x27, 0x0a, 0x0b, 0x6e, 0x65, 0x77, 0x5f, 0x70, 0x72, 0x69, 0x6d,
    0x61, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x48,
    0x00, 0x52, 0x0a, 0x6e, 0x65, 0x77, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x25, 0x0a,
    0x0d, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x5f, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0c, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x50, 0x72, 0x69,
    0x6d, 0x61, 0x72, 0x79, 0x42, 0x05, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x22, 0xfa, 0x01, 0x0a, 0x0a,
    0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x12, 0x26, 0x0a, 0x03, 0x6d, 0x61,
    0x70, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70,
    0x61, 0x63, 0x65, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x03, 0x6d,
    0x61, 0x70, 0x12, 0x38, 0x0a, 0x09, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x69, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63,
    0x65, 0x73, 0x2e, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x69, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x52, 0x09, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x69, 0x65, 0x73, 0x1a, 0x46, 0x0a, 0x08,
    0x4d, 0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x24, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x52, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x3a, 0x02, 0x38, 0x01, 0x1a, 0x42, 0x0a, 0x0e, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x69, 0x65,
    0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x1a, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22, 0x5b, 0x0a, 0x0d, 0x52, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x24, 0x0a, 0x03, 0x6f, 0x6c, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x52, 0x03, 0x6f, 0x6c, 0x64, 0x12,
    0x24, 0x0a, 0x03, 0x6e, 0x65, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73,
    0x52, 0x03, 0x6e, 0x65, 0x77, 0x22, 0x50, 0x0a, 0x0e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6e, 0x61, 0x6d, 0x65,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x49, 0x64, 0x22, 0xa3, 0x01, 0x0a, 0x18, 0x4e, 0x61, 0x6d, 0x65,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6e, 0x61, 0x6d, 0x65,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x49, 0x64, 0x12, 0x31, 0x0a, 0x0a, 0x6f, 0x6c, 0x64, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x52,
    0x09, 0x6f, 0x6c, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x31, 0x0a, 0x0a, 0x6e, 0x65,
    0x77, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x73, 0x52, 0x09, 0x6e, 0x65, 0x77, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0x5b, 0x0a,
    0x11, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x0e, 0x0a, 0x02, 0x6f, 0x70, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x20, 0x0a, 0x08, 0x72, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64,
    0x52, 0x08, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x22, 0xc7, 0x01, 0x0a, 0x06, 0x41,
    0x70, 0x69, 0x52, 0x70, 0x79, 0x12, 0x46, 0x0a, 0x13, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x12, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x27, 0x0a,
    0x08, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x09, 0x2e, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x48, 0x00, 0x52, 0x08, 0x72, 0x65,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x12, 0x16, 0x0a, 0x05, 0x72, 0x65, 0x74, 0x72, 0x79, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x05, 0x72, 0x65, 0x74, 0x72, 0x79, 0x12, 0x2d,
    0x0a, 0x11, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70,
    0x61, 0x63, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x10, 0x75, 0x6e, 0x6b,
    0x6e, 0x6f, 0x77, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x42, 0x05, 0x0a,
    0x03, 0x72, 0x70, 0x79, 0x22, 0x5f, 0x0a, 0x12, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x07, 0x70, 0x72,
    0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69,
    0x64, 0x52, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x29, 0x0a, 0x10, 0x6e, 0x65,
    0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x6e, 0x65, 0x77, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x45, 0x0a, 0x08, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x12, 0x1e, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72,
    0x79, 0x12, 0x19, 0x0a, 0x08, 0x61, 0x70, 0x69, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x70, 0x69, 0x41, 0x64, 0x64, 0x72, 0x22, 0xde, 0x02, 0x0a,
    0x08, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x12, 0x1f, 0x0a, 0x0a, 0x67, 0x65, 0x74,
    0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52,
    0x09, 0x67, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x1d, 0x0a, 0x04, 0x6a, 0x6f,
    0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49,
    0x64, 0x48, 0x00, 0x52, 0x04, 0x6a, 0x6f, 0x69, 0x6e, 0x12, 0x32, 0x0a, 0x10, 0x63, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50, 0x69, 0x64, 0x73, 0x48, 0x00, 0x52, 0x0f, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x12, 0x27, 0x0a,
    0x0e, 0x67, 0x65, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0d, 0x67, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x12, 0x32, 0x0a, 0x11, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x48, 0x00, 0x52, 0x0f, 0x67, 0x65, 0x74, 0x52, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x67, 0x65,
    0x74, 0x5f, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x48,
    0x00, 0x52, 0x0a, 0x67, 0x65, 0x74, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x2e, 0x0a,
    0x12, 0x67, 0x65, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x10, 0x67, 0x65, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x27, 0x0a,
    0x0b, 0x67, 0x65, 0x74, 0x5f, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x48, 0x00, 0x52, 0x0a, 0x67, 0x65, 0x74, 0x4d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x42, 0x05, 0x0a, 0x03, 0x72, 0x65, 0x71, 0x22, 0x84, 0x03,
    0x0a, 0x08, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x70, 0x79, 0x12, 0x10, 0x0a, 0x02, 0x6f, 0x6b,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x02, 0x6f, 0x6b, 0x12, 0x1a, 0x0a, 0x07,
    0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52,
    0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12, 0x16, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x12, 0x21, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x07, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x48, 0x00, 0x52, 0x06, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x12, 0x23, 0x0a, 0x0c, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x6e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x49, 0x64, 0x12, 0x2d, 0x0a, 0x0a, 0x6e, 0x61, 0x6d, 0x65,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4e,
    0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x48, 0x00, 0x52, 0x0a, 0x6e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x12, 0x34, 0x0a, 0x0d, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x56, 0x72, 0x43, 0x74, 0x78, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x48, 0x00, 0x52,
    0x0c, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x32, 0x0a,
    0x11, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x5f, 0x6e, 0x6f, 0x74, 0x5f, 0x66, 0x6f, 0x75,
    0x6e, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x48, 0x00,
    0x52, 0x0f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e,
    0x64, 0x12, 0x24, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x08, 0x2e, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x48, 0x00, 0x52, 0x07,
    0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x24, 0x0a, 0x07, 0x6d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x4d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x73, 0x48, 0x00, 0x52, 0x07, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x42, 0x05, 0x0a,
    0x03, 0x72, 0x70, 0x79, 0x22, 0x82, 0x01, 0x0a, 0x06, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12,
    0x1b, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0b, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x48, 0x6f, 0x73, 0x74, 0x12,
    0x1d, 0x0a, 0x0a, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x09, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x48, 0x6f, 0x73, 0x74, 0x12, 0x19,
    0x0a, 0x08, 0x61, 0x70, 0x69, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x61, 0x70, 0x69, 0x48, 0x6f, 0x73, 0x74, 0x22, 0x29, 0x0a, 0x07, 0x50, 0x72, 0x69,
    0x6d, 0x61, 0x72, 0x79, 0x12, 0x1e, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x07, 0x70, 0x72, 0x69,
    0x6d, 0x61, 0x72, 0x79, 0x22, 0xf3, 0x02, 0x0a, 0x0c, 0x56, 0x72, 0x43, 0x74, 0x78, 0x53, 0x75,
    0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x16, 0x0a, 0x03, 0x70,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x03,
    0x70, 0x69, 0x64, 0x12, 0x1e, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x50, 0x69, 0x64, 0x52, 0x07, 0x70, 0x72, 0x69, 0x6d,
    0x61, 0x72, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65,
    0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x0e, 0x0a,
    0x02, 0x6f, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x6f, 0x70, 0x12, 0x1d, 0x0a,
    0x0a, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x2c, 0x0a, 0x12,
    0x6c, 0x61, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x6c, 0x61, 0x73, 0x74, 0x52, 0x65,
    0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6c, 0x61,
    0x73, 0x74, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x5f, 0x76, 0x69, 0x65, 0x77, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x6c, 0x61, 0x73, 0x74, 0x4e, 0x6f, 0x72, 0x6d, 0x61, 0x6c,
    0x56, 0x69, 0x65, 0x77, 0x12, 0x31, 0x0a, 0x0a, 0x6f, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x52, 0x09, 0x6f, 0x6c,
    0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x31, 0x0a, 0x0a, 0x6e, 0x65, 0x77, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x52,
    0x09, 0x6e, 0x65, 0x77, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2a, 0x47, 0x0a, 0x08, 0x4e, 0x6f,
    0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x08, 0x42, 0x6c, 0x6f, 0x62, 0x54, 0x79,
    0x70, 0x65, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09, 0x51, 0x75, 0x65, 0x75, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x10, 0x02,
    0x12, 0x11, 0x0a, 0x0d, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x54, 0x79, 0x70,
    0x65, 0x10, 0x03, 0x4a, 0x8b, 0x8e, 0x01, 0x0a, 0x07, 0x12, 0x05, 0x03, 0x00, 0xbe, 0x03, 0x01,
    0x0a, 0xc4, 0x01, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x03, 0x00, 0x12, 0x1a, 0xb9, 0x01, 0x20, 0x43,
    0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0xc2, 0xa9, 0x20, 0x32, 0x30, 0x31, 0x36,
    0x2d, 0x32, 0x30, 0x31, 0x37, 0x20, 0x56, 0x4d, 0x77, 0x61, 0x72, 0x65, 0x2c, 0x20, 0x49, 0x6e,
    0x63, 0x2e, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x52, 0x69, 0x67, 0x68, 0x74, 0x73, 0x20, 0x52, 0x65,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x53, 0x50, 0x44, 0x58, 0x2d, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x2d, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x3a, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x20, 0x55, 0x73,
    0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x32, 0x20, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x20,
    0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x20, 0x64,
    0x6f, 0x65, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x20,
    0x68, 0x61, 0x73, 0x58, 0x58, 0x58, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x73, 0x20, 0x69,
    0x6e, 0x20, 0x6a, 0x61, 0x76, 0x61, 0x2c, 0x20, 0x75, 0x6e, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x32, 0x2e, 0x0a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x05,
    0x07, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x10, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x08, 0x00, 0x12, 0x04, 0x08, 0x02, 0x0f, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00,
    0x01, 0x12, 0x03, 0x08, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x09, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x09, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x0a, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x0f, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0a, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x04,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x0d, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x0c, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x0d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x0e, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0e,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0e, 0x13, 0x14,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x12, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00,
    0x12, 0x04, 0x13, 0x02, 0x24, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12,
    0x03, 0x13, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x04, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x15, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x15, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15,
    0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x16, 0x04, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x16, 0x04, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x16, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x03, 0x12, 0x03, 0x17, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x17, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17,
    0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x18, 0x04, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x18, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18, 0x14, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x18, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12,
    0x03, 0x19, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x19,
    0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x19, 0x11, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x19, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x1a, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1a, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x1a, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x1a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x1b,
    0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x06, 0x12, 0x03, 0x1b, 0x04, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1b, 0x0c, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1b, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x1c, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x08, 0x06, 0x12, 0x03, 0x1c, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x1c, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x1c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x1d, 0x04, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x06, 0x12, 0x03, 0x1d, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1d, 0x14, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x0a, 0x12, 0x03, 0x1e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x06,
    0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x1e, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x1e, 0x19,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x1f, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x1f, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0b, 0x03, 0x12, 0x03, 0x1f, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0c,
    0x12, 0x03, 0x20, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x06, 0x12, 0x03,
    0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x20, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x20, 0x18, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x21, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x21, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x21, 0x15, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d,
    0x03, 0x12, 0x03, 0x21, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0e, 0x12, 0x03,
    0x22, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x22, 0x04,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x22, 0x0f, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x22, 0x1d, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x23, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0f, 0x06, 0x12, 0x03, 0x23, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f,
    0x01, 0x12, 0x03, 0x23, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x03, 0x12,
    0x03, 0x23, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x27, 0x00, 0x2b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x28, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x28, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x29, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x2a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x2d, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x2d, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x2f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x30, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x30, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x30, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30,
    0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x33, 0x00, 0x38, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x33, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x34, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x34, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x35, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x35, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x35, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x36, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x36, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x03, 0x12, 0x03, 0x37, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x37,
    0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x37, 0x14, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x37, 0x1c, 0x1d, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3a, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x3a, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x3b, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x3d, 0x02, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x3d, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x3e,
    0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x3e, 0x0b, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3e, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3e, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x41, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x41,
    0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x42, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x42, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x42, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12,
    0x03, 0x43, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x43,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x43, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x44, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x44, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x44, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x44, 0x17,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x45, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x45, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x45, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x45, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03,
    0x46, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x46, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x46, 0x12, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x46, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x05, 0x12, 0x03, 0x47, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x47, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x47,
    0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x47, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x48, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x48, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x48, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x48, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4b, 0x00, 0x51,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x4c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4c, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x4d, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12,
    0x03, 0x4e, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4e, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x4f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x4f, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4f, 0x17,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x50, 0x02, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x50, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x50, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x50, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x53, 0x00,
    0x59, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x53, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x54, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x54, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x54, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x55, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x55, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x55, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02,
    0x12, 0x03, 0x56, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x56, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x56, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x56, 0x12, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x56, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x57, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x57, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x57, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x57,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x58, 0x02, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x58, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x03, 0x58, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x58, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x58, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x5b,
    0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x5c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5c, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x5c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x02, 0x12, 0x03, 0x5e, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5e,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x12, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x5f, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x5f, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x5f, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x62, 0x00, 0x66, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x62, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x64, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x64, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x64, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x65,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x65, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05, 0x12, 0x03, 0x65, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x65, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x65, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b,
    0x12, 0x04, 0x68, 0x00, 0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x68,
    0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x69, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x69, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12,
    0x03, 0x6a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6a,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6a, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x6b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x6b, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x17,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x06, 0x12, 0x03, 0x6c, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x6c, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x6c, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x6f, 0x00,
    0x76, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x70, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x70, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x70, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x70, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x71, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02,
    0x12, 0x03, 0x72, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x72, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x72, 0x12, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x72, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x03, 0x73, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x73, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x73, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x73,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x03, 0x74, 0x02, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x03, 0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x04, 0x05, 0x12, 0x03, 0x74, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x04, 0x01, 0x12, 0x03, 0x74, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x74, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12,
    0x03, 0x75, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x03, 0x75,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x06, 0x12, 0x03, 0x75, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x03, 0x75, 0x11, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x03, 0x12, 0x03, 0x75, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x04, 0x78, 0x00, 0x7b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12,
    0x03, 0x78, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x79, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x79, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x16, 0x17, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x01, 0x12, 0x03, 0x7a, 0x02, 0x1c, 0x22, 0x10, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x20, 0x55, 0x55, 0x49, 0x44, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x7a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7a,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7a, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x05, 0x7d, 0x00, 0x85, 0x01, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x7d, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x00, 0x12, 0x03, 0x7e, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x7e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7e,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7e, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7e, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x7f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x7f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x7f, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0x80, 0x01, 0x02,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0x80, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x12, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0x80, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x81, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x81, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04, 0x12,
    0x04, 0x82, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x05, 0x12, 0x04, 0x82,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x04, 0x82, 0x01,
    0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x04, 0x82, 0x01, 0x17,
    0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x04, 0x83, 0x01, 0x02, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x05, 0x01, 0x12, 0x04, 0x83, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x04, 0x83, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x06, 0x12, 0x04, 0x84, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x06, 0x04, 0x12, 0x04, 0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06,
    0x06, 0x12, 0x04, 0x84, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x01,
    0x12, 0x04, 0x84, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x03, 0x12,
    0x04, 0x84, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x87, 0x01, 0x00,
    0x8c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x12,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0x88, 0x01, 0x02, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x88, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x01, 0x12, 0x04, 0x89, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x89, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x89, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x89, 0x01, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x89, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0x8a, 0x01,
    0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8a, 0x01, 0x0b, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x1d, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x2a, 0x2b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x1d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06,
    0x8e, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x8e,
    0x01, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x02,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x12, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0x90, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x90, 0x01, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x90, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x90, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0x93,
    0x01, 0x00, 0x98, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0x93, 0x01,
    0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x08, 0x00, 0x12, 0x06, 0x94, 0x01, 0x02, 0x97,
    0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x08, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08,
    0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x04, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x96, 0x01, 0x0c, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x96, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x9a,
    0x01, 0x00, 0x9d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x9a, 0x01,
    0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x02, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x9c, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x9c, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x9c, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x9f, 0x01,
    0x00, 0xa2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xa0, 0x01, 0x02, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xa1, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xa1, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xa1, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xa4, 0x01, 0x00,
    0xbe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x0e,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x14, 0x08, 0x00, 0x12, 0x06, 0xa5, 0x01, 0x02, 0xbd, 0x01, 0x03,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x08, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x0a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xa7, 0x01, 0x0f, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xa7, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xa8,
    0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa8, 0x01,
    0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x0b,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x19, 0x1a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x04, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xaa, 0x01, 0x0c, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xaa, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x05, 0x12, 0x04,
    0xab, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x05, 0x12, 0x04, 0xab,
    0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x01, 0x12, 0x04, 0xab, 0x01,
    0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x03, 0x12, 0x04, 0xab, 0x01, 0x16,
    0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x06, 0x12, 0x04, 0xac, 0x01, 0x04, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x05, 0x12, 0x04, 0xac, 0x01, 0x04, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x01, 0x12, 0x04, 0xac, 0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x06, 0x03, 0x12, 0x04, 0xac, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x07, 0x12, 0x04, 0xad, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x07, 0x06, 0x12, 0x04, 0xad, 0x01, 0x04, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x07, 0x01, 0x12, 0x04, 0xad, 0x01, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x07,
    0x03, 0x12, 0x04, 0xad, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x08, 0x12,
    0x04, 0xae, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x05, 0x12, 0x04,
    0xae, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x01, 0x12, 0x04, 0xae,
    0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x03, 0x12, 0x04, 0xae, 0x01,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x09, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x09, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x09, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x0b, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x09, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x19, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x14, 0x02, 0x0a, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x0a, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x0a, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x0b,
    0x12, 0x04, 0xb1, 0x01, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0b, 0x05, 0x12,
    0x04, 0xb1, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0b, 0x01, 0x12, 0x04,
    0xb1, 0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xb1,
    0x01, 0x17, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x0c, 0x12, 0x04, 0xb2, 0x01, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0c, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x0d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x0e, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x1b, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x0d, 0x12, 0x04, 0xb3, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x0d, 0x06, 0x12, 0x04, 0xb3, 0x01, 0x04, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x0d, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x1b, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02,
    0x0e, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0e, 0x06,
    0x12, 0x04, 0xb4, 0x01, 0x04, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0e, 0x01, 0x12,
    0x04, 0xb4, 0x01, 0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0e, 0x03, 0x12, 0x04,
    0xb4, 0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x0f, 0x12, 0x04, 0xb5, 0x01,
    0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0f, 0x06, 0x12, 0x04, 0xb5, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x0d, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x19, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x10, 0x12, 0x04, 0xb6, 0x01, 0x04, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x10, 0x06, 0x12, 0x04, 0xb6, 0x01, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x10, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x14, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x10, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x11, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x11,
    0x06, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x11, 0x01,
    0x12, 0x04, 0xb7, 0x01, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x11, 0x03, 0x12,
    0x04, 0xb7, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x12, 0x12, 0x04, 0xb8,
    0x01, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x12, 0x06, 0x12, 0x04, 0xb8, 0x01,
    0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x12, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x1b,
    0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x12, 0x03, 0x12, 0x04, 0xb8, 0x01, 0x36, 0x38,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x13, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x13, 0x06, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x13, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x13, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x14, 0x12, 0x04, 0xba, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x14, 0x06, 0x12, 0x04, 0xba, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x14,
    0x01, 0x12, 0x04, 0xba, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x14, 0x03,
    0x12, 0x04, 0xba, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x15, 0x12, 0x04,
    0xbb, 0x01, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x15, 0x06, 0x12, 0x04, 0xbb,
    0x01, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x15, 0x01, 0x12, 0x04, 0xbb, 0x01,
    0x14, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x15, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x28,
    0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x16, 0x12, 0x04, 0xbc, 0x01, 0x04, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x16, 0x06, 0x12, 0x04, 0xbc, 0x01, 0x04, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x16, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x13, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x16, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x06, 0xc0, 0x01, 0x00, 0xc5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x00,
    0x01, 0x12, 0x04, 0xc0, 0x01, 0x05, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xc1, 0x01, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xc1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xc1,
    0x01, 0x0d, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x02,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x0b,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xc2, 0x01, 0x0e, 0x0f, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x0e, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xc3, 0x01, 0x0c, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x04, 0xc4, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xc7,
    0x01, 0x00, 0xca, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xc7, 0x01,
    0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xc8, 0x01, 0x02, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc8, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc8, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xc9, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xc9, 0x01, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xc9, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xcc, 0x01,
    0x00, 0xcf, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08,
    0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xcd, 0x01, 0x02, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcd, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcd, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x01, 0x12, 0x04, 0xce, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xce, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xce, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xce, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xce, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xd1, 0x01, 0x00,
    0xd4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x11,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xd2, 0x01, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xd3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xd3, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xd3, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xd3, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xd6, 0x01, 0x00, 0xd9,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x11, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02,
    0x01, 0x12, 0x04, 0xd8, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xd8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xd8, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xd8, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd8,
    0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xdb, 0x01, 0x00, 0xde, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x11, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01,
    0x12, 0x04, 0xdd, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xdd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xdd, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdd,
    0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdd, 0x01,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xe0, 0x01, 0x00, 0xe3, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe1, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12,
    0x04, 0xe2, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xe2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe2,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x01,
    0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x17,
    0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe7, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe6, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xe9, 0x01,
    0x00, 0xec, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xe9, 0x01, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xea, 0x01, 0x02, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xea, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xea, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xea, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xea, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1c, 0x02, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xeb, 0x01, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xeb, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xeb, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xee, 0x01, 0x00,
    0xf1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xee, 0x01, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xef, 0x01, 0x02, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xef, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xef, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xef, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xef, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d,
    0x02, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xf0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xf0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xf0, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xf0, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xf3, 0x01, 0x00, 0xf6,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xf4, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf4, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02,
    0x01, 0x12, 0x04, 0xf5, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xf5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xf5, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xf5, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf5,
    0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xf8, 0x01, 0x00, 0xfb, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xf9, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf9, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01,
    0x12, 0x04, 0xfa, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xfa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xfa, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfa,
    0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfa, 0x01,
    0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xfd, 0x01, 0x00, 0x80, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xfe, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xfe, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12,
    0x04, 0xff, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xff, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x05, 0x12, 0x04, 0xff,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xff, 0x01,
    0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xff, 0x01, 0x1a,
    0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0x82, 0x02, 0x00, 0x85, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0x82, 0x02, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0x83, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x83, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x83, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x83, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x83, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04,
    0x84, 0x02, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0x84,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x06, 0x12, 0x04, 0x84, 0x02,
    0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0x84, 0x02, 0x0f,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0x84, 0x02, 0x15, 0x16,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0x87, 0x02, 0x00, 0x8a, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0x87, 0x02, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x22, 0x02, 0x00, 0x12, 0x04, 0x88, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x88, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x88, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x88, 0x02, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x88, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0x89,
    0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0x89, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x05, 0x12, 0x04, 0x89, 0x02, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0x89, 0x02, 0x12, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0x89, 0x02, 0x1a, 0x1b, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0x8c, 0x02, 0x00, 0x8f, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23,
    0x02, 0x00, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x8d, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x8d, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x8d, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x02,
    0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8e, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8e, 0x02, 0x0b, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x0f, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x15, 0x16, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0x91, 0x02, 0x00, 0x99, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x24, 0x01, 0x12, 0x04, 0x91, 0x02, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x24, 0x08,
    0x00, 0x12, 0x06, 0x92, 0x02, 0x02, 0x98, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x08,
    0x00, 0x01, 0x12, 0x04, 0x92, 0x02, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00,
    0x12, 0x04, 0x93, 0x02, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x93, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x93, 0x02, 0x09, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93,
    0x02, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x01, 0x12, 0x04, 0x94, 0x02, 0x04,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x06, 0x12, 0x04, 0x94, 0x02, 0x04, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x02, 0x11, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x02, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x24, 0x02, 0x02, 0x12, 0x04, 0x95, 0x02, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x02, 0x06, 0x12, 0x04, 0x95, 0x02, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x02, 0x01, 0x12, 0x04, 0x95, 0x02, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x95, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02,
    0x03, 0x12, 0x04, 0x96, 0x02, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x05,
    0x12, 0x04, 0x96, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x96, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x96, 0x02, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x04, 0x12, 0x04, 0x97, 0x02,
    0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x06, 0x12, 0x04, 0x97, 0x02, 0x04,
    0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x01, 0x12, 0x04, 0x97, 0x02, 0x0f, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x03, 0x12, 0x04, 0x97, 0x02, 0x17, 0x18, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0x9b, 0x02, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x25, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x25,
    0x08, 0x00, 0x12, 0x06, 0x9c, 0x02, 0x02, 0xae, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x08, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x00, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9d, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9d, 0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9d, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x02,
    0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9e, 0x02, 0x04,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x0b, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x02, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x02, 0x04, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25,
    0x02, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03,
    0x06, 0x12, 0x04, 0xa0, 0x02, 0x04, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xa0, 0x02, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xa0, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x04, 0x12, 0x04, 0xa1,
    0x02, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa1, 0x02,
    0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x0b,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x05, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x05, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x05, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x05, 0x03, 0x12, 0x04, 0xa2, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x25, 0x02, 0x06, 0x12, 0x04, 0xa3, 0x02, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x06, 0x06, 0x12, 0x04, 0xa3, 0x02, 0x04, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x06,
    0x01, 0x12, 0x04, 0xa3, 0x02, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x06, 0x03,
    0x12, 0x04, 0xa3, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x07, 0x12, 0x04,
    0xa4, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x07, 0x05, 0x12, 0x04, 0xa4,
    0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x07, 0x01, 0x12, 0x04, 0xa4, 0x02,
    0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x07, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x18,
    0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x08, 0x12, 0x04, 0xa5, 0x02, 0x04, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x08, 0x05, 0x12, 0x04, 0xa5, 0x02, 0x04, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x08, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x0b, 0x0d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x08, 0x03, 0x12, 0x04, 0xa5, 0x02, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x25, 0x02, 0x09, 0x12, 0x04, 0xa6, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x09, 0x05, 0x12, 0x04, 0xa6, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x09, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x09,
    0x03, 0x12, 0x04, 0xa6, 0x02, 0x1c, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x0a, 0x12,
    0x04, 0xa7, 0x02, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0a, 0x05, 0x12, 0x04,
    0xa7, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xa7,
    0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xa7, 0x02,
    0x19, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x0b, 0x12, 0x04, 0xa8, 0x02, 0x04, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xa8, 0x02, 0x04, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x0b, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x11, 0x13, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x25, 0x02, 0x0c, 0x12, 0x04, 0xa9, 0x02, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xa9, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x0c, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x09, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x0c, 0x03, 0x12, 0x04, 0xa9, 0x02, 0x1e, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x0d,
    0x12, 0x04, 0xaa, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0d, 0x05, 0x12,
    0x04, 0xaa, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0d, 0x01, 0x12, 0x04,
    0xaa, 0x02, 0x09, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xaa,
    0x02, 0x17, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x0e, 0x12, 0x04, 0xab, 0x02, 0x04,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0e, 0x05, 0x12, 0x04, 0xab, 0x02, 0x04, 0x08,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xab, 0x02, 0x09, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xab, 0x02, 0x13, 0x15, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x25, 0x02, 0x0f, 0x12, 0x04, 0xac, 0x02, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x0f, 0x05, 0x12, 0x04, 0xac, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xac, 0x02, 0x09, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x0f, 0x03, 0x12, 0x04, 0xac, 0x02, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x10, 0x12, 0x04, 0xad, 0x02, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x10, 0x05,
    0x12, 0x04, 0xad, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x10, 0x01, 0x12,
    0x04, 0xad, 0x02, 0x09, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x10, 0x03, 0x12, 0x04,
    0xad, 0x02, 0x15, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0xb1, 0x02, 0x00, 0xb4,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x11, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb2, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xb2, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02,
    0x01, 0x12, 0x04, 0xb3, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xb3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xb3, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xb3, 0x02, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb3,
    0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27, 0x12, 0x06, 0xb6, 0x02, 0x00, 0xba, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x08, 0x11, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb7, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01,
    0x12, 0x04, 0xb8, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xb8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xb8, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb8,
    0x02, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb8, 0x02,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x02, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb9, 0x02, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x28, 0x12, 0x06, 0xbc, 0x02, 0x00, 0xc7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28,
    0x01, 0x12, 0x04, 0xbc, 0x02, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12,
    0x04, 0xbd, 0x02, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xbd, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbd,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbd, 0x02,
    0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x1c,
    0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x28, 0x08, 0x00, 0x12, 0x06, 0xbe, 0x02, 0x02, 0xc6, 0x02,
    0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x08, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x04, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbf, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x09, 0x0b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbf, 0x02, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x28, 0x02, 0x02, 0x12, 0x04, 0xc0, 0x02, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xc0, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xc0, 0x02, 0x09, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xc0, 0x02, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x03, 0x12, 0x04,
    0xc1, 0x02, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc1,
    0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc1, 0x02,
    0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc1, 0x02, 0x10,
    0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x04, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x04, 0x05, 0x12, 0x04, 0xc2, 0x02, 0x04, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x0a, 0x0e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x28, 0x02, 0x05, 0x12, 0x04, 0xc3, 0x02, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x05, 0x05, 0x12, 0x04, 0xc3, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x05, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x05,
    0x03, 0x12, 0x04, 0xc3, 0x02, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x06, 0x12,
    0x04, 0xc4, 0x02, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x06, 0x06, 0x12, 0x04,
    0xc4, 0x02, 0x04, 0x07, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x06, 0x01, 0x12, 0x04, 0xc4,
    0x02, 0x08, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x06, 0x03, 0x12, 0x04, 0xc4, 0x02,
    0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x07, 0x12, 0x04, 0xc5, 0x02, 0x04, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x07, 0x06, 0x12, 0x04, 0xc5, 0x02, 0x04, 0x08, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x07, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x09, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x07, 0x03, 0x12, 0x04, 0xc5, 0x02, 0x10, 0x11, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x29, 0x12, 0x06, 0xc9, 0x02, 0x00, 0xcb, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x29, 0x01, 0x12, 0x04, 0xc9, 0x02, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00,
    0x12, 0x04, 0xca, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xca, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xca, 0x02, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0xca,
    0x02, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0xca, 0x02,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0xcd, 0x02, 0x00, 0xcf, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x0c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0xce, 0x02, 0x02, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x00, 0x04, 0x12, 0x06, 0xce, 0x02, 0x02, 0xcd, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xce, 0x02, 0x02, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x02, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xce, 0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12,
    0x06, 0xd1, 0x02, 0x00, 0xdb, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04,
    0xd1, 0x02, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x2b, 0x08, 0x00, 0x12, 0x06, 0xd2, 0x02,
    0x02, 0xda, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x08, 0x00, 0x01, 0x12, 0x04, 0xd2,
    0x02, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0xd3, 0x02, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd3, 0x02, 0x04, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd3, 0x02, 0x0f, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd3, 0x02, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2b, 0x02, 0x01, 0x12, 0x04, 0xd4, 0x02, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd4, 0x02, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd4, 0x02, 0x13, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xd4, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02,
    0x02, 0x12, 0x04, 0xd5, 0x02, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xd5, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xd5, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xd5, 0x02, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x03, 0x12, 0x04, 0xd6, 0x02,
    0x04, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x06, 0x12, 0x04, 0xd6, 0x02, 0x04,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd6, 0x02, 0x1d, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd6, 0x02, 0x2f, 0x30, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x04, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x04, 0x06, 0x12, 0x04, 0xd7, 0x02, 0x04, 0x07, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2b, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd7, 0x02, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b,
    0x02, 0x05, 0x12, 0x04, 0xd8, 0x02, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05,
    0x06, 0x12, 0x04, 0xd8, 0x02, 0x04, 0x07, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xd8, 0x02, 0x08, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05, 0x03, 0x12,
    0x04, 0xd8, 0x02, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x06, 0x12, 0x04, 0xd9,
    0x02, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd9, 0x02,
    0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd9, 0x02, 0x0b,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd9, 0x02, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xdd, 0x02, 0x00, 0xe0, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x2c, 0x02, 0x00, 0x12, 0x04, 0xde, 0x02, 0x02, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x00, 0x04, 0x12, 0x06, 0xde, 0x02, 0x02, 0xdd, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xde, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xde, 0x02, 0x1d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xde, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x01, 0x12,
    0x04, 0xdf, 0x02, 0x02, 0x21, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x04, 0x12, 0x06,
    0xdf, 0x02, 0x02, 0xde, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xdf, 0x02, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xdf, 0x02, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdf,
    0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0xe2, 0x02, 0x00, 0xe5, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x08, 0x15, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x02, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe3, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xe3, 0x02, 0x1d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xe3, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01,
    0x12, 0x04, 0xe4, 0x02, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xe4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xe4, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe4,
    0x02, 0x1d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe4, 0x02,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xe7, 0x02, 0x00, 0xea, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xe7, 0x02, 0x08, 0x16, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xe8, 0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe8, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe8, 0x02, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe8, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x01, 0x12,
    0x04, 0xe9, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xe9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe9,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe9, 0x02,
    0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe9, 0x02, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xec, 0x02, 0x00, 0xf0, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2f, 0x01, 0x12, 0x04, 0xec, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2f, 0x02, 0x00, 0x12, 0x04, 0xed, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xed, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xed, 0x02, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xed, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x01, 0x12, 0x04,
    0xee, 0x02, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x06, 0x12, 0x04, 0xee, 0x02,
    0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x02, 0x1d,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x02, 0x2a, 0x2b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x02, 0x12, 0x04, 0xef, 0x02, 0x02, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xef, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2f, 0x02, 0x02, 0x06, 0x12, 0x04, 0xef, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xef, 0x02, 0x1d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xef, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x30,
    0x12, 0x06, 0xf2, 0x02, 0x00, 0xf6, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x30, 0x01, 0x12,
    0x04, 0xf2, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00, 0x12, 0x04, 0xf3,
    0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf3, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf3, 0x02, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf3, 0x02, 0x12, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf3, 0x02, 0x1a, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x01, 0x12, 0x04, 0xf4, 0x02, 0x02, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf4, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf4, 0x02, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xf4, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02,
    0x02, 0x12, 0x04, 0xf5, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xf5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x06, 0x12,
    0x04, 0xf5, 0x02, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xf5, 0x02, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf5,
    0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x31, 0x12, 0x06, 0xf8, 0x02, 0x00, 0xff, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x31, 0x01, 0x12, 0x04, 0xf8, 0x02, 0x08, 0x0e, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x31, 0x08, 0x00, 0x12, 0x06, 0xf9, 0x02, 0x02, 0xfe, 0x02, 0x03, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x31, 0x08, 0x00, 0x01, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x0b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x31, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x02, 0x04, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfa, 0x02, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xfa, 0x02, 0x17, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xfa, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x01,
    0x12, 0x04, 0xfb, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xfb, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xfb, 0x02, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfb,
    0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x02, 0x12, 0x04, 0xfc, 0x02, 0x04,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfc, 0x02, 0x04, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfc, 0x02, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfc, 0x02, 0x13, 0x14, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x31, 0x02, 0x03, 0x12, 0x04, 0xfd, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x03, 0x05, 0x12, 0x04, 0xfd, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x03, 0x01, 0x12, 0x04, 0xfd, 0x02, 0x09, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xfd, 0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12,
    0x06, 0x81, 0x03, 0x00, 0x84, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04,
    0x81, 0x03, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0x82, 0x03,
    0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x06, 0x12, 0x04, 0x82, 0x03, 0x0b, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x03, 0x0f, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x03, 0x19, 0x1a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x32, 0x02, 0x01, 0x12, 0x04, 0x83, 0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x32, 0x02, 0x01, 0x04, 0x12, 0x04, 0x83, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x01, 0x05, 0x12, 0x04, 0x83, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x03, 0x10, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x83, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x33, 0x12, 0x06,
    0x86, 0x03, 0x00, 0x89, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33, 0x01, 0x12, 0x04, 0x86,
    0x03, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x00, 0x12, 0x04, 0x87, 0x03, 0x02,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x06, 0x12, 0x04, 0x87, 0x03, 0x0b, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x03, 0x0f, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x03, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x33, 0x02, 0x01, 0x12, 0x04, 0x88, 0x03, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x33, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x88, 0x03, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x88, 0x03, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x34, 0x12, 0x06, 0x8b,
    0x03, 0x00, 0x96, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x34, 0x01, 0x12, 0x04, 0x8b, 0x03,
    0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x34, 0x08, 0x00, 0x12, 0x06, 0x8c, 0x03, 0x02, 0x95,
    0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x08, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x08,
    0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x00, 0x12, 0x04, 0x8d, 0x03, 0x04, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8d, 0x03, 0x04, 0x08, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x09, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x34, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8d, 0x03, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x34, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x8e, 0x03, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x8e, 0x03, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x02, 0x12,
    0x04, 0x8f, 0x03, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x06, 0x12, 0x04,
    0x8f, 0x03, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8f,
    0x03, 0x09, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x03,
    0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x03, 0x12, 0x04, 0x90, 0x03, 0x04, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x03, 0x05, 0x12, 0x04, 0x90, 0x03, 0x04, 0x08, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x03, 0x01, 0x12, 0x04, 0x90, 0x03, 0x09, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x34, 0x02, 0x03, 0x03, 0x12, 0x04, 0x90, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x34, 0x02, 0x04, 0x12, 0x04, 0x91, 0x03, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x34, 0x02, 0x04, 0x06, 0x12, 0x04, 0x91, 0x03, 0x04, 0x07, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x91, 0x03, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x91, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x05,
    0x12, 0x04, 0x92, 0x03, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x05, 0x05, 0x12,
    0x04, 0x92, 0x03, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x05, 0x01, 0x12, 0x04,
    0x92, 0x03, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x05, 0x03, 0x12, 0x04, 0x92,
    0x03, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x06, 0x12, 0x04, 0x93, 0x03, 0x04,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x06, 0x05, 0x12, 0x04, 0x93, 0x03, 0x04, 0x08,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x06, 0x01, 0x12, 0x04, 0x93, 0x03, 0x09, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x06, 0x03, 0x12, 0x04, 0x93, 0x03, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x34, 0x02, 0x07, 0x12, 0x04, 0x94, 0x03, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x34, 0x02, 0x07, 0x06, 0x12, 0x04, 0x94, 0x03, 0x04, 0x07, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x34, 0x02, 0x07, 0x01, 0x12, 0x04, 0x94, 0x03, 0x08, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34,
    0x02, 0x07, 0x03, 0x12, 0x04, 0x94, 0x03, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x35, 0x12,
    0x06, 0x98, 0x03, 0x00, 0xa5, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x35, 0x01, 0x12, 0x04,
    0x98, 0x03, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x35, 0x08, 0x00, 0x12, 0x06, 0x99, 0x03,
    0x02, 0xa4, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x08, 0x00, 0x01, 0x12, 0x04, 0x99,
    0x03, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x00, 0x12, 0x04, 0x9a, 0x03, 0x04,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9a, 0x03, 0x04, 0x08,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x03, 0x09, 0x0b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9a, 0x03, 0x0e, 0x0f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x35, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x03, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x35, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9b, 0x03, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x35, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9b, 0x03, 0x09, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x9b, 0x03, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02,
    0x02, 0x12, 0x04, 0x9c, 0x03, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x02, 0x05,
    0x12, 0x04, 0x9c, 0x03, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x9c, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x9c, 0x03, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x03, 0x12, 0x04, 0x9d, 0x03,
    0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x03, 0x06, 0x12, 0x04, 0x9d, 0x03, 0x04,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9d, 0x03, 0x14, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x04, 0x12, 0x04, 0x9e, 0x03, 0x04, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x35, 0x02, 0x04, 0x05, 0x12, 0x04, 0x9e, 0x03, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x35, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x35, 0x02, 0x04, 0x03, 0x12, 0x04, 0x9e, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35,
    0x02, 0x05, 0x12, 0x04, 0x9f, 0x03, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x05,
    0x06, 0x12, 0x04, 0x9f, 0x03, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x9f, 0x03, 0x0f, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x05, 0x03, 0x12,
    0x04, 0x9f, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x06, 0x12, 0x04, 0xa0,
    0x03, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x06, 0x06, 0x12, 0x04, 0xa0, 0x03,
    0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x06, 0x01, 0x12, 0x04, 0xa0, 0x03, 0x11,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x06, 0x03, 0x12, 0x04, 0xa0, 0x03, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x07, 0x12, 0x04, 0xa1, 0x03, 0x04, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x35, 0x02, 0x07, 0x06, 0x12, 0x04, 0xa1, 0x03, 0x04, 0x07, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x35, 0x02, 0x07, 0x01, 0x12, 0x04, 0xa1, 0x03, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x35, 0x02, 0x07, 0x03, 0x12, 0x04, 0xa1, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x35, 0x02, 0x08, 0x12, 0x04, 0xa2, 0x03, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02,
    0x08, 0x06, 0x12, 0x04, 0xa2, 0x03, 0x04, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xa2, 0x03, 0x0c, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x08, 0x03,
    0x12, 0x04, 0xa2, 0x03, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x09, 0x12, 0x04,
    0xa3, 0x03, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x09, 0x06, 0x12, 0x04, 0xa3,
    0x03, 0x04, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x09, 0x01, 0x12, 0x04, 0xa3, 0x03,
    0x0c, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa3, 0x03, 0x16,
    0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x36, 0x12, 0x06, 0xa7, 0x03, 0x00, 0xac, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x36, 0x01, 0x12, 0x04, 0xa7, 0x03, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x36, 0x02, 0x00, 0x12, 0x04, 0xa8, 0x03, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa8, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xa8, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa8, 0x03, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa8, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x01, 0x12, 0x04,
    0xa9, 0x03, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa9,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa9, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x03, 0x12,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9, 0x03, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x02, 0x12, 0x04, 0xaa, 0x03, 0x02, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x02, 0x02, 0x05, 0x12, 0x04, 0xaa, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x36, 0x02, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x36, 0x02, 0x02, 0x03, 0x12, 0x04, 0xaa, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36,
    0x02, 0x03, 0x12, 0x04, 0xab, 0x03, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xab, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xab, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xab, 0x03, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xab, 0x03, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x37, 0x12, 0x06, 0xae, 0x03, 0x00, 0xb0,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x37, 0x01, 0x12, 0x04, 0xae, 0x03, 0x08, 0x0f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x00, 0x12, 0x04, 0xaf, 0x03, 0x02, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x37, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaf, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x00, 0x06, 0x12, 0x04, 0xaf, 0x03, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x37, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaf, 0x03, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xaf, 0x03, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x38, 0x12,
    0x06, 0xb2, 0x03, 0x00, 0xbe, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x38, 0x01, 0x12, 0x04,
    0xb2, 0x03, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x00, 0x12, 0x04, 0xb3, 0x03,
    0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb3, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb3, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb3, 0x03, 0x12, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb3, 0x03, 0x1a, 0x1b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x38, 0x02, 0x01, 0x12, 0x04, 0xb4, 0x03, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb4, 0x03, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xb4, 0x03, 0x0f, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb4, 0x03, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x02,
    0x12, 0x04, 0xb5, 0x03, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xb5, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xb5, 0x03, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb5,
    0x03, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb5, 0x03,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x03, 0x12, 0x04, 0xb6, 0x03, 0x02, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb6, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x05, 0x12, 0x04, 0xb6, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb6, 0x03, 0x12, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb6, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x38, 0x02, 0x04, 0x12, 0x04, 0xb7, 0x03, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xb7, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xb7, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xb7, 0x03, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xb7, 0x03, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x05, 0x12, 0x04,
    0xb8, 0x03, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x04, 0x12, 0x04, 0xb8,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x05, 0x12, 0x04, 0xb8, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x12,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x03, 0x12, 0x04, 0xb8, 0x03, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x06, 0x12, 0x04, 0xb9, 0x03, 0x02, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x06, 0x04, 0x12, 0x04, 0xb9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x06, 0x05, 0x12, 0x04, 0xb9, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb9, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x06, 0x03, 0x12, 0x04, 0xb9, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38,
    0x02, 0x07, 0x12, 0x04, 0xba, 0x03, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x07,
    0x04, 0x12, 0x04, 0xba, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x07, 0x05,
    0x12, 0x04, 0xba, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xba, 0x03, 0x12, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xba, 0x03, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x08, 0x12, 0x04, 0xbb, 0x03,
    0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x04, 0x12, 0x04, 0xbb, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x05, 0x12, 0x04, 0xbb, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x01, 0x12, 0x04, 0xbb, 0x03, 0x12, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x03, 0x12, 0x04, 0xbb, 0x03, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x38, 0x02, 0x09, 0x12, 0x04, 0xbc, 0x03, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x09, 0x04, 0x12, 0x04, 0xbc, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x09, 0x06, 0x12, 0x04, 0xbc, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x09, 0x01, 0x12, 0x04, 0xbc, 0x03, 0x1d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x09, 0x03, 0x12, 0x04, 0xbc, 0x03, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x0a,
    0x12, 0x04, 0xbd, 0x03, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0xbd, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x06, 0x12, 0x04,
    0xbd, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xbd,
    0x03, 0x1d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xbd, 0x03,
    0x2a, 0x2c,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
