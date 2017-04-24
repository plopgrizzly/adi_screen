/**
 * Aldaron's Device Interface - "vw.h"
 * Copyright 2017 (c) Jeron Lau - Licensed under the GNU GENERAL PUBLIC LICENSE
**/

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#if defined(__ANDROID__) // Android
	#define VK_USE_PLATFORM_ANDROID_KHR
#elif defined(_WIN32) // Windows
	#define VK_USE_PLATFORM_WIN32_KHR
#else // Linux / Mac
	#define VK_USE_PLATFORM_XCB_KHR
#endif

//#define VK_NO_PROTOTYPES
#include "vulkan/vulkan.h"

#define vw_EVENT_REDRAW 0
#define vw_EVENT_RESIZE 1

typedef struct {
	#if defined(VK_USE_PLATFORM_ANDROID_KHR)
        
	#elif defined(VK_USE_PLATFORM_WIN32_KHR)
        HINSTANCE connection;
	HWND window;
	#else
	xcb_connection_t *connection;
	xcb_screen_t *screen;
	xcb_window_t window;
	#endif
} vw_window_t;

typedef struct {
	VkInstance instance; // Vulkan instance
	VkSurfaceKHR surface; // Surface that we render to.
	uint32_t present_queue_index;
	VkQueue present_queue;
	VkPhysicalDevice gpu;
	VkDevice device; // The logical device
	VkCommandBuffer command_buffer;
	VkSwapchainKHR swapchain;
	uint32_t width, height; // Swapchain Dimensions.
	VkImage present_images[2]; // 2 for double-buffering
	VkFramebuffer frame_buffers[2]; // 2 for double-buffering
	VkFormat color_format;
	uint32_t image_count; // 1 (single-buffering) or 2 (double-buffering)
	VkFence submit_fence; // The submit fence
	VkImageView present_image_views[2]; // 2 for double-buffering
	VkImage depth_image;
	VkImageView depth_image_view;
	VkDeviceMemory depth_image_memory;
	VkRenderPass render_pass;
	uint32_t next_image_index;
	VkSemaphore presenting_complete_sem, rendering_complete_sem;
	VkDeviceSize offset;
	VkCommandPool cmd_pool;
	uint8_t do_draw;
} vw_t;

typedef struct {
	VkShaderModule vertex;
	VkShaderModule fragment;
	uint32_t textures;
} vw_shader_t;

typedef struct {
	VkPipeline pipeline;
	VkDescriptorSetLayout descsetlayout;
	VkPipelineLayout pipeline_layout;
} vw_pipeline_t;

typedef struct {
	VkBuffer matrix_buffer;
	VkDeviceMemory uniform_memory;
	VkDescriptorSet desc_set;
	VkDescriptorPool desc_pool;
} vw_instance_t;

typedef struct {
	VkDeviceMemory vertex_buffer_memory;
	VkBuffer vertex_input_buffer;
	uint32_t vertice_count;
	vw_pipeline_t* pipeline;
} vw_shape_t;

typedef struct {
	VkImage mappable_image;
	VkDeviceMemory mappable_memory;
	VkImage image;
	VkDeviceMemory memory;
	VkSampler sampler;
	VkImageView view;
	uint32_t w, h;
	uint32_t size;
	uint32_t pitch;
	uint8_t staged;
} vw_texture_t;

// Vulkan loaders.
//VKAPI_ATTR PFN_vkVoidFunction VKAPI_CALL vkGetInstanceProcAddr(VkInstance instance, const char *pName);

void vw_fullscreen(vw_window_t window);
void vw_vulkan_matrix(vw_t*, const float*);
void vw_vulkan_shape(vw_shape_t* shape, vw_t vulkan, const float* v,
	uint32_t size);
vw_texture_t vw_vulkan_texture(vw_t*, uint32_t, uint32_t, const uint8_t*,
	uint8_t ka, uint8_t kr, uint8_t kg, uint8_t kb);
void vw_vulkan_shader(vw_shader_t*,vw_t,void*,uint32_t,void*,uint32_t);
void vw_vulkan_pipeline(vw_pipeline_t*, vw_t*, vw_shader_t*, uint32_t);
void vw_vulkan_resize(vw_t*);
void vw_vulkan_swapchain_delete(vw_t* vulkan);
void vw_vulkan_draw_begin(vw_t* vulkan, float r, float g, float b);
void vw_vulkan_draw_shape(vw_t*, vw_shape_t*, const float*, vw_instance_t);
void vw_vulkan_draw_update(vw_t* vulkan);
void vw_close(vw_t wrapper);
