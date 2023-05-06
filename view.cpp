#define FMT_HEADER_ONLY

#include "view.hpp"

#include "board.hpp"

#include <fmt/core.h>

// Disable linting warning, asking us to use reference to shared_ptr<>
// Ignore it because we want to share ownership with View. Also very small performance difference
View::View(std::shared_ptr<Board> board_ptr) // NOLINT(performance-unnecessary-value-param)
    : board{ std::move(board_ptr) }, board_color_queue{ View::board->get_color_queue() },
      board_detail_queue{ View::board->get_detail_queue() },
      board_player_queue{ View::board->get_player_queue() } {}

void View::move_to_top() { fmt::print("\x1b[{};{}H", container_position.row, 0); }

void View::move_to_bot() {
  constexpr int height = container_position.row + container_size.height;
  fmt::print("\x1b[{};{}H", height, 0);
}

void View::clear_screen() { fmt::print("\x1b[2J"); }

void View::draw_board() const {
  View::move_to_top();
  fmt::print("{}", board->get_board_str());
}

void View::draw_board_colors() {
  while (!(board_color_queue->empty())) {
    const int tile_id = board_color_queue->front();
    const Position *position = board->get_color_position(tile_id);
    fmt::print("\x1b[{};{}H{}", position->row, position->col, board->get_tile_color(tile_id));
    board_color_queue->pop();
  }
}

void View::draw_board_details() {
  while (!(board_detail_queue->empty())) {
    const int tile_id = board_detail_queue->front();
    const Position *position = board->get_detail_position(tile_id);
    fmt::print("\x1b[{};{}H{}", position->row, position->col, board->get_tile_detail(tile_id));
    board_detail_queue->pop();
  }
}

void View::draw_board_players() {
  while (!(board_player_queue->empty())) {
    const int tile_id = board_player_queue->front();
    const Position *position = board->get_player_position(tile_id);
    fmt::print("\x1b[{};{}H{}", position->row, position->col, board->get_tile_player(tile_id));
    board_player_queue->pop();
  }
}