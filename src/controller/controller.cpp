#include "src/controller/controller.hpp"

#include "src/controller/exit_codes.hpp"
#include "src/model/board.hpp"
#include "src/model/players/player.hpp"
#include "src/view/view.hpp"

#include <re2/re2.h>
#include <re2/stringpiece.h>

#include <iostream>
#include <string>
#include <vector>

Controller::Controller(
    std::unique_ptr<View> view_ptr, std::shared_ptr<Board> board_ptr,
    std::shared_ptr<std::vector<Player>> players_ptr
)
    : players{ std::move(players_ptr) },
      board{ std::move(board_ptr) },
      view{ std::move(view_ptr) } {

  current_player = players->begin();

  game_commands["roll"] = &Controller::move_player;
  game_commands["r"] = &Controller::move_player;
  game_commands["end"] = &Controller::end_turn;
  game_commands["e"] = &Controller::end_turn;
  game_commands["buy"] = &Controller::buy_tile;
  game_commands["b"] = &Controller::buy_tile;
  game_commands["view"] = &Controller::view_tile;
  game_commands["v"] = &Controller::view_tile;

  debug_commands["exit"] = &Controller::exit;
  debug_commands["x"] = &Controller::exit;
}

void Controller::visualize_game() {
  view->clear_screen();
  view->draw_board();
  view->draw_board_colors();
  view->draw_board_details();
  view->draw_board_players();
}

auto Controller::prompt() -> std::string {
  view->draw_prompt(current_player->get_avatar());

  std::string command; // std::getline() only allows std::string
  std::getline(std::cin, command);

  return command;
}

// Given some command made up of words or enclosed by brackets, separated by whitespace,
// returns a list of args so functions can take it and do the actions easily.
// Any numbers enclosed inside a brackets `{}` counts as 1 arg
// Otherwise, each token separated by whitespace is considered an individual arg
// Example:
//    1 arg: {23 a 19 $2}
//    3 args: {1 2 3 4 5 6} arg_num_2 arg_num_3
auto Controller::parse_command(std::string_view command) -> args_list {
  if (command.empty()) {
    return args_list{};
  }

  static const RE2 re(R"((\{([0-9$]+\s*)+\})|(\w+))");
  re2::StringPiece input(command);
  re2::StringPiece bracketed_arg;
  re2::StringPiece normal_arg;
  args_list args{};

  // Seperate the command str into a list of arguments. Empty list on no matches (e.g. only space)
  // @StringPiece hold the result of a capture group. 2nd capture group is everything inside a
  // bracket `{}` arg. We do not care, so put nullptr there.
  while (RE2::FindAndConsume(&input, re, &bracketed_arg, (void *)nullptr, &normal_arg)) {
    if (bracketed_arg != nullptr) {
      args.push_back(bracketed_arg.as_string());
    } else if (normal_arg != nullptr) {
      args.push_back(normal_arg.as_string());
    }
  }

  return args;
}

// Every different has a unique keyword (the first arg). It specifies the main action to be done.
// All the args that follow this keyword modifies, adds detail, or specifies the behaviour
// Example:
//    buy <tile_id>
//    `buy` is the keyword and `<tile_id>` is the arg that specifies which tile to buy
// Unique keywords are good for design both in player use and code.
//  Player: more clear for what command to use and what it does
//  Code: we use a map to link a keyword with its associated action (function). Much more
//  clearer and extensible than many if/switch statements. Less duplicate code.
//  Disadvantage: slower.
auto Controller::run_command(args_list &args) -> StatusCode {
  if (args.empty()) {
    return StatusCode::Success;
  }

  if (auto it = game_commands.find(args[0]); it != game_commands.end()) {
    try {
      const std::string log = it->second(this, args);
      view->output(log);
      return StatusCode::Success;
    } catch (const std::exception &e) {
      view->output(e.what());
      return StatusCode::Failure;
    }
  } else if (auto it = debug_commands.find(args[0]); it != debug_commands.end()) {
    try {
      const StatusCode exit_code = it->second(this, args);
      view->output(static_cast<int>(exit_code));
      return exit_code;
    } catch (const std::exception &e) {
      view->output(e.what());
      return StatusCode::Failure;
    }
  } else {
    view->clear_output();
    return StatusCode::Success;
  }
}
