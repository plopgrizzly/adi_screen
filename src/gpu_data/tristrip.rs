// gpu_data/tristrip.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

fn normalize(oa: (f32, f32)) -> (f32, f32) {
	let magnitude = ((oa.0 * oa.0) + (oa.1 * oa.1)).sqrt();

	(oa.0 / magnitude, oa.1 / magnitude)
}

fn dot_product(oa: (f32, f32), ob: (f32, f32)) -> f32 {
	(oa.0 * ob.0) + (oa.1 * ob.1)
}

fn perp(oa: (f32, f32)) -> (f32, f32) {
	(-oa.1, oa.0)
}

pub struct TriStrip {
	pub points: Vec<Vec<[f32; 4]>>,
}

impl TriStrip {
	pub fn new() -> Self {
		TriStrip {
			points: vec![],
		}
	}

	pub fn push(&mut self, vertices: &[[f32; 4]]) {
		println!("V: {:?}", vertices);

		// Step 1. Which side has least change not included in 2D.
/*		let mut xmin = vertices[0][0];
		let mut xmax = vertices[0][0];
		let mut ymin = vertices[0][1];
		let mut ymax = vertices[0][1];
		let mut zmin = vertices[0][2];
		let mut zmax = vertices[0][2];
		for i in vertices.iter().skip(1) {
			xmin = xmin.min(i[0]);
			xmax = xmax.max(i[0]);
			ymin = ymin.min(i[1]);
			ymax = ymax.max(i[1]);
			zmin = zmin.min(i[2]);
			zmax = zmax.max(i[2]);
		}
		let xdif = xmax - xmin;
		let ydif = ymax - ymin;
		let zdif = zmax - zmin;
		let two_d = if xdif < ydif {
			if xdif < zdif {
				(1, 2)
			} else {
				(0, 1)
			}
		} else {
			if ydif < zdif {
				(0, 2)
			} else {
				(0, 1)
			}
		};*/
//		let two_d = (0, 1);

		// Step 2. Generate Shapes
		// self.shape(vertices, None, 0, two_d);

		// Add points
		let mut polygon = vec![]; // points

		for i in vertices {
			polygon.push(*i);
		}

		// Find Split points
		let mut ysorted = vec![]; // indices
		let mut i = 0;
		for i in 0..polygon.len() {
			ysorted.push(i);
		}
		self.polygon(0, None, polygon.as_slice(), ysorted.as_slice(),
			&mut i);

		for i in &self.points {
			println!("POLYGON: {:?}", i);
		}

		for i in &mut self.points {
			ysorted.clear();
			for j in 0..i.len() {
				ysorted.push(j);
			}

			// Find Min Point
			ysorted.sort_by(|x,y|
				if i[*x][1] < i[*y][1] - ::std::f32::EPSILON {
					::std::cmp::Ordering::Greater
				} else if i[*x][1] > i[*y][1] + ::std::f32::EPSILON {
					::std::cmp::Ordering::Less
				} else {
					if i[*x][0] < i[*y][0] - ::std::f32::EPSILON {
						::std::cmp::Ordering::Greater
					} else if i[*x][0] > i[*y][0] + ::std::f32::EPSILON {
						::std::cmp::Ordering::Less
					} else {
						::std::cmp::Ordering::Equal
					}
				}
			);

	//		let mut polygon_stack = vec![0];
	//		let mut polygon_count = 1; // Next Index

			let least_y = ysorted[0];

			// Sort Alternating Either side.
			ysorted.clear();
			ysorted.push(least_y);
			let mut ly = least_y;
			let mut ry = least_y;
			let mut going = true;
//			let mut left = {
//				let y1 = if ly > 0 { ly - 1 } else { i.len() - 1 };
//				let y2 = if ry < i.len() - 1 { ry + 1 } else { 0 };
//				i[y1][1] < i[y2][1]
//			};
			while going {
				ly = if ly > 0 { ly - 1 } else { i.len() - 1 };
				ry = if ry < i.len() - 1 { ry + 1 } else { 0 };

				if ly == ry {
					ysorted.push(ly);
					going = false;
				} else {
//					if left {
//						ysorted.push(ly);
//						ysorted.push(ry);
//					} else {
						ysorted.push(ry);
						ysorted.push(ly);
//					}

					let nry = if ry < i.len() - 1 { ry + 1 } else { 0 };
					if ly == nry {
						going = false;
					}
				}
			}

			println!("LENGHT {:?}", ysorted);

			// Actually re-order the vertices.
			let mut new: Vec<[f32; 4]> = vec![];

			for k in ysorted.iter() {
				new.push(i[*k]);
			}

			*i = new;
		}

		// Step 3. Sort Vertices for each shape Along 2nd Axis (two_d.1)
		/*for i in &mut self.points {
			i.sort_by(|x,y|
				if x[two_d.1] < y[two_d.1] {
					::std::cmp::Ordering::Greater
				} else if x[two_d.1] > y[two_d.1] {
					::std::cmp::Ordering::Less
				} else {
					::std::cmp::Ordering::Equal
				}
			);
		}*/
	}

