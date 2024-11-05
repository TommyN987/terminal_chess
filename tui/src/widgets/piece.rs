use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Widget},
};

use domain::{
    pieces::{Piece as DomainPiece, PieceType},
    Color as DomainColor,
};

use crate::constants::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK};

pub struct Piece(DomainPiece);

impl From<&DomainPiece> for Piece {
    fn from(value: &DomainPiece) -> Self {
        Self(*value)
    }
}

impl Piece {
    pub fn inner(self) -> DomainPiece {
        self.0
    }
}

impl Widget for Piece {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let piece = self.inner();
        let color: Color = match &piece.piece_color {
            DomainColor::Black => Color::Black,
            DomainColor::White => Color::White,
        };

        let piece_str = match &piece.piece_type {
            PieceType::Bishop(_) => BISHOP,
            PieceType::King(_) => KING,
            PieceType::Knight(_) => KNIGHT,
            PieceType::Pawn(_) => PAWN,
            PieceType::Queen(_) => QUEEN,
            PieceType::Rook(_) => ROOK,
        };

        let piece = Paragraph::new(piece_str)
            .style(Style::default())
            .fg(color)
            .alignment(Alignment::Center);

        Widget::render(piece, area, buf);
    }
}
