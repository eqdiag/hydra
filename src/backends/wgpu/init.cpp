#include "init.h"

#include <iostream>
#include <cassert>
#include <vector>

std::optional<WGPUAdapter> backends::wgpu::requestAdapter(WGPUInstance instance,WGPUSurface surface)
{
    


    struct RequestData {
        WGPUAdapter adapter = nullptr;
        bool complete = false;
        bool success = false;
    };

    auto create_callback = [](WGPURequestAdapterStatus status, WGPUAdapter adapter, char const* message, void* userdata)
        {
            RequestData& request = *static_cast<RequestData*>(userdata);

            if (status == WGPURequestAdapterStatus_Success) {
                request.adapter = adapter;
                request.success = true;
            }
            else {
                std::cerr << "Failed to get adapter: " << message << std::endl;
            }

            request.complete = true;
        };


    WGPURequestAdapterOptions options{};
    options.compatibleSurface = surface;

    RequestData request{};

    wgpuInstanceRequestAdapter
    (
        instance,
        &options,
        create_callback,
        &request
    );

    assert(request.complete);

    if (request.complete) return request.adapter;
    return {};

}

void backends::wgpu::listAdapterFeatures(WGPUAdapter adapter)
{
    size_t num_features;
    std::vector<WGPUFeatureName> feature_names{};
    num_features = wgpuAdapterEnumerateFeatures(adapter, nullptr);
    feature_names.resize(num_features);
    wgpuAdapterEnumerateFeatures(adapter, feature_names.data());

    std::cout << "ADAPTER FEATURES:" << std::endl;
    for (auto f : feature_names) {
        std::cout << std::hex << " - " << f << std::endl;
    }
}

std::optional<WGPUDevice> backends::wgpu::requestDevice(WGPUAdapter adapter, bool useErrorCallback, bool useLostCallback)
{
    struct RequestData {
        WGPUDevice device = nullptr;
        bool complete = false;
        bool success = false;
    };

    auto create_callback = [](WGPURequestDeviceStatus status, WGPUDevice device, char const* message, void* userdata)
        {
            RequestData& request = *static_cast<RequestData*>(userdata);

            if (status == WGPURequestAdapterStatus_Success) {
                request.device = device;
                request.success = true;
            }
            else {
                std::cerr << "Failed to get device: " << message << std::endl;
            }

            request.complete = true;
        };


    WGPUDeviceDescriptor descriptor{};
    descriptor.nextInChain = nullptr;
    descriptor.label = "my device";
    //No specific features rn
    descriptor.requiredFeaturesCount = 0;
    descriptor.requiredFeatures = nullptr;
    descriptor.requiredLimits = nullptr;
    descriptor.defaultQueue.nextInChain = nullptr;
    descriptor.defaultQueue.label = "default queue";

    RequestData request{};


    wgpuAdapterRequestDevice
    (
        adapter,
        &descriptor,
        create_callback,
        &request
    );

    assert(request.complete);

    //Add all callbacks that the user requests

    if (useErrorCallback) {
        auto device_error_callback = [](WGPUErrorType type, char const* message, void*)
            {
                std::cout << "Uncaptured device error: type " << type << std::endl;
                if (message) std::cout << "[ " << message << " ]" << std::endl;
            };

        wgpuDeviceSetUncapturedErrorCallback(
            request.device,
            device_error_callback,
            nullptr
        );
    }

    auto device_lost_callback = [](WGPUDeviceLostReason reason, char const* message, void*)
        {
            std::cout << "Lost device error: reason " << reason << std::endl;
            if (message) std::cout << "[ " << message << " ]" << std::endl;
        };

    if (useLostCallback) {
        wgpuDeviceSetDeviceLostCallback(
            request.device,
            device_lost_callback,
            nullptr
        );
    }
    else {
        //Set no callback to get dawn to shut up
        wgpuDeviceSetDeviceLostCallback(
            request.device,
            nullptr,
            nullptr
        );
    }

    

    if (request.complete) return request.device;
    return {};

}

