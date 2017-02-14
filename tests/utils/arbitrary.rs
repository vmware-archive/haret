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

//! This file contains arbitrary types for use across quickcheck tests

use rand::thread_rng;
use rand::distributions::range::Range;
use rand::distributions::IndependentSample;
use quickcheck::{Arbitrary, Gen};
use haret::vr::{VrMsg, VrApiReq, TreeOp, NodeType};

#[derive(Debug, Clone)]
struct Path(pub String);

impl Arbitrary for Path {
    fn arbitrary<G: Gen>(g: &mut G) -> Path {
        let range = Range::new(1u8, 11u8);
        let depth = range.ind_sample(g);
        let labels = ['a', 'b', 'c', 'd', 'e'];
        let mut path = String::with_capacity((depth*2 - 1) as usize);
        path = (0..depth).fold(path, |mut acc, _| {
                acc.push('/');
                acc.push(*g.choose(&labels).unwrap());
                acc
        });
        Path(path)
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    ClientRequest(ClientRequest),
    Commit,
    ViewChange,
    CrashBackup,
    CrashPrimary,
    Restart
}

impl Arbitrary for Op {
    fn arbitrary<G: Gen>(g: &mut G) -> Op {
        let range = Range::new(0, 100);
        match range.ind_sample(&mut thread_rng()) {
            0...80 => Op::ClientRequest(ClientRequest::arbitrary(g)),
            80...85 => Op::Commit,
            85...90 => Op::ViewChange,
            90...92 => Op::CrashPrimary,
            92...95 => Op::CrashBackup,
            _ => Op::Restart
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientRequest(pub VrMsg);

impl Arbitrary for ClientRequest {
    fn arbitrary<G: Gen>(g: &mut G) -> ClientRequest {
        ClientRequest(VrMsg::ClientRequest {
            client_id: "test-client".to_string(),
            request_num: 0, // This will get mutated
            op: ApiReq::arbitrary(g).0
        })
    }
}

#[derive(Debug, Clone)]
pub struct ApiReq(pub VrApiReq);

impl Arbitrary for ApiReq {
    fn arbitrary<G: Gen>(g: &mut G) -> ApiReq {
        let range = Range::new(0, 3);
        let path = Path::arbitrary(g).0;
        let op = match range.ind_sample(&mut thread_rng()) {
            0 => TreeOp::CreateNode {path: path, ty: NodeType::Blob},
            1 => TreeOp::BlobPut {path: path, val: b"hello".to_vec()},
            _ => TreeOp::BlobGet {path: path}
        };
        ApiReq(VrApiReq::TreeOp(op))
    }
}
