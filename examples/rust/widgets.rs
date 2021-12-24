#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use azul::prelude::*;
use azul::widgets::*;
use azul::str::String as AzString;

#[derive(Default)]
struct WidgetShowcase {
    enable_padding: bool,
    active_tab: usize,
}

extern "C" fn layout(data: &mut RefAny, _: &mut LayoutCallbackInfo) -> StyledDom {

    let (enable_padding, active_tab) = match data.downcast_ref::<WidgetShowcase>() {
        Some(s) => (s.enable_padding, s.active_tab),
        None => return StyledDom::default(),
    };

    let text = if enable_padding {
        "Disable padding"
    } else {
        "Enable padding"
    };

    Dom::body()
    .with_menu_bar(Menu::new(vec![
        MenuItem::String(StringMenuItem::new("Menu Item 1".into()).with_children(vec![
            MenuItem::String(StringMenuItem::new("Submenu Item 1...".into()))
        ].into()))
    ].into()))
    .with_inline_style(if enable_padding {
        "padding: 50px"
    } else {
        ""
    }.into())
    .with_child(
        TabHeader::new(vec![format!("Test"), format!("Inactive"), format!("Inactive 2")].into())
        .with_active_tab(active_tab)
        .with_on_click(data.clone(), switch_active_tab)
        .dom()
    ).with_child(
        TabContent::new(match active_tab {
            0 => Frame::new("Frame".into(),
                Dom::div()
                .with_children(vec![
                    Button::new(text.into())
                    .with_on_click(data.clone(), enable_disable_padding)
                    .dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    CheckBox::new(enable_padding)
                    .with_on_toggle(data.clone(), enable_disable_padding_check)
                    .dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    DropDown::new(Vec::<AzString>::new().into()).dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    ProgressBar::new(20.0).dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    ColorInput::new(ColorU { r: 0, g: 0, b: 0, a: 255 }).dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    TextInput::new("Input text...".into()).dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    NumberInput::new(5.0).dom()
                    .with_inline_style("margin-bottom: 5px;".into()),

                    Dom::div()
                    .with_inline_style("flex-direction: row;".into())
                    .with_children(vec![
                        TreeView::new("".into()).dom(),
                        ListView::new(Vec::<AzString>::new().into()).dom(),
                    ].into())
                ].into())
                .with_inline_style("flex-grow: 1;".into())
            ).dom(),
            _ => Dom::div(),
        })
        .with_padding(enable_padding)
        .dom()
    ).style(Css::empty())
}

extern "C" fn switch_active_tab(data: &mut RefAny, _: &mut CallbackInfo, h: &TabHeaderState) -> Update {
    match data.downcast_mut::<WidgetShowcase>() {
        Some(mut s) => { s.active_tab = h.active_tab; Update::RefreshDom },
        None => Update::DoNothing,
    }
}

extern "C" fn enable_disable_padding_check(data: &mut RefAny, _: &mut CallbackInfo, c: &CheckBoxState) -> Update {
    match data.downcast_mut::<WidgetShowcase>() {
        Some(mut s) => { s.enable_padding = c.checked; Update::RefreshDom },
        None => Update::DoNothing,
    }
}

extern "C" fn enable_disable_padding(data: &mut RefAny, _: &mut CallbackInfo) -> Update {
    match data.downcast_mut::<WidgetShowcase>() {
        Some(mut s) => { s.enable_padding = !s.enable_padding; Update::RefreshDom },
        None => Update::DoNothing,
    }
}

fn main() {
    let data = RefAny::new(WidgetShowcase { enable_padding: true, active_tab: 1 });
    let app = App::new(data, AppConfig::new(LayoutSolver::Default));
    let mut options = WindowCreateOptions::new(layout);
    options.state.flags.frame = WindowFrame::Maximized;
    app.run(options);
}
