/**
 * adi_screen - Aldaron's Device Interface - Screen - "transforms.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::f32::consts::PI;
use std::ops::Mul;

pub struct Matrix {
	pub data: [f32; 16],
}

impl Matrix {
	pub fn identity() -> Matrix {
		Matrix { data: [
			1., 0., 0., 0.,
			0., 1., 0., 0.,
			0., 0., 1., 0.,
			0., 0., 0., 1.,
		] }
	}

	pub fn perspective(fov: f32) -> Matrix {
		let scale = (fov * 0.5 * PI / 180.).tan().recip();

		Matrix { data: [
			scale,	0.,	 0.,	 0.,
			0.,	scale,	 0.,	 0.,
			0.,	0.,	 1.,	 1.,
			0.,	0.,	-1., 	 1.,
		] }
	}

	pub fn translate(&self, x:f32, y:f32, z:f32) -> Matrix {
		let mut data = self.data;
		data[12] += x;
		data[13] += y;
		data[14] += z;
		Matrix { data: data }
	}

	pub fn scale(&self, x:f32, y:f32, z:f32) -> Matrix {
		let mut data = self.data;
		data[0] *= x;
		data[5] *= y;
		data[15] *= z;
		Matrix { data: data }
	}

	pub fn rotate(&self, yaw:f32, pitch:f32, roll:f32) -> Matrix {
		let num9 = roll * PI;
		let num6 = num9.sin();
		let num5 = num9.cos();
		let num8 = pitch * PI;
		let num4 = num8.sin();
		let num3 = num8.cos();
		let num7 = yaw * PI;
		let num2 = num7.sin();
		let num = num7.cos();

		let qx = ((num * num4) * num5) + ((num2 * num3) * num6);
		let qy = ((num2 * num3) * num5) - ((num * num4) * num6);
		let qz = ((num * num3) * num6) - ((num2 * num4) * num5);
		let qw = ((num * num3) * num5) + ((num2 * num4) * num6);

		let m1 = Matrix { data: [
			qw, qz, -qy, qx,
			-qz, qw, qx, qy,
			qy, -qx, qw, qz,
			-qx, -qy, -qz, qw,
		] };
		let m2 = Matrix { data: [
			qw, qz, -qy, -qx,
			-qz, qw, qx, -qy,
			qy, -qx, qw, -qz,
			qx, qy, qz, qw,
		] };
		Matrix { data: self.data } * (m1 * m2)
	}

	pub fn set_perspective(&self, fov: f32) -> Matrix {
		(Matrix { data: self.data } * Matrix::perspective(fov)).translate(0.0, 0.0, 1.0)
	}
}

impl Mul for Matrix {
	type Output = Matrix;

	fn mul(self, matrix: Matrix) -> Matrix {
		Matrix { data: [
			(self.data[0] * matrix.data[0]) +
			(self.data[1] * matrix.data[4]) +
			(self.data[2] * matrix.data[8]) +
			(self.data[3] * matrix.data[12]),
			(self.data[0] * matrix.data[1]) +
			(self.data[1] * matrix.data[5]) +
			(self.data[2] * matrix.data[9]) +
			(self.data[3] * matrix.data[13]),
			(self.data[0] * matrix.data[2]) +
			(self.data[1] * matrix.data[6]) +
			(self.data[2] * matrix.data[10]) +
			(self.data[3] * matrix.data[14]),
			(self.data[0] * matrix.data[3]) +
			(self.data[1] * matrix.data[7]) +
			(self.data[2] * matrix.data[11]) +
			(self.data[3] * matrix.data[15]),

			(self.data[4] * matrix.data[0]) +
			(self.data[5] * matrix.data[4]) +
			(self.data[6] * matrix.data[8]) +
			(self.data[7] * matrix.data[12]),
			(self.data[4] * matrix.data[1]) +
			(self.data[5] * matrix.data[5]) +
			(self.data[6] * matrix.data[9]) +
			(self.data[7] * matrix.data[13]),
			(self.data[4] * matrix.data[2]) +
			(self.data[5] * matrix.data[6]) +
			(self.data[6] * matrix.data[10]) +
			(self.data[7] * matrix.data[14]),
			(self.data[4] * matrix.data[3]) +
			(self.data[5] * matrix.data[7]) +
			(self.data[6] * matrix.data[11]) +
			(self.data[7] * matrix.data[15]),

			(self.data[8] * matrix.data[0]) +
			(self.data[9] * matrix.data[4]) +
			(self.data[10] * matrix.data[8]) +
			(self.data[11] * matrix.data[12]),
			(self.data[8] * matrix.data[1]) +
			(self.data[9] * matrix.data[5]) +
			(self.data[10] * matrix.data[9]) +
			(self.data[11] * matrix.data[13]),
			(self.data[8] * matrix.data[2]) +
			(self.data[9] * matrix.data[6]) +
			(self.data[10] * matrix.data[10]) +
			(self.data[11] * matrix.data[14]),
			(self.data[8] * matrix.data[3]) +
			(self.data[9] * matrix.data[7]) +
			(self.data[10] * matrix.data[11]) +
			(self.data[11] * matrix.data[15]),

			(self.data[12] * matrix.data[0]) +
			(self.data[13] * matrix.data[4]) +
			(self.data[14] * matrix.data[8]) +
			(self.data[15] * matrix.data[12]),
			(self.data[12] * matrix.data[1]) +
			(self.data[13] * matrix.data[5]) +
			(self.data[14] * matrix.data[9]) +
			(self.data[15] * matrix.data[13]),
			(self.data[12] * matrix.data[2]) +
			(self.data[13] * matrix.data[6]) +
			(self.data[14] * matrix.data[10]) +
			(self.data[15] * matrix.data[14]),
			(self.data[12] * matrix.data[3]) +
			(self.data[13] * matrix.data[7]) +
			(self.data[14] * matrix.data[11]) +
			(self.data[15] * matrix.data[15]),
		] }
	}
}
