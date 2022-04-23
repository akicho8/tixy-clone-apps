require "bundler/setup"
Bundler.require(:default)

SDL2.init(SDL2::INIT_EVERYTHING)
window = SDL2::Window.create("(title)", SDL2::Window::POS_CENTERED, SDL2::Window::POS_CENTERED, 640, 480, 0)
window_flags = 0
window_flags |= SDL2::Renderer::Flags::ACCELERATED
window_flags |= SDL2::Renderer::Flags::PRESENTVSYNC
renderer = window.create_renderer(-1, window_flags)

include Math

counter = 0
loop do
  while ev = SDL2::Event.poll
    case ev
    when SDL2::Event::KeyDown
      case ev.scancode
      when SDL2::Key::Scan::ESCAPE
        exit
      when SDL2::Key::Scan::Q
        exit
      end
    end
  end

  renderer.draw_color = [0, 0, 0]
  renderer.clear

  renderer.draw_color = [0, 0, 255]

  w, h = window.size
  24.times do |i|
    a = PI * 2 / (24 * 1.5) * i
    x = w / 2 + cos(a + PI * counter * 0.04 * 0.7) * w * 0.4
    y = h / 2 + sin(a + PI * counter * 0.04 * 0.8) * h * 0.4
    radius = 32
    renderer.draw_rect(SDL2::Rect.new(x - radius, y - radius, radius * 2, radius * 2))
  end

  renderer.present
  counter += 1
end
