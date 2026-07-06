{
  lib,
  llvmPackages,
  pkg-config,
  qt5,
  qt6,
  dbus,
  rustPlatform,
  version ? "0.1.0-unstable",
  useQt6 ? false,
  darkFlavor ? "mocha",
  lightFlavor ? "latte",
}:

let
  qtTarget = if useQt6 then qt6 else qt5;

  buildFeatures = [
    lightFlavor
    darkFlavor
  ]
  ++ (lib.optional useQt6 "qt6");
in
rustPlatform.buildRustPackage {
  pname = "qt-wayland-catppuccin-decorations";
  inherit version buildFeatures;

  src = ./.;

  cargoHash = "sha256-plKiNE+LY4NR22aqnhygPJEbqJ6yrboHXscHv7subXE=";

  nativeBuildInputs = [
    pkg-config
    llvmPackages.libclang
    qtTarget.wrapQtAppsHook
  ];

  buildInputs = [
    qtTarget.qtbase
    qtTarget.qtwayland
    dbus
  ];

  preBuild = ''
    export PATH="${qtTarget.qtbase}/libexec:$PATH"
    export LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib"
  '';

  meta = with lib; {
    description = "Catppuccin styling engine module for Qt Wayland client-side decorations";
    homepage = "https://github.com/catppuccin/rust";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
