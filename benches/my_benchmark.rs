use std::time::{Duration, Instant};

use chess_bot::{board::FastBoard, search::MoveEngine};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let expiry = Instant::now() + Duration::new(60 * 60 * 24, 0);
    c.bench_function("bestmove 4", |b| {
        b.iter(|| {
            let mut engine = MoveEngine::new();
            engine.find_best_move(&mut black_box(FastBoard::initial()), 4, expiry)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
