pub struct Square {
    pub width: u32,
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub trait Area {
    fn graph_area(&self) -> u32;
}

impl Area for Square {
    fn graph_area(&self) -> u32 {
        self.width * self.width
    }
}

impl Area for Rectangle {
    fn graph_area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn graph_area(graph: &impl Area) {
    let graph = graph.graph_area();
    println!("area is {}", graph);
}
