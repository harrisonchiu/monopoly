#define FMT_HEADER_ONLY

#include "src/controller/controller.hpp"
#include "src/model/board.hpp"
#include "src/model/player.hpp"
#include "src/view/view.hpp"

#include <nlohmann/json.hpp>

#include <fstream>

auto main() -> int {
  using json = nlohmann::json;
  std::ifstream file("data/classic_board.json"); // runtime lookup
  json tile_data = json::parse(file);

  std::shared_ptr<std::vector<Player>> players =
      std::make_shared<std::vector<Player>>(Player::create_multiple(4));
  auto board = std::make_shared<Board>(tile_data);
  board->set_players(players, 0);

  auto view = std::make_unique<View>(board);

  Controller game(std::move(view), board, players);
  game.visualize_game();

  while (true) {
    auto command = Controller::parse_command(game.prompt());
    game.run_command(command);
  }

  View::move_to_bot();
}
