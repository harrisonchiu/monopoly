#ifndef TOKEN_HPP
#define TOKEN_HPP

#include "src/utils/color.hpp"

#include <fmt/color.h> // should include <fmt/core.h>

#include <string_view>

// Officially, Monopoly playing pieces are called tokens, but I found the word to be too general
// and that "pieces" is a better and more common word for playing pieces.
// This is why "token" is the name of this file.

// Must have default values so we can create a default Avatar
// or also known as a no-owner Avatar
// struct Piece {
//   // Ideally, it should be `char` type but no fmt function supports formatting chars
//   // Is a std::string_view for convenience and make this compile-time possible
//   // static constexpr fmt::text_style color{ Color::none() };

//   fmt::detail::styled_arg<char[2]> avatar{ fmt::styled("_", Color::none()) };

//   // public:
//   //   constexpr auto get_char() const -> std::string_view { return avatar.value; }
// };

using style = fmt::detail::styled_arg<std::basic_string_view<char, std::char_traits<char>>>;

struct Piece {
  std::string_view character;
  fmt::text_style color;
};

template <typename Char> struct fmt::formatter<Piece, Char> {
  template <typename ParseContext>
  constexpr auto parse(ParseContext &ctx) -> ParseContext::iterator {
    return ctx.begin();
  }

  template <typename FormatContext>
  auto format(const Piece &piece, FormatContext &ctx) const -> decltype(ctx.out()) {
    const fmt::text_style &color = piece.color;
    const std::string_view &character = piece.character;
    auto out = ctx.out();

    bool has_style = false;
    if (color.has_foreground()) {
      has_style = true;
      auto foreground = fmt::detail::make_foreground_color<Char>(color.get_foreground());
      out = std::copy(foreground.begin(), foreground.end(), out);
    }
    if (color.has_background()) {
      has_style = true;
      auto background = fmt::detail::make_background_color<Char>(color.get_background());
      out = std::copy(background.begin(), background.end(), out);
    }

    out = formatter<Piece, Char>::format(character, ctx);
    if (has_style) {
      auto reset_color = string_view("\x1b[0m");
      out = std::copy(reset_color.begin(), reset_color.end(), out);
    }
    return out;
  }
};

enum class PlayerIds {
  Player1,
  Player2,
  Player3,
  Player4,
};

#endif // TOKEN_HPP