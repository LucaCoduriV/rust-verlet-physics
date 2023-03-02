use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quadtree::quad_tree::{QuadTree, Aabb};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const HEIGHT:u32 = 1000;
const WIDTH:u32 = 1000;

fn criterion_benchmark(c: &mut Criterion) {
    let mut objects:Vec<Aabb> = black_box(Vec::new());
    let mut rng = StdRng::seed_from_u64(42);
    let mut quad_tree = QuadTree::new(Aabb::new(0, 0., 0., WIDTH as f32, HEIGHT as f32), 30,);

    for id in 0..200 {
        let bb = Aabb {
            id: id,
            x: rng.gen_range(0..WIDTH-20) as f32,
            y: rng.gen_range(0..HEIGHT-20) as f32,
            width: 20.,
            height: 20.,
        };
        objects.push(bb);
    }



    c.bench_function("Quadtree fin all intersection", |b| b.iter(|| {
        for o in objects.iter(){
            quad_tree.insert(o.clone());
        }
        quad_tree.find_all_intersection();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);