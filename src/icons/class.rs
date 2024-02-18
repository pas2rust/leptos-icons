use super::colors::Color;

pub fn class<'a>(color: Color) -> &'a str {
    match color {
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
        Color::Custom(class) => class,
    }
}