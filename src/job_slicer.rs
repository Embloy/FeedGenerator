pub fn initialize() {
    use rstar::{RTree, Point};

    let mut tree = RTree::new();
    tree.insert([0.1, 0.0f32, 0.2, 1.0]);
    tree.insert([0.2, 0.1, 0.1, 1.0]);
    tree.insert([0.3, 0.0, 0.4, 1.4]);
    tree.insert([0.3, 0.0, 0.4, 1.6]);
    tree.nearest_neighbor(&[-1., 0.0, 0.2, 0.1]);

    for point in &tree {
        println!("Tree contains a point {:?}", point);
    }
}