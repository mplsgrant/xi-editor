// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate xi_rope;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xi_rope::compare;
use xi_rope::diff::{Diff, LineHashDiff};
use xi_rope::rope::{Rope, RopeDelta};

static EDITOR_STR: &str = include_str!("../../core-lib/src/editor.rs");
static VIEW_STR: &str = include_str!("../../core-lib/src/view.rs");

static INTERVAL_STR: &str = include_str!("../src/interval.rs");
static BREAKS_STR: &str = include_str!("../src/breaks.rs");

static BASE_STR: &str = "This adds FixedSizeAdler32, that has a size set at construction, and keeps bytes in a cyclic buffer of that size to be removed when it fills up.

Current logic (and implementing Write) might be too much, since bytes will probably always be fed one by one anyway. Otherwise a faster way of removing a sequence might be needed (one by one is inefficient).";

static TARG_STR: &str = "This adds some function, I guess?, that has a size set at construction, and keeps bytes in a cyclic buffer of that size to be ground up and injested when it fills up.

Currently my sense of smell (and the pain of implementing Write) might be too much, since bytes will probably always be fed one by one anyway. Otherwise crying might be needed (one by one is inefficient).";

fn make_test_data() -> (Vec<u8>, Vec<u8>) {
    let one = [EDITOR_STR, VIEW_STR, INTERVAL_STR, BREAKS_STR].concat().into_bytes();
    let mut two = one.clone();
    let idx = one.len() / 2;
    two[idx] = 0x02;
    (one, two)
}

fn ne_idx_sw(c: &mut Criterion) {
    let (one, two) = make_test_data();

    c.bench_function("ne_idx_sw", |b| {
        b.iter(|| {
            compare::ne_idx_fallback(&one, &one);
            compare::ne_idx_fallback(&one, &two);
        })
    });
}

#[cfg(target_arch = "x86_64")]
fn ne_idx_sse(c: &mut Criterion) {
    if !is_x86_feature_detected!("sse4.2") {
        return;
    }
    let (one, two) = make_test_data();

    let mut x = 0;
    c.bench_function("ne_idx_sse", |b| {
        b.iter(|| {
            x += unsafe { compare::ne_idx_sse(&one, &one).unwrap_or_default() };
            x += unsafe { compare::ne_idx_sse(&one, &two).unwrap_or_default() };
        })
    });
}

#[cfg(target_arch = "x86_64")]
fn ne_idx_avx(c: &mut Criterion) {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let (one, two) = make_test_data();

    let mut dont_opt_me = 0;
    c.bench_function("ne_idx_avx", |b| {
        b.iter(|| {
            dont_opt_me += unsafe { compare::ne_idx_avx(&one, &two).unwrap_or_default() };
            dont_opt_me += unsafe { compare::ne_idx_avx(&one, &one).unwrap_or_default() };
        })
    });
}

fn ne_idx_detect(c: &mut Criterion) {
    let (one, two) = make_test_data();

    let mut dont_opt_me = 0;
    c.bench_function("ne_idx_detect", |b| {
        b.iter(|| {
            dont_opt_me += compare::ne_idx(&one, &two).unwrap_or_default();
            dont_opt_me += compare::ne_idx(&one, &one).unwrap_or_default();
        })
    });
}

fn ne_idx_rev_sw(c: &mut Criterion) {
    let (one, two) = make_test_data();

    let mut x = 0;
    c.bench_function("ne_idx_rev_sw", |b| {
        b.iter(|| {
            x += compare::ne_idx_rev_fallback(&one, &one).unwrap_or_default();
            x += compare::ne_idx_rev_fallback(&one, &two).unwrap_or_default();
        })
    });
}

#[cfg(target_arch = "x86_64")]
fn ne_idx_rev_sse(c: &mut Criterion) {
    if !is_x86_feature_detected!("sse4.2") {
        return;
    }
    let (one, two) = make_test_data();

    c.bench_function("ne_idx_rev_sse", |b| {
        b.iter(|| unsafe {
            compare::ne_idx_rev_sse(&one, &one);
            compare::ne_idx_rev_sse(&one, &two);
        })
    });
}

fn scanner(c: &mut Criterion) {
    let (one, two) = make_test_data();
    let one = Rope::from(String::from_utf8(one).unwrap());
    let two = Rope::from(String::from_utf8(two).unwrap());

    let mut scanner = compare::RopeScanner::new(&one, &two);
    c.bench_function("scanner", |b| {
        b.iter(|| {
            scanner.find_ne_char(0, 0, None);
            scanner.find_ne_char_back(one.len(), two.len(), None);
        })
    });
}

fn hash_diff(c: &mut Criterion) {
    let one = BASE_STR.into();
    let two = TARG_STR.into();
    let mut delta: Option<RopeDelta> = None;
    c.bench_function("hash_diff", |b| {
        b.iter(|| {
            delta = Some(LineHashDiff::compute_delta(&one, &two));
        })
    });

    let _result = delta.unwrap().apply(&one);
    assert_eq!(String::from(_result), String::from(&two));
}

fn hash_diff_med(c: &mut Criterion) {
    let one = INTERVAL_STR.into();
    let two = BREAKS_STR.into();
    let mut delta: Option<RopeDelta> = None;
    c.bench_function("hash_diff_med", |b| {
        b.iter(|| {
            delta = Some(LineHashDiff::compute_delta(&one, &two));
        })
    });

    let _result = delta.unwrap().apply(&one);
    assert_eq!(String::from(_result), String::from(&two));
}

fn hash_diff_big(c: &mut Criterion) {
    let one = EDITOR_STR.into();
    let two = VIEW_STR.into();
    let mut delta: Option<RopeDelta> = None;
    c.bench_function("hash_diff_big", |b| {
        b.iter(|| {
            delta = Some(LineHashDiff::compute_delta(&one, &two));
        })
    });

    let _result = delta.unwrap().apply(&one);
    assert_eq!(String::from(_result), String::from(&two));
}

fn simple_insertion(c: &mut Criterion) {
    let one: Rope =
        ["start", EDITOR_STR, VIEW_STR, INTERVAL_STR, BREAKS_STR, "end"].concat().into();
    let two = "startend".into();
    let mut delta: Option<RopeDelta> = None;
    c.bench_function("simple_insertion", |b| {
        b.iter(|| {
            delta = Some(LineHashDiff::compute_delta(&one, &two));
        })
    });

    let _result = delta.unwrap().apply(&one);
    assert_eq!(String::from(_result), String::from(&two));
}

fn simple_deletion(c: &mut Criterion) {
    let one: Rope =
        ["start", EDITOR_STR, VIEW_STR, INTERVAL_STR, BREAKS_STR, "end"].concat().into();
    let two = "startend".into();
    let mut delta: Option<RopeDelta> = None;
    c.bench_function("simple_deletion", |b| {
        b.iter(|| {
            delta = Some(LineHashDiff::compute_delta(&two, &one));
        })
    });

    let _result = delta.unwrap().apply(&two);
    assert_eq!(String::from(_result), String::from(&one));
}

criterion_group!(
    rope_diff_benches,
    ne_idx_sw,
    ne_idx_sse,
    ne_idx_avx,
    ne_idx_detect,
    ne_idx_rev_sw,
    ne_idx_rev_sse,
    scanner,
    hash_diff,
    hash_diff_med,
    hash_diff_big,
    simple_insertion,
    simple_deletion
);
criterion_main!(rope_diff_benches);
