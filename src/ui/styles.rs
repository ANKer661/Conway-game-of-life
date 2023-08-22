use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.4, 0.8, 0.8);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.1, 1.0, 1.0);

pub const UI_ROOT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.justify_content = JustifyContent::SpaceBetween;
    style
};

pub const BUTTON_BG_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Px(100.0);
    style.border = UiRect::all(Val::Px(5.0));
    style
};

pub const BUTTON_FILL_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.justify_content = JustifyContent::FlexEnd;
    style
};

pub const CLASSIC_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(150.0);
    style.height = Val::Px(50.0);
    style.margin = UiRect::all(Val::Auto);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn get_classic_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Symtext.ttf"),
        font_size: 32.0,
        color: Color::WHITE.into(),
    }
}
