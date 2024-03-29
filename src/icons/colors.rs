#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
    Violet100,
    Violet200,
    Violet300,
    Violet400,
    Violet500,
    Violet600,
    Violet700,
    Violet800,
    Violet900,
    Violet950,
    Fuchsia100,
    Fuchsia200,
    Fuchsia300,
    Fuchsia400,
    Fuchsia500,
    Fuchsia600,
    Fuchsia700,
    Fuchsia800,
    Fuchsia900,
    Fuchsia950,
    Red100,
    Red200,
    Red300,
    Red400,
    Red500,
    Red600,
    Red700,
    Red800,
    Red900,
    Red950,
    Yellow100,
    Yellow200,
    Yellow300,
    Yellow400,
    Yellow500,
    Yellow600,
    Yellow700,
    Yellow800,
    Yellow900,
    Yellow950,
    Green100,
    Green200,
    Green300,
    Green400,
    Green500,
    Green600,
    Green700,
    Green800,
    Green900,
    Green950,
    Blue100,
    Blue200,
    Blue300,
    Blue400,
    Blue500,
    Blue600,
    Blue700,
    Blue800,
    Blue900,
    Blue950,
    Indigo100,
    Indigo200,
    Indigo300,
    Indigo400,
    Indigo500,
    Indigo600,
    Indigo700,
    Indigo800,
    Indigo900,
    Indigo950,
    Purple100,
    Purple200,
    Purple300,
    Purple400,
    Purple500,
    Purple600,
    Purple700,
    Purple800,
    Purple900,
    Purple950,
    Pink100,
    Pink200,
    Pink300,
    Pink400,
    Pink500,
    Pink600,
    Pink700,
    Pink800,
    Pink900,
    Pink950,
    Rose100,
    Rose200,
    Rose300,
    Rose400,
    Rose500,
    Rose600,
    Rose700,
    Rose800,
    Rose900,
    Rose950,
    Orange100,
    Orange200,
    Orange300,
    Orange400,
    Orange500,
    Orange600,
    Orange700,
    Orange800,
    Orange900,
    Orange950,
    Amber100,
    Amber200,
    Amber300,
    Amber400,
    Amber500,
    Amber600,
    Amber700,
    Amber800,
    Amber900,
    Amber950,
    Lime100,
    Lime200,
    Lime300,
    Lime400,
    Lime500,
    Lime600,
    Lime700,
    Lime800,
    Lime900,
    Lime950,
    Emerald100,
    Emerald200,
    Emerald300,
    Emerald400,
    Emerald500,
    Emerald600,
    Emerald700,
    Emerald800,
    Emerald900,
    Emerald950,
    Teal100,
    Teal200,
    Teal300,
    Teal400,
    Teal500,
    Teal600,
    Teal700,
    Teal800,
    Teal900,
    Teal950,
    Cyan100,
    Cyan200,
    Cyan300,
    Cyan400,
    Cyan500,
    Cyan600,
    Cyan700,
    Cyan800,
    Cyan900,
    Cyan950,
    Sky100,
    Sky200,
    Sky300,
    Sky400,
    Sky500,
    Sky600,
    Sky700,
    Sky800,
    Sky900,
    Sky950,
    None,
    Custom(&'static str),
}

pub trait ColorTrait {
    fn to_vec() -> Vec<Color>;
}

impl ColorTrait for Color {
    fn to_vec() -> Vec<Color> {
        use Color::*;
        vec![
            White,
            Black,
            Violet100,
            Violet200,
            Violet300,
            Violet400,
            Violet500,
            Violet600,
            Violet700,
            Violet800,
            Violet900,
            Violet950,
            Fuchsia100,
            Fuchsia200,
            Fuchsia300,
            Fuchsia400,
            Fuchsia500,
            Fuchsia600,
            Fuchsia700,
            Fuchsia800,
            Fuchsia900,
            Fuchsia950,
            Red100,
            Red200,
            Red300,
            Red400,
            Red500,
            Red600,
            Red700,
            Red800,
            Red900,
            Red950,
            Yellow100,
            Yellow200,
            Yellow300,
            Yellow400,
            Yellow500,
            Yellow600,
            Yellow700,
            Yellow800,
            Yellow900,
            Yellow950,
            Green100,
            Green200,
            Green300,
            Green400,
            Green500,
            Green600,
            Green700,
            Green800,
            Green900,
            Green950,
            Blue100,
            Blue200,
            Blue300,
            Blue400,
            Blue500,
            Blue600,
            Blue700,
            Blue800,
            Blue900,
            Blue950,
            Indigo100,
            Indigo200,
            Indigo300,
            Indigo400,
            Indigo500,
            Indigo600,
            Indigo700,
            Indigo800,
            Indigo900,
            Indigo950,
            Purple100,
            Purple200,
            Purple300,
            Purple400,
            Purple500,
            Purple600,
            Purple700,
            Purple800,
            Purple900,
            Purple950,
            Pink100,
            Pink200,
            Pink300,
            Pink400,
            Pink500,
            Pink600,
            Pink700,
            Pink800,
            Pink900,
            Pink950,
            Rose100,
            Rose200,
            Rose300,
            Rose400,
            Rose500,
            Rose600,
            Rose700,
            Rose800,
            Rose900,
            Rose950,
            Orange100,
            Orange200,
            Orange300,
            Orange400,
            Orange500,
            Orange600,
            Orange700,
            Orange800,
            Orange900,
            Orange950,
            Amber100,
            Amber200,
            Amber300,
            Amber400,
            Amber500,
            Amber600,
            Amber700,
            Amber800,
            Amber900,
            Amber950,
            Lime100,
            Lime200,
            Lime300,
            Lime400,
            Lime500,
            Lime600,
            Lime700,
            Lime800,
            Lime900,
            Lime950,
            Emerald100,
            Emerald200,
            Emerald300,
            Emerald400,
            Emerald500,
            Emerald600,
            Emerald700,
            Emerald800,
            Emerald900,
            Emerald950,
            Teal100,
            Teal200,
            Teal300,
            Teal400,
            Teal500,
            Teal600,
            Teal700,
            Teal800,
            Teal900,
            Teal950,
            Cyan100,
            Cyan200,
            Cyan300,
            Cyan400,
            Cyan500,
            Cyan600,
            Cyan700,
            Cyan800,
            Cyan900,
            Cyan950,
            Sky100,
            Sky200,
            Sky300,
            Sky400,
            Sky500,
            Sky600,
            Sky700,
            Sky800,
            Sky900,
            Sky950,
            None,
            Custom("text-[#782AB6]"),
        ]
    }
}
