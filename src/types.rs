// ==== STRUCT DEFINITIONS ====
pub struct BoundingBox {
	pub x: u32,
	pub y: u32,
	pub width: u32,
	pub height: u32
}

pub struct Counter {
	pub step_size: u32,
	pub position: std::cell::Cell<u32>,
	pub limit: u32,
}

pub struct EdgePoints {
	pub left: u32,
	pub right: u32,
	pub top: u32,
	pub bottom: u32
}

pub struct Edges {
	pub north: Vec<u32>,
	pub south: Vec<u32>,
	pub east: Vec<u32>,
	pub west: Vec<u32>
}

// ==== STRUCT METHODS ====

impl Counter {
	pub fn done(&self) -> bool {
		if self.position.get() >= self.limit { true } else { false }
	}

	pub fn max(&self) -> usize {
		(self.position.get()*2) as usize
	}

	pub fn new(step: u32) -> Self {
		Counter {
			step_size: step,
			position: std::cell::Cell::new(step * 50), // Start at 2nd Quarter
			limit: step * 150 // Stop after the 3rd Quarter
		}
	}

	pub fn next(&self) {
		self.position.set(
			self.position.get() + self.step_size
		);
	}
}

impl EdgePoints {
	pub fn new() -> Self {
		EdgePoints { left: 0, right: 0, top: 0, bottom: 0 }
	}

	pub fn reset(&mut self) {
		self.left = 0;
		self.right = 0;
		self.top = 0;
		self.bottom = 0;
	}
}

impl Edges {
	pub fn new(h_capacity: usize, v_capacity: usize) -> Self {
		Edges {
			north:	Vec::with_capacity(v_capacity),
			south:	Vec::with_capacity(v_capacity),
			east:	Vec::with_capacity(h_capacity),
			west:	Vec::with_capacity(h_capacity)
		}
	}

	pub fn to_bounds(&mut self) -> BoundingBox {
		self.north.sort();
		self.south.sort();
		self.east.sort();
		self.west.sort();

		let n_med: u32 = self.north[self.north.len()/2];
		let s_med: u32 = self.south[self.south.len()/2];
		let e_med: u32 = self.east[self.east.len()/2];
		let w_med: u32 = self.west[self.west.len()/2];

		BoundingBox {
			x: w_med,
			y: n_med,
			width: e_med - w_med,
			height: s_med - n_med
		}
	}

	pub fn push(&mut self, ep: &EdgePoints) {
		if ep.left		!= 0 { self.west.push(ep.left); }
		if ep.right		!= 0 { self.east.push(ep.right); }
		if ep.top		!= 0 { self.north.push(ep.top); }
		if ep.bottom	!= 0 { self.south.push(ep.bottom); }
	}
}
