#[macro_use]
extern crate criterion;
extern crate smparser;

use criterion::Criterion;
use smparser::parse_simfile;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn parse_simfiles_bench(c: &mut Criterion) {
    // Load example file
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("example_files/goin_under.sm");
    let file = File::open(d).unwrap();
    let mut reader = BufReader::new(file);

    // Parse it!
    c.bench_function("parse simfile", move |b| {
        b.iter(|| {
            crate::parse_simfile(&mut reader);
        })
    });
}

criterion_group!(benches, parse_simfiles_bench);
criterion_main!(benches);
