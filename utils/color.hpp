#ifndef COLOR_HPP
#define COLOR_HPP

#include <string>
#include <string_view>

#include <fmt/color.h>

#include <utils/map.hpp>

class Color {
private:
  using sv = std::string_view;
  using ts = fmt::text_style;

  // White foreground is considered nothing because terminals usually have white
  // text and thus appear like normal. We are assuming white terminal text color
  static constexpr ts no_color = fmt::fg(fmt::color::white);
  static constexpr ts default_color = fmt::bg(fmt::color::black);
  static constexpr std::array<std::pair<sv, ts>, 10> colors = {{
      {"Red", fmt::bg(fmt::color::red)},
      {"Orange", fmt::bg(fmt::color::dark_orange)},
      {"Yellow", fmt::bg(fmt::color::yellow)},
      {"Green", fmt::bg(fmt::color::green)},
      {"Cyan", fmt::bg(fmt::color::dark_turquoise)},
      {"Blue", fmt::bg(fmt::color::navy)},
      {"Magenta", fmt::bg(fmt::color::deep_pink)},
      {"Brown", fmt::bg(fmt::color::saddle_brown)},
      {"Gray", fmt::bg(fmt::color::slate_gray)},
      {"White", fmt::bg(fmt::color::snow)},
  }};
  static constexpr auto map = CompileTimeMap<sv, ts, colors.size()>{{colors}};

public:
  static constexpr ts get(const std::string &color) {
    return map.at(color, no_color);
  };

  static constexpr ts none() { return no_color; }
  static constexpr std::string empty(int spaces) {
    return fmt::format(no_color, std::string(spaces, ' '));
  };
};

#endif // COLOR_HPP