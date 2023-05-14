#include "src/controller/controller.hpp"

#include "src/model/board.hpp"
#include "src/model/player.hpp"
#include "src/model/tiles/attributes.hpp"
#include "src/view/view.hpp"

auto Controller::move_player([[maybe_unused]] args_list &args) -> std::string {
  constexpr int steps = 1;
  current_player->walk(steps);
  view->draw_board_players();

  const int player_id = current_player->get_id();
  const std::string_view landed_tile = board->get_current_tile(player_id)->get_name();

  const std::string_view player_avatar = current_player->get_avatar();
  std::string log =
      fmt::format("Player {} rolled {} and landed on {}.", player_avatar, steps, landed_tile);
  return log;
}

auto Controller::end_turn([[maybe_unused]] args_list &args) -> std::string {
  const std::string_view turn_ender_avatar = current_player->get_avatar();

  ++turn_number;
  ++current_player;
  if (current_player == players->end()) {
    current_player = players->begin();
  }

  const std::string_view next_player_avatar = current_player->get_avatar();
  std::string log = fmt::format(
      "Player {} ended their turn. Now Player {} turn.", turn_ender_avatar, next_player_avatar
  );

  return log;
}

auto Controller::buy_tile(args_list &args) -> std::string {
  const int player_id = current_player->get_id();
  const TileType tile_type = board->get_current_tile(player_id)->get_type();

  std::string log;
  if (tile_type == TileType::Property) {
  }

  return args[0];
}