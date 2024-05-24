dav1d: install

clear:
    rm -rf {{justfile_directory()}}/target/dav1d_build

install-meson:
    python --version
    pip install -U meson setuptools ninja

clone:
    -git clone --branch 1.3.0 --depth 1 https://code.videolan.org/videolan/dav1d.git {{justfile_directory()}}/target/dav1d

compile: clone install-meson
    cd {{justfile_directory()}}/target/dav1d # todo!
    pwd
    -meson build -Denable_tools=false -Denable_examples=false --buildtype release
    pwd
    ninja -C build

install: compile
    cd {{justfile_directory()}}/target/dav1d
    ninja -C build install