	fn polygon(&mut self, sh: usize, older: Option<(f32,f32)>,
		polygon: &[[f32;4]], ysorted: &[usize], i: &mut usize)
		-> Option<[f32;4]>
	{
		let mut ignorance = true;

		self.points.push(vec![]);

		while let Some(j) = ysorted.get(*i) {
			let j = *j;
			let c1 = polygon[if j > 0 { j - 1 } else { polygon.len() - 1 }];
			let c2 = polygon[if j < polygon.len() - 1 { j + 1 } else { 0 }];
			let c = polygon[j];
			let y1 = c1[1];
			let y2 = c2[1];
			let y = c[1];
			let x1 = c1[0];
			let x2 = c2[0];
			let x = c[0];

			if ignorance == false {
				// is a split point
				if direction((x1, y1), (x, y), (x2, y2))
//				if (y1 < y - ::std::f32::EPSILON && y2 < y - ::std::f32::EPSILON) || (y1 > y + ::std::f32::EPSILON && y2 > y + ::std::f32::EPSILON)
//					|| (y == y1 || y == y2)
				{
					println!("Found split point: {} {} {}", x, y, j);

					// Add new convex polygon
					let next_sh = self.points.len();
					let v = self.polygon(next_sh, Some((x,y)),
						polygon, ysorted, i);
					if let Some(w) = v {
						self.points[sh].push(w);
					}
				} else {
					// Add point to this convex polygon
					self.points[sh].push(polygon[j]);
				}

				// split sector can be finished.
				if let Some(k) = older {
					// wouldn't be a split point.
					if !direction(k, (x, y), (x2, y2))
					// if !((y1 < k - ::std::f32::EPSILON && y2 < k - ::std::f32::EPSILON) || (y1 > k + ::std::f32::EPSILON && y2 > k + ::std::f32::EPSILON)
						/*|| (y == y1 || y == y2))*/
					{
						return Some(polygon[j]);
					}
				}
			} else {
				// Add point to this convex polygon
				self.points[sh].push(polygon[j]);
			}

			*i += 1;

			ignorance = false;
		}
		None
	}

	// If older is Some, try and close shape.
	fn shape(&mut self, vertices: &[[f32; 4]], mut older: Option<(f32,f32)>,
		sh: usize, two_d: (usize, usize)) -> usize
	{
		println!("shape");
		if sh > 100 { panic!("too many"); }

		self.points.push(vec![]);
		self.points[sh].push(vertices[0]); // 1st vertex
		self.points[sh].push(vertices[1]); // 2nd vertex, first side
		let mut prev = normalize(( // Normalize First Side
			vertices[1][two_d.0] - vertices[0][two_d.0],
			vertices[1][two_d.1] - vertices[0][two_d.1]));
		let mut this;

		for i in 2..vertices.len() { // Rest of vertices
			this = normalize(( // Normalize Next Side
				vertices[i][two_d.0] - vertices[i-1][two_d.0],
				vertices[i][two_d.1] - vertices[i-1][two_d.1]));
			let mut dot = dot_product(this, perp(prev));

			if dot > 0.0 { // Shape isn't convex, create new shape
				// Convex section
				let next_sh = self.points.len();
				let i = self.shape(&vertices[(i-1)..],
					Some(this), next_sh, two_d);
				// Rest of this section
				return self.shape(&vertices[i..], older, sh,
					two_d);
			} else if let Some(normal) = older {
				dot = dot_product(this, perp(normal));

				if dot > 0.0 {
					// still convex
					self.points[sh].push(vertices[i]);
				} else {
					// can close this shape
					return i;
				}
			} else {
				self.points[sh].push(vertices[i]);
			}
			prev = this;
		}

		return ::std::usize::MAX;
	}
}

// Returns true or false depending on clockwise or counter-clockwise
fn direction(v1: (f32, f32), v2: (f32, f32), v3: (f32, f32)) -> bool {
	let side1 = normalize(( // Normalize First Side
		v2.0 - v1.0,
		v2.1 - v1.1));
	let side2 = normalize(( // Normalize Next Side
		v3.0 - v2.0,
		v3.1 - v2.1));

	let mut dot = dot_product(side2, perp(side1));

	dot < 0.0
}
