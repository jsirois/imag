{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustStable; [
    rustc
    cargo
    pkgs.llvmPackages.lldb
  ];

  dependencies = with pkgs; [
    openssl
    zlib
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env dependencies ];

}

