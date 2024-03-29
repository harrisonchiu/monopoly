#ifndef CONTROLLER_HPP
#define CONTROLLER_HPP

#include "src/controller/exit_codes.hpp"
#include "src/model/board.hpp"
#include "src/model/players/player.hpp"
#include "src/view/view.hpp"

#include <functional>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

class Controller {
  // @args_list must be string to actually own the data. It cannot be string_view because re2
  //  constantly returns matches to the same reference. Therefore, if using string_view, every args
  //  would just end up referencing the last regex match.
  // @game_actions must return string. Each action function locally generates a string based on the
  //  game state. It returns that to be shown in View. Therefore, it cannot be non-owning because
  //  the reference will be invalid.
  using args_list = std::vector<std::string>;
  using game_actions = std::function<std::string(Controller *, args_list &args)>;
  using debug_actions = std::function<StatusCode(Controller *, args_list &args)>;

private:
  std::unordered_map<std::string_view, game_actions> game_commands;
  std::unordered_map<std::string_view, debug_actions> debug_commands;

  std::shared_ptr<std::vector<Player>> players;
  std::shared_ptr<Board> board;
  std::unique_ptr<View> view;

  int turn_number{ 0 };
  std::vector<Player>::iterator current_player;

  auto land(const std::shared_ptr<Tile> &tile) -> std::string;
  auto give_money(int amount) -> std::string;
  auto pay_rent(const std::shared_ptr<Tile> &tile) -> std::string;

public:
  Controller(
      std::unique_ptr<View> view, std::shared_ptr<Board> board,
      std::shared_ptr<std::vector<Player>> players
  );

  void visualize_game();

  auto prompt() -> std::string;
  static auto parse_command(std::string_view command) -> args_list;
  auto run_command(args_list &args) -> StatusCode;

  // Game actions return string which are logs that tell the user any needed game details
  auto move_player(args_list &args) -> std::string;
  auto end_turn(args_list &args) -> std::string;
  auto buy_tile(args_list &args) -> std::string;
  auto view_toggle(args_list &args) -> std::string;
  auto view_tile(args_list &args) -> std::string;

  auto interact(args_list &args) -> std::string; // do things when not on tile

  // Debug actions return enum values for details about its operation (e.g. success, failure)
  auto exit(args_list &args) -> StatusCode;
  auto redraw(args_list &args) -> StatusCode;
};

#endif // CONTROLLER_HPP