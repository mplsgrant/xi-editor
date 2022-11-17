// Copyright 2016 The xi-editor Authors.
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

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cmp::max;
use xi_unicode::linebreak_property;
use xi_unicode::linebreak_property_str;
use xi_unicode::LineBreakIterator;

fn linebreak_property_chars(s: &str) -> u8 {
    linebreak_property(black_box(s).chars().next().unwrap())
}

// compute the maximum numeric value of the lb, a model for iterating a string
fn max_lb_chars(s: &str) -> u8 {
    let mut result = 0;
    for c in s.chars() {
        result = max(result, linebreak_property(c))
    }
    result
}

fn max_lb(s: &str) -> u8 {
    let mut result = 0;
    let mut ix = 0;
    while ix < s.len() {
        let (lb, len) = linebreak_property_str(s, ix);
        result = max(result, lb);
        ix += len;
    }
    result
}

fn linebreak_lo(c: &mut Criterion) {
    c.bench_function("linebreak_lo", |b| b.iter(|| linebreak_property(black_box('\u{0042}'))));
}

fn linebreak_lo2(c: &mut Criterion) {
    c.bench_function("linebreak_lo2", |b| b.iter(|| linebreak_property(black_box('\u{0644}'))));
}

fn linebreak_med(c: &mut Criterion) {
    c.bench_function("linebreak_med", |b| b.iter(|| linebreak_property(black_box('\u{200D}'))));
}

fn linebreak_hi(c: &mut Criterion) {
    c.bench_function("linebreak_hi", |b| b.iter(|| linebreak_property(black_box('\u{1F680}'))));
}

fn linebreak_str_lo(c: &mut Criterion) {
    c.bench_function("linebreak_str_lo", |b| b.iter(|| linebreak_property_str("\\u{0042}", 0)));
}

fn linebreak_str_lo2(c: &mut Criterion) {
    c.bench_function("linebreak_str_lo2", |b| b.iter(|| linebreak_property_str("\\u{0644}", 0)));
}

fn linebreak_str_med(c: &mut Criterion) {
    c.bench_function("linebreak_str_med", |b| b.iter(|| linebreak_property_str("\\u{200D}", 0)));
}

fn linebreak_str_hi(c: &mut Criterion) {
    c.bench_function("linebreak_str_hi", |b| b.iter(|| linebreak_property_str("\u{1F680}", 0)));
}

fn linebreak_chars_lo2(c: &mut Criterion) {
    c.bench_function("linebreak_chars_lo2", |b| b.iter(|| linebreak_property_chars("\\u{0644}")));
}

fn linebreak_chars_hi(c: &mut Criterion) {
    c.bench_function("linebreak_chars_hi", |b| b.iter(|| linebreak_property_chars("\\u{1F680}")));
}

fn max_lb_chars_hi(c: &mut Criterion) {
    c.bench_function("max_lb_chars_hi", |b|  b.iter(|| max_lb_chars("\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}")));
}

fn max_lb_hi(c: &mut Criterion) {
    c.bench_function("max_lb_hi", |b|   b.iter(|| max_lb("\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}\\u{1F680}")));
}

fn max_lb_lo(c: &mut Criterion) {
    c.bench_function("max_lb_lo",|b|   b.iter(|| max_lb("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")));
}

fn max_lb_chars_lo(c: &mut Criterion) {
    c.bench_function("max_lb_chars_lo", |b|  b.iter(|| max_lb_chars("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")));
}

fn lb_iter(c: &mut Criterion) {
    // 73 ASCII characters
    let s = "Now is the time for all good persons to come to the aid of their country.";
    c.bench_function("lb_iter", |b| b.iter(|| LineBreakIterator::new(s).count()));
}

criterion_group!(
    unicode_benches,
    linebreak_lo,
    linebreak_lo2,
    linebreak_med,
    linebreak_hi,
    linebreak_str_lo,
    linebreak_str_lo2,
    linebreak_str_med,
    linebreak_str_hi,
    linebreak_chars_lo2,
    linebreak_chars_hi,
    max_lb_chars_hi,
    max_lb_hi,
    max_lb_lo,
    max_lb_chars_lo,
    lb_iter,
);
criterion_main!(unicode_benches);
