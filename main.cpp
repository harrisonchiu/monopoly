#define FMT_HEADER_ONLY

#include "board.hpp"
#include "view.hpp"

#include <nlohmann/json.hpp>

#include <fstream>
#include <iostream>

auto main() -> int {
  using json = nlohmann::json;
  std::ifstream file("tiles/board_data.json"); // runtime lookup
  json tile_data = json::parse(file);

  std::shared_ptr<Board> board = std::make_shared<Board>(tile_data);

  View view(board);
  View::clear_screen();
  view.draw_board();
  view.draw_board_colors();
  view.draw_board_details();
  view.draw_board_players();
  View::move_to_bot();
}