void backends::wgpu::listDeviceFeatures(WGPUDevice device)
{
    size_t num_features;
    std::vector<WGPUFeatureName> features{};
    num_features = wgpuDeviceEnumerateFeatures(device, nullptr);
    features.resize(num_features);
    wgpuDeviceEnumerateFeatures(device, features.data());

    std::cout << "DEVICE FEATURES:" << std::endl;
    for (auto f : features) {
        std::cout << std::hex << " - " << f << std::endl;
    }
}

void backends::wgpu::listDeviceLimits(WGPUDevice device)
{
    WGPUSupportedLimits limits{};
    bool success = wgpuDeviceGetLimits(device, &limits);
    if (success) {
        std::cout << "DEVICE LIMITS:" << std::endl;
        std::cout << " - maxTextureDimension1D: " << limits.limits.maxTextureDimension1D << std::endl;
        std::cout << " - maxTextureDimension2D: " << limits.limits.maxTextureDimension2D << std::endl;
        std::cout << " - maxTextureDimension3D: " << limits.limits.maxTextureDimension3D << std::endl;
        std::cout << " - maxTextureArrayLayers: " << limits.limits.maxTextureArrayLayers << std::endl;
        std::cout << " - maxBindGroups: " << limits.limits.maxBindGroups << std::endl;
        std::cout << " - maxBindGroupsPlusVertexBuffers: " << limits.limits.maxBindGroupsPlusVertexBuffers << std::endl;
        std::cout << " - maxBindingsPerBindGroup: " << limits.limits.maxBindingsPerBindGroup << std::endl;
        std::cout << " - maxDynamicUniformBuffersPerPipelineLayout: " << limits.limits.maxDynamicUniformBuffersPerPipelineLayout << std::endl;
        std::cout << " - maxDynamicStorageBuffersPerPipelineLayout: " << limits.limits.maxDynamicStorageBuffersPerPipelineLayout << std::endl;
        std::cout << " - maxSampledTexturesPerShaderStage: " << limits.limits.maxSampledTexturesPerShaderStage << std::endl;
        std::cout << " - maxSamplersPerShaderStage: " << limits.limits.maxSamplersPerShaderStage << std::endl;
        std::cout << " - maxStorageBuffersPerShaderStage: " << limits.limits.maxStorageBuffersPerShaderStage << std::endl;
        std::cout << " - maxStorageTexturesPerShaderStage: " << limits.limits.maxStorageTexturesPerShaderStage << std::endl;
        std::cout << " - maxUniformBuffersPerShaderStage: " << limits.limits.maxUniformBuffersPerShaderStage << std::endl;
        std::cout << " - maxUniformBufferBindingSize: " << limits.limits.maxUniformBufferBindingSize << std::endl;
        std::cout << " - maxStorageBufferBindingSize: " << limits.limits.maxStorageBufferBindingSize << std::endl;
        std::cout << " - minUniformBufferOffsetAlignment: " << limits.limits.minUniformBufferOffsetAlignment << std::endl;
        std::cout << " - minStorageBufferOffsetAlignment: " << limits.limits.minStorageBufferOffsetAlignment << std::endl;
        std::cout << " - maxVertexBuffers: " << limits.limits.maxVertexBuffers << std::endl;
        std::cout << " - maxBufferSize: " << limits.limits.maxBufferSize << std::endl;
        std::cout << " - maxVertexAttributes: " << limits.limits.maxVertexAttributes << std::endl;
        std::cout << " - maxVertexBufferArrayStride: " << limits.limits.maxVertexBufferArrayStride << std::endl;
        std::cout << " - maxInterStageShaderComponents: " << limits.limits.maxInterStageShaderComponents << std::endl;
        std::cout << " - maxInterStageShaderVariables: " << limits.limits.maxInterStageShaderVariables << std::endl;
        std::cout << " - maxColorAttachments: " << limits.limits.maxColorAttachments << std::endl;
        std::cout << " - maxColorAttachmentBytesPerSample: " << limits.limits.maxColorAttachmentBytesPerSample << std::endl;
        std::cout << " - maxComputeWorkgroupStorageSize: " << limits.limits.maxComputeWorkgroupStorageSize << std::endl;
        std::cout << " - maxComputeInvocationsPerWorkgroup: " << limits.limits.maxComputeInvocationsPerWorkgroup << std::endl;
        std::cout << " - maxComputeWorkgroupSizeX: " << limits.limits.maxComputeWorkgroupSizeX << std::endl;
        std::cout << " - maxComputeWorkgroupSizeY: " << limits.limits.maxComputeWorkgroupSizeY << std::endl;
        std::cout << " - maxComputeWorkgroupSizeZ: " << limits.limits.maxComputeWorkgroupSizeZ << std::endl;
        std::cout << " - maxComputeWorkgroupsPerDimension: " << limits.limits.maxComputeWorkgroupsPerDimension << std::endl;
    }

 
}

