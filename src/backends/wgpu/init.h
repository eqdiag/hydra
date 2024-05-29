#pragma once

#include <webgpu/webgpu.h>

#include <optional>
#include <vector>

namespace backends {

	namespace wgpu {

		//Adapters

		std::optional<WGPUAdapter> requestAdapter(WGPUInstance instance, WGPUSurface surface);
		void listAdapterFeatures(WGPUAdapter adapter);

		//Devices
		std::optional<WGPUDevice> requestDevice(WGPUAdapter adapter,bool useErrorCallback = false,bool useLostCallback = false);
		void listDeviceFeatures(WGPUDevice device);
		void listDeviceLimits(WGPUDevice device);

		//Swapchain
		std::optional<WGPUSwapChain> createSwapchain(WGPUSurface surface,WGPUAdapter adapter,WGPUDevice device, uint32_t width,uint32_t height,WGPUPresentMode presentMode = WGPUPresentMode_Fifo,WGPUTextureUsageFlags usage = WGPUTextureUsage_RenderAttachment);
		
		//Command Encoder
		WGPUCommandEncoder createEncoder(WGPUDevice device);




		/* Render Passes */

		//Color attachments
		WGPURenderPassColorAttachment createColorAttachment(WGPUTextureView textureView,WGPULoadOp loadOp = WGPULoadOp_Load, WGPUColor clearColor = { 0,0,0,0 },WGPUStoreOp storeOp = WGPUStoreOp_Store);

		//Depth attachments

		//Render pass builder
		class RenderPassBuilder {
		public:
			RenderPassBuilder& addColorAttachment(WGPURenderPassColorAttachment attachment);
			RenderPassBuilder& setDepthAttachment(WGPURenderPassDepthStencilAttachment attachment);
			WGPURenderPassEncoder build(WGPUCommandEncoder encoder);
		private:
			std::vector<WGPURenderPassColorAttachment> colorAttachments{};
			std::optional<WGPURenderPassDepthStencilAttachment> depthAttachment{};
		};


		/* Render Pipelines */

		enum class ShaderSourceType {
			WGSL,
			SPIRV
		};

		WGPUShaderModule createShaderModule(WGPUDevice device,ShaderSourceType sourceType,const char* shaderSource);

		//Blend State Presets
		WGPUBlendState createAlphaBlendState();

		//Render pipeline builder

		class RenderPipelineBuilder {
		public:
			RenderPipelineBuilder();

			RenderPipelineBuilder& setVertexShaderModule(WGPUShaderModule module);
			RenderPipelineBuilder& setFragmentShaderModule(WGPUShaderModule module);

			RenderPipelineBuilder& setTopology(WGPUPrimitiveTopology topology);
			RenderPipelineBuilder& setCullMode(WGPUFrontFace frontFace,WGPUCullMode cullMode);

			RenderPipelineBuilder& addColorTarget(WGPUTextureFormat textureFormat,WGPUBlendState blendState,WGPUColorWriteMask writeMask);


			WGPURenderPipeline build(WGPUDevice device);
		private:
			WGPURenderPipelineDescriptor descriptor{};

			//Add depth stencil attachment
			WGPUFragmentState fragment_state{};

			std::vector<WGPUColorTargetState> colorTargets{};
		};
		
	}
}