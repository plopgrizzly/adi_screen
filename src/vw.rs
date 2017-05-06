/**
 * adi_screen - Aldaron's Device Interface - Screen - "vw.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use style;
use Window;
use window::WindowFunctions;
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
	pub fn create(vw: &Vw, vert: &'static [u8], frag:&'static [u8],
		textures: u32) -> Shader
	{
		let mut shader = Shader { vertex: 0, fragment: 0,
			textures: textures };
		unsafe {
			vw_vulkan_shader(&mut shader, *vw, &vert[0],
				vert.len() as u32, &frag[0], frag.len() as u32);
		}
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
#[derive(PartialEq,Copy,Clone)]
pub struct NativeTexture {
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

#[repr(C)]
pub struct VwShape {
	vertex_buffer_memory: VkDeviceMemory,
	vertex_input_buffer: VkBuffer,
	vertice_count: u32,
	pub pipeline: Style,
}

pub struct Shape {
	shape: VwShape,
	hastx: bool,
	instances: Vec<VwLinkedInstance>,
}

impl Shape {
	pub fn create(window: &mut Window, v: &[f32], style: style::Style) -> Shape {
		let size = v.len() as u32;
		let mut shape = VwShape {
			vertex_buffer_memory: 0,
			vertex_input_buffer: 0,
			vertice_count: size / 8,
			pipeline: window.shader(match style {
				style::Style::Solid(shader) |
				style::Style::CustomSolid(shader) |
				style::Style::Opaque(shader, _) |
				style::Style::Subtransparent(shader, _) |
				style::Style::CustomTexture(shader, _)
					=> shader,
				style::Style::Invisible => !0
			}),
		};
		unsafe { vw_vulkan_shape(&mut shape, window.vw, &v[0], size); }
		let hastx = match style {
			style::Style::Invisible |
			style::Style::Solid(_) |
			style::Style::CustomSolid(_)
				=> false,
			style::Style::Opaque(_, _) |
			style::Style::Subtransparent(_, _) |
			style::Style::CustomTexture(_, _)
				=> true,
		};
		Shape {
			shape: shape,
			hastx: hastx,
			instances: Vec::new(),
		}
	}

	pub fn animate(window: &mut Window, index: usize, i: usize,
		texture: *const NativeTexture)
	{
		unsafe {
			vw_vulkan_txuniform(&window.vw,
				&mut window.sprites[index].shape.instances[i].instance, texture,
				if window.sprites[index].shape.hastx { 1 } else { 0 });
		}
	}

	pub fn add(window: &mut Window, index: usize, tx: *const NativeTexture) {
		let shape = &mut window.sprites[index].shape;
		let mem = VwLinkedInstance {
			instance: unsafe {
				vw_vulkan_uniforms(&window.vw,
					&shape.shape, tx,
					if shape.hastx { 1 } else { 0 })
			},
			matrix: [ 1.0, 0.0, 0.0, 0.0,	0.0, 1.0, 0.0, 0.0,
				  0.0, 0.0, 1.0, 0.0,	0.0, 0.0, 0.0, 1.0],
		};
		vulkan::copy_memory(window.vw.device,
			mem.instance.uniform_memory, &mem.matrix);
		shape.instances.push(mem);
	}

//	pub fn clone(&mut self, transform: [f32; 16]) {
//		let tx = if self.hastx {
//			self.instances[0].texture
//		}else{
//			null_mut()
//		};
//		self.texclone(transform, tx);
//	}

	pub fn draw(window: &mut Window, index: usize) {
		let shape = &window.sprites[index].shape;
		if !window.sprites[index].enabled { return; }
		for i in 0..shape.instances.len() {
			unsafe {
				vw_vulkan_draw_shape(&mut window.vw,
					&shape.shape,
					&shape.instances[i].matrix[0],
					shape.instances[i].instance);
			}
			vulkan::cmd_draw(window.vw.command_buffer,
				shape.shape.vertice_count);
		}
	}

	pub fn matrix(window: &mut Window, index: usize, i: usize,
		matrix: [f32; 16])
	{
		window.sprites[index].shape.instances[i].matrix = matrix;
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].shape.instances[i].instance.uniform_memory,
			&window.sprites[index].shape.instances[i].matrix);
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

	pub fn vertices(window: &Window, index: usize, v: &[f32]) {
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].shape.shape.vertex_buffer_memory, v);
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
	matrix: [f32; 16],
}

extern {
	fn vw_vulkan_shape(a: *mut VwShape, b: Vw, c: *const f32, d: u32) -> ();
//
	fn vw_vulkan_shader(a: *mut Shader, b: Vw, c: *const u8, d: u32,
		e: *const u8, f: u32) -> ();
	fn vw_vulkan_pipeline(z: *mut Style, a: *mut Vw, b: *const Shader,
		c: u32);
	fn vw_vulkan_draw_begin(v: *mut Vw, r: f32, g: f32, b: f32) -> ();
	fn vw_vulkan_txuniform(vw: *const Vw, b: *mut VwInstance,
		c: *const NativeTexture, d: u8) -> ();
	fn vw_vulkan_uniforms(a: *const Vw, b: &VwShape, c: *const NativeTexture,
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

pub fn make_styles(vw: &mut Vw, extrashaders: &[Shader], shaders: &mut Vec<Style>)
{
	let mut shadev = Vec::new();
	let default_shaders = [
		Shader::create(vw, include_bytes!("res/color-vert.spv"),
			include_bytes!("res/color-frag.spv"), 0),
		Shader::create(vw, include_bytes!("res/texture-vert.spv"),
			include_bytes!("res/texture-frag.spv"), 1),
	];
	shadev.extend(default_shaders.iter().cloned());
	shadev.extend(extrashaders.iter().cloned());

	*shaders = vec![Style { pipeline: 0, descsetlayout: 0,
		pipeline_layout: 0 }; shadev.len()];
	unsafe {
		vw_vulkan_pipeline(&mut shaders[0], vw, &shadev[0],
			shadev.len() as u32);
	}
}

pub fn resize(window: &mut Window) {
	let size = window.dim();

	if window.vw.width == size.0 && window.vw.height == size.1 {
		return;
	}
	window.vw.width = size.0;
	window.vw.height = size.1;
	unsafe {
		vw_vulkan_swapchain_delete(&mut window.vw);
		vw_vulkan_resize(&mut window.vw);
	}
}

pub fn draw_clear(window: &mut Window, r:f32, g:f32, b:f32) {
	unsafe { vw_vulkan_draw_begin(&mut window.vw, r, g, b); }
}

pub fn draw_update(window: &mut Window) {
	unsafe { vw_vulkan_draw_update(&mut window.vw); }
}

pub fn close(vw: &mut Vw) {
	unsafe { vw_vulkan_swapchain_delete(vw); }
}
