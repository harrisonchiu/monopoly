#ifndef CONTROLLER_HPP
#define CONTROLLER_HPP

#include "src/model/board.hpp"
#include "src/model/player.hpp"
#include "src/view/view.hpp"

#include <functional>
#include <iterator>
#include <ranges>
#include <string>
#include <unordered_map>

class Controller {
  // @args_list must be string to actually own the data. It cannot be string_view because re2
  //  constantly returns matches to the same reference. Therefore, if using string_view, every args
  //  would just end up referencing the last regex match.
  // @game_actions must return string. Each action function locally generates a string based on the
  //  game state. It returns that to be shown in View. Therefore, it cannot be non-owning because
  //  the reference will be invalid.
  using args_list = std::vector<std::string>;
  using game_actions = std::function<std::string(Controller *, args_list &args)>;

private:
  std::unordered_map<std::string_view, game_actions> commands;

  std::shared_ptr<std::vector<Player>> players;
  std::shared_ptr<Board> board;
  std::unique_ptr<View> view;

  int turn_number{ 0 };
  std::vector<Player>::iterator current_player;

public:
  Controller(
      std::unique_ptr<View> view, std::shared_ptr<Board> board,
      std::shared_ptr<std::vector<Player>> players
  );

  void visualize_game();

  auto prompt() -> std::string;
  static auto parse_command(std::string_view command) -> args_list;
  void run_command(args_list &args);

  auto move_player(args_list &args) -> std::string;
  auto end_turn(args_list &args) -> std::string;
  auto buy_tile(args_list &args) -> std::string;
};

#endif // CONTROLLER_HPP