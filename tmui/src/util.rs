use crate::skia_safe::Font;

pub fn skia_font_clone(src: &Font) -> Font {
    let mut font = Font::default();
    font.set_force_auto_hinting(src.is_force_auto_hinting());
    font.set_embedded_bitmaps(src.is_embedded_bitmaps());
    font.set_subpixel(src.is_subpixel());
    font.set_linear_metrics(src.is_linear_metrics());
    font.set_embolden(src.is_embolden());
    font.set_baseline_snap(src.is_baseline_snap());
    font.set_edging(src.edging());
    font.set_hinting(src.hinting());
    if let Some(typeface) = src.typeface() {
        font.set_typeface(typeface);
    }
    font.set_size(src.size());
    font.set_scale_x(src.scale_x());
    font.set_skew_x(src.skew_x());
    font
}
