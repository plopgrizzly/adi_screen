// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

pub fn convert(vertices: &mut [[f32; 4]],
	colors: Option<&mut [[f32; 4]]>, tcs: Option<&mut [[f32; 4]]>)
{
	// Sort alternating sides
	let mut ysorted = vec![0]; // indices, start at 0

	let mut ly = 0;
	let mut ry = 0;
	let mut going = true;
	while going {
		ly = if ly > 0 { ly - 1 } else { vertices.len() - 1 };
		ry = if ry < vertices.len() - 1 { ry + 1 } else { 0 };

		if ly == ry {
			ysorted.push(ly);
			going = false;
		} else {
			ysorted.push(ry);
			ysorted.push(ly);

			let nry = if ry < vertices.len() - 1 { ry + 1 } else { 0 };
			if ly == nry {
				going = false;
			}
		}
	}

	// Actually re-order the vertices.
	let mut new: Vec<[f32; 4]> = vec![];
	for k in ysorted.iter() {
		new.push(vertices[*k]);
	}
	vertices[..new.len()].clone_from_slice(&new);

	if let Some(colors) = colors {
		let mut new: Vec<[f32; 4]> = vec![];
		for k in ysorted.iter() {
			new.push(colors[*k]);
		}
		colors[..new.len()].clone_from_slice(&new);
	}

	if let Some(tcs) = tcs {
		let mut new: Vec<[f32; 4]> = vec![];
		for k in ysorted.iter() {
			new.push(tcs[*k]);
		}
		tcs[..new.len()].clone_from_slice(&new);
	}
}
