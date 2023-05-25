#include "src/controller/controller.hpp"

#include "src/model/board.hpp"
#include "src/model/players/player.hpp"
#include "src/model/players/token.hpp"
#include "src/model/tiles/tile.hpp"
#include "src/view/view.hpp"

#include <fmt/core.h>

#include <string>
#include <string_view>

// Disable linting because making the function static would change function type in the map
// Some functions may not be able to be static, so generalize it into non-static
// NOLINTNEXTLINE(readability-convert-member-functions-to-static)
auto Controller::exit([[maybe_unused]] args_list &args) -> StatusCode { return StatusCode::Exit; }

auto Controller::move_player([[maybe_unused]] args_list &args) -> std::string {
  constexpr int steps = 1;
  current_player->walk(steps);
  board->move_player_piece(*current_player);
  view->draw_board_players();

  const std::string_view piece = current_player->get_avatar();
  const std::string_view landed_tile = board->get_tile(current_player->get_pos())->get_name();
  std::string log =
      fmt::format("Player {} rolled {} and landed on {}.", piece, steps, landed_tile);
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
  const std::shared_ptr<Tile> &tile = board->get_tile(current_player->get_pos());

  const std::string_view tile_owner = tile->get_owner()->get_avatar();
  const std::string_view tile_name = tile->get_name();
  if (!tile->get_is_ownable()) {
    return fmt::format("{} is not for sale. Cannot purchase.", tile_name);
  }
  if (tile->get_owner_id() == current_player->get_id()) {
    return fmt::format("{} is already owned by you. Cannot purchase.", tile_name);
  }
  if (tile->get_owner_id() != Token::get_default_id()) {
    return fmt::format("{} is owned by Player {}. Cannot purchase.", tile_name, tile_owner);
  }

  const int tile_cost = tile->get_cost();
  if (current_player->get_money() >= tile_cost) {
    current_player->withdraw(tile_cost);
    tile->set_owner(*current_player);
    tile->update_detail();
  }

  view->draw_tile_detail(tile->get_id());

  std::string log = fmt::format(
      "Player {} purchased {} for ${}. Balance remaining: ${}", current_player->get_avatar(),
      tile_name, tile_cost, current_player->get_money()
  );
  return log;
}

auto Controller::view_toggle([[maybe_unused]] args_list &args) -> std::string {
  static bool view_ids{ false }; // Initialize as false because details are initially shown

  view_ids = !view_ids;

  if (view_ids) {
    for (int tile_id = 0; tile_id < Board::get_number_of_tiles(); ++tile_id) {
      view->draw_tile_id(tile_id);
    }
    return fmt::format("Showing tile IDs. IDs are used to specify a tile in commands.");
  }

  for (int tile_id = 0; tile_id < Board::get_number_of_tiles(); ++tile_id) {
    view->draw_tile_detail(tile_id);
  }
  return fmt::format("Showing tile details: owner, property status, cost/rent");
}

auto Controller::view_tile(args_list &args) -> std::string {
  if (args.size() == 1) {
    const auto &tile = board->get_tile(1);
    const Position pos = board->get_center_pos();

    fmt::print(fmt::runtime(tile->get_card()), pos.row, pos.col);

    return view_toggle(args);
  }

  return fmt::format("Showing tile details: owner, property status, cost/rent");
}