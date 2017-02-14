// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

#[derive(PartialEq,Clone,Default)]
pub struct ApiMsg {
    // message oneof groups
    req_reply: ::std::option::Option<ApiMsg_oneof_req_reply>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiMsg {}

#[derive(Clone,PartialEq)]
pub enum ApiMsg_oneof_req_reply {
    request(ApiRequest),
    response(ApiResponse),
}

impl ApiMsg {
    pub fn new() -> ApiMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiMsg {
        static mut instance: ::protobuf::lazy::Lazy<ApiMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiMsg,
        };
        unsafe {
            instance.get(ApiMsg::new)
        }
    }

    // optional .ApiRequest request = 1;

    pub fn clear_request(&mut self) {
        self.req_reply = ::std::option::Option::None;
    }

    pub fn has_request(&self) -> bool {
        match self.req_reply {
            ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ApiRequest) {
        self.req_reply = ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request(&mut self) -> &mut ApiRequest {
        if let ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(_)) = self.req_reply {
        } else {
            self.req_reply = ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(ApiRequest::new()));
        }
        match self.req_reply {
            ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request(&mut self) -> ApiRequest {
        if self.has_request() {
            match self.req_reply.take() {
                ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(v)) => v,
                _ => panic!(),
            }
        } else {
            ApiRequest::new()
        }
    }

    pub fn get_request(&self) -> &ApiRequest {
        match self.req_reply {
            ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(ref v)) => v,
            _ => ApiRequest::default_instance(),
        }
    }

    // optional .ApiResponse response = 2;

