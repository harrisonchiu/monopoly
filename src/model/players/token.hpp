#ifndef TOKEN_HPP
#define TOKEN_HPP

#include "src/utils/color.hpp"

#include <fmt/color.h>

#include <array>
#include <string>
#include <string_view>

// Playing piece that visually represents each individual player
struct Piece {
  std::string_view character;
  fmt::text_style color;
};

// Contains the player's ID and unique @Piece and creates a formatted string of @Piece to easily
//  draw @Piece where needed.
// We need a string instead of overriding fmt::formatter<> for our type because it is easier
//  and faster to already have it as an attribute instead of formatting a string every time.
// We wanted to define pieces inline here, so it must be constexpr, so @Piece could not have a
//  formatted string. Therefore, must be different structs. Its too hard, but it could work making
//  a constexpr std::array<char> from fmt::detail::ansi_color_escape and using the std::array to
//  make a constexpr std::string_view.
// This struct also contains everything needed for @Tiles to identify its owner. Acts like an
// "owner's signature".
struct Token {
private:
  static constexpr int default_id = -1;
  static constexpr Piece default_piece = Piece{ "X", Color::none() };
  static constexpr std::array<Piece, 4> pieces{
    Piece{ "A", Color::get("Blue") },
    Piece{ "B", Color::get("Red") },
    Piece{ "C", Color::get("Green") },
    Piece{ "D", Color::get("Orange") },
  };

  int id;
  Piece piece;
  std::string avatar;

public:
  explicit Token()
      : id{ default_id },
        piece{ default_piece },
        avatar{ fmt::format(piece.color, piece.character) } {}

  explicit Token(int id)
      : id{ id },
        piece{ pieces.at(id) },
        avatar{ fmt::format(piece.color, piece.character) } {}

  auto get_id() const -> int { return id; }
  auto get_piece() const -> const Piece & { return piece; }
  auto get_color() const -> const fmt::text_style & { return piece.color; }
  auto get_character() const -> std::string_view { return piece.character; }
  auto get_avatar() const -> std::string_view { return avatar; }
};

#endif // TOKEN_HPP