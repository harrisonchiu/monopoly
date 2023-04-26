#include <fmt/core.h>
#include <fstream>
#include <iostream>

#include <concepts>

#include <nlohmann/json.hpp>

#include <board.hpp>
#include <player.hpp>

#include <fmt/args.h>
#include <fmt/core.h>

constexpr void test() {
  // fmt::format doesn't take a type-erased list of arguments
  // (fmt::basic_format_args
  //  or fmt::format_arg_store, return value of fmt::make_format_args), but a
  //  variadic pack of arguments.
  // We want to do constexpr for compile time strings for Board::ascii_board
  constexpr auto args = fmt::make_format_args(1, 2);
  fmt::vprint("{0} {1}", {args});
}

int main() {
  std::ifstream file("tiles/board_data.json"); // runtime lookup
  json tile_data = json::parse(file);

  Board board(tile_data);
  board.display_board();
  board.get_size();

  Player player(1);

  test();
}