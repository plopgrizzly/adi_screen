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
//		println!("V: {:?}", vertices);

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
		let two_d = (0, 1);

		// Step 2. Generate Shapes
		// self.shape(vertices, None, 0, two_d);

		// Add points
		let mut polygon = vec![]; // points

		for i in vertices {
			polygon.push(*i);
		}

		// Make convex shapes.
/*		let mut first = true;
		let mut length = 0;
		while polygon.is_empty() == false {
			self.points.push(vec![]);

			let mut marked = vec![]; // mark vertices for removal
			let mut keep = vec![]; // mark vertices for keeping
			let mut last: Option<((f32,f32),(f32,f32))> = None; // last concave vertex
			// Find concave verticex closest to point 0.
			let mut concaves: Vec<(f32,f32,usize)> = vec![];
			for i in 0..polygon.len() {
				// Is concave?
				let c1 = polygon[if i > 0 { i - 1 } else { polygon.len() - 1 }];
				let c2 = polygon[if i < polygon.len() - 1 { i + 1 } else { 0 }];
				let c = polygon[i];
				let y1 = c1[1];
				let y2 = c2[1];
				let y = c[1];
				let x1 = c1[0];
				let x2 = c2[0];
				let x = c[0];

				if direction((x1, y1), (x, y), (x2, y2)) {
					concaves.push((x,y, i));
				}
			}
			if concaves.is_empty() {
				for i in 0..polygon.len() {
					marked.push(i);
				}
			} else {
				// Find closest concave to 0
				let mut cc: Option<(f32,usize)> = None;
				for i in concaves {
					let x = i.0 - polygon[0][0];
					let y = i.1 - polygon[0][1];
					let d1 = x * x + y * y;

					if let Some(d2) = cc {
						if d1 < d2.0 {
							cc = Some((d1,i.2));
						}
					} else {
						cc = Some((d1,i.2))
					}
				}
				let cc = cc.unwrap();

				// Find a convex shape
				for i in 0..polygon.len() {
					// Start at a convex point
					let mut i = i + cc.1;
					if i >= polygon.len() {
						i -= polygon.len();
					}
					let i = i;

					// Check if convex polygon possible
					if let Some(l) = last {
						// Is concave to breaking vertex?
						let c = polygon[i];
						let y = c[1];
						let x = c[0];

						if !direction(l.0, l.1, (x, y)) {
							last = None;
							marked.push(i);
							keep.push(i);
							println!("BACK {:?}", (x,y));
						} else {
							println!("SKIP {:?}", (x,y));
						}
					} else {
						// Is concave?
						let c1 = polygon[if i > 0 { i - 1 } else { polygon.len() - 1 }];
						let c2 = polygon[if i < polygon.len() - 1 { i + 1 } else { 0 }];
						let c = polygon[i];
						let y1 = c1[1];
						let y2 = c2[1];
						let y = c[1];
						let x1 = c1[0];
						let x2 = c2[0];
						let x = c[0];

						if direction((x1, y1), (x, y), (x2, y2)) {
							println!("Concave {:?}", (x,y));
							last = Some(((x1, y1), (x, y)));
							keep.push(i);
						} else {
							println!("Convex {:?}", (x,y));
						}
						// Add vertex
						marked.push(i);
					}
				}
			}

			// Add marked vertices, and remove if not kept.
			marked.sort();
			marked.reverse();
			for i in marked {
				let v = if keep.contains(&i) {
					polygon[i]
				} else {
					polygon.remove(i)
				};

				self.points[length].push(v);
			}

			println!("VVV: {:?}", self.points[length]);

			length += 1;*/

			/*if polygon.len() < 3 { // 4 ?
				for i in 0..polygon.len() {
					self.points[length].push(polygon.remove(i));
				}
			}
			'a: for i in 0..polygon.len() {
				if i >= polygon.len() { break 'a }

				// Is point a split point (concave)?
				let c1 = polygon[if i > 0 { i - 1 } else { polygon.len() - 1 }];
				let c2 = polygon[if i < polygon.len() - 1 { i + 1 } else { 0 }];
				let c = polygon[i];
				let y1 = c1[1];
				let y2 = c2[1];
				let y = c[1];
				let x1 = c1[0];
				let x2 = c2[0];
				let x = c[0];

				if !direction((x1, y1), (x, y), (x2, y2)) {
					println!("Cv");
					self.points[length].push(polygon.remove(i));

					if first {
						self.points[length].push(polygon[if i == 0 { polygon.len() - 1 } else { i-1 }]);
						first = false;
					}
					// break 'a;
				} else {
					// println!("Cc");
					// concave
					if first == false {
						self.points[length].push(polygon[i]);
						first = true;
						length += 1;
						self.points.push(vec![]);
					}
				}
			}*/
//			panic!("couldn't do it");
//		}

		// Sort points
