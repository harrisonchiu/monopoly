#ifndef COMPONENT_HPP
#define COMPONENT_HPP

// Used to denote size of an object that gets visualized by View
// @width and @height are measured by number of terminal chars
struct Size {
  int width{};
  int height{};
};

// Used to denote top-left position of an object that gets visualized by View
// @col and @row are the (x, y) position in the terminal measured by chars
// where origin (0, 0) is the top-left position in the terminal.
struct Position {
  int col{};
  int row{};
};

#endif // COMPONENT_HPP