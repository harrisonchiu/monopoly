#include "controller.hpp"

#include "board.hpp"
#include "player.hpp"
#include "view.hpp"

#include <re2/re2.h>
#include <re2/stringpiece.h>

#include <iostream>
#include <string>
#include <vector>

Controller::Controller(
    std::unique_ptr<View> view_ptr, std::shared_ptr<Board> board_ptr,
    std::shared_ptr<std::vector<Player>> players_ptr
)
    : players{ std::move(players_ptr) }, board{ std::move(board_ptr) },
      view{ std::move(view_ptr) } {

  current_player = players->begin();
  commands["roll"] = &Controller::move_player;
  commands["r"] = &Controller::move_player;
  commands["end"] = &Controller::end_turn;
  commands["e"] = &Controller::end_turn;
  // commands["buy"] = &Controller : buy_tile;
}

void Controller::visualize_game() {
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

auto Controller::parse_command(std::string_view command) -> args_list {
  if (command.empty()) {
    return args_list{};
  }

  // Any numbers enclosed inside a brackets `{}` counts as 1 arg
  // Otherwise, each token separated by whitespace is considered an individual arg
  // Example:
  //    1 arg: {23 32 19 $2}
  //    3 args: {1 2 3 4 5 6} arg_num_2 arg_num_3
  const static RE2 re(R"((\{([0-9$]+\s*)+\})|(\w+))");
  re2::StringPiece input(command);
  re2::StringPiece bracketed_tokens;
  re2::StringPiece normal_tokens;
  args_list args{};

  // Seperate the command str into a list of arguments to easily do actions.
  // Each arg after the regex pattern is a StringPiece that holds the result of a capture
  // group. The 2nd capture group is each digit between `{}`. We do not care so put nullptr there.
  // If the input does not match patter (e.g. only contains whitespace), returns an empty list
  while (RE2::FindAndConsume(&input, re, &bracketed_tokens, (void *)nullptr, &normal_tokens)) {
    if (bracketed_tokens != nullptr) {
      args.push_back(bracketed_tokens.as_string());
    } else if (normal_tokens != nullptr) {
      args.push_back(normal_tokens.as_string());
    }
  }

  return args;
}

void Controller::run_command(args_list &args) {
  if (args.empty()) {
    return;
  }

  if (auto it = commands.find(args[0]); it != commands.end()) {
    try {
      const std::string log = it->second(this, args);
      view->output(log);
    } catch (const std::exception &e) {
      view->output(e.what());
    }
  } else {
    view->clear_output();
  }
}
