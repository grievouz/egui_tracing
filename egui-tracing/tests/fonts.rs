// Run this test with the library alone as well as in the workspace: eframe's
// default_fonts feature must not hide a missing egui-default feature mapping.
#[cfg(feature = "egui-default")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn egui_default_includes_bundled_fonts() {
    use egui_tracing::egui::{FontDefinitions, FontFamily};

    let fonts = FontDefinitions::default();
    assert!(!fonts.font_data.is_empty());
    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        let names = &fonts.families[&family];
        assert!(!names.is_empty());
        assert!(names.iter().all(|name| fonts.font_data.contains_key(name)));
    }
}
