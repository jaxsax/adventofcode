{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  buildInputs = [
    rustc
    cargo
    cargo-edit
    rust-analyzer
    rustfmt
  ];
}
