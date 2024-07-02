# hydra :octopus:

A project spawned out of the frustration with so many graphics APIs: I just want to easily draw things 
using my GPU and wanna make sure they can run everywhere (*just natively for now*).

Hydra's main goal is make it easy to create interactive gpu-based applications that run natively and on the web.
It provides many *utility* gpu functions and math functions to make setting up 2d and 3d wgpu applications easier.

It's also designed to be *:wavy_dash:ergonomic:wavy_dash:* 


## Examples
- [x] Window (example1_window.rs)
- [x] Render Pass (example2_renderpass.rs)
- [x] Graphics Pipeline (example3_graphics_pipeline.rs)
- [x] Buffers & Indices (example4_buffers.rs)
- [x] Textures & Bind Groups (example5_textures.rs)
- [x] Uniforms and camera (example6_uniforms.rs)
- [x] Instancing (example7_instancing.rs)
- [x] Depth buffer (example8_depth.rs)
- [x] Meshes (example9_mesh.rs)
- [x] Compute (example11_compute.rs)


## Library structure
- base (wgpu basic structure helpers for common use cases)   
    - app
    - context
    - pipeline
    - texture
    - vertex
- core (higher-level abstractions built up on wgpu)
    - camera
    - mesh
    - ui

## TODO
- [ ] arc camera
- [ ] compute shader example
- [ ] get working on web

## Installation


### Desktop Installation
```
git clone https://github.com/eqdiag/hydra
cargo run
```

### Web Installation [not an immediate focus right now to support]
```
git clone https://github.com/eqdiag/hydra
wasm-pack build --target web
<start web-server>
```