    pub fn clear_response(&mut self) {
        self.req_reply = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        match self.req_reply {
            ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ApiResponse) {
        self.req_reply = ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_response(&mut self) -> &mut ApiResponse {
        if let ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(_)) = self.req_reply {
        } else {
            self.req_reply = ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(ApiResponse::new()));
        }
        match self.req_reply {
            ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_response(&mut self) -> ApiResponse {
        if self.has_response() {
            match self.req_reply.take() {
                ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(v)) => v,
                _ => panic!(),
            }
        } else {
            ApiResponse::new()
        }
    }

    pub fn get_response(&self) -> &ApiResponse {
        match self.req_reply {
            ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(ref v)) => v,
            _ => ApiResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for ApiMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req_reply = ::std::option::Option::Some(ApiMsg_oneof_req_reply::request(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.req_reply = ::std::option::Option::Some(ApiMsg_oneof_req_reply::response(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.req_reply {
            match v {
                &ApiMsg_oneof_req_reply::request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiMsg_oneof_req_reply::response(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.req_reply {
            match v {
                &ApiMsg_oneof_req_reply::request(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiMsg_oneof_req_reply::response(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiMsg {
    fn new() -> ApiMsg {
        ApiMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ApiRequest>(
                    "request",
                    ApiMsg::has_request,
                    ApiMsg::get_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ApiResponse>(
                    "response",
                    ApiMsg::has_response,
                    ApiMsg::get_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiMsg>(
                    "ApiMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiMsg {
    fn clear(&mut self) {
        self.clear_request();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApiRequest {
    // message oneof groups
    request: ::std::option::Option<ApiRequest_oneof_request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiRequest {}

#[derive(Clone,PartialEq)]
pub enum ApiRequest_oneof_request {
    get_namespaces(bool),
    register_client(RegisterClient),
    consensus_request(ConsensusRequest),
}

impl ApiRequest {
    pub fn new() -> ApiRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiRequest {
        static mut instance: ::protobuf::lazy::Lazy<ApiRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiRequest,
        };
        unsafe {
            instance.get(ApiRequest::new)
        }
    }

    // optional bool get_namespaces = 1;

    pub fn clear_get_namespaces(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_get_namespaces(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::get_namespaces(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_namespaces(&mut self, v: bool) {
        self.request = ::std::option::Option::Some(ApiRequest_oneof_request::get_namespaces(v))
    }

    pub fn get_get_namespaces(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::get_namespaces(v)) => v,
            _ => false,
        }
    }

    // optional .RegisterClient register_client = 2;

    pub fn clear_register_client(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_register_client(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::register_client(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_register_client(&mut self, v: RegisterClient) {
        self.request = ::std::option::Option::Some(ApiRequest_oneof_request::register_client(v))
    }

    // Mutable pointer to the field.
    pub fn mut_register_client(&mut self) -> &mut RegisterClient {
        if let ::std::option::Option::Some(ApiRequest_oneof_request::register_client(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(ApiRequest_oneof_request::register_client(RegisterClient::new()));
        }
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::register_client(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_register_client(&mut self) -> RegisterClient {
        if self.has_register_client() {
            match self.request.take() {
                ::std::option::Option::Some(ApiRequest_oneof_request::register_client(v)) => v,
                _ => panic!(),
            }
        } else {
            RegisterClient::new()
        }
    }

    pub fn get_register_client(&self) -> &RegisterClient {
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::register_client(ref v)) => v,
            _ => RegisterClient::default_instance(),
        }
    }

    // optional .ConsensusRequest consensus_request = 3;

    pub fn clear_consensus_request(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_consensus_request(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_consensus_request(&mut self, v: ConsensusRequest) {
        self.request = ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_consensus_request(&mut self) -> &mut ConsensusRequest {
        if let ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(ConsensusRequest::new()));
        }
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_consensus_request(&mut self) -> ConsensusRequest {
        if self.has_consensus_request() {
            match self.request.take() {
                ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(v)) => v,
                _ => panic!(),
            }
        } else {
            ConsensusRequest::new()
        }
    }

    pub fn get_consensus_request(&self) -> &ConsensusRequest {
        match self.request {
            ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(ref v)) => v,
            _ => ConsensusRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for ApiRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.request = ::std::option::Option::Some(ApiRequest_oneof_request::get_namespaces(is.read_bool()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.request = ::std::option::Option::Some(ApiRequest_oneof_request::register_client(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.request = ::std::option::Option::Some(ApiRequest_oneof_request::consensus_request(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &ApiRequest_oneof_request::get_namespaces(v) => {
                    my_size += 2;
                },
                &ApiRequest_oneof_request::register_client(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiRequest_oneof_request::consensus_request(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &ApiRequest_oneof_request::get_namespaces(v) => {
                    os.write_bool(1, v)?;
                },
                &ApiRequest_oneof_request::register_client(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiRequest_oneof_request::consensus_request(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiRequest {
    fn new() -> ApiRequest {
        ApiRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "get_namespaces",
                    ApiRequest::has_get_namespaces,
                    ApiRequest::get_get_namespaces,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RegisterClient>(
                    "register_client",
                    ApiRequest::has_register_client,
                    ApiRequest::get_register_client,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConsensusRequest>(
                    "consensus_request",
                    ApiRequest::has_consensus_request,
                    ApiRequest::get_consensus_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiRequest>(
                    "ApiRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiRequest {
    fn clear(&mut self) {
        self.clear_get_namespaces();
        self.clear_register_client();
        self.clear_consensus_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterClient {
    // message fields
    client_id: ::protobuf::SingularField<::std::string::String>,
    namespace_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(RegisterClient::new)
        }
    }

    // required string client_id = 1;

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

    fn get_client_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.client_id
    }

    fn mut_client_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.client_id
    }

    // required string namespace_id = 2;

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

    fn get_namespace_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.namespace_id
    }

    fn mut_namespace_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.namespace_id
    }
}

impl ::protobuf::Message for RegisterClient {
    fn is_initialized(&self) -> bool {
        if self.client_id.is_none() {
            return false;
        };
        if self.namespace_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namespace_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.namespace_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.namespace_id.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client_id",
                    RegisterClient::get_client_id_for_reflect,
                    RegisterClient::mut_client_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace_id",
                    RegisterClient::get_namespace_id_for_reflect,
                    RegisterClient::mut_namespace_id_for_reflect,
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

impl ::std::fmt::Debug for RegisterClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApiPid {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    node_name: ::protobuf::SingularField<::std::string::String>,
    node_addr: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiPid {}

impl ApiPid {
    pub fn new() -> ApiPid {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiPid {
        static mut instance: ::protobuf::lazy::Lazy<ApiPid> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiPid,
        };
        unsafe {
            instance.get(ApiPid::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required string group = 2;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        };
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // required string node_name = 3;

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

    fn get_node_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.node_name
    }

    fn mut_node_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.node_name
    }

    // required string node_addr = 4;

    pub fn clear_node_addr(&mut self) {
        self.node_addr.clear();
    }

    pub fn has_node_addr(&self) -> bool {
        self.node_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_addr(&mut self, v: ::std::string::String) {
        self.node_addr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_addr(&mut self) -> &mut ::std::string::String {
        if self.node_addr.is_none() {
            self.node_addr.set_default();
        };
        self.node_addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_node_addr(&mut self) -> ::std::string::String {
        self.node_addr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_node_addr(&self) -> &str {
        match self.node_addr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_node_addr_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.node_addr
    }

    fn mut_node_addr_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.node_addr
    }
}

impl ::protobuf::Message for ApiPid {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.group.is_none() {
            return false;
        };
        if self.node_name.is_none() {
            return false;
        };
        if self.node_addr.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.node_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.node_addr)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.node_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.node_addr.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.group.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.node_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.node_addr.as_ref() {
            os.write_string(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiPid {
    fn new() -> ApiPid {
        ApiPid::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiPid>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ApiPid::get_name_for_reflect,
                    ApiPid::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    ApiPid::get_group_for_reflect,
                    ApiPid::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_name",
                    ApiPid::get_node_name_for_reflect,
                    ApiPid::mut_node_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_addr",
                    ApiPid::get_node_addr_for_reflect,
                    ApiPid::mut_node_addr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiPid>(
                    "ApiPid",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiPid {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_group();
        self.clear_node_name();
        self.clear_node_addr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiPid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiPid {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsensusRequest {
    // message fields
    to: ::protobuf::SingularPtrField<ApiPid>,
    client_id: ::protobuf::SingularField<::std::string::String>,
    client_request_num: ::std::option::Option<u64>,
    // message oneof groups
    op: ::std::option::Option<ConsensusRequest_oneof_op>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConsensusRequest {}

#[derive(Clone,PartialEq)]
pub enum ConsensusRequest_oneof_op {
    tree_op(TreeOp),
    tree_cas(TreeCas),
}

impl ConsensusRequest {
    pub fn new() -> ConsensusRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConsensusRequest {
        static mut instance: ::protobuf::lazy::Lazy<ConsensusRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConsensusRequest,
        };
        unsafe {
            instance.get(ConsensusRequest::new)
        }
    }

    // required .ApiPid to = 1;

    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    pub fn has_to(&self) -> bool {
        self.to.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ApiPid) {
        self.to = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ApiPid {
        if self.to.is_none() {
            self.to.set_default();
        };
        self.to.as_mut().unwrap()
    }

    // Take field
    pub fn take_to(&mut self) -> ApiPid {
        self.to.take().unwrap_or_else(|| ApiPid::new())
    }

    pub fn get_to(&self) -> &ApiPid {
        self.to.as_ref().unwrap_or_else(|| ApiPid::default_instance())
    }

    fn get_to_for_reflect(&self) -> &::protobuf::SingularPtrField<ApiPid> {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApiPid> {
        &mut self.to
    }

    // required string client_id = 2;

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

    fn get_client_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.client_id
    }

    fn mut_client_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.client_id
    }

    // required uint64 client_request_num = 3;

    pub fn clear_client_request_num(&mut self) {
        self.client_request_num = ::std::option::Option::None;
    }

    pub fn has_client_request_num(&self) -> bool {
        self.client_request_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_request_num(&mut self, v: u64) {
        self.client_request_num = ::std::option::Option::Some(v);
    }

    pub fn get_client_request_num(&self) -> u64 {
        self.client_request_num.unwrap_or(0)
    }

    fn get_client_request_num_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_request_num
    }

    fn mut_client_request_num_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_request_num
    }

    // optional .TreeOp tree_op = 4;

    pub fn clear_tree_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_tree_op(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_op(&mut self, v: TreeOp) {
        self.op = ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tree_op(&mut self) -> &mut TreeOp {
        if let ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(TreeOp::new()));
        }
        match self.op {
            ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_op(&mut self) -> TreeOp {
        if self.has_tree_op() {
            match self.op.take() {
                ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeOp::new()
        }
    }

    pub fn get_tree_op(&self) -> &TreeOp {
        match self.op {
            ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(ref v)) => v,
            _ => TreeOp::default_instance(),
        }
    }

    // optional .TreeCas tree_cas = 5;

    pub fn clear_tree_cas(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_tree_cas(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_cas(&mut self, v: TreeCas) {
        self.op = ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tree_cas(&mut self) -> &mut TreeCas {
        if let ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(TreeCas::new()));
        }
        match self.op {
            ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_cas(&mut self) -> TreeCas {
        if self.has_tree_cas() {
            match self.op.take() {
                ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeCas::new()
        }
    }

    pub fn get_tree_cas(&self) -> &TreeCas {
        match self.op {
            ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(ref v)) => v,
            _ => TreeCas::default_instance(),
        }
    }
}

impl ::protobuf::Message for ConsensusRequest {
    fn is_initialized(&self) -> bool {
        if self.to.is_none() {
            return false;
        };
        if self.client_id.is_none() {
            return false;
        };
        if self.client_request_num.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.client_request_num = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_op(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(ConsensusRequest_oneof_op::tree_cas(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.to.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.client_request_num {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.op {
            match v {
                &ConsensusRequest_oneof_op::tree_op(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ConsensusRequest_oneof_op::tree_cas(ref v) => {
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
        if let Some(v) = self.to.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_id.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.client_request_num {
            os.write_uint64(3, v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.op {
            match v {
                &ConsensusRequest_oneof_op::tree_op(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ConsensusRequest_oneof_op::tree_cas(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConsensusRequest {
    fn new() -> ConsensusRequest {
        ConsensusRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConsensusRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApiPid>>(
                    "to",
                    ConsensusRequest::get_to_for_reflect,
                    ConsensusRequest::mut_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client_id",
                    ConsensusRequest::get_client_id_for_reflect,
                    ConsensusRequest::mut_client_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "client_request_num",
                    ConsensusRequest::get_client_request_num_for_reflect,
                    ConsensusRequest::mut_client_request_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TreeOp>(
                    "tree_op",
                    ConsensusRequest::has_tree_op,
                    ConsensusRequest::get_tree_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TreeCas>(
                    "tree_cas",
                    ConsensusRequest::has_tree_cas,
                    ConsensusRequest::get_tree_cas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConsensusRequest>(
                    "ConsensusRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConsensusRequest {
    fn clear(&mut self) {
        self.clear_to();
        self.clear_client_id();
        self.clear_client_request_num();
        self.clear_tree_op();
        self.clear_tree_cas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsensusRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsensusRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TreeOp {
    // message oneof groups
    op: ::std::option::Option<TreeOp_oneof_op>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TreeOp {}

#[derive(Clone,PartialEq)]
pub enum TreeOp_oneof_op {
    create_node(CreateNode),
    delete_node(DeleteNode),
    list_keys(ListKeys),
    blob_put(BlobPut),
    blob_get(BlobGet),
    blob_size(BlobSize),
    queue_push(QueuePush),
    queue_pop(QueuePop),
    queue_front(QueueFront),
    queue_back(QueueBack),
    queue_len(QueueLen),
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
            instance.get(TreeOp::new)
        }
    }

    // optional .CreateNode create_node = 1;

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

    // optional .DeleteNode delete_node = 2;

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
    pub fn set_delete_node(&mut self, v: DeleteNode) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::delete_node(v))
    }

    // Mutable pointer to the field.
    pub fn mut_delete_node(&mut self) -> &mut DeleteNode {
        if let ::std::option::Option::Some(TreeOp_oneof_op::delete_node(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::delete_node(DeleteNode::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::delete_node(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_node(&mut self) -> DeleteNode {
        if self.has_delete_node() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::delete_node(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteNode::new()
        }
    }

    pub fn get_delete_node(&self) -> &DeleteNode {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::delete_node(ref v)) => v,
            _ => DeleteNode::default_instance(),
        }
    }

    // optional .ListKeys list_keys = 3;

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
    pub fn set_list_keys(&mut self, v: ListKeys) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::list_keys(v))
    }

    // Mutable pointer to the field.
    pub fn mut_list_keys(&mut self) -> &mut ListKeys {
        if let ::std::option::Option::Some(TreeOp_oneof_op::list_keys(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::list_keys(ListKeys::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::list_keys(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_list_keys(&mut self) -> ListKeys {
        if self.has_list_keys() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::list_keys(v)) => v,
                _ => panic!(),
            }
        } else {
            ListKeys::new()
        }
    }

    pub fn get_list_keys(&self) -> &ListKeys {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::list_keys(ref v)) => v,
            _ => ListKeys::default_instance(),
        }
    }

    // optional .BlobPut blob_put = 4;

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

    // optional .BlobGet blob_get = 5;

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
    pub fn set_blob_get(&mut self, v: BlobGet) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_get(v))
    }

    // Mutable pointer to the field.
    pub fn mut_blob_get(&mut self) -> &mut BlobGet {
        if let ::std::option::Option::Some(TreeOp_oneof_op::blob_get(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_get(BlobGet::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_blob_get(&mut self) -> BlobGet {
        if self.has_blob_get() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::blob_get(v)) => v,
                _ => panic!(),
            }
        } else {
            BlobGet::new()
        }
    }

    pub fn get_blob_get(&self) -> &BlobGet {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_get(ref v)) => v,
            _ => BlobGet::default_instance(),
        }
    }

    // optional .BlobSize blob_size = 6;

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
    pub fn set_blob_size(&mut self, v: BlobSize) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_size(v))
    }

    // Mutable pointer to the field.
    pub fn mut_blob_size(&mut self) -> &mut BlobSize {
        if let ::std::option::Option::Some(TreeOp_oneof_op::blob_size(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_size(BlobSize::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_size(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_blob_size(&mut self) -> BlobSize {
        if self.has_blob_size() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::blob_size(v)) => v,
                _ => panic!(),
            }
        } else {
            BlobSize::new()
        }
    }

    pub fn get_blob_size(&self) -> &BlobSize {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::blob_size(ref v)) => v,
            _ => BlobSize::default_instance(),
        }
    }

    // optional .QueuePush queue_push = 7;

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

    // optional .QueuePop queue_pop = 8;

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
    pub fn set_queue_pop(&mut self, v: QueuePop) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(v))
    }

    // Mutable pointer to the field.
    pub fn mut_queue_pop(&mut self) -> &mut QueuePop {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(QueuePop::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_pop(&mut self) -> QueuePop {
        if self.has_queue_pop() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(v)) => v,
                _ => panic!(),
            }
        } else {
            QueuePop::new()
        }
    }

    pub fn get_queue_pop(&self) -> &QueuePop {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(ref v)) => v,
            _ => QueuePop::default_instance(),
        }
    }

    // optional .QueueFront queue_front = 9;

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
    pub fn set_queue_front(&mut self, v: QueueFront) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_front(v))
    }

    // Mutable pointer to the field.
    pub fn mut_queue_front(&mut self) -> &mut QueueFront {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_front(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_front(QueueFront::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_front(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_front(&mut self) -> QueueFront {
        if self.has_queue_front() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_front(v)) => v,
                _ => panic!(),
            }
        } else {
            QueueFront::new()
        }
    }

    pub fn get_queue_front(&self) -> &QueueFront {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_front(ref v)) => v,
            _ => QueueFront::default_instance(),
        }
    }

    // optional .QueueBack queue_back = 10;

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
    pub fn set_queue_back(&mut self, v: QueueBack) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_back(v))
    }

    // Mutable pointer to the field.
    pub fn mut_queue_back(&mut self) -> &mut QueueBack {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_back(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_back(QueueBack::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_back(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_back(&mut self) -> QueueBack {
        if self.has_queue_back() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_back(v)) => v,
                _ => panic!(),
            }
        } else {
            QueueBack::new()
        }
    }

    pub fn get_queue_back(&self) -> &QueueBack {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_back(ref v)) => v,
            _ => QueueBack::default_instance(),
        }
    }

    // optional .QueueLen queue_len = 11;

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
    pub fn set_queue_len(&mut self, v: QueueLen) {
        self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_len(v))
    }

    // Mutable pointer to the field.
    pub fn mut_queue_len(&mut self) -> &mut QueueLen {
        if let ::std::option::Option::Some(TreeOp_oneof_op::queue_len(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_len(QueueLen::new()));
        }
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_len(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_queue_len(&mut self) -> QueueLen {
        if self.has_queue_len() {
            match self.op.take() {
                ::std::option::Option::Some(TreeOp_oneof_op::queue_len(v)) => v,
                _ => panic!(),
            }
        } else {
            QueueLen::new()
        }
    }

    pub fn get_queue_len(&self) -> &QueueLen {
        match self.op {
            ::std::option::Option::Some(TreeOp_oneof_op::queue_len(ref v)) => v,
            _ => QueueLen::default_instance(),
        }
    }

    // optional .SetInsert set_insert = 12;

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

    // optional .SetRemove set_remove = 13;

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

    // optional .SetContains set_contains = 14;

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

    // optional .SetUnion set_union = 15;

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

    // optional .SetIntersection set_intersection = 16;

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

    // optional .SetDifference set_difference = 17;

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

    // optional .SetSymmetricDifference set_symmetric_difference = 18;

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

    // optional .SetSubsetPath set_subset_path = 19;

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

    // optional .SetSubsetSet set_subset_set = 20;

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

    // optional .SetSupersetPath set_superset_path = 21;

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

    // optional .SetSupersetSet set_superset_set = 22;

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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::create_node(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::delete_node(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::list_keys(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_put(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_get(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::blob_size(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_push(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_pop(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_front(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_back(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::queue_len(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_insert(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_remove(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_contains(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_union(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_intersection(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_difference(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_symmetric_difference(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_path(is.read_message()?));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_subset_set(is.read_message()?));
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_path(is.read_message()?));
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.op = ::std::option::Option::Some(TreeOp_oneof_op::set_superset_set(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
                &TreeOp_oneof_op::create_node(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::delete_node(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::list_keys(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::blob_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::blob_get(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::blob_size(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::queue_push(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::queue_pop(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::queue_front(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::queue_back(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOp_oneof_op::queue_len(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
                &TreeOp_oneof_op::create_node(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::delete_node(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::list_keys(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::blob_put(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::blob_get(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::blob_size(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::queue_push(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::queue_pop(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::queue_front(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::queue_back(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::queue_len(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_insert(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_remove(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_contains(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_union(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_intersection(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_difference(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_symmetric_difference(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_subset_path(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_subset_set(ref v) => {
                    os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_superset_path(ref v) => {
                    os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOp_oneof_op::set_superset_set(ref v) => {
                    os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CreateNode>(
                    "create_node",
                    TreeOp::has_create_node,
                    TreeOp::get_create_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeleteNode>(
                    "delete_node",
                    TreeOp::has_delete_node,
                    TreeOp::get_delete_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ListKeys>(
                    "list_keys",
                    TreeOp::has_list_keys,
                    TreeOp::get_list_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BlobPut>(
                    "blob_put",
                    TreeOp::has_blob_put,
                    TreeOp::get_blob_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BlobGet>(
                    "blob_get",
                    TreeOp::has_blob_get,
                    TreeOp::get_blob_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BlobSize>(
                    "blob_size",
                    TreeOp::has_blob_size,
                    TreeOp::get_blob_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueuePush>(
                    "queue_push",
                    TreeOp::has_queue_push,
                    TreeOp::get_queue_push,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueuePop>(
                    "queue_pop",
                    TreeOp::has_queue_pop,
                    TreeOp::get_queue_pop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueueFront>(
                    "queue_front",
                    TreeOp::has_queue_front,
                    TreeOp::get_queue_front,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueueBack>(
                    "queue_back",
                    TreeOp::has_queue_back,
                    TreeOp::get_queue_back,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, QueueLen>(
                    "queue_len",
                    TreeOp::has_queue_len,
                    TreeOp::get_queue_len,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetInsert>(
                    "set_insert",
                    TreeOp::has_set_insert,
                    TreeOp::get_set_insert,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetRemove>(
                    "set_remove",
                    TreeOp::has_set_remove,
                    TreeOp::get_set_remove,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetContains>(
                    "set_contains",
                    TreeOp::has_set_contains,
                    TreeOp::get_set_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetUnion>(
                    "set_union",
                    TreeOp::has_set_union,
                    TreeOp::get_set_union,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetIntersection>(
                    "set_intersection",
                    TreeOp::has_set_intersection,
                    TreeOp::get_set_intersection,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetDifference>(
                    "set_difference",
                    TreeOp::has_set_difference,
                    TreeOp::get_set_difference,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetSymmetricDifference>(
                    "set_symmetric_difference",
                    TreeOp::has_set_symmetric_difference,
                    TreeOp::get_set_symmetric_difference,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetSubsetPath>(
                    "set_subset_path",
                    TreeOp::has_set_subset_path,
                    TreeOp::get_set_subset_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetSubsetSet>(
                    "set_subset_set",
                    TreeOp::has_set_subset_set,
                    TreeOp::get_set_subset_set,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetSupersetPath>(
                    "set_superset_path",
                    TreeOp::has_set_superset_path,
                    TreeOp::get_set_superset_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetSupersetSet>(
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

impl ::std::fmt::Debug for TreeOp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TreeOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TreeOpResult {
    // message fields
    optional_version: ::std::option::Option<u64>,
    // message oneof groups
    result: ::std::option::Option<TreeOpResult_oneof_result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TreeOpResult {}

#[derive(Clone,PartialEq)]
pub enum TreeOpResult_oneof_result {
    ok(bool),
    bool(bool),
    blob(::std::vec::Vec<u8>),
    int(u64),
    set(Set),
    keys(Keys),
    empty(bool),
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
            instance.get(TreeOpResult::new)
        }
    }

    // optional uint64 optional_version = 1;

    pub fn clear_optional_version(&mut self) {
        self.optional_version = ::std::option::Option::None;
    }

    pub fn has_optional_version(&self) -> bool {
        self.optional_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_optional_version(&mut self, v: u64) {
        self.optional_version = ::std::option::Option::Some(v);
    }

    pub fn get_optional_version(&self) -> u64 {
        self.optional_version.unwrap_or(0)
    }

    fn get_optional_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.optional_version
    }

    fn mut_optional_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.optional_version
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

    // optional bool bool = 3;

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

    // optional bytes blob = 4;

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

    // optional uint64 int = 5;

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

    // optional .Set set = 6;

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

    // optional .Keys keys = 7;

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

    // optional bool empty = 8;

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
}

impl ::protobuf::Message for TreeOpResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.optional_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::ok(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::bool(is.read_bool()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::blob(is.read_bytes()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::int(is.read_uint64()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::set(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::keys(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.result = ::std::option::Option::Some(TreeOpResult_oneof_result::empty(is.read_bool()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.optional_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &TreeOpResult_oneof_result::ok(v) => {
                    my_size += 2;
                },
                &TreeOpResult_oneof_result::bool(v) => {
                    my_size += 2;
                },
                &TreeOpResult_oneof_result::blob(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(4, &v);
                },
                &TreeOpResult_oneof_result::int(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TreeOpResult_oneof_result::set(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOpResult_oneof_result::keys(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &TreeOpResult_oneof_result::empty(v) => {
                    my_size += 2;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.optional_version {
            os.write_uint64(1, v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &TreeOpResult_oneof_result::ok(v) => {
                    os.write_bool(2, v)?;
                },
                &TreeOpResult_oneof_result::bool(v) => {
                    os.write_bool(3, v)?;
                },
                &TreeOpResult_oneof_result::blob(ref v) => {
                    os.write_bytes(4, v)?;
                },
                &TreeOpResult_oneof_result::int(v) => {
                    os.write_uint64(5, v)?;
                },
                &TreeOpResult_oneof_result::set(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOpResult_oneof_result::keys(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &TreeOpResult_oneof_result::empty(v) => {
                    os.write_bool(8, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "optional_version",
                    TreeOpResult::get_optional_version_for_reflect,
                    TreeOpResult::mut_optional_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "ok",
                    TreeOpResult::has_ok,
                    TreeOpResult::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "bool",
                    TreeOpResult::has_bool,
                    TreeOpResult::get_bool,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "blob",
                    TreeOpResult::has_blob,
                    TreeOpResult::get_blob,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "int",
                    TreeOpResult::has_int,
                    TreeOpResult::get_int,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Set>(
                    "set",
                    TreeOpResult::has_set,
                    TreeOpResult::get_set,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Keys>(
                    "keys",
                    TreeOpResult::has_keys,
                    TreeOpResult::get_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "empty",
                    TreeOpResult::has_empty,
                    TreeOpResult::get_empty,
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
        self.clear_optional_version();
        self.clear_ok();
        self.clear_bool();
        self.clear_blob();
        self.clear_int();
        self.clear_set();
        self.clear_keys();
        self.clear_empty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TreeOpResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TreeOpResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TreeCasResult {
    // message fields
    results: ::protobuf::RepeatedField<TreeOpResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TreeCasResult::new)
        }
    }

    // repeated .TreeOpResult results = 1;

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    // Param is passed by value, moved
    pub fn set_results(&mut self, v: ::protobuf::RepeatedField<TreeOpResult>) {
        self.results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_results(&mut self) -> &mut ::protobuf::RepeatedField<TreeOpResult> {
        &mut self.results
    }

    // Take field
    pub fn take_results(&mut self) -> ::protobuf::RepeatedField<TreeOpResult> {
        ::std::mem::replace(&mut self.results, ::protobuf::RepeatedField::new())
    }

    pub fn get_results(&self) -> &[TreeOpResult] {
        &self.results
    }

    fn get_results_for_reflect(&self) -> &::protobuf::RepeatedField<TreeOpResult> {
        &self.results
    }

    fn mut_results_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TreeOpResult> {
        &mut self.results
    }
}

impl ::protobuf::Message for TreeCasResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.results)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.results {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TreeOpResult>>(
                    "results",
                    TreeCasResult::get_results_for_reflect,
                    TreeCasResult::mut_results_for_reflect,
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
        self.clear_results();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TreeCasResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TreeCasResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Keys {
    // message fields
    keys: ::protobuf::RepeatedField<Key>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Keys::new)
        }
    }

    // repeated .Key keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<Key>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<Key> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<Key> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[Key] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<Key> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Key> {
        &mut self.keys
    }
}

impl ::protobuf::Message for Keys {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Key>>(
                    "keys",
                    Keys::get_keys_for_reflect,
                    Keys::mut_keys_for_reflect,
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

impl ::std::fmt::Debug for Keys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Keys {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Key {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Key {}

impl Key {
    pub fn new() -> Key {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Key {
        static mut instance: ::protobuf::lazy::Lazy<Key> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Key,
        };
        unsafe {
            instance.get(Key::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required uint64 version = 2;

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

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for Key {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.version.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Key {
    fn new() -> Key {
        Key::new()
    }

    fn descriptor_static(_: ::std::option::Option<Key>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Key::get_name_for_reflect,
                    Key::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    Key::get_version_for_reflect,
                    Key::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Key>(
                    "Key",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Key {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Key {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Guard {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Guard::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required uint64 version = 2;

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

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for Guard {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.version.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    Guard::get_path_for_reflect,
                    Guard::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    Guard::get_version_for_reflect,
                    Guard::mut_version_for_reflect,
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

impl ::std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Guard {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TreeCas {
    // message fields
    guards: ::protobuf::RepeatedField<Guard>,
    tree_ops: ::protobuf::RepeatedField<TreeOp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TreeCas::new)
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

    fn get_guards_for_reflect(&self) -> &::protobuf::RepeatedField<Guard> {
        &self.guards
    }

    fn mut_guards_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Guard> {
        &mut self.guards
    }

    // repeated .TreeOp tree_ops = 2;

    pub fn clear_tree_ops(&mut self) {
        self.tree_ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_tree_ops(&mut self, v: ::protobuf::RepeatedField<TreeOp>) {
        self.tree_ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tree_ops(&mut self) -> &mut ::protobuf::RepeatedField<TreeOp> {
        &mut self.tree_ops
    }

    // Take field
    pub fn take_tree_ops(&mut self) -> ::protobuf::RepeatedField<TreeOp> {
        ::std::mem::replace(&mut self.tree_ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_tree_ops(&self) -> &[TreeOp] {
        &self.tree_ops
    }

    fn get_tree_ops_for_reflect(&self) -> &::protobuf::RepeatedField<TreeOp> {
        &self.tree_ops
    }

    fn mut_tree_ops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TreeOp> {
        &mut self.tree_ops
    }
}

impl ::protobuf::Message for TreeCas {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.guards)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tree_ops)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        for value in &self.tree_ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.guards {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tree_ops {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Guard>>(
                    "guards",
                    TreeCas::get_guards_for_reflect,
                    TreeCas::mut_guards_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TreeOp>>(
                    "tree_ops",
                    TreeCas::get_tree_ops_for_reflect,
                    TreeCas::mut_tree_ops_for_reflect,
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
        self.clear_tree_ops();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TreeCas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TreeCas {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateNode {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    node_type: ::std::option::Option<NodeType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(CreateNode::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required .NodeType node_type = 2;

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
        self.node_type.unwrap_or(NodeType::BLOB)
    }

    fn get_node_type_for_reflect(&self) -> &::std::option::Option<NodeType> {
        &self.node_type
    }

    fn mut_node_type_for_reflect(&mut self) -> &mut ::std::option::Option<NodeType> {
        &mut self.node_type
    }
}

impl ::protobuf::Message for CreateNode {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.node_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.node_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.node_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.node_type {
            os.write_enum(2, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    CreateNode::get_path_for_reflect,
                    CreateNode::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NodeType>>(
                    "node_type",
                    CreateNode::get_node_type_for_reflect,
                    CreateNode::mut_node_type_for_reflect,
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

impl ::std::fmt::Debug for CreateNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteNode {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteNode {}

impl DeleteNode {
    pub fn new() -> DeleteNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteNode {
        static mut instance: ::protobuf::lazy::Lazy<DeleteNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteNode,
        };
        unsafe {
            instance.get(DeleteNode::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for DeleteNode {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteNode {
    fn new() -> DeleteNode {
        DeleteNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    DeleteNode::get_path_for_reflect,
                    DeleteNode::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteNode>(
                    "DeleteNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteNode {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListKeys {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListKeys {}

impl ListKeys {
    pub fn new() -> ListKeys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListKeys {
        static mut instance: ::protobuf::lazy::Lazy<ListKeys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListKeys,
        };
        unsafe {
            instance.get(ListKeys::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for ListKeys {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListKeys {
    fn new() -> ListKeys {
        ListKeys::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListKeys>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    ListKeys::get_path_for_reflect,
                    ListKeys::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListKeys>(
                    "ListKeys",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListKeys {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListKeys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListKeys {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlobPut {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(BlobPut::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required bytes val = 2;

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

    fn get_val_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val
    }
}

impl ::protobuf::Message for BlobPut {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.val.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    BlobPut::get_path_for_reflect,
                    BlobPut::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    BlobPut::get_val_for_reflect,
                    BlobPut::mut_val_for_reflect,
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

impl ::std::fmt::Debug for BlobPut {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlobPut {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlobGet {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlobGet {}

impl BlobGet {
    pub fn new() -> BlobGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlobGet {
        static mut instance: ::protobuf::lazy::Lazy<BlobGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlobGet,
        };
        unsafe {
            instance.get(BlobGet::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for BlobGet {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlobGet {
    fn new() -> BlobGet {
        BlobGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlobGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    BlobGet::get_path_for_reflect,
                    BlobGet::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlobGet>(
                    "BlobGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlobGet {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlobGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlobGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlobSize {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlobSize {}

impl BlobSize {
    pub fn new() -> BlobSize {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlobSize {
        static mut instance: ::protobuf::lazy::Lazy<BlobSize> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlobSize,
        };
        unsafe {
            instance.get(BlobSize::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for BlobSize {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlobSize {
    fn new() -> BlobSize {
        BlobSize::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlobSize>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    BlobSize::get_path_for_reflect,
                    BlobSize::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlobSize>(
                    "BlobSize",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlobSize {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlobSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlobSize {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueuePush {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(QueuePush::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required bytes val = 2;

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

    fn get_val_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val
    }
}

impl ::protobuf::Message for QueuePush {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.val.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    QueuePush::get_path_for_reflect,
                    QueuePush::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    QueuePush::get_val_for_reflect,
                    QueuePush::mut_val_for_reflect,
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

impl ::std::fmt::Debug for QueuePush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueuePush {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueuePop {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueuePop {}

impl QueuePop {
    pub fn new() -> QueuePop {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueuePop {
        static mut instance: ::protobuf::lazy::Lazy<QueuePop> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueuePop,
        };
        unsafe {
            instance.get(QueuePop::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for QueuePop {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueuePop {
    fn new() -> QueuePop {
        QueuePop::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueuePop>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    QueuePop::get_path_for_reflect,
                    QueuePop::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueuePop>(
                    "QueuePop",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueuePop {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueuePop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueuePop {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueueFront {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueueFront {}

impl QueueFront {
    pub fn new() -> QueueFront {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueueFront {
        static mut instance: ::protobuf::lazy::Lazy<QueueFront> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueueFront,
        };
        unsafe {
            instance.get(QueueFront::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for QueueFront {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueueFront {
    fn new() -> QueueFront {
        QueueFront::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueueFront>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    QueueFront::get_path_for_reflect,
                    QueueFront::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueueFront>(
                    "QueueFront",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueueFront {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueueFront {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueueFront {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueueBack {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueueBack {}

impl QueueBack {
    pub fn new() -> QueueBack {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueueBack {
        static mut instance: ::protobuf::lazy::Lazy<QueueBack> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueueBack,
        };
        unsafe {
            instance.get(QueueBack::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for QueueBack {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueueBack {
    fn new() -> QueueBack {
        QueueBack::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueueBack>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    QueueBack::get_path_for_reflect,
                    QueueBack::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueueBack>(
                    "QueueBack",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueueBack {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueueBack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueueBack {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueueLen {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueueLen {}

impl QueueLen {
    pub fn new() -> QueueLen {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueueLen {
        static mut instance: ::protobuf::lazy::Lazy<QueueLen> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueueLen,
        };
        unsafe {
            instance.get(QueueLen::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for QueueLen {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueueLen {
    fn new() -> QueueLen {
        QueueLen::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueueLen>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    QueueLen::get_path_for_reflect,
                    QueueLen::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueueLen>(
                    "QueueLen",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueueLen {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueueLen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueueLen {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetInsert {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetInsert::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required bytes val = 2;

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

    fn get_val_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val
    }
}

impl ::protobuf::Message for SetInsert {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.val.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    SetInsert::get_path_for_reflect,
                    SetInsert::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    SetInsert::get_val_for_reflect,
                    SetInsert::mut_val_for_reflect,
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

impl ::std::fmt::Debug for SetInsert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetInsert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetRemove {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetRemove::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required bytes val = 2;

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

    fn get_val_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val
    }
}

impl ::protobuf::Message for SetRemove {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.val.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    SetRemove::get_path_for_reflect,
                    SetRemove::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    SetRemove::get_val_for_reflect,
                    SetRemove::mut_val_for_reflect,
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

impl ::std::fmt::Debug for SetRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetRemove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetContains {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    val: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetContains::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required bytes val = 2;

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

    fn get_val_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val
    }
}

impl ::protobuf::Message for SetContains {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.val.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.val.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    SetContains::get_path_for_reflect,
                    SetContains::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    SetContains::get_val_for_reflect,
                    SetContains::mut_val_for_reflect,
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

impl ::std::fmt::Debug for SetContains {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetContains {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Set {
    // message fields
    val: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Set::new)
        }
    }

    // repeated bytes val = 1;

    pub fn clear_val(&mut self) {
        self.val.clear();
    }

    // Param is passed by value, moved
    pub fn set_val(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_val(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.val
    }

    // Take field
    pub fn take_val(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.val, ::protobuf::RepeatedField::new())
    }

    pub fn get_val(&self) -> &[::std::vec::Vec<u8>] {
        &self.val
    }

    fn get_val_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.val
    }

    fn mut_val_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.val
    }
}

impl ::protobuf::Message for Set {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.val)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.val {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.val {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val",
                    Set::get_val_for_reflect,
                    Set::mut_val_for_reflect,
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
        self.clear_val();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Set {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Set {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetUnion {
    // message fields
    paths: ::protobuf::RepeatedField<::std::string::String>,
    sets: ::protobuf::RepeatedField<Set>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetUnion::new)
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

    fn get_paths_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.paths
    }

    fn mut_paths_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.paths
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

    fn get_sets_for_reflect(&self) -> &::protobuf::RepeatedField<Set> {
        &self.sets
    }

    fn mut_sets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Set> {
        &mut self.sets
    }
}

impl ::protobuf::Message for SetUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.paths)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sets)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
            os.write_string(1, &v)?;
        };
        for v in &self.sets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "paths",
                    SetUnion::get_paths_for_reflect,
                    SetUnion::mut_paths_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Set>>(
                    "sets",
                    SetUnion::get_sets_for_reflect,
                    SetUnion::mut_sets_for_reflect,
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

impl ::std::fmt::Debug for SetUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetIntersection {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetIntersection::new)
        }
    }

    // required string path1 = 1;

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

    fn get_path1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path1
    }

    fn mut_path1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path1
    }

    // required string path2 = 2;

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

    fn get_path2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path2
    }

    fn mut_path2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path2
    }
}

impl ::protobuf::Message for SetIntersection {
    fn is_initialized(&self) -> bool {
        if self.path1.is_none() {
            return false;
        };
        if self.path2.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path1.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.path2.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.path2.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path1",
                    SetIntersection::get_path1_for_reflect,
                    SetIntersection::mut_path1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path2",
                    SetIntersection::get_path2_for_reflect,
                    SetIntersection::mut_path2_for_reflect,
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

impl ::std::fmt::Debug for SetIntersection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetIntersection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetDifference {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetDifference::new)
        }
    }

    // required string path1 = 1;

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

    fn get_path1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path1
    }

    fn mut_path1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path1
    }

    // required string path2 = 2;

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

    fn get_path2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path2
    }

    fn mut_path2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path2
    }
}

impl ::protobuf::Message for SetDifference {
    fn is_initialized(&self) -> bool {
        if self.path1.is_none() {
            return false;
        };
        if self.path2.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path1.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.path2.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.path2.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path1",
                    SetDifference::get_path1_for_reflect,
                    SetDifference::mut_path1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path2",
                    SetDifference::get_path2_for_reflect,
                    SetDifference::mut_path2_for_reflect,
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

impl ::std::fmt::Debug for SetDifference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetDifference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetSymmetricDifference {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetSymmetricDifference::new)
        }
    }

    // required string path1 = 1;

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

    fn get_path1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path1
    }

    fn mut_path1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path1
    }

    // required string path2 = 2;

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

    fn get_path2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path2
    }

    fn mut_path2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path2
    }
}

impl ::protobuf::Message for SetSymmetricDifference {
    fn is_initialized(&self) -> bool {
        if self.path1.is_none() {
            return false;
        };
        if self.path2.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path1.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.path2.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.path2.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path1",
                    SetSymmetricDifference::get_path1_for_reflect,
                    SetSymmetricDifference::mut_path1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path2",
                    SetSymmetricDifference::get_path2_for_reflect,
                    SetSymmetricDifference::mut_path2_for_reflect,
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

impl ::std::fmt::Debug for SetSymmetricDifference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetSymmetricDifference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetSubsetPath {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetSubsetPath::new)
        }
    }

    // required string path1 = 1;

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

    fn get_path1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path1
    }

    fn mut_path1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path1
    }

    // required string path2 = 2;

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

    fn get_path2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path2
    }

    fn mut_path2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path2
    }
}

impl ::protobuf::Message for SetSubsetPath {
    fn is_initialized(&self) -> bool {
        if self.path1.is_none() {
            return false;
        };
        if self.path2.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path1.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.path2.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.path2.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path1",
                    SetSubsetPath::get_path1_for_reflect,
                    SetSubsetPath::mut_path1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path2",
                    SetSubsetPath::get_path2_for_reflect,
                    SetSubsetPath::mut_path2_for_reflect,
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

impl ::std::fmt::Debug for SetSubsetPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetSubsetPath {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetSubsetSet {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    set: ::protobuf::SingularPtrField<Set>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetSubsetSet::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required .Set set = 2;

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

    fn get_set_for_reflect(&self) -> &::protobuf::SingularPtrField<Set> {
        &self.set
    }

    fn mut_set_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Set> {
        &mut self.set
    }
}

impl ::protobuf::Message for SetSubsetSet {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.set.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.set)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.set.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.set.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    SetSubsetSet::get_path_for_reflect,
                    SetSubsetSet::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Set>>(
                    "set",
                    SetSubsetSet::get_set_for_reflect,
                    SetSubsetSet::mut_set_for_reflect,
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

impl ::std::fmt::Debug for SetSubsetSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetSubsetSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetSupersetPath {
    // message fields
    path1: ::protobuf::SingularField<::std::string::String>,
    path2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetSupersetPath::new)
        }
    }

    // required string path1 = 1;

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

    fn get_path1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path1
    }

    fn mut_path1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path1
    }

    // required string path2 = 2;

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

    fn get_path2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path2
    }

    fn mut_path2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path2
    }
}

impl ::protobuf::Message for SetSupersetPath {
    fn is_initialized(&self) -> bool {
        if self.path1.is_none() {
            return false;
        };
        if self.path2.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path2)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path1.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.path2.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path1.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.path2.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path1",
                    SetSupersetPath::get_path1_for_reflect,
                    SetSupersetPath::mut_path1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path2",
                    SetSupersetPath::get_path2_for_reflect,
                    SetSupersetPath::mut_path2_for_reflect,
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

impl ::std::fmt::Debug for SetSupersetPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetSupersetPath {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetSupersetSet {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    set: ::protobuf::SingularPtrField<Set>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SetSupersetSet::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required .Set set = 2;

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

    fn get_set_for_reflect(&self) -> &::protobuf::SingularPtrField<Set> {
        &self.set
    }

    fn mut_set_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Set> {
        &mut self.set
    }
}

impl ::protobuf::Message for SetSupersetSet {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.set.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.set)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.set.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.set.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    SetSupersetSet::get_path_for_reflect,
                    SetSupersetSet::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Set>>(
                    "set",
                    SetSupersetSet::get_set_for_reflect,
                    SetSupersetSet::mut_set_for_reflect,
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

impl ::std::fmt::Debug for SetSupersetSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetSupersetSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApiResponse {
    // message oneof groups
    response: ::std::option::Option<ApiResponse_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiResponse {}

#[derive(Clone,PartialEq)]
pub enum ApiResponse_oneof_response {
    consensus_reply(ConsensusReply),
    namespaces(Namespaces),
    client_registration(ClientRegistration),
    redirect(Redirect),
    retry(Retry),
    timeout(bool),
    error(ApiError),
    unknown_namespace(bool),
}

impl ApiResponse {
    pub fn new() -> ApiResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiResponse {
        static mut instance: ::protobuf::lazy::Lazy<ApiResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiResponse,
        };
        unsafe {
            instance.get(ApiResponse::new)
        }
    }

    // optional .ConsensusReply consensus_reply = 1;

    pub fn clear_consensus_reply(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_consensus_reply(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_consensus_reply(&mut self, v: ConsensusReply) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(v))
    }

    // Mutable pointer to the field.
    pub fn mut_consensus_reply(&mut self) -> &mut ConsensusReply {
        if let ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(ConsensusReply::new()));
        }
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_consensus_reply(&mut self) -> ConsensusReply {
        if self.has_consensus_reply() {
            match self.response.take() {
                ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(v)) => v,
                _ => panic!(),
            }
        } else {
            ConsensusReply::new()
        }
    }

    pub fn get_consensus_reply(&self) -> &ConsensusReply {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(ref v)) => v,
            _ => ConsensusReply::default_instance(),
        }
    }

    // optional .Namespaces namespaces = 2;

    pub fn clear_namespaces(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_namespaces(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_namespaces(&mut self, v: Namespaces) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(v))
    }

    // Mutable pointer to the field.
    pub fn mut_namespaces(&mut self) -> &mut Namespaces {
        if let ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(Namespaces::new()));
        }
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_namespaces(&mut self) -> Namespaces {
        if self.has_namespaces() {
            match self.response.take() {
                ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(v)) => v,
                _ => panic!(),
            }
        } else {
            Namespaces::new()
        }
    }

    pub fn get_namespaces(&self) -> &Namespaces {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(ref v)) => v,
            _ => Namespaces::default_instance(),
        }
    }

    // optional .ClientRegistration client_registration = 3;

    pub fn clear_client_registration(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_client_registration(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_client_registration(&mut self, v: ClientRegistration) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(v))
    }

    // Mutable pointer to the field.
    pub fn mut_client_registration(&mut self) -> &mut ClientRegistration {
        if let ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(ClientRegistration::new()));
        }
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_client_registration(&mut self) -> ClientRegistration {
        if self.has_client_registration() {
            match self.response.take() {
                ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(v)) => v,
                _ => panic!(),
            }
        } else {
            ClientRegistration::new()
        }
    }

    pub fn get_client_registration(&self) -> &ClientRegistration {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(ref v)) => v,
            _ => ClientRegistration::default_instance(),
        }
    }

    // optional .Redirect redirect = 4;

    pub fn clear_redirect(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_redirect(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::redirect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_redirect(&mut self, v: Redirect) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::redirect(v))
    }

    // Mutable pointer to the field.
    pub fn mut_redirect(&mut self) -> &mut Redirect {
        if let ::std::option::Option::Some(ApiResponse_oneof_response::redirect(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ApiResponse_oneof_response::redirect(Redirect::new()));
        }
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::redirect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_redirect(&mut self) -> Redirect {
        if self.has_redirect() {
            match self.response.take() {
                ::std::option::Option::Some(ApiResponse_oneof_response::redirect(v)) => v,
                _ => panic!(),
            }
        } else {
            Redirect::new()
        }
    }

    pub fn get_redirect(&self) -> &Redirect {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::redirect(ref v)) => v,
            _ => Redirect::default_instance(),
        }
    }

    // optional .Retry retry = 5;

    pub fn clear_retry(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_retry(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::retry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_retry(&mut self, v: Retry) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::retry(v))
    }

    // Mutable pointer to the field.
    pub fn mut_retry(&mut self) -> &mut Retry {
        if let ::std::option::Option::Some(ApiResponse_oneof_response::retry(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ApiResponse_oneof_response::retry(Retry::new()));
        }
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::retry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_retry(&mut self) -> Retry {
        if self.has_retry() {
            match self.response.take() {
                ::std::option::Option::Some(ApiResponse_oneof_response::retry(v)) => v,
                _ => panic!(),
            }
        } else {
            Retry::new()
        }
    }

    pub fn get_retry(&self) -> &Retry {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::retry(ref v)) => v,
            _ => Retry::default_instance(),
        }
    }

    // optional bool timeout = 6;

    pub fn clear_timeout(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::timeout(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: bool) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::timeout(v))
    }

    pub fn get_timeout(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::timeout(v)) => v,
            _ => false,
        }
    }

    // optional .ApiError error = 7;

    pub fn clear_error(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ApiError) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ApiError {
        if let ::std::option::Option::Some(ApiResponse_oneof_response::error(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(ApiResponse_oneof_response::error(ApiError::new()));
        }
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ApiError {
        if self.has_error() {
            match self.response.take() {
                ::std::option::Option::Some(ApiResponse_oneof_response::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ApiError::new()
        }
    }

    pub fn get_error(&self) -> &ApiError {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::error(ref v)) => v,
            _ => ApiError::default_instance(),
        }
    }

    // optional bool unknown_namespace = 8;

    pub fn clear_unknown_namespace(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_unknown_namespace(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::unknown_namespace(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unknown_namespace(&mut self, v: bool) {
        self.response = ::std::option::Option::Some(ApiResponse_oneof_response::unknown_namespace(v))
    }

    pub fn get_unknown_namespace(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(ApiResponse_oneof_response::unknown_namespace(v)) => v,
            _ => false,
        }
    }
}

impl ::protobuf::Message for ApiResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::consensus_reply(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::namespaces(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::client_registration(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::redirect(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::retry(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::timeout(is.read_bool()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::error(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.response = ::std::option::Option::Some(ApiResponse_oneof_response::unknown_namespace(is.read_bool()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &ApiResponse_oneof_response::consensus_reply(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiResponse_oneof_response::namespaces(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiResponse_oneof_response::client_registration(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiResponse_oneof_response::redirect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiResponse_oneof_response::retry(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiResponse_oneof_response::timeout(v) => {
                    my_size += 2;
                },
                &ApiResponse_oneof_response::error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiResponse_oneof_response::unknown_namespace(v) => {
                    my_size += 2;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &ApiResponse_oneof_response::consensus_reply(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiResponse_oneof_response::namespaces(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiResponse_oneof_response::client_registration(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiResponse_oneof_response::redirect(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiResponse_oneof_response::retry(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiResponse_oneof_response::timeout(v) => {
                    os.write_bool(6, v)?;
                },
                &ApiResponse_oneof_response::error(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiResponse_oneof_response::unknown_namespace(v) => {
                    os.write_bool(8, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiResponse {
    fn new() -> ApiResponse {
        ApiResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConsensusReply>(
                    "consensus_reply",
                    ApiResponse::has_consensus_reply,
                    ApiResponse::get_consensus_reply,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Namespaces>(
                    "namespaces",
                    ApiResponse::has_namespaces,
                    ApiResponse::get_namespaces,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ClientRegistration>(
                    "client_registration",
                    ApiResponse::has_client_registration,
                    ApiResponse::get_client_registration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Redirect>(
                    "redirect",
                    ApiResponse::has_redirect,
                    ApiResponse::get_redirect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Retry>(
                    "retry",
                    ApiResponse::has_retry,
                    ApiResponse::get_retry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "timeout",
                    ApiResponse::has_timeout,
                    ApiResponse::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ApiError>(
                    "error",
                    ApiResponse::has_error,
                    ApiResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "unknown_namespace",
                    ApiResponse::has_unknown_namespace,
                    ApiResponse::get_unknown_namespace,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiResponse>(
                    "ApiResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiResponse {
    fn clear(&mut self) {
        self.clear_consensus_reply();
        self.clear_namespaces();
        self.clear_client_registration();
        self.clear_redirect();
        self.clear_retry();
        self.clear_timeout();
        self.clear_error();
        self.clear_unknown_namespace();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Namespaces {
    // message fields
    ids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Namespaces::new)
        }
    }

    // repeated string ids = 1;

    pub fn clear_ids(&mut self) {
        self.ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ids
    }

    // Take field
    pub fn take_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_ids(&self) -> &[::std::string::String] {
        &self.ids
    }

    fn get_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.ids
    }

    fn mut_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ids
    }
}

impl ::protobuf::Message for Namespaces {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ids {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ids {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ids",
                    Namespaces::get_ids_for_reflect,
                    Namespaces::mut_ids_for_reflect,
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
        self.clear_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Namespaces {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Namespaces {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsensusReply {
    // message fields
    epoch: ::std::option::Option<u64>,
    view: ::std::option::Option<u64>,
    request_num: ::std::option::Option<u64>,
    // message oneof groups
    value: ::std::option::Option<ConsensusReply_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConsensusReply {}

#[derive(Clone,PartialEq)]
pub enum ConsensusReply_oneof_value {
    ok(bool),
    tree_op_result(TreeOpResult),
    tree_cas_result(TreeCasResult),
    path(::std::string::String),
    error(ApiError),
}

impl ConsensusReply {
    pub fn new() -> ConsensusReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConsensusReply {
        static mut instance: ::protobuf::lazy::Lazy<ConsensusReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConsensusReply,
        };
        unsafe {
            instance.get(ConsensusReply::new)
        }
    }

    // required uint64 epoch = 1;

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

    fn get_epoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.epoch
    }

    fn mut_epoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.epoch
    }

    // required uint64 view = 2;

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

    fn get_view_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.view
    }

    fn mut_view_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.view
    }

    // required uint64 request_num = 3;

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

    fn get_request_num_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.request_num
    }

    fn mut_request_num_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.request_num
    }

    // optional bool ok = 4;

    pub fn clear_ok(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_ok(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::ok(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ok(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::ok(v))
    }

    pub fn get_ok(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::ok(v)) => v,
            _ => false,
        }
    }

    // optional .TreeOpResult tree_op_result = 5;

    pub fn clear_tree_op_result(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_tree_op_result(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_op_result(&mut self, v: TreeOpResult) {
        self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tree_op_result(&mut self) -> &mut TreeOpResult {
        if let ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(TreeOpResult::new()));
        }
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_op_result(&mut self) -> TreeOpResult {
        if self.has_tree_op_result() {
            match self.value.take() {
                ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeOpResult::new()
        }
    }

    pub fn get_tree_op_result(&self) -> &TreeOpResult {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(ref v)) => v,
            _ => TreeOpResult::default_instance(),
        }
    }

    // optional .TreeCasResult tree_cas_result = 6;

    pub fn clear_tree_cas_result(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_tree_cas_result(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tree_cas_result(&mut self, v: TreeCasResult) {
        self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tree_cas_result(&mut self) -> &mut TreeCasResult {
        if let ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(TreeCasResult::new()));
        }
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tree_cas_result(&mut self) -> TreeCasResult {
        if self.has_tree_cas_result() {
            match self.value.take() {
                ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(v)) => v,
                _ => panic!(),
            }
        } else {
            TreeCasResult::new()
        }
    }

    pub fn get_tree_cas_result(&self) -> &TreeCasResult {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(ref v)) => v,
            _ => TreeCasResult::default_instance(),
        }
    }

    // optional string path = 7;

    pub fn clear_path(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_path(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::path(v))
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ConsensusReply_oneof_value::path(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::path(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        if self.has_path() {
            match self.value.take() {
                ::std::option::Option::Some(ConsensusReply_oneof_value::path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_path(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::path(ref v)) => v,
            _ => "",
        }
    }

    // optional .ApiError error = 8;

    pub fn clear_error(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ApiError) {
        self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ApiError {
        if let ::std::option::Option::Some(ConsensusReply_oneof_value::error(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::error(ApiError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> ApiError {
        if self.has_error() {
            match self.value.take() {
                ::std::option::Option::Some(ConsensusReply_oneof_value::error(v)) => v,
                _ => panic!(),
            }
        } else {
            ApiError::new()
        }
    }

    pub fn get_error(&self) -> &ApiError {
        match self.value {
            ::std::option::Option::Some(ConsensusReply_oneof_value::error(ref v)) => v,
            _ => ApiError::default_instance(),
        }
    }
}

impl ::protobuf::Message for ConsensusReply {
    fn is_initialized(&self) -> bool {
        if self.epoch.is_none() {
            return false;
        };
        if self.view.is_none() {
            return false;
        };
        if self.request_num.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.view = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.request_num = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::ok(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::tree_op_result(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::tree_cas_result(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::path(is.read_string()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.value = ::std::option::Option::Some(ConsensusReply_oneof_value::error(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.epoch {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.view {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.request_num {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &ConsensusReply_oneof_value::ok(v) => {
                    my_size += 2;
                },
                &ConsensusReply_oneof_value::tree_op_result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ConsensusReply_oneof_value::tree_cas_result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ConsensusReply_oneof_value::path(ref v) => {
                    my_size += ::protobuf::rt::string_size(7, &v);
                },
                &ConsensusReply_oneof_value::error(ref v) => {
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
        if let Some(v) = self.epoch {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.view {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.request_num {
            os.write_uint64(3, v)?;
        };
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &ConsensusReply_oneof_value::ok(v) => {
                    os.write_bool(4, v)?;
                },
                &ConsensusReply_oneof_value::tree_op_result(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ConsensusReply_oneof_value::tree_cas_result(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ConsensusReply_oneof_value::path(ref v) => {
                    os.write_string(7, v)?;
                },
                &ConsensusReply_oneof_value::error(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConsensusReply {
    fn new() -> ConsensusReply {
        ConsensusReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConsensusReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "epoch",
                    ConsensusReply::get_epoch_for_reflect,
                    ConsensusReply::mut_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "view",
                    ConsensusReply::get_view_for_reflect,
                    ConsensusReply::mut_view_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "request_num",
                    ConsensusReply::get_request_num_for_reflect,
                    ConsensusReply::mut_request_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "ok",
                    ConsensusReply::has_ok,
                    ConsensusReply::get_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TreeOpResult>(
                    "tree_op_result",
                    ConsensusReply::has_tree_op_result,
                    ConsensusReply::get_tree_op_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TreeCasResult>(
                    "tree_cas_result",
                    ConsensusReply::has_tree_cas_result,
                    ConsensusReply::get_tree_cas_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "path",
                    ConsensusReply::has_path,
                    ConsensusReply::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ApiError>(
                    "error",
                    ConsensusReply::has_error,
                    ConsensusReply::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConsensusReply>(
                    "ConsensusReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConsensusReply {
    fn clear(&mut self) {
        self.clear_epoch();
        self.clear_view();
        self.clear_request_num();
        self.clear_ok();
        self.clear_tree_op_result();
        self.clear_tree_cas_result();
        self.clear_path();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsensusReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsensusReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientRegistration {
    // message fields
    primary: ::protobuf::SingularPtrField<ApiPid>,
    new_registration: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ClientRegistration::new)
        }
    }

    // required .ApiPid primary = 1;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: ApiPid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut ApiPid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> ApiPid {
        self.primary.take().unwrap_or_else(|| ApiPid::new())
    }

    pub fn get_primary(&self) -> &ApiPid {
        self.primary.as_ref().unwrap_or_else(|| ApiPid::default_instance())
    }

    fn get_primary_for_reflect(&self) -> &::protobuf::SingularPtrField<ApiPid> {
        &self.primary
    }

    fn mut_primary_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApiPid> {
        &mut self.primary
    }

    // required bool new_registration = 2;

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

    fn get_new_registration_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.new_registration
    }

    fn mut_new_registration_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.new_registration
    }
}

impl ::protobuf::Message for ClientRegistration {
    fn is_initialized(&self) -> bool {
        if self.primary.is_none() {
            return false;
        };
        if self.new_registration.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.new_registration = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.primary.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.new_registration {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.new_registration {
            os.write_bool(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApiPid>>(
                    "primary",
                    ClientRegistration::get_primary_for_reflect,
                    ClientRegistration::mut_primary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "new_registration",
                    ClientRegistration::get_new_registration_for_reflect,
                    ClientRegistration::mut_new_registration_for_reflect,
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

impl ::std::fmt::Debug for ClientRegistration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientRegistration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Redirect {
    // message fields
    primary: ::protobuf::SingularPtrField<ApiPid>,
    api_addr: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Redirect::new)
        }
    }

    // required .ApiPid primary = 1;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: ApiPid) {
        self.primary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut ApiPid {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> ApiPid {
        self.primary.take().unwrap_or_else(|| ApiPid::new())
    }

    pub fn get_primary(&self) -> &ApiPid {
        self.primary.as_ref().unwrap_or_else(|| ApiPid::default_instance())
    }

    fn get_primary_for_reflect(&self) -> &::protobuf::SingularPtrField<ApiPid> {
        &self.primary
    }

    fn mut_primary_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApiPid> {
        &mut self.primary
    }

    // required string api_addr = 2;

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

    fn get_api_addr_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.api_addr
    }

    fn mut_api_addr_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.api_addr
    }
}

impl ::protobuf::Message for Redirect {
    fn is_initialized(&self) -> bool {
        if self.primary.is_none() {
            return false;
        };
        if self.api_addr.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.primary)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.api_addr)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.primary.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.api_addr.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.api_addr.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApiPid>>(
                    "primary",
                    Redirect::get_primary_for_reflect,
                    Redirect::mut_primary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "api_addr",
                    Redirect::get_api_addr_for_reflect,
                    Redirect::mut_api_addr_for_reflect,
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

impl ::std::fmt::Debug for Redirect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Redirect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Retry {
    // message fields
    milliseconds: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Retry {}

impl Retry {
    pub fn new() -> Retry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Retry {
        static mut instance: ::protobuf::lazy::Lazy<Retry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Retry,
        };
        unsafe {
            instance.get(Retry::new)
        }
    }

    // required uint64 milliseconds = 1;

    pub fn clear_milliseconds(&mut self) {
        self.milliseconds = ::std::option::Option::None;
    }

    pub fn has_milliseconds(&self) -> bool {
        self.milliseconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_milliseconds(&mut self, v: u64) {
        self.milliseconds = ::std::option::Option::Some(v);
    }

    pub fn get_milliseconds(&self) -> u64 {
        self.milliseconds.unwrap_or(0)
    }

    fn get_milliseconds_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.milliseconds
    }

    fn mut_milliseconds_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.milliseconds
    }
}

impl ::protobuf::Message for Retry {
    fn is_initialized(&self) -> bool {
        if self.milliseconds.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.milliseconds = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.milliseconds {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.milliseconds {
            os.write_uint64(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Retry {
    fn new() -> Retry {
        Retry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Retry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "milliseconds",
                    Retry::get_milliseconds_for_reflect,
                    Retry::mut_milliseconds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Retry>(
                    "Retry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Retry {
    fn clear(&mut self) {
        self.clear_milliseconds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Retry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Retry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApiError {
    // message oneof groups
    error: ::std::option::Option<ApiError_oneof_error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiError {}

#[derive(Clone,PartialEq)]
pub enum ApiError_oneof_error {
    not_found(NotFound),
    already_exists(AlreadyExists),
    does_not_exist(DoesNotExist),
    wrong_type(WrongType),
    path_must_end_in_directory(PathMustEndInDirectory),
    path_must_be_absolute(PathMustBeAbsolute),
    cas_failed(CasFailed),
    bad_format(BadFormat),
    io(Io),
    encoding(EncodingError),
    invalid_cas(InvalidCas),
    msg(::std::string::String),
    cannot_delete_root(bool),
    invalid_msg(bool),
    timeout(bool),
    not_enough_replicas(bool),
    bad_epoch(bool),
}

impl ApiError {
    pub fn new() -> ApiError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiError {
        static mut instance: ::protobuf::lazy::Lazy<ApiError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiError,
        };
        unsafe {
            instance.get(ApiError::new)
        }
    }

    // optional .NotFound not_found = 1;

    pub fn clear_not_found(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_not_found(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::not_found(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_not_found(&mut self, v: NotFound) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::not_found(v))
    }

    // Mutable pointer to the field.
    pub fn mut_not_found(&mut self) -> &mut NotFound {
        if let ::std::option::Option::Some(ApiError_oneof_error::not_found(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::not_found(NotFound::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::not_found(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_not_found(&mut self) -> NotFound {
        if self.has_not_found() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::not_found(v)) => v,
                _ => panic!(),
            }
        } else {
            NotFound::new()
        }
    }

    pub fn get_not_found(&self) -> &NotFound {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::not_found(ref v)) => v,
            _ => NotFound::default_instance(),
        }
    }

    // optional .AlreadyExists already_exists = 2;

    pub fn clear_already_exists(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_already_exists(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::already_exists(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_already_exists(&mut self, v: AlreadyExists) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::already_exists(v))
    }

    // Mutable pointer to the field.
    pub fn mut_already_exists(&mut self) -> &mut AlreadyExists {
        if let ::std::option::Option::Some(ApiError_oneof_error::already_exists(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::already_exists(AlreadyExists::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::already_exists(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_already_exists(&mut self) -> AlreadyExists {
        if self.has_already_exists() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::already_exists(v)) => v,
                _ => panic!(),
            }
        } else {
            AlreadyExists::new()
        }
    }

    pub fn get_already_exists(&self) -> &AlreadyExists {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::already_exists(ref v)) => v,
            _ => AlreadyExists::default_instance(),
        }
    }

    // optional .DoesNotExist does_not_exist = 3;

    pub fn clear_does_not_exist(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_does_not_exist(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_does_not_exist(&mut self, v: DoesNotExist) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(v))
    }

    // Mutable pointer to the field.
    pub fn mut_does_not_exist(&mut self) -> &mut DoesNotExist {
        if let ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(DoesNotExist::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_does_not_exist(&mut self) -> DoesNotExist {
        if self.has_does_not_exist() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(v)) => v,
                _ => panic!(),
            }
        } else {
            DoesNotExist::new()
        }
    }

    pub fn get_does_not_exist(&self) -> &DoesNotExist {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(ref v)) => v,
            _ => DoesNotExist::default_instance(),
        }
    }

    // optional .WrongType wrong_type = 4;

    pub fn clear_wrong_type(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_wrong_type(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::wrong_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_wrong_type(&mut self, v: WrongType) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::wrong_type(v))
    }

    // Mutable pointer to the field.
    pub fn mut_wrong_type(&mut self) -> &mut WrongType {
        if let ::std::option::Option::Some(ApiError_oneof_error::wrong_type(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::wrong_type(WrongType::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::wrong_type(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_wrong_type(&mut self) -> WrongType {
        if self.has_wrong_type() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::wrong_type(v)) => v,
                _ => panic!(),
            }
        } else {
            WrongType::new()
        }
    }

    pub fn get_wrong_type(&self) -> &WrongType {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::wrong_type(ref v)) => v,
            _ => WrongType::default_instance(),
        }
    }

    // optional .PathMustEndInDirectory path_must_end_in_directory = 5;

    pub fn clear_path_must_end_in_directory(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_path_must_end_in_directory(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path_must_end_in_directory(&mut self, v: PathMustEndInDirectory) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(v))
    }

    // Mutable pointer to the field.
    pub fn mut_path_must_end_in_directory(&mut self) -> &mut PathMustEndInDirectory {
        if let ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(PathMustEndInDirectory::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path_must_end_in_directory(&mut self) -> PathMustEndInDirectory {
        if self.has_path_must_end_in_directory() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(v)) => v,
                _ => panic!(),
            }
        } else {
            PathMustEndInDirectory::new()
        }
    }

    pub fn get_path_must_end_in_directory(&self) -> &PathMustEndInDirectory {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(ref v)) => v,
            _ => PathMustEndInDirectory::default_instance(),
        }
    }

    // optional .PathMustBeAbsolute path_must_be_absolute = 6;

    pub fn clear_path_must_be_absolute(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_path_must_be_absolute(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path_must_be_absolute(&mut self, v: PathMustBeAbsolute) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(v))
    }

    // Mutable pointer to the field.
    pub fn mut_path_must_be_absolute(&mut self) -> &mut PathMustBeAbsolute {
        if let ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(PathMustBeAbsolute::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path_must_be_absolute(&mut self) -> PathMustBeAbsolute {
        if self.has_path_must_be_absolute() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(v)) => v,
                _ => panic!(),
            }
        } else {
            PathMustBeAbsolute::new()
        }
    }

    pub fn get_path_must_be_absolute(&self) -> &PathMustBeAbsolute {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(ref v)) => v,
            _ => PathMustBeAbsolute::default_instance(),
        }
    }

    // optional .CasFailed cas_failed = 7;

    pub fn clear_cas_failed(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_cas_failed(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::cas_failed(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cas_failed(&mut self, v: CasFailed) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::cas_failed(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cas_failed(&mut self) -> &mut CasFailed {
        if let ::std::option::Option::Some(ApiError_oneof_error::cas_failed(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::cas_failed(CasFailed::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::cas_failed(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cas_failed(&mut self) -> CasFailed {
        if self.has_cas_failed() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::cas_failed(v)) => v,
                _ => panic!(),
            }
        } else {
            CasFailed::new()
        }
    }

    pub fn get_cas_failed(&self) -> &CasFailed {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::cas_failed(ref v)) => v,
            _ => CasFailed::default_instance(),
        }
    }

    // optional .BadFormat bad_format = 8;

    pub fn clear_bad_format(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_bad_format(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::bad_format(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bad_format(&mut self, v: BadFormat) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::bad_format(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bad_format(&mut self) -> &mut BadFormat {
        if let ::std::option::Option::Some(ApiError_oneof_error::bad_format(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::bad_format(BadFormat::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::bad_format(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bad_format(&mut self) -> BadFormat {
        if self.has_bad_format() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::bad_format(v)) => v,
                _ => panic!(),
            }
        } else {
            BadFormat::new()
        }
    }

    pub fn get_bad_format(&self) -> &BadFormat {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::bad_format(ref v)) => v,
            _ => BadFormat::default_instance(),
        }
    }

    // optional .Io io = 9;

    pub fn clear_io(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_io(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::io(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_io(&mut self, v: Io) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::io(v))
    }

    // Mutable pointer to the field.
    pub fn mut_io(&mut self) -> &mut Io {
        if let ::std::option::Option::Some(ApiError_oneof_error::io(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::io(Io::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::io(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_io(&mut self) -> Io {
        if self.has_io() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::io(v)) => v,
                _ => panic!(),
            }
        } else {
            Io::new()
        }
    }

    pub fn get_io(&self) -> &Io {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::io(ref v)) => v,
            _ => Io::default_instance(),
        }
    }

    // optional .EncodingError encoding = 10;

    pub fn clear_encoding(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_encoding(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::encoding(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_encoding(&mut self, v: EncodingError) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::encoding(v))
    }

    // Mutable pointer to the field.
    pub fn mut_encoding(&mut self) -> &mut EncodingError {
        if let ::std::option::Option::Some(ApiError_oneof_error::encoding(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::encoding(EncodingError::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::encoding(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_encoding(&mut self) -> EncodingError {
        if self.has_encoding() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::encoding(v)) => v,
                _ => panic!(),
            }
        } else {
            EncodingError::new()
        }
    }

    pub fn get_encoding(&self) -> &EncodingError {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::encoding(ref v)) => v,
            _ => EncodingError::default_instance(),
        }
    }

    // optional .InvalidCas invalid_cas = 11;

    pub fn clear_invalid_cas(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_invalid_cas(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_invalid_cas(&mut self, v: InvalidCas) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(v))
    }

    // Mutable pointer to the field.
    pub fn mut_invalid_cas(&mut self) -> &mut InvalidCas {
        if let ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(InvalidCas::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_invalid_cas(&mut self) -> InvalidCas {
        if self.has_invalid_cas() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(v)) => v,
                _ => panic!(),
            }
        } else {
            InvalidCas::new()
        }
    }

    pub fn get_invalid_cas(&self) -> &InvalidCas {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(ref v)) => v,
            _ => InvalidCas::default_instance(),
        }
    }

    // optional string msg = 12;

    pub fn clear_msg(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_msg(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::msg(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::msg(v))
    }

    // Mutable pointer to the field.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ApiError_oneof_error::msg(_)) = self.error {
        } else {
            self.error = ::std::option::Option::Some(ApiError_oneof_error::msg(::std::string::String::new()));
        }
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::msg(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        if self.has_msg() {
            match self.error.take() {
                ::std::option::Option::Some(ApiError_oneof_error::msg(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_msg(&self) -> &str {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::msg(ref v)) => v,
            _ => "",
        }
    }

    // optional bool cannot_delete_root = 13;

    pub fn clear_cannot_delete_root(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_cannot_delete_root(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::cannot_delete_root(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cannot_delete_root(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::cannot_delete_root(v))
    }

    pub fn get_cannot_delete_root(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::cannot_delete_root(v)) => v,
            _ => false,
        }
    }

    // optional bool invalid_msg = 14;

    pub fn clear_invalid_msg(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_invalid_msg(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::invalid_msg(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_invalid_msg(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::invalid_msg(v))
    }

    pub fn get_invalid_msg(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::invalid_msg(v)) => v,
            _ => false,
        }
    }

    // optional bool timeout = 15;

    pub fn clear_timeout(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::timeout(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::timeout(v))
    }

    pub fn get_timeout(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::timeout(v)) => v,
            _ => false,
        }
    }

    // optional bool not_enough_replicas = 16;

    pub fn clear_not_enough_replicas(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_not_enough_replicas(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::not_enough_replicas(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_not_enough_replicas(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::not_enough_replicas(v))
    }

    pub fn get_not_enough_replicas(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::not_enough_replicas(v)) => v,
            _ => false,
        }
    }

    // optional bool bad_epoch = 17;

    pub fn clear_bad_epoch(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_bad_epoch(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::bad_epoch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bad_epoch(&mut self, v: bool) {
        self.error = ::std::option::Option::Some(ApiError_oneof_error::bad_epoch(v))
    }

    pub fn get_bad_epoch(&self) -> bool {
        match self.error {
            ::std::option::Option::Some(ApiError_oneof_error::bad_epoch(v)) => v,
            _ => false,
        }
    }
}

impl ::protobuf::Message for ApiError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::not_found(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::already_exists(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::does_not_exist(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::wrong_type(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::path_must_end_in_directory(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::path_must_be_absolute(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::cas_failed(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::bad_format(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::io(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::encoding(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::invalid_cas(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::msg(is.read_string()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::cannot_delete_root(is.read_bool()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::invalid_msg(is.read_bool()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::timeout(is.read_bool()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::not_enough_replicas(is.read_bool()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.error = ::std::option::Option::Some(ApiError_oneof_error::bad_epoch(is.read_bool()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
                &ApiError_oneof_error::not_found(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::already_exists(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::does_not_exist(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::wrong_type(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::path_must_end_in_directory(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::path_must_be_absolute(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::cas_failed(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::bad_format(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::io(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::encoding(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::invalid_cas(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ApiError_oneof_error::msg(ref v) => {
                    my_size += ::protobuf::rt::string_size(12, &v);
                },
                &ApiError_oneof_error::cannot_delete_root(v) => {
                    my_size += 2;
                },
                &ApiError_oneof_error::invalid_msg(v) => {
                    my_size += 2;
                },
                &ApiError_oneof_error::timeout(v) => {
                    my_size += 2;
                },
                &ApiError_oneof_error::not_enough_replicas(v) => {
                    my_size += 3;
                },
                &ApiError_oneof_error::bad_epoch(v) => {
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
                &ApiError_oneof_error::not_found(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::already_exists(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::does_not_exist(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::wrong_type(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::path_must_end_in_directory(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::path_must_be_absolute(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::cas_failed(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::bad_format(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::io(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::encoding(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::invalid_cas(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ApiError_oneof_error::msg(ref v) => {
                    os.write_string(12, v)?;
                },
                &ApiError_oneof_error::cannot_delete_root(v) => {
                    os.write_bool(13, v)?;
                },
                &ApiError_oneof_error::invalid_msg(v) => {
                    os.write_bool(14, v)?;
                },
                &ApiError_oneof_error::timeout(v) => {
                    os.write_bool(15, v)?;
                },
                &ApiError_oneof_error::not_enough_replicas(v) => {
                    os.write_bool(16, v)?;
                },
                &ApiError_oneof_error::bad_epoch(v) => {
                    os.write_bool(17, v)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiError {
    fn new() -> ApiError {
        ApiError::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, NotFound>(
                    "not_found",
                    ApiError::has_not_found,
                    ApiError::get_not_found,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, AlreadyExists>(
                    "already_exists",
                    ApiError::has_already_exists,
                    ApiError::get_already_exists,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DoesNotExist>(
                    "does_not_exist",
                    ApiError::has_does_not_exist,
                    ApiError::get_does_not_exist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WrongType>(
                    "wrong_type",
                    ApiError::has_wrong_type,
                    ApiError::get_wrong_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PathMustEndInDirectory>(
                    "path_must_end_in_directory",
                    ApiError::has_path_must_end_in_directory,
                    ApiError::get_path_must_end_in_directory,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PathMustBeAbsolute>(
                    "path_must_be_absolute",
                    ApiError::has_path_must_be_absolute,
                    ApiError::get_path_must_be_absolute,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CasFailed>(
                    "cas_failed",
                    ApiError::has_cas_failed,
                    ApiError::get_cas_failed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, BadFormat>(
                    "bad_format",
                    ApiError::has_bad_format,
                    ApiError::get_bad_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Io>(
                    "io",
                    ApiError::has_io,
                    ApiError::get_io,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, EncodingError>(
                    "encoding",
                    ApiError::has_encoding,
                    ApiError::get_encoding,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, InvalidCas>(
                    "invalid_cas",
                    ApiError::has_invalid_cas,
                    ApiError::get_invalid_cas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "msg",
                    ApiError::has_msg,
                    ApiError::get_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "cannot_delete_root",
                    ApiError::has_cannot_delete_root,
                    ApiError::get_cannot_delete_root,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "invalid_msg",
                    ApiError::has_invalid_msg,
                    ApiError::get_invalid_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "timeout",
                    ApiError::has_timeout,
                    ApiError::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "not_enough_replicas",
                    ApiError::has_not_enough_replicas,
                    ApiError::get_not_enough_replicas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "bad_epoch",
                    ApiError::has_bad_epoch,
                    ApiError::get_bad_epoch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiError>(
                    "ApiError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiError {
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
        self.clear_encoding();
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

impl ::std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NotFound {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NotFound {}

impl NotFound {
    pub fn new() -> NotFound {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NotFound {
        static mut instance: ::protobuf::lazy::Lazy<NotFound> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NotFound,
        };
        unsafe {
            instance.get(NotFound::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for NotFound {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NotFound {
    fn new() -> NotFound {
        NotFound::new()
    }

    fn descriptor_static(_: ::std::option::Option<NotFound>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    NotFound::get_path_for_reflect,
                    NotFound::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NotFound>(
                    "NotFound",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NotFound {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NotFound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NotFound {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlreadyExists {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlreadyExists {}

impl AlreadyExists {
    pub fn new() -> AlreadyExists {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlreadyExists {
        static mut instance: ::protobuf::lazy::Lazy<AlreadyExists> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlreadyExists,
        };
        unsafe {
            instance.get(AlreadyExists::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for AlreadyExists {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlreadyExists {
    fn new() -> AlreadyExists {
        AlreadyExists::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlreadyExists>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    AlreadyExists::get_path_for_reflect,
                    AlreadyExists::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlreadyExists>(
                    "AlreadyExists",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlreadyExists {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlreadyExists {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlreadyExists {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoesNotExist {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoesNotExist {}

impl DoesNotExist {
    pub fn new() -> DoesNotExist {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoesNotExist {
        static mut instance: ::protobuf::lazy::Lazy<DoesNotExist> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoesNotExist,
        };
        unsafe {
            instance.get(DoesNotExist::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for DoesNotExist {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DoesNotExist {
    fn new() -> DoesNotExist {
        DoesNotExist::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoesNotExist>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    DoesNotExist::get_path_for_reflect,
                    DoesNotExist::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoesNotExist>(
                    "DoesNotExist",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoesNotExist {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoesNotExist {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoesNotExist {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WrongType {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    node_type: ::std::option::Option<NodeType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(WrongType::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required .NodeType node_type = 2;

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
        self.node_type.unwrap_or(NodeType::BLOB)
    }

    fn get_node_type_for_reflect(&self) -> &::std::option::Option<NodeType> {
        &self.node_type
    }

    fn mut_node_type_for_reflect(&mut self) -> &mut ::std::option::Option<NodeType> {
        &mut self.node_type
    }
}

impl ::protobuf::Message for WrongType {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.node_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.node_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.node_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.node_type {
            os.write_enum(2, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    WrongType::get_path_for_reflect,
                    WrongType::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NodeType>>(
                    "node_type",
                    WrongType::get_node_type_for_reflect,
                    WrongType::mut_node_type_for_reflect,
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

impl ::std::fmt::Debug for WrongType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WrongType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PathMustEndInDirectory {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PathMustEndInDirectory {}

impl PathMustEndInDirectory {
    pub fn new() -> PathMustEndInDirectory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PathMustEndInDirectory {
        static mut instance: ::protobuf::lazy::Lazy<PathMustEndInDirectory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PathMustEndInDirectory,
        };
        unsafe {
            instance.get(PathMustEndInDirectory::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for PathMustEndInDirectory {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PathMustEndInDirectory {
    fn new() -> PathMustEndInDirectory {
        PathMustEndInDirectory::new()
    }

    fn descriptor_static(_: ::std::option::Option<PathMustEndInDirectory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    PathMustEndInDirectory::get_path_for_reflect,
                    PathMustEndInDirectory::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PathMustEndInDirectory>(
                    "PathMustEndInDirectory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PathMustEndInDirectory {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PathMustEndInDirectory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PathMustEndInDirectory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PathMustBeAbsolute {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PathMustBeAbsolute {}

impl PathMustBeAbsolute {
    pub fn new() -> PathMustBeAbsolute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PathMustBeAbsolute {
        static mut instance: ::protobuf::lazy::Lazy<PathMustBeAbsolute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PathMustBeAbsolute,
        };
        unsafe {
            instance.get(PathMustBeAbsolute::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for PathMustBeAbsolute {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PathMustBeAbsolute {
    fn new() -> PathMustBeAbsolute {
        PathMustBeAbsolute::new()
    }

    fn descriptor_static(_: ::std::option::Option<PathMustBeAbsolute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    PathMustBeAbsolute::get_path_for_reflect,
                    PathMustBeAbsolute::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PathMustBeAbsolute>(
                    "PathMustBeAbsolute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PathMustBeAbsolute {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PathMustBeAbsolute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PathMustBeAbsolute {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CasFailed {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    expected: ::std::option::Option<u64>,
    actual: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(CasFailed::new)
        }
    }

    // required string path = 1;

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

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required uint64 expected = 2;

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

    fn get_expected_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.expected
    }

    fn mut_expected_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.expected
    }

    // required uint64 actual = 3;

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

    fn get_actual_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.actual
    }

    fn mut_actual_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.actual
    }
}

impl ::protobuf::Message for CasFailed {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.expected.is_none() {
            return false;
        };
        if self.actual.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.expected = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.actual = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.expected {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.actual {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.expected {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.actual {
            os.write_uint64(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    CasFailed::get_path_for_reflect,
                    CasFailed::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expected",
                    CasFailed::get_expected_for_reflect,
                    CasFailed::mut_expected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "actual",
                    CasFailed::get_actual_for_reflect,
                    CasFailed::mut_actual_for_reflect,
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

impl ::std::fmt::Debug for CasFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CasFailed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BadFormat {
    // message fields
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BadFormat {}

impl BadFormat {
    pub fn new() -> BadFormat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BadFormat {
        static mut instance: ::protobuf::lazy::Lazy<BadFormat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BadFormat,
        };
        unsafe {
            instance.get(BadFormat::new)
        }
    }

    // required string msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for BadFormat {
    fn is_initialized(&self) -> bool {
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BadFormat {
    fn new() -> BadFormat {
        BadFormat::new()
    }

    fn descriptor_static(_: ::std::option::Option<BadFormat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    BadFormat::get_msg_for_reflect,
                    BadFormat::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BadFormat>(
                    "BadFormat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BadFormat {
    fn clear(&mut self) {
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BadFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BadFormat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Io {
    // message fields
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Io {}

impl Io {
    pub fn new() -> Io {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Io {
        static mut instance: ::protobuf::lazy::Lazy<Io> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Io,
        };
        unsafe {
            instance.get(Io::new)
        }
    }

    // required string msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for Io {
    fn is_initialized(&self) -> bool {
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Io {
    fn new() -> Io {
        Io::new()
    }

    fn descriptor_static(_: ::std::option::Option<Io>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    Io::get_msg_for_reflect,
                    Io::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Io>(
                    "Io",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Io {
    fn clear(&mut self) {
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Io {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Io {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EncodingError {
    // message fields
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncodingError {}

impl EncodingError {
    pub fn new() -> EncodingError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncodingError {
        static mut instance: ::protobuf::lazy::Lazy<EncodingError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncodingError,
        };
        unsafe {
            instance.get(EncodingError::new)
        }
    }

    // required string msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for EncodingError {
    fn is_initialized(&self) -> bool {
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncodingError {
    fn new() -> EncodingError {
        EncodingError::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncodingError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    EncodingError::get_msg_for_reflect,
                    EncodingError::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncodingError>(
                    "EncodingError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncodingError {
    fn clear(&mut self) {
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EncodingError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EncodingError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InvalidCas {
    // message fields
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InvalidCas {}

impl InvalidCas {
    pub fn new() -> InvalidCas {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InvalidCas {
        static mut instance: ::protobuf::lazy::Lazy<InvalidCas> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InvalidCas,
        };
        unsafe {
            instance.get(InvalidCas::new)
        }
    }

    // required string msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for InvalidCas {
    fn is_initialized(&self) -> bool {
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InvalidCas {
    fn new() -> InvalidCas {
        InvalidCas::new()
    }

    fn descriptor_static(_: ::std::option::Option<InvalidCas>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    InvalidCas::get_msg_for_reflect,
                    InvalidCas::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InvalidCas>(
                    "InvalidCas",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InvalidCas {
    fn clear(&mut self) {
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InvalidCas {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InvalidCas {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NodeType {
    BLOB = 0,
    QUEUE = 1,
    SET = 2,
    DIRECTORY = 3,
}

impl ::protobuf::ProtobufEnum for NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NodeType> {
        match value {
            0 => ::std::option::Option::Some(NodeType::BLOB),
            1 => ::std::option::Option::Some(NodeType::QUEUE),
            2 => ::std::option::Option::Some(NodeType::SET),
            3 => ::std::option::Option::Some(NodeType::DIRECTORY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NodeType] = &[
            NodeType::BLOB,
            NodeType::QUEUE,
            NodeType::SET,
            NodeType::DIRECTORY,
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

impl ::protobuf::reflect::ProtobufValue for NodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x6a, 0x0a, 0x06, 0x41, 0x70, 0x69, 0x4d, 0x73, 0x67, 0x12, 0x27, 0x0a, 0x07, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x41, 0x70,
    0x69, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x2a, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x41, 0x70, 0x69, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42,
    0x0b, 0x0a, 0x09, 0x72, 0x65, 0x71, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x22, 0xbe, 0x01, 0x0a,
    0x0a, 0x41, 0x70, 0x69, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x27, 0x0a, 0x0e, 0x67,
    0x65, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0d, 0x67, 0x65, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70,
    0x61, 0x63, 0x65, 0x73, 0x12, 0x3a, 0x0a, 0x0f, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72,
    0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x48, 0x00,
    0x52, 0x0e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x12, 0x40, 0x0a, 0x11, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x5f, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x43, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00,
    0x52, 0x10, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x42, 0x09, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x50, 0x0a,
    0x0e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12,
    0x1b, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c,
    0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x0b, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x49, 0x64, 0x22,
    0x6c, 0x0a, 0x06, 0x41, 0x70, 0x69, 0x50, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a,
    0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x12, 0x1b, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x1b, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x09, 0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x41, 0x64, 0x64, 0x72, 0x22, 0xc7, 0x01,
    0x0a, 0x10, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x17, 0x0a, 0x02, 0x74, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x07,
    0x2e, 0x41, 0x70, 0x69, 0x50, 0x69, 0x64, 0x52, 0x02, 0x74, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x08,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x04, 0x52, 0x10, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x22, 0x0a, 0x07, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x6f,
    0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70,
    0x48, 0x00, 0x52, 0x06, 0x74, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x12, 0x25, 0x0a, 0x08, 0x74, 0x72,
    0x65, 0x65, 0x5f, 0x63, 0x61, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x54,
    0x72, 0x65, 0x65, 0x43, 0x61, 0x73, 0x48, 0x00, 0x52, 0x07, 0x74, 0x72, 0x65, 0x65, 0x43, 0x61,
    0x73, 0x42, 0x04, 0x0a, 0x02, 0x6f, 0x70, 0x22, 0xe0, 0x08, 0x0a, 0x06, 0x54, 0x72, 0x65, 0x65,
    0x4f, 0x70, 0x12, 0x2e, 0x0a, 0x0b, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x6e, 0x6f, 0x64,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x4e, 0x6f, 0x64, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x4e, 0x6f,
    0x64, 0x65, 0x12, 0x2e, 0x0a, 0x0b, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x6e, 0x6f, 0x64,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x4e, 0x6f, 0x64, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x4e, 0x6f,
    0x64, 0x65, 0x12, 0x28, 0x0a, 0x09, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73,
    0x48, 0x00, 0x52, 0x08, 0x6c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x12, 0x25, 0x0a, 0x08,
    0x62, 0x6c, 0x6f, 0x62, 0x5f, 0x70, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08,
    0x2e, 0x42, 0x6c, 0x6f, 0x62, 0x50, 0x75, 0x74, 0x48, 0x00, 0x52, 0x07, 0x62, 0x6c, 0x6f, 0x62,
    0x50, 0x75, 0x74, 0x12, 0x25, 0x0a, 0x08, 0x62, 0x6c, 0x6f, 0x62, 0x5f, 0x67, 0x65, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x42, 0x6c, 0x6f, 0x62, 0x47, 0x65, 0x74, 0x48,
    0x00, 0x52, 0x07, 0x62, 0x6c, 0x6f, 0x62, 0x47, 0x65, 0x74, 0x12, 0x28, 0x0a, 0x09, 0x62, 0x6c,
    0x6f, 0x62, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e,
    0x42, 0x6c, 0x6f, 0x62, 0x53, 0x69, 0x7a, 0x65, 0x48, 0x00, 0x52, 0x08, 0x62, 0x6c, 0x6f, 0x62,
    0x53, 0x69, 0x7a, 0x65, 0x12, 0x2b, 0x0a, 0x0a, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x70, 0x75,
    0x73, 0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x51, 0x75, 0x65, 0x75, 0x65,
    0x50, 0x75, 0x73, 0x68, 0x48, 0x00, 0x52, 0x09, 0x71, 0x75, 0x65, 0x75, 0x65, 0x50, 0x75, 0x73,
    0x68, 0x12, 0x28, 0x0a, 0x09, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x70, 0x6f, 0x70, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x51, 0x75, 0x65, 0x75, 0x65, 0x50, 0x6f, 0x70, 0x48,
    0x00, 0x52, 0x08, 0x71, 0x75, 0x65, 0x75, 0x65, 0x50, 0x6f, 0x70, 0x12, 0x2e, 0x0a, 0x0b, 0x71,
    0x75, 0x65, 0x75, 0x65, 0x5f, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0b, 0x2e, 0x51, 0x75, 0x65, 0x75, 0x65, 0x46, 0x72, 0x6f, 0x6e, 0x74, 0x48, 0x00, 0x52,
    0x0a, 0x71, 0x75, 0x65, 0x75, 0x65, 0x46, 0x72, 0x6f, 0x6e, 0x74, 0x12, 0x2b, 0x0a, 0x0a, 0x71,
    0x75, 0x65, 0x75, 0x65, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0a, 0x2e, 0x51, 0x75, 0x65, 0x75, 0x65, 0x42, 0x61, 0x63, 0x6b, 0x48, 0x00, 0x52, 0x09, 0x71,
    0x75, 0x65, 0x75, 0x65, 0x42, 0x61, 0x63, 0x6b, 0x12, 0x28, 0x0a, 0x09, 0x71, 0x75, 0x65, 0x75,
    0x65, 0x5f, 0x6c, 0x65, 0x6e, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x51, 0x75,
    0x65, 0x75, 0x65, 0x4c, 0x65, 0x6e, 0x48, 0x00, 0x52, 0x08, 0x71, 0x75, 0x65, 0x75, 0x65, 0x4c,
    0x65, 0x6e, 0x12, 0x2b, 0x0a, 0x0a, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x6e, 0x73, 0x65, 0x72, 0x74,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x53, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x65,
    0x72, 0x74, 0x48, 0x00, 0x52, 0x09, 0x73, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x12,
    0x2b, 0x0a, 0x0a, 0x73, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x18, 0x0d, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x48,
    0x00, 0x52, 0x09, 0x73, 0x65, 0x74, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x12, 0x31, 0x0a, 0x0c,
    0x73, 0x65, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
    0x48, 0x00, 0x52, 0x0b, 0x73, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x12,
    0x28, 0x0a, 0x09, 0x73, 0x65, 0x74, 0x5f, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x09, 0x2e, 0x53, 0x65, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52,
    0x08, 0x73, 0x65, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x3d, 0x0a, 0x10, 0x73, 0x65, 0x74,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x10, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x53, 0x65, 0x74, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0f, 0x73, 0x65, 0x74, 0x49, 0x6e, 0x74, 0x65,
    0x72, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x37, 0x0a, 0x0e, 0x73, 0x65, 0x74, 0x5f,
    0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x53, 0x65, 0x74, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65,
    0x48, 0x00, 0x52, 0x0d, 0x73, 0x65, 0x74, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63,
    0x65, 0x12, 0x53, 0x0a, 0x18, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x12, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x48, 0x00, 0x52, 0x16,
    0x73, 0x65, 0x74, 0x53, 0x79, 0x6d, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x44, 0x69, 0x66, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x38, 0x0a, 0x0f, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x75,
    0x62, 0x73, 0x65, 0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0e, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x48,
    0x00, 0x52, 0x0d, 0x73, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68,
    0x12, 0x35, 0x0a, 0x0e, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x75, 0x62, 0x73, 0x65, 0x74, 0x5f, 0x73,
    0x65, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75,
    0x62, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x48, 0x00, 0x52, 0x0c, 0x73, 0x65, 0x74, 0x53, 0x75,
    0x62, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x12, 0x3e, 0x0a, 0x11, 0x73, 0x65, 0x74, 0x5f, 0x73,
    0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x15, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74,
    0x50, 0x61, 0x74, 0x68, 0x48, 0x00, 0x52, 0x0f, 0x73, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72,
    0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x12, 0x3b, 0x0a, 0x10, 0x73, 0x65, 0x74, 0x5f, 0x73,
    0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x5f, 0x73, 0x65, 0x74, 0x18, 0x16, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x53,
    0x65, 0x74, 0x48, 0x00, 0x52, 0x0e, 0x73, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72, 0x73, 0x65,
    0x74, 0x53, 0x65, 0x74, 0x42, 0x04, 0x0a, 0x02, 0x6f, 0x70, 0x22, 0xe4, 0x01, 0x0a, 0x0c, 0x54,
    0x72, 0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x6f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x10, 0x0a, 0x02, 0x6f, 0x6b, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x02, 0x6f, 0x6b, 0x12, 0x14, 0x0a, 0x04, 0x62, 0x6f, 0x6f, 0x6c,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x04, 0x62, 0x6f, 0x6f, 0x6c, 0x12, 0x14,
    0x0a, 0x04, 0x62, 0x6c, 0x6f, 0x62, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52, 0x04,
    0x62, 0x6c, 0x6f, 0x62, 0x12, 0x12, 0x0a, 0x03, 0x69, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x04, 0x48, 0x00, 0x52, 0x03, 0x69, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74, 0x48, 0x00, 0x52, 0x03, 0x73,
    0x65, 0x74, 0x12, 0x1b, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x05, 0x2e, 0x4b, 0x65, 0x79, 0x73, 0x48, 0x00, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x12,
    0x16, 0x0a, 0x05, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00,
    0x52, 0x05, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x42, 0x08, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x22, 0x38, 0x0a, 0x0d, 0x54, 0x72, 0x65, 0x65, 0x43, 0x61, 0x73, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x27, 0x0a, 0x07, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x52, 0x07, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x22, 0x20, 0x0a, 0x04, 0x4b,
    0x65, 0x79, 0x73, 0x12, 0x18, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x04, 0x2e, 0x4b, 0x65, 0x79, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x22, 0x33, 0x0a,
    0x03, 0x4b, 0x65, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x22, 0x35, 0x0a, 0x05, 0x47, 0x75, 0x61, 0x72, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
    0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04,
    0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x4d, 0x0a, 0x07, 0x54, 0x72, 0x65,
    0x65, 0x43, 0x61, 0x73, 0x12, 0x1e, 0x0a, 0x06, 0x67, 0x75, 0x61, 0x72, 0x64, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x47, 0x75, 0x61, 0x72, 0x64, 0x52, 0x06, 0x67, 0x75,
    0x61, 0x72, 0x64, 0x73, 0x12, 0x22, 0x0a, 0x08, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x6f, 0x70, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52,
    0x07, 0x74, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x73, 0x22, 0x48, 0x0a, 0x0a, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x26, 0x0a, 0x09, 0x6e, 0x6f,
    0x64, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e,
    0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x22, 0x20, 0x0a, 0x0a, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x4e, 0x6f, 0x64, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x22, 0x1e, 0x0a, 0x08, 0x4c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73,
    0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x22, 0x2f, 0x0a, 0x07, 0x42, 0x6c, 0x6f, 0x62, 0x50, 0x75, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c,
    0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x1d, 0x0a, 0x07, 0x42, 0x6c, 0x6f, 0x62, 0x47, 0x65, 0x74,
    0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x22, 0x1e, 0x0a, 0x08, 0x42, 0x6c, 0x6f, 0x62, 0x53, 0x69, 0x7a, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x22, 0x31, 0x0a, 0x09, 0x51, 0x75, 0x65, 0x75, 0x65, 0x50, 0x75, 0x73,
    0x68, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52,
    0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x1e, 0x0a, 0x08, 0x51, 0x75, 0x65, 0x75, 0x65,
    0x50, 0x6f, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x20, 0x0a, 0x0a, 0x51, 0x75, 0x65, 0x75, 0x65,
    0x46, 0x72, 0x6f, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x1f, 0x0a, 0x09, 0x51, 0x75, 0x65,
    0x75, 0x65, 0x42, 0x61, 0x63, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x1e, 0x0a, 0x08, 0x51, 0x75,
    0x65, 0x75, 0x65, 0x4c, 0x65, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x31, 0x0a, 0x09, 0x53, 0x65,
    0x74, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76,
    0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x31, 0x0a,
    0x09, 0x53, 0x65, 0x74, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61,
    0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x10,
    0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c,
    0x22, 0x33, 0x0a, 0x0b, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x12,
    0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c,
    0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x17, 0x0a, 0x03, 0x53, 0x65, 0x74, 0x12, 0x10, 0x0a, 0x03,
    0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x03, 0x76, 0x61, 0x6c, 0x22, 0x3a,
    0x0a, 0x08, 0x53, 0x65, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x73,
    0x12, 0x18, 0x0a, 0x04, 0x73, 0x65, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x04,
    0x2e, 0x53, 0x65, 0x74, 0x52, 0x04, 0x73, 0x65, 0x74, 0x73, 0x22, 0x3d, 0x0a, 0x0f, 0x53, 0x65,
    0x74, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x31, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x22, 0x3b, 0x0a, 0x0d, 0x53, 0x65, 0x74,
    0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31,
    0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x22, 0x44, 0x0a, 0x16, 0x53, 0x65, 0x74, 0x53, 0x79, 0x6d,
    0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x44, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65,
    0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x22, 0x3b, 0x0a, 0x0d,
    0x53, 0x65, 0x74, 0x53, 0x75, 0x62, 0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x12, 0x14, 0x0a,
    0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61,
    0x74, 0x68, 0x31, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x22, 0x3a, 0x0a, 0x0c, 0x53, 0x65, 0x74,
    0x53, 0x75, 0x62, 0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74,
    0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x16, 0x0a,
    0x03, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74,
    0x52, 0x03, 0x73, 0x65, 0x74, 0x22, 0x3d, 0x0a, 0x0f, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65,
    0x72, 0x73, 0x65, 0x74, 0x50, 0x61, 0x74, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x68,
    0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x74, 0x68, 0x31, 0x12, 0x14,
    0x0a, 0x05, 0x70, 0x61, 0x74, 0x68, 0x32, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x05, 0x70,
    0x61, 0x74, 0x68, 0x32, 0x22, 0x3c, 0x0a, 0x0e, 0x53, 0x65, 0x74, 0x53, 0x75, 0x70, 0x65, 0x72,
    0x73, 0x65, 0x74, 0x53, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x16, 0x0a, 0x03, 0x73, 0x65,
    0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x03, 0x73,
    0x65, 0x74, 0x22, 0x83, 0x03, 0x0a, 0x0b, 0x41, 0x70, 0x69, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x3a, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x5f,
    0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x43, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x48, 0x00, 0x52, 0x0e,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x2d,
    0x0a, 0x0a, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x48,
    0x00, 0x52, 0x0a, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x12, 0x46, 0x0a,
    0x13, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48,
    0x00, 0x52, 0x12, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x27, 0x0a, 0x08, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65,
    0x63, 0x74, 0x48, 0x00, 0x52, 0x08, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x12, 0x1e,
    0x0a, 0x05, 0x72, 0x65, 0x74, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e,
    0x52, 0x65, 0x74, 0x72, 0x79, 0x48, 0x00, 0x52, 0x05, 0x72, 0x65, 0x74, 0x72, 0x79, 0x12, 0x1a,
    0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x48,
    0x00, 0x52, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12, 0x21, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x41, 0x70, 0x69, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2d, 0x0a,
    0x11, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61,
    0x63, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x10, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x42, 0x0a, 0x0a, 0x08,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x1e, 0x0a, 0x0a, 0x4e, 0x61, 0x6d, 0x65,
    0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x12, 0x10, 0x0a, 0x03, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x09, 0x52, 0x03, 0x69, 0x64, 0x73, 0x22, 0xa0, 0x02, 0x0a, 0x0e, 0x43, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x65,
    0x70, 0x6f, 0x63, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x52, 0x05, 0x65, 0x70, 0x6f, 0x63,
    0x68, 0x12, 0x12, 0x0a, 0x04, 0x76, 0x69, 0x65, 0x77, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x52,
    0x04, 0x76, 0x69, 0x65, 0x77, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x02, 0x28, 0x04, 0x52, 0x0a, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x10, 0x0a, 0x02, 0x6f, 0x6b, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x02, 0x6f, 0x6b, 0x12, 0x35, 0x0a, 0x0e, 0x74, 0x72, 0x65, 0x65,
    0x5f, 0x6f, 0x70, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0d, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48,
    0x00, 0x52, 0x0c, 0x74, 0x72, 0x65, 0x65, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x38, 0x0a, 0x0f, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x63, 0x61, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x54, 0x72, 0x65, 0x65, 0x43,
    0x61, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x74, 0x72, 0x65, 0x65,
    0x43, 0x61, 0x73, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x14, 0x0a, 0x04, 0x70, 0x61, 0x74,
    0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
    0x21, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09,
    0x2e, 0x41, 0x70, 0x69, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x52, 0x05, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x62, 0x0a, 0x12, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x21, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x07, 0x2e, 0x41, 0x70, 0x69, 0x50, 0x69, 0x64, 0x52, 0x07, 0x70, 0x72, 0x69,
    0x6d, 0x61, 0x72, 0x79, 0x12, 0x29, 0x0a, 0x10, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x52, 0x0f,
    0x6e, 0x65, 0x77, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22,
    0x48, 0x0a, 0x08, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x12, 0x21, 0x0a, 0x07, 0x70,
    0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x41,
    0x70, 0x69, 0x50, 0x69, 0x64, 0x52, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x19,
    0x0a, 0x08, 0x61, 0x70, 0x69, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x07, 0x61, 0x70, 0x69, 0x41, 0x64, 0x64, 0x72, 0x22, 0x2b, 0x0a, 0x05, 0x52, 0x65, 0x74,
    0x72, 0x79, 0x12, 0x22, 0x0a, 0x0c, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x52, 0x0c, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x22, 0x9e, 0x06, 0x0a, 0x08, 0x41, 0x70, 0x69, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x12, 0x28, 0x0a, 0x09, 0x6e, 0x6f, 0x74, 0x5f, 0x66, 0x6f, 0x75, 0x6e, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e,
    0x64, 0x48, 0x00, 0x52, 0x08, 0x6e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x37, 0x0a,
    0x0e, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x5f, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x41, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x45,
    0x78, 0x69, 0x73, 0x74, 0x73, 0x48, 0x00, 0x52, 0x0d, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79,
    0x45, 0x78, 0x69, 0x73, 0x74, 0x73, 0x12, 0x35, 0x0a, 0x0e, 0x64, 0x6f, 0x65, 0x73, 0x5f, 0x6e,
    0x6f, 0x74, 0x5f, 0x65, 0x78, 0x69, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x44, 0x6f, 0x65, 0x73, 0x4e, 0x6f, 0x74, 0x45, 0x78, 0x69, 0x73, 0x74, 0x48, 0x00, 0x52,
    0x0c, 0x64, 0x6f, 0x65, 0x73, 0x4e, 0x6f, 0x74, 0x45, 0x78, 0x69, 0x73, 0x74, 0x12, 0x2b, 0x0a,
    0x0a, 0x77, 0x72, 0x6f, 0x6e, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0a, 0x2e, 0x57, 0x72, 0x6f, 0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x48, 0x00, 0x52,
    0x09, 0x77, 0x72, 0x6f, 0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x55, 0x0a, 0x1a, 0x70, 0x61,
    0x74, 0x68, 0x5f, 0x6d, 0x75, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x5f, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x50, 0x61, 0x74, 0x68, 0x4d, 0x75, 0x73, 0x74, 0x45, 0x6e, 0x64, 0x49, 0x6e, 0x44, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x48, 0x00, 0x52, 0x16, 0x70, 0x61, 0x74, 0x68, 0x4d,
    0x75, 0x73, 0x74, 0x45, 0x6e, 0x64, 0x49, 0x6e, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x79, 0x12, 0x48, 0x0a, 0x15, 0x70, 0x61, 0x74, 0x68, 0x5f, 0x6d, 0x75, 0x73, 0x74, 0x5f, 0x62,
    0x65, 0x5f, 0x61, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x13, 0x2e, 0x50, 0x61, 0x74, 0x68, 0x4d, 0x75, 0x73, 0x74, 0x42, 0x65, 0x41, 0x62, 0x73,
    0x6f, 0x6c, 0x75, 0x74, 0x65, 0x48, 0x00, 0x52, 0x12, 0x70, 0x61, 0x74, 0x68, 0x4d, 0x75, 0x73,
    0x74, 0x42, 0x65, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x12, 0x2b, 0x0a, 0x0a, 0x63,
    0x61, 0x73, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0a, 0x2e, 0x43, 0x61, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x48, 0x00, 0x52, 0x09, 0x63,
    0x61, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x12, 0x2b, 0x0a, 0x0a, 0x62, 0x61, 0x64, 0x5f,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x42,
    0x61, 0x64, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x48, 0x00, 0x52, 0x09, 0x62, 0x61, 0x64, 0x46,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x15, 0x0a, 0x02, 0x69, 0x6f, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x03, 0x2e, 0x49, 0x6f, 0x48, 0x00, 0x52, 0x02, 0x69, 0x6f, 0x12, 0x2c, 0x0a, 0x08,
    0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e,
    0x2e, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00,
    0x52, 0x08, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x12, 0x2e, 0x0a, 0x0b, 0x69, 0x6e,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x63, 0x61, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x43, 0x61, 0x73, 0x48, 0x00, 0x52, 0x0a,
    0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x43, 0x61, 0x73, 0x12, 0x12, 0x0a, 0x03, 0x6d, 0x73,
    0x67, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x12, 0x2e,
    0x0a, 0x12, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f,
    0x72, 0x6f, 0x6f, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x10, 0x63, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x6f, 0x6f, 0x74, 0x12, 0x21,
    0x0a, 0x0b, 0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x6d, 0x73, 0x67, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x0a, 0x69, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x4d, 0x73,
    0x67, 0x12, 0x1a, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x12, 0x30, 0x0a,
    0x13, 0x6e, 0x6f, 0x74, 0x5f, 0x65, 0x6e, 0x6f, 0x75, 0x67, 0x68, 0x5f, 0x72, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x11, 0x6e, 0x6f,
    0x74, 0x45, 0x6e, 0x6f, 0x75, 0x67, 0x68, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x12,
    0x1d, 0x0a, 0x09, 0x62, 0x61, 0x64, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x11, 0x20, 0x01,
    0x28, 0x08, 0x48, 0x00, 0x52, 0x08, 0x62, 0x61, 0x64, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x42, 0x07,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x1e, 0x0a, 0x08, 0x4e, 0x6f, 0x74, 0x46, 0x6f,
    0x75, 0x6e, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x23, 0x0a, 0x0d, 0x41, 0x6c, 0x72, 0x65, 0x61,
    0x64, 0x79, 0x45, 0x78, 0x69, 0x73, 0x74, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x22, 0x0a, 0x0c,
    0x44, 0x6f, 0x65, 0x73, 0x4e, 0x6f, 0x74, 0x45, 0x78, 0x69, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68,
    0x22, 0x47, 0x0a, 0x09, 0x57, 0x72, 0x6f, 0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74,
    0x68, 0x12, 0x26, 0x0a, 0x09, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x08, 0x6e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0x2c, 0x0a, 0x16, 0x50, 0x61, 0x74,
    0x68, 0x4d, 0x75, 0x73, 0x74, 0x45, 0x6e, 0x64, 0x49, 0x6e, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x28, 0x0a, 0x12, 0x50, 0x61, 0x74, 0x68, 0x4d,
    0x75, 0x73, 0x74, 0x42, 0x65, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74,
    0x68, 0x22, 0x53, 0x0a, 0x09, 0x43, 0x61, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x12, 0x12,
    0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61,
    0x74, 0x68, 0x12, 0x1a, 0x0a, 0x08, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x04, 0x52, 0x08, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x12, 0x16,
    0x0a, 0x06, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x02, 0x28, 0x04, 0x52, 0x06,
    0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x22, 0x1d, 0x0a, 0x09, 0x42, 0x61, 0x64, 0x46, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x16, 0x0a, 0x02, 0x49, 0x6f, 0x12, 0x10, 0x0a, 0x03, 0x6d,
    0x73, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x21, 0x0a,
    0x0d, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x10,
    0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67,
    0x22, 0x1e, 0x0a, 0x0a, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x43, 0x61, 0x73, 0x12, 0x10,
    0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67,
    0x2a, 0x37, 0x0a, 0x08, 0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04,
    0x42, 0x4c, 0x4f, 0x42, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x51, 0x55, 0x45, 0x55, 0x45, 0x10,
    0x01, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x45, 0x54, 0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x44, 0x49,
    0x52, 0x45, 0x43, 0x54, 0x4f, 0x52, 0x59, 0x10, 0x03, 0x4a, 0xb5, 0x58, 0x0a, 0x07, 0x12, 0x05,
    0x01, 0x00, 0xc8, 0x02, 0x01, 0x0a, 0x64, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x1a,
    0x5a, 0x20, 0x55, 0x73, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x32, 0x20, 0x73, 0x79, 0x6e,
    0x74, 0x61, 0x78, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x20, 0x68, 0x61, 0x73, 0x58, 0x58, 0x58, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x6a, 0x61, 0x76, 0x61, 0x2c, 0x20, 0x75, 0x6e, 0x6c, 0x69,
    0x6b, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x32, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x04, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x05, 0x02, 0x08,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x06, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x06, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x07, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x07, 0x04,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x10, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x0b, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x0c,
    0x02, 0x10, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x0e, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x0e, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x13,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x15, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0f, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x13, 0x00,
    0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x14, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x18, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x19, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x19, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1a,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x02, 0x12, 0x03, 0x1b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x12,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x1c, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x1c, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1f, 0x00, 0x27, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x20, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x17,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x21, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x21, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03,
    0x22, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x22, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x12, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x22, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x04, 0x08, 0x00, 0x12, 0x04, 0x23, 0x02, 0x26, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12,
    0x03, 0x24, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x24,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x25, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x06, 0x12, 0x03, 0x25, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x25, 0x0c, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x25, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x29, 0x00, 0x42,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x29, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x05, 0x08, 0x00, 0x12, 0x04, 0x2a, 0x02, 0x41, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x2b, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x2b, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b,
    0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x04, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x2c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12,
    0x03, 0x2d, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2d,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x0d, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x19, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2e, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2e, 0x0c, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x2e, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x2f,
    0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2f, 0x04, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x0c, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x30, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x05, 0x06, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x30, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x30, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06, 0x12, 0x03, 0x31, 0x04, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x06, 0x12, 0x03, 0x31, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12, 0x03, 0x31, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x03, 0x31, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x07, 0x12, 0x03, 0x32, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x32, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x03, 0x12, 0x03, 0x32, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x08, 0x12, 0x03, 0x33, 0x04, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x06, 0x12, 0x03, 0x33, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x08, 0x01, 0x12, 0x03, 0x33, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x33, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x09,
    0x12, 0x03, 0x34, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x06, 0x12, 0x03,
    0x34, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x01, 0x12, 0x03, 0x34, 0x0e,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x03, 0x12, 0x03, 0x34, 0x1b, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0a, 0x12, 0x03, 0x35, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x35, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x35, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0b, 0x12, 0x03,
    0x36, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x36, 0x04,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x36, 0x0e, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x36, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x0c, 0x12, 0x03, 0x37, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0c, 0x06, 0x12, 0x03, 0x37, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x37, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x37, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0d, 0x12, 0x03, 0x38, 0x04,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x38, 0x04, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x38, 0x10, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x38, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x0e, 0x12, 0x03, 0x39, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e,
    0x06, 0x12, 0x03, 0x39, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x01, 0x12,
    0x03, 0x39, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x39,
    0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0f, 0x12, 0x03, 0x3a, 0x04, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x3a, 0x04, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x3a, 0x14, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x3a, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x10, 0x12, 0x03, 0x3b, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x06, 0x12,
    0x03, 0x3b, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x01, 0x12, 0x03, 0x3b,
    0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x03, 0x12, 0x03, 0x3b, 0x23, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x11, 0x12, 0x03, 0x3c, 0x04, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x11, 0x06, 0x12, 0x03, 0x3c, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x11, 0x01, 0x12, 0x03, 0x3c, 0x1b, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x11, 0x03, 0x12, 0x03, 0x3c, 0x36, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x12, 0x12,
    0x03, 0x3d, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x12, 0x06, 0x12, 0x03, 0x3d,
    0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x12, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x12, 0x03, 0x12, 0x03, 0x3d, 0x24, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x13, 0x12, 0x03, 0x3e, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x13, 0x06, 0x12, 0x03, 0x3e, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x13, 0x01, 0x12, 0x03, 0x3e, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x13, 0x03,
    0x12, 0x03, 0x3e, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x14, 0x12, 0x03, 0x3f,
    0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x14, 0x06, 0x12, 0x03, 0x3f, 0x04, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x14, 0x01, 0x12, 0x03, 0x3f, 0x14, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x14, 0x03, 0x12, 0x03, 0x3f, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x15, 0x12, 0x03, 0x40, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x15, 0x06, 0x12, 0x03, 0x40, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x15, 0x01,
    0x12, 0x03, 0x40, 0x13, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x15, 0x03, 0x12, 0x03,
    0x40, 0x26, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x44, 0x00, 0x4f, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x44, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x45, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45,
    0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x45, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x08, 0x00, 0x12, 0x04, 0x46, 0x02, 0x4e, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x08, 0x00, 0x01, 0x12, 0x03, 0x46, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x47, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x47, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x47, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x47, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x48, 0x04, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x48, 0x04, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x48, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x48, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x03, 0x12, 0x03, 0x49, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x49, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x49, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x49, 0x11,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4a, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x4a, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05,
    0x12, 0x03, 0x4b, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x06, 0x12, 0x03,
    0x4b, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4b, 0x08,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x4b, 0x0e, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x4c, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x06, 0x06, 0x12, 0x03, 0x4c, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x4c, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x4c, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x03,
    0x4d, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4d, 0x04,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4d, 0x09, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4d, 0x11, 0x12, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x07, 0x12, 0x04, 0x51, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01,
    0x12, 0x03, 0x51, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x52,
    0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x52, 0x0b, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x55, 0x00, 0x57, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x55,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x56, 0x02, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x56, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x56, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x59,
    0x00, 0x5c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x59, 0x08, 0x0b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x5a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x5a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x02,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5b, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5b, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12,
    0x04, 0x5e, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x5e, 0x08,
    0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x5f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03,
    0x60, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x60, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x60, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x60, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x60, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0b, 0x12, 0x04, 0x63, 0x00, 0x66, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03,
    0x63, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x64, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x64, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01,
    0x12, 0x03, 0x65, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x65, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x65, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x12, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x68, 0x00, 0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x68, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x69,
    0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x02, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x69, 0x09, 0x0a, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x6a, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6b,
    0x02, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x6b, 0x08, 0x09,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6c, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x6c, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12,
    0x04, 0x6f, 0x00, 0x72, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6f, 0x08,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x70, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x70, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03,
    0x71, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x03, 0x71, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x14, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x04, 0x74, 0x00, 0x76, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03,
    0x74, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x75, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x75, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x75, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x75, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x75, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04,
    0x78, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x78, 0x08, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x79, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x79, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x79, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x79, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x7c, 0x00, 0x7f,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x7d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x7d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x7d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7d,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x7e, 0x02, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7e, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x7e, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x81,
    0x01, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x81, 0x01,
    0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x02, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x11, 0x12, 0x06, 0x85, 0x01, 0x00, 0x87, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x04, 0x85, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0x86, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x86, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01,
    0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x19,
    0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x89, 0x01, 0x00, 0x8c, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8a, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8a, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04,
    0x8b, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8b, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x11,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x8e, 0x01, 0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x8f, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8f, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8f, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0x92, 0x01, 0x00,
    0x94, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0x92, 0x01, 0x08, 0x12,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x93, 0x01, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0x93, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0x93, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15,
    0x12, 0x06, 0x96, 0x01, 0x00, 0x98, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12,
    0x04, 0x96, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x97,
    0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x97, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x97, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x01, 0x12, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x01, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x9a, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16,
    0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9b, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9b, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9b, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9b, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa1,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x11, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x01, 0x12, 0x04, 0xa0, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xa0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa0, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa0, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa0,
    0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xa6, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x11, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01,
    0x12, 0x04, 0xa5, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xa5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xa5, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5,
    0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x01,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xa8, 0x01, 0x00, 0xab, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa9, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12,
    0x04, 0xaa, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xaa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xaa,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x01,
    0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x17,
    0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xad, 0x01, 0x00, 0xaf, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xad, 0x01, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xae, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xae, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xae, 0x01, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xae, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xb1, 0x01,
    0x00, 0xb4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x01, 0x02, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1b, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xb3, 0x01, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb3, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb3, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xb6, 0x01, 0x00,
    0xb9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c,
    0x02, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xb8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xb8, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xb8, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xb8, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xbb, 0x01, 0x00, 0xbe,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02,
    0x01, 0x12, 0x04, 0xbd, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xbd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xbd, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xbd, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbd,
    0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xc0, 0x01, 0x00, 0xc3, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xc1, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01,
    0x12, 0x04, 0xc2, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xc2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xc2, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc2,
    0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc2, 0x01,
    0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xc5, 0x01, 0x00, 0xc8, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc6, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01, 0x12,
    0x04, 0xc7, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xc7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc7,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc7, 0x01,
    0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x1a,
    0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xca, 0x01, 0x00, 0xcd, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xcb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xcb, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xcb, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xcb, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04,
    0xcc, 0x01, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcc,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x06, 0x12, 0x04, 0xcc, 0x01,
    0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x0f,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x15, 0x16,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xcf, 0x01, 0x00, 0xd2, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x21, 0x02, 0x00, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xd0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xd0, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xd0, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0xd1,
    0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd1, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x12, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x1a, 0x1b, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xd4, 0x01, 0x00, 0xd7, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22,
    0x02, 0x00, 0x12, 0x04, 0xd5, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xd5, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd5, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd5, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xd6, 0x01,
    0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd6, 0x01, 0x0b, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x0f, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x15, 0x16, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xd9, 0x01, 0x00, 0xe4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x23, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x23, 0x08,
    0x00, 0x12, 0x06, 0xda, 0x01, 0x02, 0xe3, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x08,
    0x00, 0x01, 0x12, 0x04, 0xda, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00,
    0x12, 0x04, 0xdb, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xdb, 0x01, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xdb, 0x01, 0x13, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdb,
    0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x06, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x0f, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x23, 0x02, 0x02, 0x12, 0x04, 0xdd, 0x01, 0x04, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x02, 0x06, 0x12, 0x04, 0xdd, 0x01, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x02, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x17, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02,
    0x03, 0x12, 0x04, 0xde, 0x01, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xde, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xde, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xde, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x04, 0x12, 0x04, 0xdf, 0x01,
    0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x04, 0x06, 0x12, 0x04, 0xdf, 0x01, 0x04,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x04, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x0a, 0x0f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x04, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x12, 0x13, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x09, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x05, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23,
    0x02, 0x06, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06,
    0x06, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06, 0x01,
    0x12, 0x04, 0xe1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06, 0x03, 0x12,
    0x04, 0xe1, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x07, 0x12, 0x04, 0xe2,
    0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x07, 0x05, 0x12, 0x04, 0xe2, 0x01,
    0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x07, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x09,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x07, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xe6, 0x01, 0x00, 0xe8, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x24, 0x02, 0x00, 0x12, 0x04, 0xe7, 0x01, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xe7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xe7, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xe7, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0xea, 0x01, 0x00,
    0xf5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12, 0x04, 0xea, 0x01, 0x08, 0x16,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x00, 0x05, 0x12, 0x04, 0xeb, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25,
    0x02, 0x01, 0x12, 0x04, 0xec, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xec, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xec, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xec, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xec, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0xed, 0x01,
    0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x04, 0x12, 0x04, 0xed, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x05, 0x12, 0x04, 0xed, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0xed, 0x01, 0x12, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x03, 0x12, 0x04, 0xed, 0x01, 0x20, 0x21, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x25, 0x08, 0x00, 0x12, 0x06, 0xee, 0x01, 0x02, 0xf4, 0x01, 0x03, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x08, 0x00, 0x01, 0x12, 0x04, 0xee, 0x01, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x25, 0x02, 0x03, 0x12, 0x04, 0xef, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x03, 0x05, 0x12, 0x04, 0xef, 0x01, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xef, 0x01, 0x09, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xef, 0x01, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x04,
    0x12, 0x04, 0xf0, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x06, 0x12,
    0x04, 0xf0, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xf0, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x03, 0x12, 0x04, 0xf0,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x04,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x06, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x12, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x25, 0x02, 0x06, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x06, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x07, 0x12, 0x04, 0xf3, 0x01, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x07, 0x06,
    0x12, 0x04, 0xf3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xf3, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xf3, 0x01, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0xf7, 0x01, 0x00, 0xfa,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xf8, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf8, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02,
    0x01, 0x12, 0x04, 0xf9, 0x01, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xf9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xf9, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xf9, 0x01, 0x10, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf9,
    0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27, 0x12, 0x06, 0xfc, 0x01, 0x00, 0xff, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01,
    0x12, 0x04, 0xfe, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xfe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xfe, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfe,
    0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfe, 0x01,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0x81, 0x02, 0x00, 0x83, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0x81, 0x02, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0x82, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x82, 0x02, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x82, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12, 0x06, 0x85,
    0x02, 0x00, 0x99, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0x85, 0x02,
    0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x08, 0x00, 0x12, 0x06, 0x86, 0x02, 0x02, 0x98,
    0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x08, 0x00, 0x01, 0x12, 0x04, 0x86, 0x02, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12, 0x04, 0x87, 0x02, 0x04, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x06, 0x12, 0x04, 0x87, 0x02, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x02, 0x0d, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x29, 0x02, 0x01, 0x12, 0x04, 0x88, 0x02, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x88, 0x02, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x88, 0x02, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x88, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x02, 0x12,
    0x04, 0x89, 0x02, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x06, 0x12, 0x04,
    0x89, 0x02, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89,
    0x02, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89, 0x02,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x0d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x0e, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x29, 0x02, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x04, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8b, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x1b, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x05,
    0x12, 0x04, 0x8c, 0x02, 0x04, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x05, 0x06, 0x12,
    0x04, 0x8c, 0x02, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x05, 0x01, 0x12, 0x04,
    0x8c, 0x02, 0x17, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x05, 0x03, 0x12, 0x04, 0x8c,
    0x02, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x04,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x0d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x0e, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x1b, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x29, 0x02, 0x07, 0x12, 0x04, 0x8e, 0x02, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x07, 0x06, 0x12, 0x04, 0x8e, 0x02, 0x04, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x07, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02,
    0x08, 0x12, 0x04, 0x8f, 0x02, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x06,
    0x12, 0x04, 0x8f, 0x02, 0x04, 0x06, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x01, 0x12,
    0x04, 0x8f, 0x02, 0x07, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x03, 0x12, 0x04,
    0x8f, 0x02, 0x0c, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x09, 0x12, 0x04, 0x90, 0x02,
    0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x06, 0x12, 0x04, 0x90, 0x02, 0x04,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x01, 0x12, 0x04, 0x90, 0x02, 0x12, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x03, 0x12, 0x04, 0x90, 0x02, 0x1d, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0a, 0x12, 0x04, 0x91, 0x02, 0x04, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x91, 0x02, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x91, 0x02, 0x0f, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x91, 0x02, 0x1d, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29,
    0x02, 0x0b, 0x12, 0x04, 0x92, 0x02, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b,
    0x05, 0x12, 0x04, 0x92, 0x02, 0x04, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b, 0x01,
    0x12, 0x04, 0x92, 0x02, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b, 0x03, 0x12,
    0x04, 0x92, 0x02, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0c, 0x12, 0x04, 0x93,
    0x02, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0c, 0x05, 0x12, 0x04, 0x93, 0x02,
    0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x93, 0x02, 0x09,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x93, 0x02, 0x1e, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0d, 0x12, 0x04, 0x94, 0x02, 0x04, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x0d, 0x05, 0x12, 0x04, 0x94, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x94, 0x02, 0x09, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x0d, 0x03, 0x12, 0x04, 0x94, 0x02, 0x17, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x29, 0x02, 0x0e, 0x12, 0x04, 0x95, 0x02, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x0e, 0x05, 0x12, 0x04, 0x95, 0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0e,
    0x01, 0x12, 0x04, 0x95, 0x02, 0x09, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0e, 0x03,
    0x12, 0x04, 0x95, 0x02, 0x13, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0f, 0x12, 0x04,
    0x96, 0x02, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0f, 0x05, 0x12, 0x04, 0x96,
    0x02, 0x04, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x96, 0x02,
    0x09, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x96, 0x02, 0x1f,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x10, 0x12, 0x04, 0x97, 0x02, 0x04, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x10, 0x05, 0x12, 0x04, 0x97, 0x02, 0x04, 0x08, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x10, 0x01, 0x12, 0x04, 0x97, 0x02, 0x09, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x10, 0x03, 0x12, 0x04, 0x97, 0x02, 0x15, 0x17, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x2a, 0x12, 0x06, 0x9b, 0x02, 0x00, 0x9d, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a,
    0x01, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00, 0x12,
    0x04, 0x9c, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x9c, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9c,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x02,
    0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x02, 0x19,
    0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0x9f, 0x02, 0x00, 0xa1, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa0, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa0, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xa3, 0x02,
    0x00, 0xa5, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x02, 0x02, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa4, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x2d, 0x12, 0x06, 0xa7, 0x02, 0x00, 0xaa, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01,
    0x12, 0x04, 0xa7, 0x02, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04,
    0xa8, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa8,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa8, 0x02,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x12,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x19, 0x1a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x02, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa9, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e,
    0x12, 0x06, 0xac, 0x02, 0x00, 0xae, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12,
    0x04, 0xac, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xad,
    0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xad, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xad, 0x02, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x02, 0x12, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xad, 0x02, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xb0, 0x02, 0x00, 0xb2, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x2f, 0x01, 0x12, 0x04, 0xb0, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f,
    0x02, 0x00, 0x12, 0x04, 0xb1, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xb1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xb1, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb1, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb1, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x30, 0x12, 0x06, 0xb4, 0x02, 0x00, 0xb8,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x30, 0x01, 0x12, 0x04, 0xb4, 0x02, 0x08, 0x11, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb5, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb5, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xb5, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02,
    0x01, 0x12, 0x04, 0xb6, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xb6, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xb6, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xb6, 0x02, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6,
    0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x02, 0x12, 0x04, 0xb7, 0x02, 0x02,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb7, 0x02, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x12, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x31, 0x12, 0x06, 0xba, 0x02, 0x00, 0xbc, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x31, 0x01, 0x12, 0x04, 0xba, 0x02, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x00,
    0x12, 0x04, 0xbb, 0x02, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xbb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xbb, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb,
    0x02, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x02,
    0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12, 0x06, 0xbe, 0x02, 0x00, 0xc0, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x08, 0x0a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0xbf, 0x02, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbf, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xbf, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xbf, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x33, 0x12, 0x06, 0xc2,
    0x02, 0x00, 0xc4, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33, 0x01, 0x12, 0x04, 0xc2, 0x02,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x00, 0x12, 0x04, 0xc3, 0x02, 0x02, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc3, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc3, 0x02, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x12, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x33, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc3, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x34, 0x12, 0x06, 0xc6, 0x02, 0x00, 0xc8, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x34,
    0x01, 0x12, 0x04, 0xc6, 0x02, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x00, 0x12,
    0x04, 0xc7, 0x02, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc7,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc7, 0x02,
    0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x18,
    0x19,
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
