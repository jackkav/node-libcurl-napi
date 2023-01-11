with import <nixpkgs> {};

mkShell {
    nativeBuildInputs = [
        nodejs-16_x
        yarn
        rustc
        cargo
        rust-analyzer
        cargo-watch
        curl
    ];
}