# shell.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
# These are the packages that will be available in your shell
buildInputs = with pkgs; [
  # --- Rust Toolchain Manager ---
  # Instead of trying to guess the right Nix package for the toolchain,
  # we just install `rustup` and let it manage the toolchain.
  rustup

  # --- Node.js / SvelteKit Environment ---
  nodejs_20
  pnpm

  # --- Wasm Build Tools ---
  wasm-pack
  lld

  # --- General C/C++ Toolchain ---
  clang
  rust-analyzer # For editor support (LSP)
];

# This hook runs every time you enter the shell.
# It will configure our local Rust environment.
shellHook = ''
  echo "--- Configuring Nix Development Shell ---"

  # Set RUSTUP_HOME to a local directory to avoid touching the user's
  # global ~/.rustup configuration. This keeps the project self-contained.
  export RUSTUP_HOME=$(pwd)/.rustup
  export CARGO_HOME=$(pwd)/.cargo

  # Ensure the stable toolchain is installed and is the default for this shell.
  # This will only download it the first time you enter the shell.
  rustup toolchain install stable --profile minimal
  rustup default stable

  # Ensure the wasm target is installed for the stable toolchain.
  rustup target add wasm32-unknown-unknown

  echo "--- Environment Ready ---"
  echo "Node.js:   $(node --version)"
  echo "Rust:      $(rustc --version)"
  echo "Wasm Pack: $(wasm-pack --version)"
  echo "-------------------------"

  # Add local node packages to the path
  export PATH="$PWD/node_modules/.bin:$PATH"
'';
}
