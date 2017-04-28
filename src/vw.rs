/**
 * adi_screen - Aldaron's Device Interface - Screen - "vw.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ptr::null_mut;

use transforms::Matrix;

use Screen;
use image::{ Image };
use ffi::{ NativeWindow, vulkan };

type VkInstance = usize;
type VkQueue = usize;
type VkPhysicalDevice = usize;
type VkDevice = usize;
type VkCommandBuffer = usize;

type VkSurface = u64;
type VkSwapchain = u64;
type VkImage = u64;
type VkFramebuffer = u64;
type VkFence = u64;
type VkDescriptorPool = u64;
type VkCommandPool = u64;
type VkDescriptorSetLayout = u64;
type VkDescriptorSet = u64;
type VkImageView = u64;
type VkDeviceMemory = u64;
type VkRenderPass = u64;
type VkPipelineLayout = u64;
type VkSemaphore = u64;
type VkBuffer = u64;
type VkShaderModule = u64;
type VkSampler = u64;
pub type VkPipeline = u64;

type VkC = u32; // Size of enum is 4 bytes

#[repr(C)]
#[derive(Copy, Clone)] // TODO: don't copy this.
pub struct Vw {
	pub instance: VkInstance, // Vulkan instance
	surface: VkSurface, // Surface that we render to.
	present_queue_index: u32,
	present_queue: VkQueue,
	gpu: VkPhysicalDevice,
	device: VkDevice, // The logical device
	command_buffer: VkCommandBuffer,
	swapchain: VkSwapchain,
	width:u32, height:u32, // Swapchain Dimensions.
	present_images: [VkImage; 2], // 2 for double-buffering
	frame_buffers: [VkFramebuffer; 2], // 2 for double-buffering
	color_format: VkC, // VkFormat
	image_count: u32, // 1 (single-buffering) or 2 (double-buffering)
	submit_fence: VkFence, // The submit fence
	present_image_views: [VkImageView; 2], // 2 for double-buffering
	depth_image: VkImage,
	depth_image_view: VkImageView,
	depth_image_memory: VkDeviceMemory,
	render_pass: VkRenderPass,
	next_image_index: u32,
	presenting_complete_sem: VkSemaphore,
	rendering_complete_sem: VkSemaphore,
	offsets: u64, // VkDeviceSize
	command_pool: VkCommandPool,
	pub do_draw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Shader {
	vertex: VkShaderModule,
	fragment: VkShaderModule,
	textures: u32,
}

impl Shader {
	pub fn new(screen: &Screen, vert: &'static [u8], frag:&'static [u8],
		textures: u32) -> Shader
	{
		let mut shader = Shader { vertex: 0, fragment: 0,
			textures: textures };
		unsafe { vw_vulkan_shader(&mut shader, screen.vw, &vert[0],
			vert.len() as u32, &frag[0], frag.len() as u32); }
		shader
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Style {
	pipeline: VkPipeline,
	pub descsetlayout: VkDescriptorSetLayout,
	pipeline_layout: VkPipelineLayout,
}

#[repr(C)]
#[derive(PartialEq)]
pub struct Texture {
	mappable_image: VkImage,
	mappable_memory: VkDeviceMemory,
	image: VkImage,
	memory: VkDeviceMemory,
	sampler: VkSampler,
	view: VkImageView,
	w: u32,
	h: u32,
	size: u32,
	pitch: u32,
	staged: u8,
}

impl Texture {
	pub fn opaque(screen: &mut Screen, icon: &'static [u8]) -> Texture {
		let icon = Image::load(icon);
		unsafe {
			vw_vulkan_texture(&mut screen.vw, icon.size.0,
				icon.size.1, &icon.pixels[0], 0, 0, 0, 0)
		}
	}

	pub fn akeyed(screen: &mut Screen, icon: &'static [u8],
		key: (u8,u8,u8)) -> Texture
	{
		let icon = Image::load(icon).alpha_key(key);

		unsafe {
			vw_vulkan_texture(&mut screen.vw, icon.size.0,
				icon.size.1, &icon.pixels[0], 1,
				key.0, key.1, key.2)
		}
	}
}

#[repr(C)]
pub struct VwShape {
	vertex_buffer_memory: VkDeviceMemory,
	vertex_input_buffer: VkBuffer,
	vertice_count: u32,
	pub pipeline: *const Style,
}

pub struct Shape {
	shape: VwShape,
	hastx: bool,
	pub pipeline: Style,
	instances: Vec<VwLinkedInstance>,
	screen: *mut Screen,
}

impl Shape {
	pub fn colored(screen: &mut Screen, v: &[f32], pipeline: &Style)
		-> Shape
	{
		let size = v.len() as u32;
		let mut shape = VwShape {
			vertex_buffer_memory: 0,
			vertex_input_buffer: 0,
			vertice_count: size / 8,
			pipeline: pipeline,
		};
		unsafe { vw_vulkan_shape(&mut shape, screen.vw, &v[0], size); }
		Shape {
			shape: shape,
			hastx: false,
			pipeline: *pipeline,
			instances: Vec::new(),
			screen: screen,
		}
	}

	pub fn textured(screen: &mut Screen, v: &[f32], pipeline: &Style)
		-> Shape
	{
		let size = v.len() as u32;
		let mut shape = VwShape {
			vertex_buffer_memory: 0,
			vertex_input_buffer: 0,
			vertice_count: size / 8,
			pipeline: pipeline,
		};
		unsafe { vw_vulkan_shape(&mut shape, screen.vw, &v[0], size); }
		Shape {
			shape: shape,
			hastx: true,
			pipeline: *pipeline,
			instances: Vec::new(),
			screen: screen,
		}
	}

	pub fn animate(&mut self, i: usize, texture: *const Texture,
		matrix: &Matrix)
	{
		unsafe {
			vw_vulkan_txuniform(&((*self.screen).vw),
				&mut self.instances[i].instance, texture,
				if self.hastx { 1 } else { 0 });
		}
		self.matrix(i, matrix);
	}

/*	pub fn add(mut self, texture: *const Texture, matrix: &Matrix) -> Shape{
		let mem = VwLinkedInstance {
			instance: unsafe {
				vw_vulkan_uniforms((*self.screen).vw,
					&self.shape, texture,
					if self.hastx { 1 } else { 0 })
			},
			texture: texture,
			matrix: matrix.data,
		};
		vulkan::copy_memory( unsafe{ (*self.screen).vw.device },
			mem.instance.uniform_memory,
			&mem.matrix);
		self.instances.push(mem);
		self
	}*/

	pub fn texclone(&mut self, matrix: &Matrix, tx: *const Texture) {
		let mem = VwLinkedInstance {
			instance: unsafe {
				vw_vulkan_uniforms(&((*self.screen).vw),
					&self.shape, tx,
					if self.hastx { 1 } else { 0 })
			},
			texture: tx,
			matrix: matrix.data,
		};
		let device = unsafe{ (*self.screen).vw.device };
		let memory = mem.instance.uniform_memory;
		println!("DEVICE {0} TEXCLONE {1}", device, memory);
		vulkan::copy_memory(device, memory, &mem.matrix);
		self.instances.push(mem);
	}

	pub fn clone(&mut self, matrix: &Matrix) {
		let tx = if self.hastx {
			self.instances[0].texture
		}else{
			null_mut()
		};
		self.texclone(matrix, tx);
	}

	pub fn draw(&self) {
		for i in 0..self.instances.len() {
			unsafe {
				vw_vulkan_draw_shape(&mut (*self.screen).vw,
					&self.shape,
					&self.instances[i].matrix[0],
					self.instances[i].instance);
			}
			vulkan::cmd_draw(unsafe {
					(*self.screen).vw.command_buffer
				}, self.shape.vertice_count);
		}
	}

	pub fn matrix(&mut self, i: usize, matrix: &Matrix) {
		self.instances[i].matrix = matrix.data;
		vulkan::copy_memory(unsafe { (*self.screen).vw.device },
			self.instances[i].instance.uniform_memory,
			&self.instances[i].matrix);
	}

