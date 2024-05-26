dav1d: dirs

install-meson:
    python --version
    pip install -U meson setuptools ninja

clone:
    -git clone --branch 1.3.0 --depth 1 https://code.videolan.org/videolan/dav1d.git {{ join(justfile_directory(), 'target', 'dav1d') }}

[unix]
prepare: clone install-meson
    pwd
    meson setup -Denable_tools=false -Denable_examples=false --buildtype release {{ join(justfile_directory(), 'target', 'dav1d_build') }} {{ join(justfile_directory(), 'target', 'dav1d') }}

[windows]
setup-msvc:
    $LinkGlob = "VC\Tools\MSVC\*\bin\Hostx64\x64"
    $env:PATH = "$env:PATH;${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer"
    $LinkPath = vswhere -latest -products * -find "$LinkGlob" | Select-Object -Last 1
    $env:PATH = "$env:PATH;$LinkPath"

[windows]
setup-nasm:
    @echo 'TODO'

[windows]
prepare: clone install-meson setup-msvc setup-nasm
    pwd
    meson setup -Denable_tools=false -Denable_examples=false --buildtype release {{ join(justfile_directory(), 'target', 'dav1d_build') }} {{ join(justfile_directory(), 'target', 'dav1d') }}

compile: prepare
    pwd
    ninja -C {{ join(justfile_directory(), 'target', 'dav1d_build') }}

install: compile
    ninja -C {{ join(justfile_directory(), 'target', 'dav1d_build') }} install

dirs:
    @echo {{ just_executable() }}
    @echo {{ join(justfile_directory(), 'target', 'dav1d') }}
    @echo {{ join(justfile_directory(), 'target', 'dav1d_build') }}
    @echo 'After compiling dav1d set:'
    @echo PKG_CONFIG_PATH={{ join(justfile_directory(), 'target', 'dav1d_build', 'pkgconfig') }}
    @echo LD_LIBRARY_PATH={{ join(justfile_directory(), 'target', 'dav1d_build') }}

[confirm("Are you sure you want to remove dav1d and dav1d_build?")]
clear:
    rm -rf {{ join(justfile_directory(), 'target', 'dav1d') }}
    rm -rf {{ join(justfile_directory(), 'target', 'dav1d_build') }}
