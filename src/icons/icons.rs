use super::colors::Color;
use leptos::*;

#[derive(Clone, Copy, Debug)]
pub enum Icon {
    Info,
    Ok,
    X,
    Exclamation,
    Question,
    BellBounce,
    Bug,
    ExternalLink,
    Bell,
    Heart,
    Donation,
    Star,
}

pub trait IconTrait {
    fn to_vec() -> Vec<Icon>;
}

impl IconTrait for Icon {
    fn to_vec() -> Vec<Icon> {
        use Icon::*;
        vec![
            Info,
            Ok,
            X,
            Exclamation,
            Question,
            BellBounce,
            Bug,
            ExternalLink,
            Bell,
            Heart,
            Star,
            Donation,
        ]
    }
}

#[component]
pub fn Icons(style: (Icon, Color, usize)) -> impl IntoView {
    let (icon, color, size) = style;
    let class = match color {
        Color::White => "text-white",
        Color::Black => "text-black",

        Color::Red100 => "text-red-100",
        Color::Red200 => "text-red-200",
        Color::Red300 => "text-red-300",
        Color::Red400 => "text-red-400",
        Color::Red500 => "text-red-500",
        Color::Red600 => "text-red-600",
        Color::Red700 => "text-red-700",
        Color::Red800 => "text-red-800",
        Color::Red900 => "text-red-900",
        Color::Red950 => "text-red-950",

        Color::Green100 => "text-green-100",
        Color::Green200 => "text-green-200",
        Color::Green300 => "text-green-300",
        Color::Green400 => "text-green-400",
        Color::Green500 => "text-green-500",
        Color::Green600 => "text-green-600",
        Color::Green700 => "text-green-700",
        Color::Green800 => "text-green-800",
        Color::Green900 => "text-green-900",
        Color::Green950 => "text-green-950",

        Color::Purple100 => "text-purple-100",
        Color::Purple200 => "text-purple-200",
        Color::Purple300 => "text-purple-300",
        Color::Purple400 => "text-purple-400",
        Color::Purple500 => "text-purple-500",
        Color::Purple600 => "text-purple-600",
        Color::Purple700 => "text-purple-700",
        Color::Purple800 => "text-purple-800",
        Color::Purple900 => "text-purple-900",
        Color::Purple950 => "text-purple-950",

        Color::Rose100 => "text-rose-100",
        Color::Rose200 => "text-rose-200",
        Color::Rose300 => "text-rose-300",
        Color::Rose400 => "text-rose-400",
        Color::Rose500 => "text-rose-500",
        Color::Rose600 => "text-rose-600",
        Color::Rose700 => "text-rose-700",
        Color::Rose800 => "text-rose-800",
        Color::Rose900 => "text-rose-900",
        Color::Rose950 => "text-rose-950",

        Color::Orange100 => "text-orange-100",
        Color::Orange200 => "text-orange-200",
        Color::Orange300 => "text-orange-300",
        Color::Orange400 => "text-orange-400",
        Color::Orange500 => "text-orange-500",
        Color::Orange600 => "text-orange-600",
        Color::Orange700 => "text-orange-700",
        Color::Orange800 => "text-orange-800",
        Color::Orange900 => "text-orange-900",
        Color::Orange950 => "text-orange-950",

        Color::Amber100 => "text-amber-100",
        Color::Amber200 => "text-amber-200",
        Color::Amber300 => "text-amber-300",
        Color::Amber400 => "text-amber-400",
        Color::Amber500 => "text-amber-500",
        Color::Amber600 => "text-amber-600",
        Color::Amber700 => "text-amber-700",
        Color::Amber800 => "text-amber-800",
        Color::Amber900 => "text-amber-900",
        Color::Amber950 => "text-amber-950",

        Color::Lime100 => "text-lime-100",
        Color::Lime200 => "text-lime-200",
        Color::Lime300 => "text-lime-300",
        Color::Lime400 => "text-lime-400",
        Color::Lime500 => "text-lime-500",
        Color::Lime600 => "text-lime-600",
        Color::Lime700 => "text-lime-700",
        Color::Lime800 => "text-lime-800",
        Color::Lime900 => "text-lime-900",
        Color::Lime950 => "text-lime-950",

        Color::Yellow100 => "text-yellow-100",
        Color::Yellow200 => "text-yellow-200",
        Color::Yellow300 => "text-yellow-300",
        Color::Yellow400 => "text-yellow-400",
        Color::Yellow500 => "text-yellow-500",
        Color::Yellow600 => "text-yellow-600",
        Color::Yellow700 => "text-yellow-700",
        Color::Yellow800 => "text-yellow-800",
        Color::Yellow900 => "text-yellow-900",
        Color::Yellow950 => "text-yellow-950",

        Color::Blue100 => "text-blue-100",
        Color::Blue200 => "text-blue-200",
        Color::Blue300 => "text-blue-300",
        Color::Blue400 => "text-blue-400",
        Color::Blue500 => "text-blue-500",
        Color::Blue600 => "text-blue-600",
        Color::Blue700 => "text-blue-700",
        Color::Blue800 => "text-blue-800",
        Color::Blue900 => "text-blue-900",
        Color::Blue950 => "text-blue-950",

        Color::Emerald100 => "text-emerald-100",
        Color::Emerald200 => "text-emerald-200",
        Color::Emerald300 => "text-emerald-300",
        Color::Emerald400 => "text-emerald-400",
        Color::Emerald500 => "text-emerald-500",
        Color::Emerald600 => "text-emerald-600",
        Color::Emerald700 => "text-emerald-700",
        Color::Emerald800 => "text-emerald-800",
        Color::Emerald900 => "text-emerald-900",
        Color::Emerald950 => "text-emerald-950",

        Color::Violet100 => "text-violet-100",
        Color::Violet200 => "text-violet-200",
        Color::Violet300 => "text-violet-300",
        Color::Violet400 => "text-violet-400",
        Color::Violet500 => "text-violet-500",
        Color::Violet600 => "text-violet-600",
        Color::Violet700 => "text-violet-700",
        Color::Violet800 => "text-violet-800",
        Color::Violet900 => "text-violet-900",
        Color::Violet950 => "text-violet-950",

        Color::Fuchsia100 => "text-fuchsia-100",
        Color::Fuchsia200 => "text-fuchsia-200",
        Color::Fuchsia300 => "text-fuchsia-300",
        Color::Fuchsia400 => "text-fuchsia-400",
        Color::Fuchsia500 => "text-fuchsia-500",
        Color::Fuchsia600 => "text-fuchsia-600",
        Color::Fuchsia700 => "text-fuchsia-700",
        Color::Fuchsia800 => "text-fuchsia-800",
        Color::Fuchsia900 => "text-fuchsia-900",
        Color::Fuchsia950 => "text-fuchsia-950",

        Color::Pink100 => "text-pink-100",
        Color::Pink200 => "text-pink-200",
        Color::Pink300 => "text-pink-300",
        Color::Pink400 => "text-pink-400",
        Color::Pink500 => "text-pink-500",
        Color::Pink600 => "text-pink-600",
        Color::Pink700 => "text-pink-700",
        Color::Pink800 => "text-pink-800",
        Color::Pink900 => "text-pink-900",
        Color::Pink950 => "text-pink-950",

        Color::Teal100 => "text-teal-100",
        Color::Teal200 => "text-teal-200",
        Color::Teal300 => "text-teal-300",
        Color::Teal400 => "text-teal-400",
        Color::Teal500 => "text-teal-500",
        Color::Teal600 => "text-teal-600",
        Color::Teal700 => "text-teal-700",
        Color::Teal800 => "text-teal-800",
        Color::Teal900 => "text-teal-900",
        Color::Teal950 => "text-teal-950",

        Color::Cyan100 => "text-cyan-100",
        Color::Cyan200 => "text-cyan-200",
        Color::Cyan300 => "text-cyan-300",
        Color::Cyan400 => "text-cyan-400",
        Color::Cyan500 => "text-cyan-500",
        Color::Cyan600 => "text-cyan-600",
        Color::Cyan700 => "text-cyan-700",
        Color::Cyan800 => "text-cyan-800",
        Color::Cyan900 => "text-cyan-900",
        Color::Cyan950 => "text-cyan-950",

        Color::Sky100 => "text-sky-100",
        Color::Sky200 => "text-sky-200",
        Color::Sky300 => "text-sky-300",
        Color::Sky400 => "text-sky-400",
        Color::Sky500 => "text-sky-500",
        Color::Sky600 => "text-sky-600",
        Color::Sky700 => "text-sky-700",
        Color::Sky800 => "text-sky-800",
        Color::Sky900 => "text-sky-900",
        Color::Sky950 => "text-sky-950",

        Color::Indigo100 => "text-indigo-100",
        Color::Indigo200 => "text-indigo-200",
        Color::Indigo300 => "text-indigo-300",
        Color::Indigo400 => "text-indigo-400",
        Color::Indigo500 => "text-indigo-500",
        Color::Indigo600 => "text-indigo-600",
        Color::Indigo700 => "text-indigo-700",
        Color::Indigo800 => "text-indigo-800",
        Color::Indigo900 => "text-indigo-900",
        Color::Indigo950 => "text-indigo-950",
        Color::None => "",
    };
    let render = match icon {
        Icon::Info => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                <path class=class d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z"/>
            </svg>
        },
        Icon::Ok => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 18 20">
                <path class=class d="m17.351 3.063-8-3a1.009 1.009 0 0 0-.7 0l-8 3A1 1 0 0 0 0 4a19.394 19.394 0 0 0 8.47 15.848 1 1 0 0 0 1.06 0A19.394 19.394 0 0 0 18 4a1 1 0 0 0-.649-.937Zm-3.644 4.644-5 5A1 1 0 0 1 8 13c-.033 0-.065 0-.1-.005a1 1 0 0 1-.733-.44l-2-3a1 1 0 0 1 1.664-1.11l1.323 1.986 4.138-4.138a1 1 0 0 1 1.414 1.414h.001Z"/>
            </svg>
        },
        Icon::X => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                <path class=class d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 11.793a1 1 0 1 1-1.414 1.414L10 11.414l-2.293 2.293a1 1 0 0 1-1.414-1.414L8.586 10 6.293 7.707a1 1 0 0 1 1.414-1.414L10 8.586l2.293-2.293a1 1 0 0 1 1.414 1.414L11.414 10l2.293 2.293Z"/>
            </svg>
        },
        Icon::Exclamation => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                <path class=class d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM10 15a1 1 0 1 1 0-2 1 1 0 0 1 0 2Zm1-4a1 1 0 0 1-2 0V6a1 1 0 0 1 2 0v5Z"/>
            </svg>
        },
        Icon::Question => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                <path class=class d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm0 16a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3Zm1-5.034V12a1 1 0 0 1-2 0v-1.418a1 1 0 0 1 1.038-.999 1.436 1.436 0 0 0 1.488-1.441 1.501 1.501 0 1 0-3-.116.986.986 0 0 1-1.037.961 1 1 0 0 1-.96-1.037A3.5 3.5 0 1 1 11 11.466Z"/>
            </svg>
        },
        Icon::BellBounce => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                <path class=class d="M15.133 10.632v-1.8a5.406 5.406 0 0 0-4.154-5.262.955.955 0 0 0 .021-.106V1.1a1 1 0 0 0-2 0v2.364a.946.946 0 0 0 .021.106 5.406 5.406 0 0 0-4.154 5.262v1.8C4.867 13.018 3 13.614 3 14.807 3 15.4 3 16 3.538 16h12.924C17 16 17 15.4 17 14.807c0-1.193-1.867-1.789-1.867-4.175ZM4 4a1 1 0 0 1-.707-.293l-1-1a1 1 0 0 1 1.414-1.414l1 1A1 1 0 0 1 4 4ZM2 8H1a1 1 0 0 1 0-2h1a1 1 0 1 1 0 2Zm14-4a1 1 0 0 1-.707-1.707l1-1a1 1 0 1 1 1.414 1.414l-1 1A1 1 0 0 1 16 4Zm3 4h-1a1 1 0 1 1 0-2h1a1 1 0 1 1 0 2ZM6.823 17a3.453 3.453 0 0 0 6.354 0H6.823Z"/>
            </svg>
        },
        Icon::Bug => view! {
            <svg width=size height=size aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 19 20">
                <path class=class d="M16.025 15H14.91c.058-.33.088-.665.09-1v-1h2a1 1 0 0 0 0-2h-2.09a5.97 5.97 0 0 0-.26-1h.375a2 2 0 0 0 2-2V6a1 1 0 0 0-2 0v2H13.46a6.239 6.239 0 0 0-.46-.46V6a3.963 3.963 0 0 0-.986-2.6l.693-.693A1 1 0 0 0 13 2V1a1 1 0 0 0-2 0v.586l-.661.661a3.753 3.753 0 0 0-2.678 0L7 1.586V1a1 1 0 0 0-2 0v1a1 1 0 0 0 .293.707l.693.693A3.963 3.963 0 0 0 5 6v1.54a6.239 6.239 0 0 0-.46.46H3V6a1 1 0 0 0-2 0v2a2 2 0 0 0 2 2h.35a5.97 5.97 0 0 0-.26 1H1a1 1 0 0 0 0 2h2v1a6 6 0 0 0 .09 1H2a2 2 0 0 0-2 2v2a1 1 0 1 0 2 0v-2h1.812A6.012 6.012 0 0 0 8 19.907V10a1 1 0 0 1 2 0v9.907A6.011 6.011 0 0 0 14.188 17h1.837v2a1 1 0 0 0 2 0v-2a2 2 0 0 0-2-2ZM11 6.35a5.922 5.922 0 0 0-.941-.251l-.111-.017a5.52 5.52 0 0 0-1.9 0l-.111.017A5.924 5.924 0 0 0 7 6.35V6a2 2 0 1 1 4 0v.35Z"/>
            </svg>
        },
        Icon::ExternalLink => view! {
            <svg width=size height=size xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path class=class d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                <polyline class=class points="15 3 21 3 21 9"></polyline>
                <line class=class x1="10" y1="14" x2="21" y2="3"></line>
            </svg>
        },
        Icon::Bell => view! {
            <svg width=size fill="currentColor" height=size xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path class=class d="M10,20h4a2,2,0,0,1-4,0Zm8-4V10a6,6,0,0,0-5-5.91V3a1,1,0,0,0-2,0V4.09A6,6,0,0,0,6,10v6L4,18H20Z"/>
            </svg>
        },
        Icon::Heart => view! {
            <svg width=size fill="currentColor" height=size xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128">
            <path class=class d="M92.591 6.124a35.52 35.52 0 0 0-17.862 4.82 35.593 35.593 0 0 0-9.623 8.246l-1.11 1.364-1.109-1.364a35.501 35.501 0 0 0-9.619-8.246 35.522 35.522 0 0 0-17.865-4.82C16.212 6.124.673 21.608.022 41.374c-.33 9.918 2.848 19.973 9.439 29.902 5.194 7.816 54.535 50.6 54.535 50.6s49.345-42.791 54.535-50.6c6.598-9.93 9.77-19.988 9.445-29.902-.655-19.766-16.196-35.25-35.385-35.25z"/></svg>
        },
        Icon::Star => view! {
            <svg width=size fill="currentColor" height=size xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path class=class d="m21.5 9.757-5.278 4.354 1.649 7.389L12 17.278 6.129 21.5l1.649-7.389L2.5 9.757l6.333-.924L12 2.5l3.167 6.333z"/>
            </svg>
        },
        Icon::Donation => view! {
            <svg width=size fill="currentColor" height=size
            xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" x="0px" y="0px" viewBox="0 0 100 125"
            >
                <g class=class>
                    <path class=class d="M63.701,99.994c0,0-0.021,0.001-0.049,0.002C63.668,99.996,63.683,99.995,63.701,99.994z M63.53,100   c0.004,0,0.015,0,0.034-0.001C63.549,99.999,63.535,100,63.53,100z M97.795,72.836c-2.979-3.031-7.846-3.076-10.875-0.098   c-3.148,3.074-14.023,11.849-21.849,11.849c-0.093,0.007-11.544,0.45-14.841-0.524c-2.023-0.598-2.748-1.665-3.392-2.396h20.173   c4.25,0,7.693-3.444,7.693-7.693s-3.443-7.693-7.693-7.693H54.003C38.435,54.545,19.864,65.92,19.864,65.92H0v27.134h29.409   c3.81,2.085,8.126,4.446,12.9,5.383c8.582,1.685,18.396,1.558,21.392,1.558C79.174,99.913,96.01,85.379,97.697,83.72   C100.728,80.74,100.773,75.865,97.795,72.836z M77.893,78.715L86.6,71.68c0,0-1.542-2.418-4.09-1.784   C79.961,70.529,77.893,78.715,77.893,78.715z M81.709,0c-6.468,0-12.15,3.356-15.404,8.422C63.053,3.356,57.37,0,50.902,0   C40.8,0,32.611,8.189,32.611,18.291c0,21.652,33.693,41.671,33.693,41.671S100,39.943,100,18.291C100,8.189,91.811,0,81.709,0z"/>
                </g>
            </svg>
        },
    };
    render
}
