

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

## Library structure
- core (wgpu basic structure helpers for common use cases)   
    - pipeline
    - texture
    - vertex
    - context
    - app
- util (higher-level abstractions built up on wgpu)
    - [x] camera
    - [x] mesh
    - [ ] ui (egui) (seems useful to get started https://github.com/ejb004/egui-wgpu-demo/blob/master/src/gui.rs)



## Features
- [ ] Multiple cameras
    - [ ] Arc camera
    - [x] Fly camera
- [ ] Gradient sky background (compute)
- [ ] Blinn-Phong model
- [ ] Gamma correction
- [ ] Shadows
- [ ] Normal mapping
- [ ] Parallax mapping
- [ ] HDR
- [ ] Bloom
- [ ] Deferred shading
- [ ] SSAO
- [ ] Descriptor set allocator
- [ ] Material system
- [ ] Object system
- [ ] GLTF loading
