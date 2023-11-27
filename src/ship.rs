pub const HULL_CLASS_2: usize = 2;
pub const HULL_CLASS_3: usize = 3;
pub const HULL_CLASS_4: usize = 4;
pub const HULL_CLASS_5: usize = 5;

#[derive(Copy, Clone)]
pub enum Heading {
    EastWest,
    NorthSouth,
}
pub struct Ship {
    name: String,
    length: usize,
    heading: Heading,
    location: Vec<(usize, usize)>,
    hits: Vec<(usize, usize)>,
}

impl Ship {
    pub fn new(name: &str, length: usize) -> Self {
        Self {
            name: String::from(name),
            length: length,
            heading: Heading::EastWest,
            location: Vec::new(),
            hits: Vec::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn get_heading(&self) -> Heading {
        self.heading.clone()
    }

    pub fn set_heading(&mut self, heading: Heading) {
        self.heading = heading;
    }

    pub fn locate(&mut self, point: (usize, usize)) {
        self.location.push(point);
    }
}
