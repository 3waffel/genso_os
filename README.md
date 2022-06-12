# OS Demo Built with Rust

## Post 2

Install `bootimage`

```
cargo install bootimage

rustup component add llvm-tools-preview

cargo bootimage
```


Run the image on QEMU (Require QEMU installed)

```
cargo run 
```

+ bootloader version issue
```
bootloader = "0.9.18"
```