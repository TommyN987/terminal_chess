use ratatui::style::Color;

pub const TITLE: &str = r#"
 ██████╗██╗  ██╗███████╗███████╗███████╗
██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝
██║     ███████║█████╗  ███████╗███████╗
██║     ██╔══██║██╔══╝  ╚════██║╚════██║
╚██████╗██║  ██║███████╗███████║███████║
 ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
"#;

pub const WHITE: Color = Color::Rgb(160, 160, 160);
pub const BLACK: Color = Color::Rgb(128, 95, 69);

pub const BISHOP: &str = "\
    \n\
       ⭘\n\
      █✝█\n\
      ███\n\
    ▗█████▖\n\
    ";

pub const KING: &str = "\
      ✚\n\
    ▞▀▄▀▚\n\
    ▙▄█▄▟\n\
    ▐███▌\n\
   ▗█████▖\n\
";

pub const KNIGHT: &str = "\
    \n\
    ▟▛██▙\n\
   ▟█████\n\
   ▀▀▟██▌\n\
    ▟████\n\
";

pub const PAWN: &str = "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
";

pub const QUEEN: &str = "\
    \n\
◀█▟█▙█▶\n\
  ◥█◈█◤\n\
  ███\n\
▗█████▖\n\
";

pub const ROOK: &str = "\
    \n\
    █▟█▙█\n\
    ▜███▛\n\
    ▐███▌\n\
   ▗█████▖\n\
";
