# GBA Emulator

Welcome to my **Game Boy Advance Emulator** project! This emulator simulates the hardware environment of the Game Boy Advance (GBA), providing support for core components such as the **ARM7TDMI CPU**, **GPU**, **audio output**, and **memory management**. This emulator aims to replicate the GBA experience through accurate hardware emulation, real-time graphics, and audio playback, providing a challenging yet rewarding exploration into console emulation.

## Motivation and Development Journey

The creation of this GBA emulator was fueled by a passion for low-level system architecture and retro gaming. Emulating a GBA system requires a deep understanding of hardware, multi-threading, synchronization, and real-time graphics and audio output. Building this emulator not only brought to life classic games but also reinforced knowledge in CPU instruction handling, memory management, and efficient graphics rendering.

Developing this project was both technically challenging and insightful, with each component presenting unique learning experiences. Implementing the ARM7TDMI CPU emulation, for example, demanded a thorough understanding of the ARM instruction set, while creating a high-performance GPU required efficient use of SDL2 for frame rendering. Integrating CPAL for audio further expanded the project’s real-time capabilities, delivering a complete emulation experience.

## Features

- **ARM7TDMI CPU Emulation**: Simulates the ARM7TDMI instruction set, handling fundamental GBA CPU instructions for accurate game logic processing.
- **GPU Rendering with SDL2**: Real-time frame rendering with SDL2, featuring tile-based graphics, a basic color palette, and sprite drawing.
- **Audio Playback with CPAL**: Emulates GBA’s audio channels using PCM audio playback with CPAL, delivering authentic sound output.
- **Memory Management**: Manages memory regions like ROM, RAM, and video memory, simulating the memory layout of the GBA.
- **Multithreaded Architecture**: Separate threads for CPU, GPU, and audio processing, enabling concurrent execution of each component.
- **Real-time Emulation**: Achieves frame rates close to 60 FPS, providing smooth and responsive gameplay.

## Architecture

The emulator is structured around five main modules: **CPU**, **GPU**, **Audio**, **Memory**, and **Graphics**. Each module operates within its own thread, accessing shared resources with thread-safe structures. The multithreaded architecture enables the emulator to efficiently handle GBA components concurrently, providing both visual and audio output in real-time.

### CPU (ARM7TDMI)

The CPU module emulates the ARM7TDMI processor, executing instructions based on the ARM architecture. Implementing the ARM7TDMI required understanding and replicating the GBA’s data processing, memory access, and branch instructions. The emulator currently supports essential instructions and handles control flow for gameplay logic.

### GPU

The GPU module provides tile-based rendering, a fundamental feature of the GBA graphics system. Using SDL2 for real-time frame rendering, the emulator generates a color palette and displays tiles and sprites on the screen. Each frame is rendered at approximately 60 FPS, ensuring smooth graphics output.

### Audio

Audio is handled by CPAL, which plays back PCM samples, emulating GBA’s Direct Sound channels. By streaming samples from a shared buffer, the emulator simulates audio channels in stereo at a 44.1kHz sample rate. Future enhancements will include synthesizing audio waveforms to emulate GBA’s audio generation further.

### Memory

The memory module organizes GBA memory regions, including ROM, RAM, video memory, and framebuffer. Each region is accessed through thread-safe methods, ensuring consistent memory state across threads. The module supports 32-bit reads and writes, which align with the GBA’s memory bus.

### Graphics

Graphics rendering is handled by SDL2, which displays the emulator’s framebuffer in a dedicated window. SDL2 allows the emulator to maintain high frame rates and smooth transitions, simulating the GBA screen’s 240x160 resolution.
