#ifndef BOARD_HPP
#define BOARD_HPP

#include <array>
#include <string>
#include <vector>

#include <fmt/args.h>
#include <fmt/compile.h>
#include <fmt/format.h>
#include <nlohmann/json.hpp>

#include <tiles/tile.hpp>
#include <utils/color.hpp>

struct Size {
  const int width{};
  const int height{};
};

class Board {
private:
  static constexpr int length_of_tile = 7;
  static constexpr int number_of_tiles = 40;

  // typedef std::variant<Property, Event> board_tile;
  // std::vector<std::variant<std::unique_ptr<Property>,
  // std::unique_ptr<Event>>> board;
  std::vector<std::unique_ptr<Tile>> board;

  // Actual ids label each tile starting from GO (0) to the last tile (39)
  // Visual ids label each tile iterated as a multi-lined string
  // starting from top to bottom, left to right

  // Given the actual tile ids (index), get the visual tile ids (value)
  static constexpr std::array<int, number_of_tiles> actual_to_visual_order{
      39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 27, 25, 23,
      21, 19, 17, 15, 13, 11, 0,  1,  2,  3,  4,  5,  6,  7,
      8,  9,  10, 12, 14, 16, 18, 20, 22, 24, 26, 28};

  // Given the visual tile ids (index), get the actual tile ids (value)
  static constexpr std::array<int, number_of_tiles> visual_to_actual_order{
      30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 31, 18,
      32, 17, 33, 16, 34, 15, 35, 14, 36, 13, 37, 12, 38, 11,
      39, 10, 9,  8,  7,  6,  5,  4,  3,  2,  1,  0};

  // kColorPlaceholder MUST be 21 chars long because its replacement "▔▔▔▔▔▔▔"
  // is a unicode str that is 21 in length. So when we take note of every index
  // where this color placeholder occurs and then replace it, the size of the
  // string and thus the index location does not change. This is also why the
  // color placeholder already has a background colored formatted string: it
  // adds chars and changes the length. We want the placeholder to not only
  // be unique and descriptive, it should also be similar visually and bitwise.
  // A background colored ▔▔▔▔▔▔▔ using fmtlib seems to always be 44 chars.
  const std::string color_placeholder =
      fmt::format(Color::none(), "COLOR_PLACEHOLDER_21L");

  // Similar to kColorPlaceholder, we want the placeholder to be similar
  // visually and bitwise to maintain consistency when we replace this
  // placeholder. They take the form of what will appear.
  // Info will always be a string of $ with 4 chars reserved
  // for the digits (price) seperated by a space with 1 styled ASCII char
  const std::string detail_placeholder =
      fmt::format("$NNNN|{}", fmt::styled("X", Color::none()));
  const std::string player_placeholder =
      fmt::format("{} {} {} {}", Color::empty(1), Color::empty(1),
                  Color::empty(1), Color::empty(1));

  // Indices of the placeholders found in $ascii_board
  std::vector<int> color_indices;
  std::vector<int> detail_indices;
  std::vector<int> player_indices;

  std::string ascii_board = R"(
{INDENT} {{31:^7}} {{33:^7}} {{35:^7}} {{37:^7}} {{39:^7}} {{41:^7}} {{43:^7}} {{45:^7}} {{47:^7}} {{49:^7}} {{51:^7}}
{INDENT} {{32:^7}} {{34:^7}} {{36:^7}} {{38:^7}} {{40:^7}} {{42:^7}} {{44:^7}} {{46:^7}} {{48:^7}} {{50:^7}} {{52:^7}}
{INDENT}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|
{INDENT}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|
{INDENT}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|
{INDENT}|{COLOR}|▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔|{COLOR}|
{INDENT}|{INFOS}| {{30:<15}} | {CORE_PAD}                                  | {{53:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{29:<15}} | {CORE_PAD}                                  | {{54:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{28:<15}} | {CORE_PAD}                                  | {{55:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{27:<15}} | {CORE_PAD}                                  | {{56:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{26:<15}} | {CORE_PAD}                                  | {{57:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{25:<15}} | {CORE_PAD}                                  | {{58:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{24:<15}} | {CORE_PAD}                                  | {{59:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{23:<15}} | {CORE_PAD}                                  | {{60:>15}} |{INFOS}|
{INDENT}|{USERS}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{USERS}|
{INDENT}|{COLOR}| {SIDE_PAD} | {CORE_PAD}                                  | {SIDE_PAD} |{COLOR}|
{INDENT}|{INFOS}| {{22:<15}} | {CORE_PAD}                                  | {{61:>15}} |{INFOS}|
{INDENT}|{USERS}|                                                                       |{USERS}|
{INDENT}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|{COLOR}|
{INDENT}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|
{INDENT}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|
{INDENT} ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔ 
{INDENT} {{20:^7}} {{18:^7}} {{16:^7}} {{14:^7}} {{12:^7}} {{10:^7}} {{8:^7}} {{6:^7}} {{4:^7}} {{2:^7}} {{0:^7}}
{INDENT} {{21:^7}} {{19:^7}} {{17:^7}} {{15:^7}} {{13:^7}} {{11:^7}} {{9:^7}} {{7:^7}} {{5:^7}} {{3:^7}} {{1:^7}}
)";

  std::string create_base_board(const json &board_data);
  std::vector<int> find_substrings(const std::string &str,
                                   const std::string &substr) const;

public:
  Board(json &board_data);
  static constexpr int get_length_of_tile() { return length_of_tile; }
  static constexpr int get_number_of_tiles() { return number_of_tiles; }
  const Size get_size() const;
  const std::string get_board() const { return ascii_board; }

  void update_tile_color(const std::string &group, int tile_id);
  void update_tile_detail(const std::string &detail, int tile_id);
  void display_board() const;
};

#endif // BOARD_HPP