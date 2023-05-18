#include "src/controller/controller.hpp"

#include "src/model/board.hpp"
#include "src/model/player.hpp"
#include "src/view/view.hpp"

auto Controller::move_player([[maybe_unused]] args_list &args) -> std::string {
  constexpr int steps = 1;
  current_player->walk(steps);
  view->draw_board_players();

  const int player_id = current_player->get_id();
  const std::string_view player = current_player->get_avatar();
  const std::string_view landed_tile = board->get_current_tile(player_id)->get_name();
  std::string log =
      fmt::format("Player {} rolled {} and landed on {}.", player, steps, landed_tile);
  return log;
}

auto Controller::end_turn([[maybe_unused]] args_list &args) -> std::string {
  const std::string_view turn_ender = current_player->get_avatar();

  ++turn_number;
  ++current_player;
  if (current_player == players->end()) {
    current_player = players->begin();
  }

  const std::string_view next_player = current_player->get_avatar();
  std::string log =
      fmt::format("Player {} ended their turn. Now Player {} turn.", turn_ender, next_player);
  return log;
}

auto Controller::buy_tile([[maybe_unused]] args_list &args) -> std::string {
  const int player_id = current_player->get_id();
  const std::shared_ptr<Tile> &tile = board->get_current_tile(player_id);
  const int tile_cost = tile->get_cost();

  if (current_player->get_money() >= tile_cost) {
    current_player->withdraw(tile_cost);
    tile->set_owner(*current_player);
  }

  board->update_detail_tile(tile->get_id());
  view->draw_board_details();

  std::string log = fmt::format(
      "Player {} purchased {} for ${}.", current_player->get_avatar(), tile->get_name(), tile_cost
  );
  return log;
}