#ifndef VIEW_HPP
#define VIEW_HPP

#include "board.hpp"
#include "utils/component.hpp"

#include <queue>

class View {
private:
  static constexpr Size board_size = Board::get_size();
  static constexpr Size container_size = { board_size.width + 100, board_size.height + 2 };
  static constexpr Position container_pos = { 0, 0 };
  std::shared_ptr<Board> board;

  using update_queue = std::shared_ptr<std::queue<int>>;
  update_queue board_color_update_queue;
  update_queue board_detail_update_queue;
  update_queue board_player_update_queue;

public:
  explicit View(std::shared_ptr<Board> board);

  static void move_to_top();
  static void move_to_bot();
  static void clear_screen();

  void draw_board() const;
  void draw_board_colors();
  void draw_board_details();
  void draw_board_players();
};

#endif // VIEW_HPP