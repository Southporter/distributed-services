{ pkgs ? import <nixpkgs> {} }:

let
  protoc = pkgs.fetchzip {
    url = "https://github.com/protocolbuffers/protobuf/releases/download/v3.20.0/protoc-3.20.0-linux-x86_64.zip";
  };
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    protobuf
  ];
}
