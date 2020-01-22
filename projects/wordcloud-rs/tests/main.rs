use quadtree::{Bounded, Bounds, QuadTree};

#[deriving(Eq)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Bounded for Rect {
    fn bounds(&self) -> Bounds {
        Bounds { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

fn main() {
    let mut qt = QuadTree::new(Bounds { x: 0., y: 0., width: 8., height: 8. });
    let r1 = &Rect { x: 0., y: 0., width: 2., height: 2. };
    let r2 = &Rect { x: 3., y: 3., width: 1., height: 1. };
    let r3 = &Rect { x: 1., y: 1., width: 1., height: 1. };
    qt.insert(r1);
    qt.insert(r2);
    qt.insert(r3);
    let results: Vec<&Rect> = qt.query(r3).collect();
    assert!(results.contains(&r1));
}
