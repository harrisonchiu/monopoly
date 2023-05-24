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