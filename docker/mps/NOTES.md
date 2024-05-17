# Implementation Notes

# Context

Nowadays there is a clear proliferation of open-source models for Machine Learning and AI, many of which can be used by individuals that have access to enough computing power, such is the case of people using M1/M2 Macbook Pro machines, which operate really efficiently but only for apple-native software. Arguably the most used language for ML is Python, and pytorch is the most popular python package for it, still, native support for M1/M2 chips is barely usable. Other design pattern is to use docker containers as encapsulated VM, yet, macOS is not publicly available as base image, thus native support for their hardware is not readily available for engineers using macOS as the base OS. 

# Problem

Currently there is not clean support for Pytorch to be used in a M1 MacbookPro machine while taking advantage of the GPU capabilities. That is, if somebody wants to use his/her M1/M2 Macbook's GPU forMachine Learning Models, it has to be with CPU-only setup. This is because the porting of the [MPS](https://developer.apple.com/documentation/metalperformanceshaders) framework, required for the Apple Silicon GPU to work, is barely supported by the core tech stack for AI/ML projects, mainly pytorch, docker.

# Solution: MPS-supported Container

A linux-based VM that has the necessesary drivers to support a pytorch version that natively uses MPS for GPU computations.

## main components

1. Base operative system: Linux OS (Fedora)
2. Drivers: MPS drivers (Inside Fedora Asahi Remix)
3. Python & ML Framework support: pytorch with MPS support
4. Use case: LLM | 6GB GPU | M1/M2 MacbookPro 
5. Benchmarks
    1. Macbook CPU-only
    3. Macbook >> Docker Linux Fedora Asahi Remix (ARM64) >> Torch GPU MPS 

## Base image

The Docker Hub image for the base OS is: [fedora:40](https://hub.docker.com/layers/library/fedora/40/images/sha256-79c6d37cfa65044246e2b7a42e6ac47a71b498c6aaf79ef18ce27b8f1912b1c3?context=explore), this is because later we build the Fedora Asahi Remix 
