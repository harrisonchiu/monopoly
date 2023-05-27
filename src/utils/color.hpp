#ifndef COLOR_HPP
#define COLOR_HPP

#include "src/utils/map.hpp"

#include <fmt/color.h>

#include <array>
#include <string>
#include <string_view>

class Color {
  using sv = std::string_view;
  using ts = fmt::text_style;

private:
  static constexpr ts no_color = fmt::text_style();
  static constexpr std::array<std::pair<sv, ts>, 10> colors = { {
      { "Red", fmt::bg(fmt::color::red) },
      { "Orange", fmt::bg(fmt::color::dark_orange) },
      { "Yellow", fmt::bg(fmt::color::yellow) },
      { "Green", fmt::bg(fmt::color::green) },
      { "Cyan", fmt::bg(fmt::color::dark_turquoise) },
      { "Blue", fmt::bg(fmt::color::navy) },
      { "Magenta", fmt::bg(fmt::color::deep_pink) },
      { "Brown", fmt::bg(fmt::color::saddle_brown) },
      { "Gray", fmt::bg(fmt::color::slate_gray) },
      { "White", fmt::bg(fmt::color::snow) },
  } };
  static constexpr auto map = CompileTimeMap<sv, ts, colors.size()>{ { colors } };

public:
  static constexpr auto get(const std::string_view color) -> ts {
    return map.get(color, no_color);
  };

  static constexpr auto get(const std::string_view color, const ts default_color) -> ts {
    return map.get(color, default_color);
  };

  static constexpr auto none() -> ts { return no_color; }
  static constexpr auto empty(int spaces) -> std::string {
    return fmt::format(no_color, std::string(spaces, ' '));
  };
};

#endif // COLOR_HPP