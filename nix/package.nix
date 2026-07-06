{
  lib,
  stdenv,
  cmake,
  qt5,
  qt6,
  wayland,

  version ? "0-unstable",
  useQt6 ? false,

  darkFlavor ? "Mocha",
  lightFlavor ? "Latte",

  # Shadows support on Qt5 requires the feature backported from Qt6:
  # https://src.fedoraproject.org/rpms/qt5-qtwayland/blob/rawhide/f/qtwayland-decoration-support-backports-from-qt6.patch
  qt5ShadowsSupport ? false,
}:

let
  qt = if useQt6 then qt6 else qt5;
  qtVersion = if useQt6 then "6" else "5";

  pname = "qacatppuccindecorations";

  validFlavors = [
    "Mocha"
    "Macchiato"
    "Frappe"
    "Latte"
  ];
in

lib.checkListOfEnum "${pname} Valid theme flavors(s)" validFlavors
  [
    darkFlavor
    lightFlavor
  ]
  stdenv.mkDerivation
  rec {
    inherit version pname;

    src = ../.;

    nativeBuildInputs = [
      cmake
    ];

    buildInputs = with qt; [
      qtbase
      qtsvg
      qtwayland
      wayland
    ];

    dontWrapQtApps = true;

    cmakeFlags = [
      "-DQT_PLUGINS_DIR=${placeholder "out"}/${qt.qtbase.qtPluginPrefix}"
      "-DPROJECT_VERSION=${version}"
      "-DCATPPUCCIN_DARK_FLAVOR=${darkFlavor}"
      "-DCATPPUCCIN_LIGHT_FLAVOR=${lightFlavor}"
    ]
    ++ lib.optional useQt6 "-DUSE_QT6=true"
    ++ lib.optional qt5ShadowsSupport "-DHAS_QT6_SUPPORT=true";

    meta = {
      description = "Qt${qtVersion} Wayland Catppuccin-style client-side decoration plugin";
      homepage = "https://github.com/FedoraQt/QAdwaitaDecorations";
      license = lib.licenses.lgpl21Plus;
      maintainers = with lib.maintainers; [ sofiedotcafe ];
      platforms = lib.platforms.linux;
    };
  }
