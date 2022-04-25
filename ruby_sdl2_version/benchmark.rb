require "bundler/setup"
Bundler.require(:default)

class Vector2d < Vector
  def *(v)
    if v.kind_of?(self.class)
      map2(v) { |a, b| a * b }
    else
      super
    end
  end
end

def Vector2d(*args)
  Vector2d[*args]
end

CELL_N = 16

SDL2.init(SDL2::INIT_EVERYTHING)

flags = 0
# flags |= SDL2::Window::Flags::FULLSCREEN
# flags |= SDL2::Window::Flags::FULLSCREEN_DESKTOP
pos = SDL2::Window::POS_CENTERED
window = SDL2::Window.create("(Title)", pos, pos, 640, 480, flags)

flags = 0
flags |= SDL2::Renderer::Flags::ACCELERATED
flags |= SDL2::Renderer::Flags::PRESENTVSYNC
renderer = window.create_renderer(-1, flags)

window_rect    = Vector2d(*window.size)
cell_wh        = window_rect * 0.9 / CELL_N
inner_top_left = window_rect * 0.5 - cell_wh * CELL_N * 0.5

frame_counter = 0

fps = 60
fps_counter = 0
old_time = SDL2.get_ticks

loop do
  while ev = SDL2::Event.poll
    case ev
    when SDL2::Event::Quit
      exit
    when SDL2::Event::KeyDown
      case ev.scancode
      when SDL2::Key::Scan::ESCAPE
        exit
      when SDL2::Key::Scan::Q
        exit
      end
    end
  end

  fps_counter += 1
  v = SDL2.get_ticks
  t = v - old_time
  if t >= 1000
    fps = fps_counter
    old_time = v
    fps_counter = 0
    puts fps
  end

  renderer.draw_color = [0, 0, 0]
  renderer.clear

  radius = cell_wh * 0.5 * 0.95
  CELL_N.times do |y|
    CELL_N.times do |x|
      center = inner_top_left + cell_wh * Vector2d(x, y) + cell_wh * 0.5
      top_left = center - radius
      renderer.draw_color = rand(2).zero? ? [255, 0, 0] : [255, 255, 255]
      renderer.fill_rect(SDL2::Rect.new(*top_left, *(radius * 2)))
      # renderer.fill_rect(SDL2::Rect.new(0, 0, *cell_wh))
      # renderer.fill_rect(SDL2::Rect.new(rand(window_rect.x), rand(window_rect.y), *cell_wh))
    end
  end

  frame_counter += 1
  renderer.present
end