std::optional<WGPUSwapChain> backends::wgpu::createSwapchain(WGPUSurface surface, WGPUAdapter adapter,WGPUDevice device, uint32_t width, uint32_t height, WGPUPresentMode presentMode, WGPUTextureUsageFlags usage)
{
    #ifdef WEBGPU_BACKEND_WGPU
        WGPUTextureFormat swapChainFormat = wgpuSurfaceGetPreferredFormat(surface, adapter);
    #else
        //Dawn only supports this format
        (void)adapter;
        WGPUTextureFormat swapChainFormat = WGPUTextureFormat_BGRA8Unorm;
    #endif

    WGPUSwapChainDescriptor swapchain_desc{};
    swapchain_desc.nextInChain = nullptr;
    swapchain_desc.label = "my swapchain";
    swapchain_desc.format = swapChainFormat;
    //Used for output of rendering commands
    swapchain_desc.usage = usage;
    swapchain_desc.width = width;
    swapchain_desc.height = height;
    swapchain_desc.presentMode = presentMode;

    WGPUSwapChain swapchain = wgpuDeviceCreateSwapChain(device, surface, &swapchain_desc);

    return swapchain;
}

WGPUCommandEncoder backends::wgpu::createEncoder(WGPUDevice device)
{
    WGPUCommandEncoderDescriptor descriptor{};
    descriptor.nextInChain = nullptr;
    descriptor.label = "my encoder";
    WGPUCommandEncoder encoder = wgpuDeviceCreateCommandEncoder(device, &descriptor);
    return encoder;
}

WGPURenderPassColorAttachment backends::wgpu::createColorAttachment(WGPUTextureView textureView, WGPULoadOp loadOp, WGPUColor clearColor, WGPUStoreOp storeOp)
{
    WGPURenderPassColorAttachment color_attachment{};
    color_attachment.nextInChain = nullptr;
    color_attachment.view = textureView;
    //For multisampling
    color_attachment.resolveTarget = nullptr;
    //Load op occurs right before render pass
    color_attachment.loadOp = loadOp;
    //Store op occurs right after render pass
    color_attachment.storeOp = storeOp;
    color_attachment.clearValue = clearColor;

    return color_attachment;
}

WGPUShaderModule backends::wgpu::createShaderModule(WGPUDevice device, ShaderSourceType sourceType,const char* shaderSource)
{
    WGPUShaderModuleDescriptor shader_descriptor{};
    shader_descriptor.nextInChain = nullptr;
    #ifdef WEBGPU_BACKEND_WGPU
        shader_descriptor.hintCount = 0;
        shader_descriptor.hints = nullptr;
    #endif
    shader_descriptor.label = "my shader module";

    if (sourceType == ShaderSourceType::WGSL) {
        WGPUShaderModuleWGSLDescriptor shader_code_descriptor{};
        // Set the chained struct's header
        shader_code_descriptor.chain.next = nullptr;
        shader_code_descriptor.chain.sType = WGPUSType_ShaderModuleWGSLDescriptor;
        // Connect the chain
        shader_descriptor.nextInChain = &shader_code_descriptor.chain;

        // Setup the actual payload of the shader code descriptor
        shader_code_descriptor.code = shaderSource;
    }
    else {
        std::cerr << "Only support wgsl shader right now!\n";
        abort();
    }

    auto shader_module = wgpuDeviceCreateShaderModule(device, &shader_descriptor);
    return shader_module;
}

WGPUBlendState backends::wgpu::createAlphaBlendState()
{
    WGPUBlendState state{};

    //Color
    state.color.srcFactor = WGPUBlendFactor_SrcAlpha;
    state.color.dstFactor = WGPUBlendFactor_OneMinusSrcAlpha;
    state.color.operation = WGPUBlendOperation_Add;

    //Alpha
    state.alpha.srcFactor = WGPUBlendFactor_Zero;
    state.alpha.dstFactor = WGPUBlendFactor_One;
    state.alpha.operation = WGPUBlendOperation_Add;

    return state;
}