/*	pub fn draw_index(&self, i: usize) {
		unsafe {
			vw_vulkan_draw_shape(&mut (*self.screen).vw,
				&self.shape, &self.instances[i].matrix[0],
				self.instances[i].instance);
		}
		vulkan::cmd_draw(unsafe {(*self.screen).vw.command_buffer},
			self.shape.vertice_count);
	}*/

	pub fn vertices(&self, v: &[f32]) {
		vulkan::copy_memory( unsafe { (*self.screen).vw.device },
			self.shape.vertex_buffer_memory, v);
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VwInstance {
	matrix_buffer: VkBuffer,
	uniform_memory: VkDeviceMemory,
	pub desc_set: VkDescriptorSet,
	pub desc_pool: VkDescriptorPool,
}

#[derive(Copy, Clone)]
pub struct VwLinkedInstance {
	instance: VwInstance,
	texture: *const Texture,
	matrix: [f32; 16],
}

extern {
	fn vw_vulkan_shape(a: *mut VwShape, b: Vw, c: *const f32, d: u32) -> ();
	fn vw_vulkan_texture(a: *mut Vw, b: u32, c: u32, d: *const u8, e: u8,
		f: u8, g: u8, h: u8) -> Texture;
	fn vw_vulkan_shader(a: *mut Shader, b: Vw, c: *const u8, d: u32,
		e: *const u8, f: u32) -> ();
	fn vw_vulkan_pipeline(z: *mut Style, a: *mut Vw, b: *const Shader,
		c: u32);
	fn vw_vulkan_draw_begin(v: *mut Vw, r: f32, g: f32, b: f32) -> ();
	fn vw_vulkan_txuniform(vw: *const Vw, b: *mut VwInstance,
		c: *const Texture, d: u8) -> ();
	fn vw_vulkan_uniforms(a: *const Vw, b: &VwShape, c: *const Texture,
		d: u8) -> VwInstance;
	fn vw_vulkan_draw_shape(v: *mut Vw, s: *const VwShape, e: *const f32,
		f: VwInstance) -> ();
	fn vw_vulkan_draw_update(v: *mut Vw) -> ();
	fn vw_vulkan_resize(v: *mut Vw) -> ();
	fn vw_vulkan_swapchain_delete(v: *mut Vw) -> ();
}

pub fn open(window_name: &str, native: &NativeWindow) -> Vw {
	let instance = vulkan::Instance::create(window_name);
	let surface = vulkan::Surface::create(&instance, native);
	let gpu = vulkan::Gpu::create(&surface);
	let gpu_interface = vulkan::GpuInterface::create(&gpu);
	let queue = vulkan::Queue::create(&gpu_interface, &gpu);
	let command_buffer = vulkan::CommandBuffer::create(&gpu_interface,&gpu);

	let mut vw = Vw {
		instance: instance.native,
		surface: surface.native,
		present_queue_index: gpu.present_queue_index,
		present_queue: queue.native,
		gpu: gpu.native,
		device: gpu_interface.native,
		command_buffer: command_buffer.native,
		swapchain: 0,
		width: 0, height: 0,
		present_images: [0, 0],
		frame_buffers: [0, 0],
		color_format: 0,
		image_count: 0,
		submit_fence: 0,
		present_image_views: [0, 0],
		depth_image: 0,
		depth_image_view: 0,
		depth_image_memory: 0,
		render_pass: 0,
		next_image_index: 0,
		presenting_complete_sem: 0,
		rendering_complete_sem: 0,
		offsets: 0,
		command_pool: 0, // TODO: not needed.
		do_draw: 0,
	};

	unsafe {
		vw_vulkan_resize(&mut vw);
	}

	vw
}

pub fn make_styles(screen: &mut Screen, extrashaders: &[Shader]) -> Vec<Style> {
	let mut shadev = Vec::new();
	let default_shaders = [
		Shader::new(screen, include_bytes!("res/color-vert.spv"),
			include_bytes!("res/color-frag.spv"), 0),
		Shader::new(screen, include_bytes!("res/texture-vert.spv"),
			include_bytes!("res/texture-frag.spv"), 1),
	];
	shadev.extend(default_shaders.iter().cloned());
	shadev.extend(extrashaders.iter().cloned());

	let mut pipeline = vec![Style { pipeline: 0, descsetlayout: 0,
		pipeline_layout: 0 }; shadev.len()];
	unsafe {
		vw_vulkan_pipeline(&mut pipeline[0], &mut screen.vw, &shadev[0],
			shadev.len() as u32);
	}
	pipeline
}

pub fn resize(screen: &mut Screen) {
	if screen.vw.width==screen.size.0 && screen.vw.height==screen.size.1 {
		return;
	}
	screen.vw.width = screen.size.0;
	screen.vw.height = screen.size.1;
	unsafe {
		vw_vulkan_swapchain_delete(&mut screen.vw);
		vw_vulkan_resize(&mut screen.vw);
	}
}

pub fn draw_clear(screen: &mut Screen, r:f32, g:f32, b:f32) {
	unsafe { vw_vulkan_draw_begin(&mut screen.vw, r, g, b); }
}

pub fn draw_update(screen: &mut Screen) {
	unsafe { vw_vulkan_draw_update(&mut screen.vw); }
}

pub fn close(vw: &mut Vw) {
	unsafe { vw_vulkan_swapchain_delete(vw); }
}
