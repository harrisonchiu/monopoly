#define FMT_HEADER_ONLY

#include "src/controller/controller.hpp"
#include "src/controller/exit_codes.hpp"
#include "src/model/board.hpp"
#include "src/model/players/player.hpp"
#include "src/view/view.hpp"

// #include "data/json_validator.hpp"

#include <nlohmann/json.hpp>

#include <fstream>

auto main() -> int {
  using json = nlohmann::json;

  const std::ifstream file("data/classic_board.json"); // runtime lookup

  const json board_data = json::parse(file);

  // std::vector<std::string> errors = validation::validate_board_json(board_data);

  auto players = std::make_shared<std::vector<Player>>(Player::create_multiple(4));
  auto board = std::make_shared<Board>(board_data);
  auto view = std::make_unique<View>(board);

  // Add players to the game
  for (const Player &player : *players) {
    board->place_player_pieces(player, 0);
  }

  Controller game(std::move(view), board, players);
  game.visualize_game();

  while (true) {
    auto command = Controller::parse_command(game.prompt());
    const ExitCode exit_code = game.run_command(command);

    if (exit_code == ExitCode::Exit) {
      break;
    }
  }

  View::move_to_bot();
  return 0;
}
