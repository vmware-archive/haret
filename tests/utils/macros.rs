// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/// An assert that doesn't panic on failure, but instead returns a result<(),
/// String> with an
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
