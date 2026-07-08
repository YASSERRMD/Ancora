use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};
use ancora_ffi::version::ancora_version;

fn bench_version_call(c: &mut Criterion) {
    c.bench_function("ffi_ancora_version", |b| {
        b.iter(|| {
            let ptr = black_box(unsafe { ancora_version() });
            assert!(!ptr.is_null());
        })
    });
}

fn bench_runtime_create_free(c: &mut Criterion) {
    c.bench_function("ffi_runtime_new_and_free", |b| {
        b.iter(|| {
            let mut rt_ptr = std::ptr::null_mut();
            let code = unsafe { ancora_runtime_new(&mut rt_ptr) };
            assert_eq!(code, AncorErrorCode::Ok);
            ancora_free_runtime(rt_ptr);
        })
    });
}

criterion_group!(benches, bench_version_call, bench_runtime_create_free);
criterion_main!(benches);
