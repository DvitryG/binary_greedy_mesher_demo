use criterion::{criterion_group, criterion_main, Criterion};
use new_voxel_testing::{
    chunks_refs::ChunksRefs,
    culled_mesher, greedy_mesher_optimized,
    lod::Lod,
};

fn binary_mesh_optimized(chunks_refs: ChunksRefs) {
    let _m = greedy_mesher_optimized::build_chunk_mesh(&chunks_refs, Lod::L32);
}

fn culled_mesh_ao(chunks_refs: ChunksRefs) {
    culled_mesher::build_chunk_mesh_ao(&chunks_refs, Lod::L32);
}

// helper for incrementing and constructing chunksrefs
fn make_chunks_refs(s: &mut u64) -> ChunksRefs {
    *s += 1;
    ChunksRefs::make_dummy_chunk_refs(*s)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("CULLED meshing: 1 chunk [ao]", |b| {
        let mut s = 0;
        b.iter_with_setup(|| make_chunks_refs(&mut s), |i| culled_mesh_ao(i))
    });

    c.bench_function("GREEDY meshing OPTIMIZED: 1 chunk [ao]", |b| {
        let mut s = 0;
        b.iter_with_setup(|| make_chunks_refs(&mut s), |i| binary_mesh_optimized(i))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
