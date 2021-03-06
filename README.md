# openrr: `Open Rust Robotics`

![Build and Test](https://github.com/openrr/openrr/workflows/Build%20and%20Test/badge.svg) [![crates.io](https://img.shields.io/crates/v/openrr.svg)](https://crates.io/crates/openrr) [![codecov](https://codecov.io/gh/openrr/openrr/branch/main/graph/badge.svg?token=28GTOOT4RY)](https://codecov.io/gh/openrr/openrr) [![docs](https://docs.rs/openrr/badge.svg)](https://docs.rs/openrr)

Open Rust Robotics platform.

## Dependencies

### Linux

```bash
sudo apt install cmake build-essential libudev-dev
```

* cmake build-essential (openrr-planner (assimp-sys))
* libudev-dev (arci-gamepad-gilrs)

## Architecture

![architecture](img/architecture.png)

`arci` is a hardware abstruction layer for openrr.
Currently [ROS1](https://ros.org) and [urdf-viz](https://github.com/openrr/urdf-viz) (as a static simulator (actually it's just a viewer)) are implemented.

You can write platform/hardware independent code if you use `arci` traits.

## Examples

TODO:

## License

Apache2

## Related openrr repositories

* [k](https://github.com/OpenRR/k) : kinematics library
* [ros-nalgebra](https://github.com/OpenRR/ros-nalgebra) : rosrust nalgebra converter generator
* [rrt](https://github.com/OpenRR/rrt) : RRT-dual-connect path planner
* [trajectory](https://github.com/OpenRR/trajectory) : trajectory interpolator
* [urdf-rs](https://github.com/OpenRR/urdf-rs) : URDF parser
* [urdf-viz](https://github.com/OpenRR/urdf-viz): URDF visualizer
* ~~[gear](https://github.com/OpenRR/gear)~~ : (deprecated) motion planning library, but it is openrr-planner now.
