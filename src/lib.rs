#![allow(unused_imports, dead_code)]

use catppuccin::Flavor;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::OnceLock;

#[cfg(not(feature = "latte"))]
compile_error!("The `latte` feature must always be enabled.");

#[cfg(not(any(feature = "frappe", feature = "macchiato", feature = "mocha")))]
compile_error!("You must enable exactly two features: `latte` and one secondary variant (`frappe`, `macchiato`, or `mocha`).");

#[cfg(any(
    all(feature = "frappe", any(feature = "macchiato", feature = "mocha")),
    all(feature = "macchiato", any(feature = "frappe", feature = "mocha")),
    all(feature = "mocha", any(feature = "frappe", feature = "macchiato")),
))]
compile_error!("Features `frappe`, `macchiato`, and `mocha` are mutually exclusive. Only choose one to accompany `latte`.");

autocxx::include_cpp! {
    #include "bridge.h"
    safety!(unsafe)
}

extern "C" {
    fn register_rust_vtable(
        p: extern "C" fn(*mut c_void, i32, i32),
        m: extern "C" fn(bool, *mut i32, *mut i32, *mut i32, *mut i32),
        ms: extern "C" fn(i32, f64, f64, i32) -> bool,
        t: extern "C" fn(i32, f64, f64, bool) -> bool,
    );
    fn fill_rect(painter: *mut c_void, x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorRole {
    Background,
    BackgroundInactive,
    Foreground,
    ForegroundInactive,
    Border,
    BorderInactive,
    ButtonBackground,
    ButtonBackgroundInactive,
    HoveredButtonBackground,
    PressedButtonBackground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatppuccinFlavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl CatppuccinFlavor {
    fn palette(self) -> &'static Flavor {
        match self {
            CatppuccinFlavor::Latte => &catppuccin::PALETTE.latte,
            CatppuccinFlavor::Frappe => &catppuccin::PALETTE.frappe,
            CatppuccinFlavor::Macchiato => &catppuccin::PALETTE.macchiato,
            CatppuccinFlavor::Mocha => &catppuccin::PALETTE.mocha,
        }
    }
}

pub struct RustDecorationEngine {
    placement: i32, 
    buttons: HashMap<i32, u32>,
    current_flavor: CatppuccinFlavor,
}

impl RustDecorationEngine {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();
        buttons.insert(1, 1); 
        buttons.insert(2, 2); 
        buttons.insert(3, 3); 
        
        let current_flavor = if cfg!(feature = "mocha") {
            CatppuccinFlavor::Mocha
        } else if cfg!(feature = "macchiato") {
            CatppuccinFlavor::Macchiato
        } else if cfg!(feature = "frappe") {
            CatppuccinFlavor::Frappe
        } else {
            CatppuccinFlavor::Latte
        };
        Self {
            placement: 1, 
            buttons,
            current_flavor,
        }
    }

    pub fn get_color_rgb(&self, role: ColorRole, active: bool) -> (i32, i32, i32) {
        let p = self.current_flavor.palette();
        let rgb_obj = if !active {
            match role {
                ColorRole::Background | ColorRole::BackgroundInactive => p.colors.mantle.rgb,
                ColorRole::Foreground | ColorRole::ForegroundInactive => p.colors.subtext0.rgb,
                _ => p.colors.surface0.rgb,
            }
        } else {
            match role {
                ColorRole::Background => p.colors.base.rgb,
                ColorRole::BackgroundInactive => p.colors.mantle.rgb,
                ColorRole::Foreground => p.colors.text.rgb,
                ColorRole::ForegroundInactive => p.colors.subtext0.rgb,
                ColorRole::Border => p.colors.surface1.rgb,
                ColorRole::BorderInactive => p.colors.surface0.rgb,
                ColorRole::ButtonBackground => p.colors.surface0.rgb,
                ColorRole::ButtonBackgroundInactive => p.colors.crust.rgb,
                ColorRole::HoveredButtonBackground => p.colors.surface2.rgb,
                ColorRole::PressedButtonBackground => p.colors.overlay0.rgb,
            }
        };
        (rgb_obj.r as i32, rgb_obj.g as i32, rgb_obj.b as i32)
    }

    pub fn calculate_button_rect(&self, button_id: i32, content_width: i32, top_margin: i32) -> (f64, f64, f64, f64) {
        let btn_width = 24.0;
        let btn_spacing = 12.0;
        let btn_pos = *self.buttons.get(&button_id).unwrap_or(&0) as f64;
        let x_pos = if self.placement == 1 {
            (content_width as f64) - (btn_width * btn_pos) - (btn_spacing * btn_pos) - 10.0 
        } else {
            (btn_width * btn_pos) + (btn_spacing * btn_pos) - btn_width + 10.0
        };
        let y_pos = ((top_margin as f64) - btn_width) / 2.0;
        (x_pos, y_pos, btn_width, btn_width)
    }
}

static ENGINE: OnceLock<RustDecorationEngine> = OnceLock::new();

fn get_engine() -> &'static RustDecorationEngine {
    ENGINE.get_or_init(RustDecorationEngine::new)
}

#[no_mangle]
pub unsafe extern "C" fn initialize_decorations_plugin() {
    register_rust_vtable(paint, margins, mouse, touch);
}

extern "C" fn paint(painter_ptr: *mut c_void, width: i32, _height: i32) {
    if painter_ptr.is_null() { return; }
    
    let engine = get_engine();
    let (bg_r, bg_g, bg_b) = engine.get_color_rgb(ColorRole::Background, true);
    let (brd_r, brd_g, brd_b) = engine.get_color_rgb(ColorRole::Border, true);
    
    unsafe {
        // Main titlebar background frame
        fill_rect(painter_ptr, 0, 0, width, 49, bg_r, bg_g, bg_b);
        // Single pixel accent border line under titlebar
        fill_rect(painter_ptr, 0, 48, width, 1, brd_r, brd_g, brd_b);
        
        for button_id in 1..=3 {
            let (x, y, w, h) = engine.calculate_button_rect(button_id, width, 49);
            let (btn_r, btn_g, btn_b) = engine.get_color_rgb(ColorRole::ButtonBackground, true);
            
            fill_rect(
                painter_ptr, 
                x as i32, 
                y as i32, 
                w as i32, 
                h as i32, 
                btn_r, 
                btn_g, 
                btn_b
            );
        }
    }
}

extern "C" fn margins(shadows_only: bool, l: *mut i32, t: *mut i32, r: *mut i32, b: *mut i32) {
    unsafe {
        if shadows_only {
            *l = 10; *t = 10; *r = 10; *b = 10;
        } else {
            *l = 11; *t = 49; *r = 11; *b = 11;
        }
    }
}

extern "C" fn mouse(width: i32, local_x: f64, local_y: f64, buttons: i32) -> bool {
    let engine = get_engine();
    if local_y <= 49.0 {
        for button_id in 1..=3 {
            let (x, y, w, h) = engine.calculate_button_rect(button_id, width, 49);
            if local_x >= x && local_x <= (x + w) && local_y >= y && local_y <= (y + h) {
                if buttons != 0 { return true; }
            }
        }
        return true; 
    }
    false
}

extern "C" fn touch(width: i32, local_x: f64, local_y: f64, active: bool) -> bool {
    let engine = get_engine();
    if active && local_y <= 49.0 {
        for button_id in 1..=3 {
            let (x, y, w, h) = engine.calculate_button_rect(button_id, width, 49);
            if local_x >= x && local_x <= (x + w) && local_y >= y && local_y <= (y + h) {
                return true; 
            }
        }
        return true;
    }
    false
}