/*		let mut ysorted = vec![]; // indices
		for i in 0..polygon.len() {
			// Is point a split point (concave)?
			let c1 = polygon[if i > 0 { i - 1 } else { polygon.len() - 1 }];
			let c2 = polygon[if i < polygon.len() - 1 { i + 1 } else { 0 }];
			let c = polygon[i];
			let y1 = c1[1];
			let y2 = c2[1];
			let y = c[1];
			let x1 = c1[0];
			let x2 = c2[0];
			let x = c[0];

			ysorted.push((i,direction((x1, y1), (x, y), (x2, y2))));
		}
		ysorted.sort_by(|x,y|
			if polygon[x.0][1] > polygon[y.0][1] - ::std::f32::EPSILON {
				::std::cmp::Ordering::Greater
			} else if polygon[x.0][1] < polygon[y.0][1] + ::std::f32::EPSILON {
				::std::cmp::Ordering::Less
			} else {
				// Tie-breaker
				if x.1 && !y.1 {
					::std::cmp::Ordering::Greater
				} else if y.1 && !x.1 {
					::std::cmp::Ordering::Less
				} else {
//				if polygon[x.0][0] < polygon[y.0][0] - ::std::f32::EPSILON {
//					::std::cmp::Ordering::Greater
//				} else if polygon[x.0][0] > polygon[y.0][0] + ::std::f32::EPSILON {
//					::std::cmp::Ordering::Less
//				} else {
					::std::cmp::Ordering::Equal
				}
			}
		);*/
//		ysorted.reverse();

		// Break into convex polygons based on split points.
/*		let mut contours = vec![vec![]];
		for i in 0..ysorted.len() {
			if ysorted[i].1 == false {
				contours[contours.len()-1].push(ysorted[i].0);
				continue;
			}

			contours[contours.len()-1].push(ysorted[i].0);
			contours.push(vec![]);
			contours[contours.len()-1].push(ysorted[i].0);
		}
		for i in 0..contours.len() {
			let sp = contours[i][contours[i].len()-1];
			contours[i].sort();
			let sp = contours[i].binary_search(&sp).unwrap();
			let nf = contours[i][if sp == 0 { contours[i].len() - 1 } else { sp - 1 }];
			let fv = if nf > contours[i].len() - 1 { 0 } else { nf + 1 };
		}*/

/*		let mut last_split = None;
		let mut last_fail = None;

		for i in 0..ysorted.len() {
			// Skip if not a split point.
			if ysorted[i].1 == false { continue }
			// Add convex polygon
			self.points.push(vec![]);
			// Index of this split point.
			let p = ysorted[i].0;
			// Last failed added index
			let mut fail = None;
			// Add vertices as long as y doesn't exceed split point
			// (index is less than i)
			let mut j = p;
			loop {
				// Add only if ysort is
				// * less than split point
				// * more than last_split
				// * or is last_split or last_fail
				{
					// find index of j index in ysorted
					let mut index = None;
					for k in 0..ysorted.len() {
						if j == ysorted[k].0 {
							index = Some(k);
						}
					}
					let index = index.unwrap(); // nvr panic
					// if index is less than i = Good and more than/equal to last_split
					if (index < i && index >= last_split.unwrap_or(0)) || last_fail == Some(index) {
						let last_polygon = self.points.len()-1;
						self.points[last_polygon].push(polygon[index]);
					} else {
						// Use point before fail to repeat in next polygon.
						fail = Some(if index == 0 { polygon.len() - 1 } else { index - 1 });
					}
				}
				// Next vertex
				j += 1;
				if j >= polygon.len() { j = 0 }
				// Back to split point
				if j == p { break }
			}
			// Add split point and last failed back
			last_split = Some(p);
			last_fail = fail;
		}*/

		// end

		// Find Split points
		let mut ysorted = vec![]; // indices
		let mut i = 0;
		for i in 0..polygon.len() {
			ysorted.push(i);
		}
		self.polygon(0, None, polygon.as_slice(), ysorted.as_slice(),
			&mut i);

		for i in &self.points {
//			println!("POLYGON: {:?}", i);
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

//			println!("LENGHT {:?}", ysorted);

			// Actually re-order the vertices.
			let mut new: Vec<[f32; 4]> = vec![];

			for k in ysorted.iter() {
				new.push(i[*k]);
			}

			*i = new;
		}

		// Step 3. Sort Vertices for each shape Along 2nd Axis (two_d.1)*/
		for i in &mut self.points {
			i.sort_by(|x,y|
				if x[two_d.1] < y[two_d.1] {
					::std::cmp::Ordering::Greater
				} else if x[two_d.1] > y[two_d.1] {
					::std::cmp::Ordering::Less
				} else {
					::std::cmp::Ordering::Equal
				}
			);
		}
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
					// println!("Found split point: {} {} {}", x, y, j);

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
//		println!("shape");
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
