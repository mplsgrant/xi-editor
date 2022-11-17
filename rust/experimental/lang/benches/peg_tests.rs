// Copyright 2017 The xi-editor Authors.
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

//! Benchmarks of PEG parsing libraries

/// Run as:
/// ```
/// run nightly cargo bench --features "nom regex pom"
/// ```
use std::env;

extern crate xi_lang;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(feature = "pom")]
extern crate pom;

#[cfg(feature = "regex")]
extern crate regex;

#[cfg(feature = "nom")]
#[macro_use]
extern crate nom;

#[cfg(feature = "combine")]
extern crate combine;

const TEST_STR: &str = "1.2345e56";

// #[cfg(all(test, feature = "pom"))]
// mod pom_benches {
//     use super::TEST_STR;
//     use criterion::{black_box, criterion_group, criterion_main, Criterion};
//     use pom::parser::{one_of, sym};
//     use pom::{DataInput, Parser};

//     fn pom_number() -> Parser<u8, usize> {
//         let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
//         let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
//         let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
//         let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
//         number.pos()
//     }

//     fn bench_pom(c: &mut Criterion) {
//         let parser = pom_number();

//         c.bench_function("bench_pom", |b| {
//             b.iter(|| {
//                 let mut buf = DataInput::new(test::black_box(TEST_STR.as_bytes()));
//                 parser.parse(&mut buf)
//             })
//         });
//     }

//     criterion_group!(experimental_pom_benches, bench_pom);
//     criterion_main!(experimental_pom_benches);
// }

#[cfg(all(test, feature = "regex"))]
mod regex_benches {
    use super::TEST_STR;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use regex::Regex;

    fn bench_regex(c: &mut Criterion) {
        let re = Regex::new(r"^(0|[1-9][0-9]*)(\.[0-9]+)?([eE]([+-])?[0-9]+)?").unwrap();
        c.bench_function("bench_regex", |b| b.iter(|| re.find(black_box(TEST_STR))));
    }

    criterion_group!(experimental_regex_benches, bench_regex);
    criterion_main!(experimental_regex_benches);
}

#[cfg(all(test, feature = "nom"))]
mod nom_benches {
    use super::TEST_STR;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use nom::digit;

    named!(digits<()>, fold_many1!(digit, (), |_, _| ()));

    named!(
        nom_num<()>,
        do_parse!(
            opt!(char!('-'))
                >> alt!(map!(char!('0'), |_| ()) | digits)
                >> opt!(do_parse!(char!('.') >> digits >> ()))
                >> opt!(do_parse!(
                    alt!(char!('e') | char!('E'))
                        >> opt!(alt!(char!('+') | char!('-')))
                        >> digits
                        >> ()
                ))
                >> ()
        )
    );

    #[cfg(feature = "nom")]
    fn bench_nom(c: &mut Criterion) {
        c.bench_function("bench_nom", |b| b.iter(|| nom_num(black_box(TEST_STR.as_bytes()))));
    }

    criterion_group!(experimental_nom_benches, bench_nom);
    criterion_main!(experimental_nom_benches);
}

// #[cfg(all(test, feature = "combine"))]
// mod combine_benches {
//     use super::{is_digit, test, TEST_STR};
//     use combine::range::take_while1;
//     use combine::*;
//     use criterion::{black_box, criterion_group, criterion_main, Criterion};

//     fn my_number(s: &[u8]) -> ParseResult<(), &[u8]> {
//         (
//             token(b'-').map(Some).or(value(None)),
//             token(b'0').map(|_| &b"0"[..]).or(take_while1(is_digit)),
//             optional((token(b'.'), take_while1(is_digit))),
//             optional((
//                 token(b'e').or(token(b'E')),
//                 token(b'-').map(Some).or(token(b'+').map(Some)).or(value(None)),
//                 take_while1(is_digit),
//             )),
//         )
//             .map(|_| ())
//             .parse_stream(s)
//     }

//     fn bench_combine(c: &mut Criterion) {
//         assert_eq!(parser(my_number).parse(TEST_STR.as_bytes()), Ok(((), &b""[..])));
//         c.bench_function("bench_combine", |b| {
//             b.iter(|| parser(my_number).parse(test::black_box(TEST_STR.as_bytes())))
//         });
//     }

//     criterion_group!(experimental_combine_benches, bench_combine);
//     criterion_main!(experimental_combine_benches);
// }

use xi_lang::peg::{Alt, OneByte, OneOrMore, Optional, Peg};

fn is_digit(c: u8) -> bool {
    (b'0'..=b'9').contains(&c)
}

fn my_number(s: &[u8]) -> Option<usize> {
    (
        Optional('-'),
        Alt('0', OneOrMore(OneByte(is_digit))),
        Optional(('.', OneOrMore(OneByte(is_digit)))),
        Optional((Alt('e', 'E'), Optional(Alt('-', '+')), OneOrMore(OneByte(is_digit)))),
    )
        .p(s)
}

fn bench_my_peg(c: &mut Criterion) {
    c.bench_function("bench_my_peg", |b| b.iter(|| my_number(black_box(TEST_STR.as_bytes()))));
}

criterion_group!(experimental_my_peg_benches, bench_my_peg);
criterion_main!(experimental_my_peg_benches);
