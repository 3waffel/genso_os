# OS Demo Built with Rust

## Setup

Add `rust-src` (Required by compiler built-in libraries)

```
rustup component add rust-src
```

Install `bootimage`

```
cargo install bootimage
rustup component add llvm-tools-preview
```

Create an image

```
cargo bootimage
```

Run the image on QEMU (Require QEMU installed)

```
cargo run
```

## Test

```
cargo run --test basic_boot
```

## Todos

-   Nixify the project
-   Add workflows

## Notes

### 测试

端口映射到 QEMU 内置的 `isa-debug-exit`设备，它将在接收到输入时退出。

`src/serial` 定义测试过程中的串口输出；通用的 UARTs (实现串口的芯片)都会兼容 `uart_16550`。
