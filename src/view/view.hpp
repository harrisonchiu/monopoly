#ifndef VIEW_HPP
#define VIEW_HPP

#include "src/model/board.hpp"
#include "src/view/components.hpp"

#include <queue>

class View {
private:
  static constexpr Size board_size = Board::get_size();
  static constexpr Position board_pos = { 0, 0 }; // {x: col #, y: row #}

  static constexpr Size console_size = { board_size.width, 4 };
  static constexpr Position console_pos = { board_pos.col, board_pos.row + board_size.height + 2 };

  static constexpr Size container_size = { board_size.width + 100,
                                           board_size.height + console_size.height };
  static constexpr Position container_pos = { 0, 0 }; // {x: col #, y: row #}

  std::shared_ptr<Board> board;

public:
  explicit View(std::shared_ptr<Board> board);

  static void move_to_top();
  static void move_to_bot();
  static void clear_screen();

  void draw_board() const;
  void draw_board_colors();
  void request_tile_color_update(int tile_id);
  void draw_board_details();
  void request_tile_detail_update(int tile_id);
  void draw_board_players();

  static void draw_prompt(std::string_view player);
  static void output(std::string_view log);
  static void output(int exit_code);
  static void clear_output();
};

#endif // VIEW_HPP