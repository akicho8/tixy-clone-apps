# require "bundler/setup"
# Bundler.require(:default)
#
# SDL2.init(SDL2::INIT_EVERYTHING)
# pos = SDL2::Window::POS_CENTERED
# window = SDL2::Window.create("(title)", pos, pos, 640, 480, 0)
# flags = 0
# flags |= SDL2::Renderer::Flags::PRESENTVSYNC
# renderer = window.create_renderer(-1, flags)
#
# 120.times do
#   SDL2::Event.poll
#   renderer.present
# end
# exit

################################################################################

# require "bundler/setup"
# Bundler.require(:default)
#
# SDL2.init(SDL2::INIT_EVERYTHING)
# pos = SDL2::Window::POS_CENTERED
# window = SDL2::Window.create("(title)", pos, pos, 640, 480, 0)
# flags = 0
# flags |= SDL2::Renderer::Flags::ACCELERATED
# flags |= SDL2::Renderer::Flags::PRESENTVSYNC
# renderer = window.create_renderer(-1, flags)
#
# include Math
#
# frame_counter = 0
# loop do
#   while ev = SDL2::Event.poll
#     case ev
#     when SDL2::Event::Quit
#       exit
#     when SDL2::Event::KeyDown
#       case ev.scancode
#       when SDL2::Key::Scan::ESCAPE
#         exit
#       when SDL2::Key::Scan::Q
#         exit
#       end
#     end
#   end
#
#   renderer.draw_blend_mode = SDL2::BlendMode::BLEND
#   renderer.draw_color = [0, 0, 64, 28]
#   renderer.fill_rect(SDL2::Rect.new(0, 0, *window.size))
#
#   renderer.draw_blend_mode = SDL2::BlendMode::NONE
#   renderer.draw_color = [255, 255, 255]
#
#   r = 64
#   w, h = window.size
#   x = w / 2 + cos(PI * frame_counter * 0.02 * 0.7) * w * 0.4
#   y = h / 2 + sin(PI * frame_counter * 0.02 * 0.8) * h * 0.4
#   renderer.fill_rect(SDL2::Rect.new(x - r, y - r, r * 2, r * 2))
#
#   renderer.present
#   frame_counter += 1
# end
# exit

################################################################################

require "bundler/setup"
Bundler.require(:default)

class Base
  include Math

  class << self
    def run(*args)
      new(*args).run
    end
  end

  def run
    setup
    loop do
      event_loop
      update
      before_view
      view
      after_view
    end
  end

  private

  def setup
    SDL2.init(SDL2::INIT_EVERYTHING)
  end

  def update
  end

  def view
  end

  def before_view
  end

  def after_view
  end

  def event_loop
    while ev = SDL2::Event.poll
      event_handle(ev)
    end
  end

  def event_handle(ev)
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
end

# Base.run
# exit

################################################################################

module WindowMethods
  attr_accessor :window
  attr_accessor :renderer
  attr_accessor :frame_counter

  def setup
    super

    flags = 0
    # flags |= SDL2::Window::Flags::FULLSCREEN
    # flags |= SDL2::Window::Flags::FULLSCREEN_DESKTOP
    pos = SDL2::Window::POS_CENTERED
    @window = SDL2::Window.create("(Title)", pos, pos, 640, 480, flags)

    flags = 0
    flags |= SDL2::Renderer::Flags::ACCELERATED
    flags |= SDL2::Renderer::Flags::PRESENTVSYNC
    @renderer = @window.create_renderer(-1, flags)

    @frame_counter = 0
  end

  def before_view
    super

    renderer.draw_blend_mode = SDL2::BlendMode::BLEND
    renderer.draw_color = [0, 0, 64, 28]
    renderer.fill_rect(SDL2::Rect.new(0, 0, *@window.size))

    renderer.draw_blend_mode = SDL2::BlendMode::NONE
    renderer.draw_color = [255, 255, 255]
  end

  def after_view
    super

    @frame_counter += 1
    renderer.present
  end
end

Base.prepend(WindowMethods)

# Base.run

################################################################################

# class App < Base
#   def view
#     super
#
#     r = 64
#     w, h = window.size
#     x = w / 2 + cos(PI * frame_counter * 0.02 * 0.7) * w * 0.4
#     y = h / 2 + sin(PI * frame_counter * 0.02 * 0.8) * h * 0.4
#     renderer.fill_rect(SDL2::Rect.new(x - r, y - r, r * 2, r * 2))
#   end
#
#   run
# end

################################################################################

module FpsCounterMethods
  attr_reader :fps

  def setup
    super

    @fps = 60
    @fps_counter = 0
    @old_time = SDL2.get_ticks
  end

  def update
    super

    @fps_counter += 1
    v = SDL2.get_ticks
    t = v - @old_time
    if t >= 1000
      @fps = @fps_counter
      @old_time = v
      @fps_counter = 0
    end
  end
end

Base.prepend(FpsCounterMethods)

# Base.run
# exit

################################################################################

require "pathname"

module FontMethods
  def setup
    super

    font_file = "~/Library/Fonts/Ricty-Regular.ttf"
    font_size = 32

    SDL2::TTF.init
    @font = SDL2::TTF.open(Pathname(font_file).expand_path.to_s, font_size)
    @font.kerning = true
  end

  def after_view
    system_line "#{frame_counter} #{fps}fps"

    super
  end

  def system_line(text)
    rect = SDL2::Rect.new(0, 0, *@font.size_text(text))

    renderer.draw_blend_mode = SDL2::BlendMode::NONE
    renderer.draw_color = [0, 0, 128]
    renderer.fill_rect(rect)

    font_color = [255, 255, 255]
    texture = renderer.create_texture_from(@font.render_blended(text, font_color))
    renderer.copy(texture, nil, rect)
  end
end

Base.prepend(FontMethods)

# Base.run
# exit

################################################################################

# bundle remove vector2d
# bundle add matrix

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

# Vector2d(2, 3) * Vector2d(2, 2) # => Vector[4, 6]
# exit

################################################################################

class TixyCloneApp < Base
  CELL_N = 16

  def setup
    super

    @window_rect    = Vector2d(*window.size)
    @cell_wh        = @window_rect * 1.0 / CELL_N
    @inner_top_left = @window_rect * 0.5 - @cell_wh * CELL_N * 0.5
  end

  def before_view
    renderer.draw_color = [0, 0, 0]
    renderer.clear
  end

  def view
    super

    time = SDL2.get_ticks.fdiv(1000)
    index = 0
    CELL_N.times do |y|
      CELL_N.times do |x|
        r = tixy_func(time, index, x, y)
        if r.nonzero?
          r = r.clamp(-1.0, 1.0)
          center = @inner_top_left + @cell_wh * Vector2d(x, y) + @cell_wh * 0.5
          radius = @cell_wh * 0.5 * r.abs * 0.95
          top_left = center - radius
          renderer.draw_color = tixy_color(r)
          renderer.fill_rect(SDL2::Rect.new(*top_left, *(radius * 2)))
        end
        index += 1
      end
    end
  end

  def tixy_func(t, i, x, y)
    sin(t - sqrt((x - 7.5)**2 + (y - 6)**2))
  end

  def tixy_color(v)
    if v.positive?
      v = 1.0
    else
      v = -1.0
    end
    c = v.abs * 255
    if v.positive?
      [c, c, c]
    else
      [c, 0, 0]
    end
  end

  # run
end
