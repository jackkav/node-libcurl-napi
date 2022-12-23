with import <nixpkgs> {};

mkShell {
    nativeBuildInputs = [
        nodejs-16_x
        yarn
        rustc
        cargo
    ];
}