#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn encode_slice() {
    let mut output = [0_u8; base32_fs::encoded_len(32)];
    base32_fs::encode(
        black_box(b"12345678901234567890123456789012"),
        &mut &mut output[..],
    );
}

struct Array {
    array: [u8; base32_fs::encoded_len(32)],
    offset: usize,
}

impl base32_fs::Output for Array {
    fn push(&mut self, ch: u8) {
        self.array[self.offset] = ch;
        self.offset += 1;
    }
}

fn encode_array() {
    let mut output = Array {
        array: [0_u8; base32_fs::encoded_len(32)],
        offset: 0,
    };
    base32_fs::encode(black_box(b"12345678901234567890123456789012"), &mut output);
}

fn decode_slice() {
    let mut output = [0_u8; 32];
    base32_fs::decode(
        black_box(b"64s36d1n6rvkge9g64s36d1n6rvkge9g64s36d1n6rvkge9g64s0".as_slice()),
        &mut &mut output[..],
    )
    .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("decode slice", |b| b.iter(decode_slice));
    c.bench_function("encode slice", |b| b.iter(encode_slice));
    c.bench_function("encode array", |b| b.iter(encode_array));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
