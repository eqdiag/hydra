#include <iostream>

#include "glfw/glfw3.h"
#include <webgpu/webgpu.h>
#include <glfw3webgpu.h>

#include <cassert>
#include <vector>

WGPUAdapter requestAdapter(WGPUInstance instance, WGPURequestAdapterOptions const* options) {
    // A simple structure holding the local information shared with the
    // onAdapterRequestEnded callback.
    struct UserData {
        WGPUAdapter adapter = nullptr;
        bool requestEnded = false;
    };
    UserData userData;

    // Callback called by wgpuInstanceRequestAdapter when the request returns
    // This is a C++ lambda function, but could be any function defined in the
    // global scope. It must be non-capturing (the brackets [] are empty) so
    // that it behaves like a regular C function pointer, which is what
    // wgpuInstanceRequestAdapter expects (WebGPU being a C API). The workaround
    // is to convey what we want to capture through the pUserData pointer,
    // provided as the last argument of wgpuInstanceRequestAdapter and received
    // by the callback as its last argument.
    auto onAdapterRequestEnded = [](WGPURequestAdapterStatus status, WGPUAdapter adapter, char const* message, void* pUserData) {
        UserData& userData = *reinterpret_cast<UserData*>(pUserData);
        if (status == WGPURequestAdapterStatus_Success) {
            userData.adapter = adapter;
        }
        else {
            std::cout << "Could not get WebGPU adapter: " << message << std::endl;
        }
        userData.requestEnded = true;
        };

    // Call to the WebGPU request adapter procedure
    wgpuInstanceRequestAdapter(
        instance /* equivalent of navigator.gpu */,
        options,
        onAdapterRequestEnded,
        (void*)&userData
    );

    // In theory we should wait until onAdapterReady has been called, which
    // could take some time (what the 'await' keyword does in the JavaScript
    // code). In practice, we know that when the wgpuInstanceRequestAdapter()
    // function returns its callback has been called.
    assert(userData.requestEnded);

    return userData.adapter;
}

WGPUDevice requestDevice(WGPUAdapter adapter, WGPUDeviceDescriptor const* descriptor) {
    struct UserData {
        WGPUDevice device = nullptr;
        bool requestEnded = false;
    };
    UserData userData;

    auto onDeviceRequestEnded = [](WGPURequestDeviceStatus status, WGPUDevice device, char const* message, void* pUserData) {
        UserData& userData = *reinterpret_cast<UserData*>(pUserData);
        if (status == WGPURequestDeviceStatus_Success) {
            userData.device = device;
        }
        else {
            std::cout << "Could not get WebGPU device: " << message << std::endl;
        }
        userData.requestEnded = true;
        };

    wgpuAdapterRequestDevice(
        adapter,
        descriptor,
        onDeviceRequestEnded,
        (void*)&userData
    );

    assert(userData.requestEnded);

    return userData.device;
}

int main() {
	

	if (!glfwInit()) {
		std::cerr << "Couldn't init glfw\n";
	}

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API); // No opengl


	GLFWwindow* window = glfwCreateWindow(800, 600, "Learn WebGPU", NULL, NULL);

	if (!window) {
		std::cerr << "Couldn't create window";
		return 1;

	}


    //Create wgpu instance
	WGPUInstanceDescriptor create_info{};
	create_info.nextInChain = nullptr;

	WGPUInstance instance = wgpuCreateInstance(&create_info);

	if (!instance) {
		std::cerr << "Couldn't create wgpu instance\n";
	}

	std::cout << "WGPU INSTANCE::" << instance << std::endl;

    //Create surface
    WGPUSurface surface = glfwGetWGPUSurface(instance, window);

    if (!surface) {
        std::cerr << "Couldn't create surface\n";
    }

    std::cout << "WGPU SURFACE::" << surface << std::endl;

    //Create wgpu adapter
    std::cout << "Requesting adapter..." << std::endl;

    WGPURequestAdapterOptions adapterOpts{};
    adapterOpts.compatibleSurface = surface;
    WGPUAdapter adapter = requestAdapter(instance, &adapterOpts);

    std::cout << "WGPU Adapter::" << adapter << std::endl;

    std::vector<WGPUFeatureName> features;

    // Call the function a first time with a null return address, just to get
    // the entry count.
    size_t featureCount = wgpuAdapterEnumerateFeatures(adapter, nullptr);

    // Allocate memory (could be a new, or a malloc() if this were a C program)
    features.resize(featureCount);

    // Call the function a second time, with a non-null return address
    wgpuAdapterEnumerateFeatures(adapter, features.data());

    std::cout << "Adapter features:" << std::endl;
    for (auto f : features) {
        std::cout << " - " << f << std::endl;
    }

    //Create device

    std::cout << "Requesting device..." << std::endl;

    WGPUDeviceDescriptor deviceDesc = {};
    deviceDesc.nextInChain = nullptr;
    deviceDesc.label = "My Device"; // anything works here, that's your call
    deviceDesc.requiredFeaturesCount = 0; // we do not require any specific feature
    deviceDesc.requiredLimits = nullptr; // we do not require any specific limit
    deviceDesc.defaultQueue.nextInChain = nullptr;
    deviceDesc.defaultQueue.label = "The default queue";
    // [...] Build device descriptor
    WGPUDevice device = requestDevice(adapter, &deviceDesc);

    std::cout << "WGPU Device::" << device << std::endl;

    auto onDeviceError = [](WGPUErrorType type, char const* message, void* /* pUserData */) {
        std::cout << "Uncaptured device error: type " << type;
        if (message) std::cout << " (" << message << ")";
        std::cout << std::endl;
        };
    wgpuDeviceSetUncapturedErrorCallback(device, onDeviceError, nullptr /* pUserData */);


    //Get queue
    WGPUQueue queue = wgpuDeviceGetQueue(device);

    //TODO: On commans
  

	while (!glfwWindowShouldClose(window)) {
		glfwPollEvents();

		//Do stuff
	}

    wgpuQueueRelease(queue);
    wgpuDeviceRelease(device);
    wgpuAdapterRelease(adapter);
    wgpuSurfaceRelease(surface);
	wgpuInstanceRelease(instance);

	
	glfwDestroyWindow(window);

	glfwTerminate();

}