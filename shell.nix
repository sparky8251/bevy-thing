{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  devShellInputs = [
    glibc
    zlib

    stdenv.cc.cc.lib
    stdenv.cc.libc

    cargo
    rustc
    rust-analyzer

    vulkan-loader
    vulkan-headers

    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    systemd

    wayland
    libxkbcommon

    alsa-lib

    zstd
  ];

in

mkShell {
  nativeBuildInputs = [
    pkg-config
    clang
    mold
    makeWrapper
  ];

  buildInputs = devShellInputs;

  LD_LIBRARY_PATH = lib.makeLibraryPath devShellInputs;

  shellHook = ''
    echo "Entering Bevy 0.16 development shell."

    export NIX_BUILD_CORES=$(nproc)
    export RUSTFLAGS="-Clink-arg=-fuse-ld=${mold}/bin/mold -C linker=${clang}/bin/clang -L native=${lib.makeLibraryPath devShellInputs}"

    export ZSTD_SYS_USE_PKG_CONFIG=true

    exec fish
  '';
}
