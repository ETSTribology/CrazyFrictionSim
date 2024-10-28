{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.wasm-pack
    pkgs.maturin
    pkgs.python39
    pkgs.openmpi
    pkgs.vim
    pkgs.docker
    pkgs.cmake
    pkgs.gcc
    pkgs.libcxx
    pkgs.pkg-config
    pkgs.gcc
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    export PKG_CONFIG_PATH=${pkgs.openmpi.lib}/pkgconfig
  '';
}
