# backstage

Personal repo for learning webgpu in c++.

All dependencies are self-contained in this project using git's submodule system.

## Installation

```
git clone https://github.com/eqdiag/learnWebgpu
cd learnWebgpu
git submodule update --init
```

## Dependencies
- imgui: For ui
- tiny_obj: Loading 3d model obj files
- stb_image: Loading image files



- [x] Setup
- [x] Window
- [x] Adapter
- [x] Device
- [x] Command Queue
- [x] Color
- [x] Triangle rendering
- [ ] Geometry
	- [ ] Buffers
	- [ ] Attributes
	- [ ] Index Buffer
	- [ ] Loading from obj
- [ ] Uniforms
- [ ] Meshes/Basic shading
- [ ] Texturing
- [ ] Compute pipeline
- [ ] Textures and so on
- [ ] Mipmaps
- [ ] Abstractions
	- [x] Pipeline Builder

# Project rough organization

## backends
- backends/wgpu/init.h: Contains wgpu descriptor initializers and wgpu object initializers + misc functions on them
- backends/wgpu/types.h: Contains wrapper types around wgpu objects (things like image,buffer,cubemap,etc...) + misc functions on them

## core
- should be backend agnostic types but ofc have a backend implementation
- core/xxx?: higher level types (model,mesh,material,camera,etc)

## math
- linear algebra type stuff mainly