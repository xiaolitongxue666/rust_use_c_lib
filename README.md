# 如何在rust中使用bindgen调用c的库
#rust/FFI

[使用Rust库bindgen之Hello World（附代码）_yue2388253的博客-CSDN博客](https://blog.csdn.net/yue2388253/article/details/88757277)
[Rust FFI 编程 - bindgen 使用示例 - MikeTang的个人空间 - OSCHINA - 中文开源技术交流社区](https://my.oschina.net/u/4581704/blog/4646876)

很多高效简洁工具库都是由c/c++来编写的,比如青大的ffmpeg、opencv等,rust中调用c库,已经变得越来越频繁.
省去了用rust重构工具的过程,rust支持直接调用c/c++的库

[A little C with your Rust - The Embedded Rust Book](https://rust-embedded.github.io/book/interoperability/c-with-rust.html)
在rust嵌入式文档中,有提到如何使用bindgen来让rust使用c库

首先,需要在项目中编译c代码并生成相应的库.
一下是我自己使用的测试流程

```bash
# Creat a rust exec project
cargo new rust_use_c_lib --bin
```

然后将需要使用的c库放入到rust项目目录中,我这里使用测试库 are_lib

编译 c 库有两种方法,三种工具
首先我们说工具
1-对于 c 代码文件书较少,库比较小的, 可以使用较为原始的 cc 编译
2-如果项目较大,文件较多采用Makefile进行自动化编译 
3-和第二类类似,采用更为年轻的CMake

其次说方法
1-采用原始的独立编译c库的方法,在编译c库的流程上和rust没有什么关系
2-在rust项目中,通过cargo 调用rustc 在编译整个rust项目之前,提前编译c库

我们这里使用第二种方法,用cc编译c库后在rust中使用

因为我们需要提前编译c的部分,而且要用到cc这个工具
[GitHub - alexcrichton/cc-rs: Rust library for build scripts to compile C/C++ code into a Rust library](https://github.com/alexcrichton/cc-rs)
https://crates.io/crates/cc
所以需要在Cargo.toml文件中添加如下代码
```
build-dependencies]
cc = "1.0.66"
```

[Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
在添加完了cc crate后,在rust项目根目录下,添加build.rs文件
```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system area
    // shared library.
    println!("cargo:rustc-link-lib=area");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=area_lib/area.c");

    //Build area lib form c source code
    cc::Build::new()
        .file("area_lib/area.c")
        .compile("area");
}
```

*  [cargo:rustc-link-lib=[KIND=]NAME](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib) ### — Adds a library to link.
The optional KIND may be one of dylib, static, or framework.

到此,提前编译需要使用的c库就完成了

接下来需要将c库链接进rust项目并做一些整形,这个时候就需要用到bindgen
https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a
https://crates.io/crates/bindgen
[The `bindgen` User Guide](https://rust-lang.github.io/rust-bindgen/print.html)

根据用户指南 
安装llvm
```
#macOS
brew install llvm
```

Cargo.toml中添加仓库
```
[build-dependencies]
bindgen = "0.53.1"
```

在build.rs中添加bindgen的操作
```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system area
    // shared library.
    println!("cargo:rustc-link-lib=area");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=area_lib/area.c");

    //Build area lib form c source code
    cc::Build::new()
        .file("area_lib/area.c")
        .compile("area");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

然后在rust的项目中执行
```
cargo build
```

会生成 bindings.rs 文件,可能会有多个
```shellS
$ find ./ -name bindings.rs
.//target/debug/build/rust_use_c_lib-42bfd16b522bb8e0/out/bindings.rs
.//target/debug/build/rust_use_c_lib-08d1fa397eaf31a4/out/bindings.rs
```

到这里所有为 在rust中使用c库的前期准备都完成了

接下里就是rust项目本身,调用并测试c库的函数了

