<!-- markdownlint-disable MD033 MD041 MD010 MD013 MD045 -->

<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin for <a href="https://isocpp.org/">C++</a>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
	<a href="https://github.com/catppuccin/qt-decorations/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/qt-decorations?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/qt-decorations/issues"><img src="https://img.shields.io/github/issues/catppuccin/qt-decorations?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://github.com/catppuccin/qt-decorations/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/qt-decorations?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

## Previews

<details>
<summary>🌻 Latte</summary>
<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/latte.webp"/>
</details>
<details>
<summary>🪴 Frappé</summary>
<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/frappe.webp"/>
</details>
<details>
<summary>🌺 Macchiato</summary>
<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/macchiato.webp"/>
</details>
<details>
<summary>🌿 Mocha</summary>
<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/mocha.webp"/>
</details>

## How to compile

This library uses private Qt headers, meaning it is not guaranteed to be forward or backward compatible. You’ll need to recompile it with each Qt update.

Build instructions:

```sh
mkdir build
cd build
cmake [OPTIONS] [-DUSE_QT6=true] [-HAS_QT6_SUPPORT] ..
make && make install
```

## Usage

It can be used by setting the QT_WAYLAND_DECORATION environment variable:

```sh
export QT_WAYLAND_DECORATION=catppuccin
```

## 💝 Thanks to

- [sofiedotcafe](https://github.com/sofiedotcafe)
- [FedoraQt/QAdwaitaDecorations](https://github.com/FedoraQt/QAdwaitaDecorations/tree/main) — This project is based on their Qt decoration plugin implementing Adwaita-like client-side decorations.

&nbsp;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
