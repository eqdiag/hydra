
#include "glfw/glfw3.h"
#include <webgpu/webgpu.h>
#include <glfw3webgpu.h>

#include <cassert>
#include <vector>
#include <iostream>

#include "backends/wgpu/init.h"

int main() {


    if (!glfwInit()) {
        std::cerr << "Couldn't init glfw\n";
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API); // No opengl
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE); //Disable reizing for now

    GLFWwindow* window = glfwCreateWindow(800, 600, "backstage", NULL, NULL);

    if (!window) {
        std::cerr << "Couldn't create window";
        return 1;

    }

    //Instance creation
    WGPUInstanceDescriptor desc_info{};
    desc_info.nextInChain = nullptr;

    WGPUInstance instance = wgpuCreateInstance(&desc_info);

    if (!instance) {
        std::cerr << "Failed to create instance!\n";
        abort();
    }

    std::cout << "INSTANCE: " << instance << std::endl;

    //Create surface
    WGPUSurface surface = glfwGetWGPUSurface(instance, window);

    if (!surface) {
        std::cerr << "Failed to create surface\n";
        abort();
    }

    std::cout << "SURFACE: " << surface << std::endl;


    //Get adapter
    auto adapter_result = backends::wgpu::requestAdapter(instance, surface);
    if (!adapter_result.has_value()) abort();
    auto adapter = adapter_result.value();


    //List adapter features
    backends::wgpu::listAdapterFeatures(adapter);
    

    

    //Create device
    auto device_result = backends::wgpu::requestDevice(adapter,true);
    if (!device_result.has_value()) abort();
    auto device = device_result.value();

    backends::wgpu::listDeviceFeatures(device);
    backends::wgpu::listDeviceLimits(device);

    //Create swapchain

#ifdef WEBGPU_BACKEND_WGPU
    WGPUTextureFormat swapChainFormat = wgpuSurfaceGetPreferredFormat(surface, adapter);
#else
    //Dawn only supports this format
    (void)adapter;
    WGPUTextureFormat swapChainFormat = WGPUTextureFormat_BGRA8Unorm;
#endif
    (void)swapChainFormat;

    auto swapchain_result = backends::wgpu::createSwapchain(surface, adapter, device, 800, 600);
    if (!swapchain_result.has_value()) abort();
    auto swapchain = swapchain_result.value();

    std::cout << "SWAPCHAIN: " << swapchain << std::endl;

    //Get a queue

    WGPUQueue queue = wgpuDeviceGetQueue(device);

    if (!queue) {
        std::cout << "test\n";
    }

    //Shaders
    const char* shader_source = R"(
        @vertex
        fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4f {
            var p = vec2f(0.0, 0.0);
            if (in_vertex_index == 0u) {
                p = vec2f(-0.5, -0.5);
            } else if (in_vertex_index == 1u) {
                p = vec2f(0.5, -0.5);
            } else {
                p = vec2f(0.0, 0.5);
            }
            return vec4f(p, 0.0, 1.0);
        }

        @fragment
        fn fs_main() -> @location(0) vec4f {
            return vec4f(0.0, 0.4, 1.0, 1.0);
        }
    )";

    //Create Render Pipeline

    WGPURenderPipelineDescriptor descriptor{};
    descriptor.nextInChain = nullptr;
    descriptor.label = "my render pipeline";

    descriptor.layout = nullptr;

    descriptor.vertex.nextInChain = nullptr;

  
    auto shader_module = backends::wgpu::createShaderModule(device, backends::wgpu::ShaderSourceType::WGSL, shader_source);

    auto blend_state = backends::wgpu::createAlphaBlendState();

    auto pipeline = backends::wgpu::RenderPipelineBuilder{}
        .setVertexShaderModule(shader_module)
        .setFragmentShaderModule(shader_module)
        .setTopology(WGPUPrimitiveTopology_TriangleList)
        .setCullMode(WGPUFrontFace_CCW, WGPUCullMode_None)
        .addColorTarget(swapChainFormat, blend_state, WGPUColorWriteMask_All)
        .build(device);

    //Don't need module after you create pipeline
    wgpuShaderModuleRelease(shader_module);

    
   
    //Render loop


    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        //Get tex view from swapchain
        
        WGPUTextureView current_texture = wgpuSwapChainGetCurrentTextureView(swapchain);
        if (!current_texture) {
            //Could happen on swapchain resize
            std::cerr << "Failed to get a swapchain texture view\n";
            break;
        }
        //std::cout << "current tex: " << current_texture << std::endl;

        //Draw to it
        
        //First, create a command encoder
        auto encoder = backends::wgpu::createEncoder(device);

        auto color_attachment = backends::wgpu::createColorAttachment(current_texture, WGPULoadOp_Clear, { 1,0,0,1 });
        
        //Begin encoding render pass
        auto renderPass = backends::wgpu::RenderPassBuilder{}
            .addColorAttachment(backends::wgpu::createColorAttachment(current_texture, WGPULoadOp_Clear, { 1,0,0,1 }))
            .build(encoder);


            //Do stuff in the pass here
        
            //Set pipeline
            wgpuRenderPassEncoderSetPipeline(renderPass, pipeline);

            //Issue draw command
            wgpuRenderPassEncoderDraw(renderPass, 3, 1, 0, 0);

        //End the pass
        wgpuRenderPassEncoderEnd(renderPass);

        //Destroy the render pass
        wgpuRenderPassEncoderRelease(renderPass);

        //Create command buffer from encoder
        WGPUCommandBufferDescriptor cmd_buffer_desc{};
        cmd_buffer_desc.nextInChain = nullptr;
        cmd_buffer_desc.label = "my command buffer";
        WGPUCommandBuffer command_buffer = wgpuCommandEncoderFinish(encoder, &cmd_buffer_desc);
        wgpuCommandEncoderRelease(encoder);


        //Submit command buffer

     
        wgpuQueueSubmit(queue, 1, &command_buffer);
        wgpuCommandBufferRelease(command_buffer);

        //Destroy tex view
        wgpuTextureViewRelease(current_texture);
       
        //Present swapchain
        wgpuSwapChainPresent(swapchain);
    }


    
    //Cleanup

    wgpuRenderPipelineRelease(pipeline);

    wgpuQueueRelease(queue);
    wgpuSwapChainRelease(swapchain);
    wgpuDeviceRelease(device);
    wgpuAdapterRelease(adapter);
    wgpuInstanceRelease(instance);    
    wgpuSurfaceRelease(surface);

	
	glfwDestroyWindow(window);

	glfwTerminate();

}