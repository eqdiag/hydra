
#include "glfw/glfw3.h"
#include <webgpu/webgpu.h>
#include <glfw3webgpu.h>

#include <cassert>
#include <vector>
#include <iostream>


int main() {


    if (!glfwInit()) {
        std::cerr << "Couldn't init glfw\n";
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API); // No opengl
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE); //Disable reizing for now

    GLFWwindow* window = glfwCreateWindow(800, 600, "Learn WebGPU", NULL, NULL);

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
    WGPURequestAdapterOptions adapter_options{};
    adapter_options.compatibleSurface = surface;


    struct UserData {
        WGPUAdapter adapter = nullptr;
        bool requestComplete = false;
        bool success = false;
    };

    auto adapter_creation_callback = [](WGPURequestAdapterStatus status, WGPUAdapter adapter, char const* message, void* userdata)
        {
            UserData& data = *static_cast<UserData*>(userdata);

            if (status == WGPURequestAdapterStatus_Success) {
                data.adapter = adapter;
                data.success = true;
            }
            else {
                std::cerr << "Failed to get adapter: " << message << std::endl;
            }

            data.requestComplete = true;
        };


    UserData user_data{};

    wgpuInstanceRequestAdapter
    (
        instance,
        &adapter_options,
        adapter_creation_callback,
        &user_data
    );

    if (user_data.requestComplete && user_data.success) {
        std::cout << "ADAPTER: " << user_data.adapter << std::endl;
    }
    else {
        abort();
    }

    WGPUAdapter adapter = user_data.adapter;

    //List adapter features

    size_t num_features;
    std::vector<WGPUFeatureName> feature_names{};
    num_features = wgpuAdapterEnumerateFeatures(adapter, nullptr);
    feature_names.resize(num_features);
    wgpuAdapterEnumerateFeatures(adapter, feature_names.data());

    std::cout << "ADAPTER FEATURES:" << std::endl;
    for (auto f : feature_names) {
        std::cout << std::hex << " - " << f << std::endl;
    }

    

    //Create device
    WGPUDeviceDescriptor device_descriptor{};
    device_descriptor.nextInChain = nullptr;
    device_descriptor.label = "My device";
    //No specific features rn
    device_descriptor.requiredFeaturesCount = 0;
    device_descriptor.requiredFeatures = nullptr;
    device_descriptor.requiredLimits = nullptr;
    device_descriptor.defaultQueue.nextInChain = nullptr;
    device_descriptor.defaultQueue.label = "default queue";


 

    struct DeviceUserData {
        WGPUDevice device;
        bool requestComplete = false;
        bool success = false;
    };

    auto device_creation_callback = [](WGPURequestDeviceStatus status, WGPUDevice device, char const* message, void* userdata)
        {
            DeviceUserData& data = *static_cast<DeviceUserData*>(userdata);

            if (status == WGPURequestAdapterStatus_Success) {
                data.device = device;
                data.success = true;
            }
            else {
                std::cerr << "Failed to get device: " << message << std::endl;
            }

            data.requestComplete = true;
        };

    DeviceUserData device_user_data{};

    wgpuAdapterRequestDevice(
        adapter,
        &device_descriptor,
        device_creation_callback,
        &device_user_data
    );

    if (device_user_data.requestComplete && device_user_data.success) {
        std::cout << "DEVICE: " << device_user_data.device << std::endl;
    }
    else {
        abort();
    }

    WGPUDevice device = device_user_data.device;

    auto device_error_callback = [](WGPUErrorType type, char const* message, void*)
        {
            std::cout << "Uncaptured device error: type " << type << std::endl;
            if (message) std::cout << "[ " << message << " ]" << std::endl;
        };


    wgpuDeviceSetUncapturedErrorCallback(
        device,
        device_error_callback,
        nullptr
    );

    auto device_lost_callback = [](WGPUDeviceLostReason reason, char const* message, void*)
        {
            std::cout << "Lost device error: reason " << reason << std::endl;
            if (message) std::cout << "[ " << message << " ]" << std::endl;
        };

    //Set no callback to get dawn to shut up
    wgpuDeviceSetDeviceLostCallback(
        device,
        //device_lost_callback,
        nullptr,
        nullptr
    );

    //Create swapchain

#ifdef WEBGPU_BACKEND_WGPU
    WGPUTextureFormat swapChainFormat = wgpuSurfaceGetPreferredFormat(surface, adapter);
#else
    //Dawn only supports this format
    WGPUTextureFormat swapChainFormat = WGPUTextureFormat_BGRA8Unorm;
#endif

    WGPUSwapChainDescriptor swapchain_desc{};
    swapchain_desc.nextInChain = nullptr;
    swapchain_desc.label = "my swapchain";
    swapchain_desc.format = swapChainFormat;
    //Used for output of rendering commands
    swapchain_desc.usage = WGPUTextureUsage_RenderAttachment;
    swapchain_desc.width = 800;
    swapchain_desc.height = 600;
    swapchain_desc.presentMode = WGPUPresentMode_Fifo;

    WGPUSwapChain swapchain = wgpuDeviceCreateSwapChain(device, surface, &swapchain_desc);

    std::cout << "SWAPCHAIN: " << swapchain << std::endl;

    //Get a queue to send commands to

    WGPUQueue queue = wgpuDeviceGetQueue(device);

    if (!queue) {
        std::cout << "test\n";
    }

    

  
   

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
        WGPUCommandEncoderDescriptor encoder_desc{};
        encoder_desc.nextInChain = nullptr;
        encoder_desc.label = "my encoder";
        WGPUCommandEncoder encoder = wgpuDeviceCreateCommandEncoder(device, &encoder_desc);

        //Encode a render pass
        WGPURenderPassDescriptor render_pass_desc{};
        //Describe render pass here
        render_pass_desc.nextInChain = nullptr;
        render_pass_desc.label = "my render pass";
       

        //Color attachments
        WGPURenderPassColorAttachment color_attachment{};
        color_attachment.nextInChain = nullptr;
        color_attachment.view = current_texture;
        //For multisampling
        color_attachment.resolveTarget = nullptr;
        //Load op occurs right before render pass
        color_attachment.loadOp = WGPULoadOp_Clear;
        //Store op occurs right after render pass
        color_attachment.storeOp = WGPUStoreOp_Store;
        color_attachment.clearValue = { 1,0,0,1 };

        render_pass_desc.colorAttachmentCount = 1;
        render_pass_desc.colorAttachments = &color_attachment;

        //No depth testing rn
        render_pass_desc.depthStencilAttachment = nullptr;

        //For render pass gpu timing
        render_pass_desc.timestampWriteCount = 0;
        render_pass_desc.timestampWrites = nullptr;


        WGPURenderPassEncoder renderPass = wgpuCommandEncoderBeginRenderPass(encoder, &render_pass_desc);
        //Do render pass stuff here
        wgpuRenderPassEncoderEnd(renderPass);
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

    wgpuQueueRelease(queue);
    wgpuSwapChainRelease(swapchain);
    wgpuDeviceRelease(device);
    wgpuAdapterRelease(adapter);
    wgpuInstanceRelease(instance);    
    wgpuSurfaceRelease(surface);

	
	glfwDestroyWindow(window);

	glfwTerminate();

}