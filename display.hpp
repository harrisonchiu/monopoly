#ifndef DISPLAY_HPP
#define DISPLAY_HPP

struct Position {
  const int col{};
  const int row{};
};

class Display {
private:
  Position board = {0, 0};

public:
  void draw_board() const;
};

#endif // DISPLAY_HPP