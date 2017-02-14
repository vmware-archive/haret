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

/// An assert that doesn't panic on failure, but instead returns a result<(), String> with an
/// appropriate error message.
#[macro_export]
macro_rules! safe_assert {
    () => {
        return Err(format!("Assertion at File: {}, Line: {}", file!(), line!()));
    };
    ($predicate:expr) => {
        if $predicate {
            let res: Result<(), String> = Ok(());
            res
        } else {
             return Err(format!("Assert failure: {}. File: {}, Line: {}",
                     stringify!($predicate), file!(), line!()))
        }
    }
}

#[macro_export]
macro_rules! safe_assert_eq {
    ($left:expr, $right:expr) => {
        if $left == $right {
            let res: Result<(), String> = Ok(());
            res
        } else {
             return Err(format!("Assert failure: left = {:#?}, right = {:#?} File: {}, Line: {}",
                     $left, $right, file!(), line!()))
        }
    };
    ($left:expr, $right:expr, $extra:expr) => {
        if $left == $right {
            let res: Result<(), String> = Ok(());
            res
        } else {
             return Err(format!("Assert failure: left = {:#?}, right = {:#?}
                                File: {}, Line: {}\nExtra Context: {:#?}",
                                $left, $right, file!(), line!(), $extra))
        }

    }
}

#[macro_export]
macro_rules! fail {
    () => {
        safe_assert!()
    }
}