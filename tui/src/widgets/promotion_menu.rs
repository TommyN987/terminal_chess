use domain::{
    pieces::{Bishop, Knight, Piece as DomainPiece, PieceType, Queen, Rook},
    Color as DomainColor,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use super::piece::Piece;

#[derive(Debug, Clone)]
pub struct PromotionMenu {
    pub pieces: [Piece; 4],
    pub selected: usize,
}

impl PromotionMenu {
    pub fn new(piece_color: DomainColor) -> Self {
        Self {
            pieces: [
                Piece::from(&DomainPiece::new(
                    PieceType::Queen(Queen::new()),
                    piece_color,
                )),
                Piece::from(&DomainPiece::new(PieceType::Rook(Rook::new()), piece_color)),
                Piece::from(&DomainPiece::new(
                    PieceType::Bishop(Bishop::new()),
                    piece_color,
                )),
                Piece::from(&DomainPiece::new(PieceType::Knight(Knight), piece_color)),
            ],
            selected: 0,
        }
    }
}

impl StatefulWidget for PromotionMenu {
    type State = usize;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Define a vertical layout to center the promotion menu in the middle of the screen
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Min(8),
                    Constraint::Percentage(45),
                ]
                .as_ref(),
            )
            .split(area);

        let middle_area = layout[1];

        // Calculate the width of the menu area based on the width of the pieces
        let piece_width = 13;
        let total_menu_width = piece_width * self.pieces.len() as u16;

        // Center the menu within middle_area
        let menu_left_padding = (middle_area.width - total_menu_width) / 2;
        let menu_area = Rect {
            x: middle_area.x + menu_left_padding,
            y: middle_area.y,
            width: total_menu_width,
            height: middle_area.height,
        };

        // Render the centered block with black background and borders around the whole widget
        let block = Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("Promote")
            .title_alignment(Alignment::Center);
        Widget::render(block, menu_area, buf);

        // Define constraints to vertically center the pieces within the menu_area
        // Adjust the length of the middle constraint to match the piece height more closely
        let piece_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Length(8),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(menu_area);

        let piece_area = piece_layout[1];

        // Split piece_area horizontally to fit the pieces
        let piece_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                &(0..self.pieces.len())
                    .map(|_| Constraint::Length(piece_width.into()))
                    .collect::<Vec<_>>()[..],
            )
            .split(piece_area);

        // Render each piece, highlighting the selected one
        for (i, piece) in self.pieces.iter().enumerate() {
            let mut area = piece_areas[i];
            area.x += 1;
            area.y += 1;
            area.width = area.width.saturating_sub(1);
            area.width = area.height.saturating_sub(1);

            // Apply highlighting if the piece is selected
            if i == *state {
                let selected_style = Style::default().bg(Color::Yellow);
                let selected_block = Block::default().style(selected_style);

                Widget::render(selected_block, area, buf);
            }

            // Render the piece in its area
            piece.clone().render(area, buf);
        }
    }
}
