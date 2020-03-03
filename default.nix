{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    clang
    openssl
    pkgconfig
  ];
}