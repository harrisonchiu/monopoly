#define FMT_HEADER_ONLY

#include <fmt/color.h>

#include "view.hpp"

View::View(std::shared_ptr<Board> board) : board{board} {
  board_color_queue = board->get_color_queue();
  board_detail_queue = board->get_detail_queue();
  board_player_queue = board->get_player_queue();
}

constexpr void View::move_to_top() const {
  fmt::print("\x1b[{};{}H", container_pos.row, 0);
}

constexpr void View::move_to_bot() const {
  constexpr int height = container_pos.row + container_size.height;
  fmt::print("\x1b[{};{}H", height, 0);
}

void View::clear_screen() const { fmt::print("\x1b[2J"); }

void View::draw_board() const {
  move_to_top();
  fmt::print("{}", board->get_board());
}

void View::draw_board_colors() {
  while (!(board_color_queue->empty())) {
    int tile_id = board_color_queue->front();
    fmt::print("\x1b[{};{}H{}", board->get_color_coord(tile_id)->row,
               board->get_color_coord(tile_id)->col,
               board->get_tile_color(tile_id));
    board_color_queue->pop();
  }
  move_to_bot();
}

void View::draw_board_details() {
  while (!(board_detail_queue->empty())) {
    int tile_id = board_detail_queue->front();
    fmt::print("\x1b[{};{}H{}", board->get_detail_coord(tile_id)->row,
               board->get_detail_coord(tile_id)->col,
               board->get_tile_detail(tile_id));
    board_detail_queue->pop();
  }
  move_to_bot();
}

void View::draw_board_players() {
  while (!(board_player_queue->empty())) {
    int tile_id = board_player_queue->front();
    fmt::print("\x1b[{};{}H{}", board->get_player_coord(tile_id)->row,
               board->get_player_coord(tile_id)->col,
               board->get_tile_player(tile_id));
    board_player_queue->pop();
  }
  move_to_bot();
}