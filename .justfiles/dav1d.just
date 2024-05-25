dav1d: install

clear:
    rm -rf {{justfile_directory() / 'target' / 'dav1d_build' }}

install-meson:
    python --version
    pip install -U meson setuptools ninja

clone:
    -git clone --branch 1.3.0 --depth 1 https://code.videolan.org/videolan/dav1d.git {{justfile_directory() / 'target' / 'dav1d'}}

prepare: clone install-meson
    pwd
    meson setup -Denable_tools=false -Denable_examples=false --buildtype release {{justfile_directory() / 'target' / 'dav1d_build' }} {{justfile_directory() / 'target' / 'dav1d' }}

compile: prepare
    pwd
    ninja -C {{justfile_directory() / 'target' / 'dav1d_build' }}

install: compile
    ninja -C {{justfile_directory() / 'target' / 'dav1d_build' }} install
