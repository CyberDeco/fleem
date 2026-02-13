# fleem

Design tokens and style recipes for [floem](https://github.com/lapce/floem).

---

## Features

- **Design tokens** — semantic color, typography, and spacing values
- **Palette** — bundled dark/light palettes with all tokens, plus a trait for custom themes
- **Variant system** — `Variant`, `Size`, and `Fill` enums that resolve to concrete colors
- **Style recipes** — functions like `button_style()`, `card_style()`, `input_style()` that return `Style` values
- **Theme system** — reactive `Appearance` signal with OS dark mode detection

---

## Quick start

```rust
use fleem::{Appearance, Variant, Size, Fill, init, palette, styles};

fn main() {
    // Initialize the theme (call once at startup)
    let appearance = init(Appearance::System);

    // In your views, use palette() + style recipes:
    button("Click me").style(move |s| {
        s.apply(styles::button_style(
            &palette(),
            Variant::Primary,
            Size::Normal,
            Fill::Filled,
        ))
    });
}
```

---

## Custom themes

Implement `ThemeDef` to create your own palette:

```rust
struct NordTheme;
impl fleem::ThemeDef for NordTheme {
    fn name(&self) -> &str { "Nord" }
    fn dark_palette(&self) -> fleem::Palette {
        let mut p = fleem::Palette::dark();
        p.colors.bg_base = Color::rgb8(46, 52, 64);
        p.colors.accent = Color::rgb8(136, 192, 208);
        p
    }
    fn light_palette(&self) -> fleem::Palette {
        fleem::Palette::light()
    }
}

fn main() {
    fleem::init_with(NordTheme, Appearance::System);
}
```

---

## Credits

Inspired by [oxytail](https://github.com/golota60/oxytail)

