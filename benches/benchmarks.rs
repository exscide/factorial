
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use factorial::*;

pub fn factorial(c: &mut Criterion) {
	c.bench_function("factorial_u16 8", |b| b.iter(||factorial_u16(black_box(8))));
	c.bench_function("factorial_u32 12", |b| b.iter(||factorial_u32(black_box(12))));
	c.bench_function("factorial_u64 12", |b| b.iter(||factorial_u64(black_box(12))));
}

pub fn double_factorial(c: &mut Criterion) {
	c.bench_function("double_factorial_u16 8", |b| b.iter(||double_factorial_u16(black_box(8))));
	c.bench_function("double_factorial_u32 12", |b| b.iter(||double_factorial_u32(black_box(12))));
	c.bench_function("double_factorial_u64 12", |b| b.iter(||double_factorial_u64(black_box(12))));
}

pub fn sub_factorial(c: &mut Criterion) {
	c.bench_function("sub_factorial_i16 8", |b| b.iter(||sub_factorial_i16(black_box(8))));
	c.bench_function("sub_factorial_i32 12", |b| b.iter(||sub_factorial_i32(black_box(12))));
	c.bench_function("sub_factorial_i64 12", |b| b.iter(||sub_factorial_i64(black_box(12))));
}

pub fn super_factorial_sloane(c: &mut Criterion) {
	c.bench_function("super_factorial_sloane_u16 4", |b| b.iter(||super_factorial_sloane_u16(black_box(4))));
	c.bench_function("super_factorial_sloane_u32 6", |b| b.iter(||super_factorial_sloane_u32(black_box(6))));
	c.bench_function("super_factorial_sloane_u64 6", |b| b.iter(||super_factorial_sloane_u64(black_box(6))));
}

pub fn swinging_factorial(c: &mut Criterion) {
	c.bench_function("swinging_factorial_u16 4", |b| b.iter(||swinging_factorial_u16(black_box(8))));
	c.bench_function("swinging_factorial_u32 6", |b| b.iter(||swinging_factorial_u32(black_box(15))));
	c.bench_function("swinging_factorial_u64 6", |b| b.iter(||swinging_factorial_u64(black_box(15))));
}


criterion_group!(benches, swinging_factorial);
criterion_main!(benches);
