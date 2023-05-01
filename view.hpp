#ifndef VIEW_HPP
#define VIEW_HPP

#include <queue>

#include <fmt/color.h>

#include "board.hpp"
#include "utils/component.hpp"

class View {
private:
  static constexpr Size board_size = Board::get_size();
  static constexpr Position container_pos = {0, 0};
  static constexpr Size container_size = {board_size.width + 100,
                                          board_size.height + 2};
  std::shared_ptr<Board> board;

  using update_queue = std::shared_ptr<std::queue<int>>;
  update_queue board_color_queue;
  update_queue board_detail_queue;
  update_queue board_player_queue;

public:
  View(std::shared_ptr<Board> board_ptr);

  constexpr void move_to_top() const;
  constexpr void move_to_bot() const;

  void clear_screen() const;
  void draw_board() const;
  void draw_board_colors();
  void draw_board_details();
  void draw_board_players();
};

#endif // VIEW_HPP