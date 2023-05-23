#define FMT_HEADER_ONLY

#include "src/view/view.hpp"

#include "src/model/board.hpp"
#include "src/view/components.hpp"

#include <fmt/ranges.h>

#include <queue>

View::View(std::shared_ptr<Board> board_ptr)
    : board{ std::move(board_ptr) } {}

void View::move_to_top() { fmt::print("\x1b[{};{}H", container_pos.row, container_pos.col); }

void View::move_to_bot() {
  constexpr int height = container_pos.row + container_size.height;
  fmt::print("\x1b[{};{}H", height, container_pos.col);
}

void View::clear_screen() { fmt::print("\x1b[2J"); }

void View::draw_board() const {
  View::move_to_top();
  fmt::print("{}", board->get_board_str());
}

// Tile colors are not embedded directly in the board's string because it is easier to generate it
// in compile time without colors. Also, it makes it consistent with how the other rows are drawn.
// Therefore, @View has to draw over it whenever @Board requests a visual update to it.
// Does not need to be called after generating the board. Tile groups do not change (for now)!
void View::draw_board_colors() {
  std::queue<int> &color_update_queue = board->get_color_update_queue();

  while (!(color_update_queue.empty())) {
    const int tile_id = color_update_queue.front();
    const Position &pos = board->get_color_pos(tile_id);
    const std::shared_ptr<Tile> &tile = board->get_tile(tile_id);

    fmt::print("\x1b[{};{}H{}", pos.row, pos.col, tile->get_color());
    color_update_queue.pop();
  }
}

void View::request_tile_color_update(const int tile_id) {
  board->get_color_update_queue().push(tile_id);
}

// Tile details are not fixed (show only property cost), to show more relevant game info as a
// quality of life improvement. Therefore, it often changes based on game state. Whenever a tile
// changes, @Board updates the properties and requests a visual update from @View.
// Should be called every turn.
void View::draw_board_details() {
  std::queue<int> &detail_update_queue = board->get_detail_update_queue();

  while (!(detail_update_queue.empty())) {
    const int tile_id = detail_update_queue.front();
    const Position &pos = board->get_detail_pos(tile_id);
    const std::shared_ptr<Tile> &tile = board->get_tile(tile_id);
    tile->update_detail();

    fmt::print("\x1b[{};{}H{}", pos.row, pos.col, tile->get_detail());
    detail_update_queue.pop();
  }
}

void View::request_tile_detail_update(const int tile_id) {
  board->get_detail_update_queue().push(tile_id);
}

// Every time a player moves to a different tile, this method should be called to
// visually show the movement. This draws every player's pieces onto their new tiles
// and removes them from their previous tiles.
// If performance needs to be improved here, remove `fmt::join()`:
//  Store both @tile_id and @player_id inside @board_player_update_queue, so it prints
//  only that specific piece on their spot instead of reprinting every player on that tile
void View::draw_board_players() {
  std::queue<int> &player_update_queue = board->get_player_update_queue();

  while (!(player_update_queue.empty())) {
    const int tile_id = player_update_queue.front();
    const Position &pos = board->get_player_pos(tile_id);
    fmt::print(
        "\x1b[{};{}H{}", pos.row, pos.col, fmt::join(board->get_tile_players(tile_id), " ")
    );
    player_update_queue.pop();
  }
}

void View::draw_prompt(std::string_view player) {
  fmt::print("\x1b[{};{}H[{}]{}\x1b[0K", console_pos.row, console_pos.col, player, ">>> ");
}

void View::output(std::string_view log) {
  fmt::print("\x1b[{};{}H\x1b[2K{}", console_pos.row + 1, console_pos.col, log);
}

void View::output(int exit_code) {
  fmt::print("\x1b[{};{}H\x1b[2KExit Code: {}", console_pos.row + 1, console_pos.col, exit_code);
}

void View::clear_output() {
  fmt::print("\x1b[{};{}H\x1b[2K", console_pos.row + 1, console_pos.col);
}