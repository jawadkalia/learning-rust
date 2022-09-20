use cursive::{
    theme::{BaseColor::*, BorderStyle, Palette, PaletteColor::*},
    view::Resizable,
    views::SelectView,
    Cursive, With,
};

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Outset,
        palette: Palette::default().with(|palette| {
            palette[Background] = Black.light();
            palette[Primary] = Black.dark();
            palette[Shadow] = White.light();
            palette[View] = White.dark();
            palette[Highlight] = Cyan.light();
            palette[HighlightInactive] = Blue.light();
            palette[HighlightText] = Blue.light()
        }),
    });

    let select = SelectView::<String>::new().fixed_size((10, 5));

    siv.run();
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer()
}
