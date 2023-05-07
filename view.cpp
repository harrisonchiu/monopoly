#define FMT_HEADER_ONLY

#include "view.hpp"

#include "board.hpp"

#include <fmt/ranges.h>

// Disable linting warning, asking us to use reference to shared_ptr<>
// Ignore it because we want to share ownership with View. Also very small performance difference
View::View(std::shared_ptr<Board> board_ptr) // NOLINT(performance-unnecessary-value-param)
    : board{ std::move(board_ptr) },
      board_color_update_queue{ View::board->get_color_update_queue() },
      board_detail_update_queue{ View::board->get_detail_update_queue() },
      board_player_update_queue{ View::board->get_player_update_queue() } {
  View::clear_screen();
}

void View::move_to_top() { fmt::print("\x1b[{};{}H", container_pos.row, 0); }

void View::move_to_bot() {
  constexpr int height = container_pos.row + container_size.height;
  fmt::print("\x1b[{};{}H", height, 0);
}

void View::clear_screen() { fmt::print("\x1b[2J"); }

void View::draw_board() const {
  View::move_to_top();
  fmt::print("{}", board->get_board_str());
}

void View::draw_board_colors() {
  while (!(board_color_update_queue->empty())) {
    const int tile_id = board_color_update_queue->front();
    const Position &pos = board->get_color_pos(tile_id);
    fmt::print("\x1b[{};{}H{}", pos.row, pos.col, board->get_tile_color(tile_id));
    board_color_update_queue->pop();
  }
}

void View::draw_board_details() {
  while (!(board_detail_update_queue->empty())) {
    const int tile_id = board_detail_update_queue->front();
    const Position &pos = board->get_detail_pos(tile_id);
    fmt::print("\x1b[{};{}H{}", pos.row, pos.col, board->get_tile_detail(tile_id));
    board_detail_update_queue->pop();
  }
}

// Every time a player moves to a different tile, this method should be called to
// visually show the movement. This draws every player's tokens onto their new tiles
// and removes them from their previous tiles.
// If performance needs to be improved here, remove `fmt::join()`:
//    Store both @tile_id and @player_id inside @board_player_update_queue, so it prints
//    only that specific token on their spot instead of reprinting every player on that tile
void View::draw_board_players() {
  board->update_all_player_pos(); // move the tokens on the board before drawing

  while (!(board_player_update_queue->empty())) {
    const int tile_id = board_player_update_queue->front();
    const Position &pos = board->get_player_pos(tile_id);
    fmt::print(
        "\x1b[{};{}H{}", pos.row, pos.col, fmt::join(board->get_tile_players(tile_id), " ")
    );
    board_player_update_queue->pop();
  }
}