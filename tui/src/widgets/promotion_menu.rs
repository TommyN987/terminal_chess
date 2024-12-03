use domain::{
    game::Color as DomainColor,
    moves::Move,
    pieces::{Bishop, Knight, Piece as DomainPiece, PieceType, Queen, Rook},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use super::{centered_rect, piece::Piece};

#[derive(Debug, Clone)]
pub struct PromotionMenu {
    pub pieces: [Piece; 4],
    pub selected: usize,
    pub m: Move,
}

impl PromotionMenu {
    pub fn new(piece_color: DomainColor, m: Move) -> Self {
        Self {
            pieces: [
                Piece::from(&DomainPiece::new(PieceType::Queen(Queen), piece_color)),
                Piece::from(&DomainPiece::new(PieceType::Rook(Rook), piece_color)),
                Piece::from(&DomainPiece::new(PieceType::Bishop(Bishop), piece_color)),
                Piece::from(&DomainPiece::new(PieceType::Knight(Knight), piece_color)),
            ],
            selected: 0,
            m,
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.pieces.len();
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.pieces.len() - 1;
        }
    }
}

impl Widget for PromotionMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("Promote")
            .title_alignment(Alignment::Center);

        let area = centered_rect(45, 40, area);

        Widget::render(block, area, buf);

        let inner_area_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(area);

        let inner_area_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 4),
                    Constraint::Ratio(1, 4),
                    Constraint::Ratio(1, 4),
                    Constraint::Ratio(1, 4),
                ]
                .as_ref(),
            )
            .split(inner_area_vertical[1]);

        // Render each piece, highlighting the selected one
        for (i, piece) in self.pieces.iter().enumerate() {
            let area = inner_area_horizontal[i];

            // Apply highlighting if the piece is selected
            if i == self.selected {
                let selected_style = Style::default().bg(Color::Yellow);
                let selected_block = Block::default().style(selected_style);

                Widget::render(selected_block, area, buf);
            }

            piece.clone().render(area, buf);
        }
    }
}