backends::wgpu::RenderPassBuilder& backends::wgpu::RenderPassBuilder::addColorAttachment(WGPURenderPassColorAttachment attachment)
{
    colorAttachments.push_back(attachment);
    return *this;
}

backends::wgpu::RenderPassBuilder& backends::wgpu::RenderPassBuilder::setDepthAttachment(WGPURenderPassDepthStencilAttachment attachment)
{
    depthAttachment = attachment;
    return *this;
}

WGPURenderPassEncoder backends::wgpu::RenderPassBuilder::build(WGPUCommandEncoder encoder)
{
    WGPURenderPassDescriptor descriptor{};
    //Describe render pass here
    descriptor.nextInChain = nullptr;
    descriptor.label = "my render pass";

    descriptor.colorAttachmentCount = colorAttachments.size();
    descriptor.colorAttachments = colorAttachments.data();

    //No depth testing rn
    descriptor.depthStencilAttachment = depthAttachment.has_value() ? &depthAttachment.value() : nullptr;

    //For render pass gpu timing
    descriptor.timestampWriteCount = 0;
    descriptor.timestampWrites = nullptr;


    WGPURenderPassEncoder renderPass = wgpuCommandEncoderBeginRenderPass(encoder, &descriptor);

    return renderPass;
}

backends::wgpu::RenderPipelineBuilder::RenderPipelineBuilder()
{
    descriptor.nextInChain = nullptr;
    descriptor.label = "my render pipeline";

    //Default pipeline layout
    descriptor.layout = nullptr;

    //Default vertex state values
    descriptor.vertex = {};
    descriptor.vertex.entryPoint = "vs_main";

    //Default primitive state
    descriptor.primitive = {};

    //Default depth stencil state
    descriptor.depthStencil = nullptr;

    //Default (no multisampling)
    descriptor.multisample.nextInChain = nullptr;
    descriptor.multisample.count = 1; //1 sampler per pixel
    descriptor.multisample.mask = ~0u; //Keep all bits (all 1s)
    descriptor.multisample.alphaToCoverageEnabled = false;

    //Fragment state
    fragment_state = {};
    fragment_state.entryPoint = "fs_main";
    descriptor.fragment = &fragment_state;
}

backends::wgpu::RenderPipelineBuilder& backends::wgpu::RenderPipelineBuilder::setVertexShaderModule(WGPUShaderModule module)
{
    descriptor.vertex.module = module;
    return *this;
}

backends::wgpu::RenderPipelineBuilder& backends::wgpu::RenderPipelineBuilder::setFragmentShaderModule(WGPUShaderModule module)
{
    fragment_state.module = module;
    return *this;
}

backends::wgpu::RenderPipelineBuilder& backends::wgpu::RenderPipelineBuilder::setTopology(WGPUPrimitiveTopology topology)
{
    descriptor.primitive.topology = topology;
    return *this;
}

backends::wgpu::RenderPipelineBuilder& backends::wgpu::RenderPipelineBuilder::setCullMode(WGPUFrontFace frontFace, WGPUCullMode cullMode)
{
    descriptor.primitive.frontFace = frontFace;
    descriptor.primitive.cullMode = cullMode;
    return *this;
}

backends::wgpu::RenderPipelineBuilder& backends::wgpu::RenderPipelineBuilder::addColorTarget(WGPUTextureFormat textureFormat, WGPUBlendState blendState, WGPUColorWriteMask writeMask)
{
    WGPUColorTargetState color_target{};
    color_target.nextInChain = nullptr;
    color_target.format = textureFormat;
    color_target.blend = &blendState;
    color_target.writeMask = writeMask;
    colorTargets.push_back(color_target);

    return *this;
}

WGPURenderPipeline backends::wgpu::RenderPipelineBuilder::build(WGPUDevice device)
{   


    fragment_state.targetCount = colorTargets.size();
    fragment_state.targets = colorTargets.data();

    auto pipeline = wgpuDeviceCreateRenderPipeline(device, &descriptor);
   
    return pipeline;
}
