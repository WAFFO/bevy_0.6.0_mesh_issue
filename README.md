# Bevy 0.6.0 Mesh Attribute Issue

Branches: v6, v5

## How to duplicate
```bash
git clone https://github.com/WAFFO/bevy_0.6.0_mesh_issue.git
cd bevy_0.6.0_mesh_issue
git checkout v5
cargo run
git checkout v6
cargo run
```

What I expect (0.5.0):
![release 0.5.0](https://i.imgur.com/0hwoeTV.png)

What actually happens (0.6.0):
![release 0.6.0](https://i.imgur.com/ueLmRkZ.png)

# System
```
MacOS Monterey Version 12.1
2022-01-09T09:57:47.727055Z  INFO bevy_render::renderer: AdapterInfo { name: "AMD Radeon Pro 5500M", vendor: 0, device: 0, device_type: DiscreteGpu, backend: Metal }
```
(Also tested on Windows, same issue)

