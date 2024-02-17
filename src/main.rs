use leptos::{leptos_dom::logging::console_warn, *};
mod icons;
use icons::{
    colors::{Color, ColorTrait},
    icons::{Icon, IconTrait, Icons},
};

fn copy_to_clipboard(text: String) {
    console_warn(text.as_str());
}

pub fn main() {
    let icons = Icon::to_vec();
    let colors = Color::to_vec();

    let build = |icon: Icon, color: Color| {
        if color == Color::Black {
            return view! {
                <div on:click=move |_| copy_to_clipboard(format!("<Icons style=(Icon::{:?}, Color::{:?}, 24) />", icon, color)) class="transition m-2 cursor-pointer bg-white p-2 ring-1 ring-white shadow-lg shadow-black hover:transition hover:opacity-50">
                    <Icons style=(icon, color, 24) />
                </div>
            };
        }
        view! {
            <div on:click=move |_| copy_to_clipboard(format!("<Icons style=(Icon::{:?}, Color::{:?}, 24) />", icon, color)) class="transition m-2 cursor-pointer p-2 ring-1 ring-white shadow-lg shadow-black hover:transition hover:opacity-50">
                <Icons style=(icon, color, 24) />
            </div>
        }
    };
    let mut app = vec![];
    for icon in icons {
        for color in &colors {
            app.push(build(icon, *color));
        }
    }
    mount_to_body(move || {
        view! {
            <div class="flex flex-wrap">
                {app}
            </div>
        }
    });
}
