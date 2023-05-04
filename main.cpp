#define FMT_HEADER_ONLY

#include <fstream>

#include <nlohmann/json.hpp>

#include "board.hpp"
#include "player.hpp"
#include "view.hpp"

#include "utils/substrings.hpp"

using json = nlohmann::json;

int main() {
  std::ifstream file("tiles/board_data.json"); // runtime lookup
  json tile_data = json::parse(file);

  std::shared_ptr<Board> board = std::make_shared<Board>(tile_data);
  Player player1(1);

  View view(board);
  view.clear_screen();
  view.draw_board();
  view.draw_board_colors();
  view.draw_board_details();
  view.draw_board_players();
  view.move_to_bot();
}
