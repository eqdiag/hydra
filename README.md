# hydra

A project spawned out of the frustration with so many graphics APIs: I just want to easily draw things 
using my GPU and wanna make sure they can run everywhere.

## Installation


### Desktop Installation
```
git clone https://github.com/eqdiag/hydra
cargo run
```

### Web Installation (currently fixing)
```
git clone https://github.com/eqdiag/hydra
wasm-pack build --target web
<start web-server>
```


## Tutorial Tasks 
- [x] Setup
- [x] Surface
- [x] Pipeline

- [x] Buffers & Indices
- [x] Textures & Bind Groups
- [x] Uniforms and camera
- [x] Instancing

- [ ] Depth buffer
- [ ] Model loading


# Multiple binaries (examples)
- useful for web support testing 
- [ ] Fix for web support

## Abstractions
- [ ] Context (holds wgpu resources), make a builder
- [ ] Window
- [ ] App (has window, context,etc?)

## Features
- [ ] Multiple cameras
    - [ ] Arc camera
    - [ ] Fly camera
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





