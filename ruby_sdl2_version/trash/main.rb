require "bundler/setup"
Bundler.require(:default)

require "pathname"
require "matrix"

class Vec2 < Vector
end

class Base
  include Math

  def initialize(params = {})
    @params = {
    }.merge(params)
  end

  def run
    setup
    loop do
      event_loop
      update
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

  def after_view
  end

  def event_loop
    while ev = SDL2::Event.poll
      event_handle(ev)
    end
  end

  def event_handle(ev)
    case ev
    when SDL2::Event::KeyDown
      if ev.scancode == SDL2::Key::Scan::ESCAPE
        exit
      end
      if ev.scancode == SDL2::Key::Scan::Q
        exit
      end
    end
  end
end

# Base.new.run
################################################################################

module WindowMethods
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
    @window_rect = Vec2[*@window.size]

    @frame_counter = 0
  end

  def after_view
    super

    @frame_counter += 1
    @renderer.present
  end
end

Base.prepend(WindowMethods)

Base.new.run
exit

################################################################################

module FpsCounterMethods
  def setup
    super

    @real_fps = 60
    @fps_counter = 0
    @old_time = SDL2.get_ticks
  end

  def update
    super

    @fps_counter += 1
    v = SDL2.get_ticks
    t = v - @old_time
    if t >= 1000
      @real_fps = @fps_counter
      @old_time = v
      @fps_counter = 0
    end
  end
end

Base.prepend(FpsCounterMethods)

################################################################################

require "./item_list"

class App < Base
  GRADATION_MODE = true    # グラデーションにするか？
  BLOCK_N      = 16      # 辺の長さ
  VIEW_SIZE_RATE = 1.0     # 画面に対する表示領域の大きさ
  COLOR_MAX      = 255     # 色の要素の最大

  def setup
    super

    @preset_index = 0
    @counter      = 0

    @cell_wh      = @window_rect * (1.0 / BLOCK_N) * VIEW_SIZE_RATE # 画面の大きさから1つのセルのサイズを求める
    @half_cell_wh = @cell_wh * 0.5                              # 扱いやすいように半分バージョンも作っておく
    @top_left     = @window_rect * 0.5 - @cell_wh * BLOCK_N * 0.5   # 左上
  end

  def event_handle(ev)
    case ev
    when SDL2::Event::KeyDown
      if ev.scancode == SDL2::Key::Scan::Z
        preset_change(1)
      end
      if ev.scancode == SDL2::Key::Scan::X
        preset_change(-1)
      end
    end
  end

  def update
    super

    @renderer.draw_color = [0, 0, 0]
    @renderer.clear

    if false
      # https://ohai.github.io/ruby-sdl2/doc-en/SDL2/Mouse.html
      @local_state = SDL2::Mouse.state
      @renderer.draw_color = [0, 0, 255]
      rect = SDL2::Rect.new(@local_state.x, @local_state.y, 32, 32)
      @renderer.fill_rect(rect)
    end

    time = @counter.fdiv(@real_fps)
    index = 0
    BLOCK_N.times do |y|
      BLOCK_N.times do |x|
        retval = func_call(time, index, x, y)
        if retval.kind_of?(Numeric)
          if retval.nonzero?
            retval = retval.clamp(-1.0, 1.0)
            v = @top_left + @cell_wh.map2([x, y]) { |a, b| a * b } # それぞれに乗算するため scale ではだめ
            radius = @half_cell_wh * value_to_radius_rate(retval)  # 楕円の半径 = 最大半径 * 割合
            center = v + @half_cell_wh                             # セルの中心
            v2 = center - radius                                   # 長方形の左上
            @renderer.draw_color = tixy_color(retval)
            @renderer.fill_rect(SDL2::Rect.new(*v2, *(radius*2)))  # v2 から [radius, radius] の長方形を描画
          end
        end
        index += 1
      end
    end

    @counter += 1
  end

  def func_call(t, i, x, y)
    if e = current_preset
      v = instance_exec(t, Float(i), Float(x), Float(y), &e[:func])
      if v == true
        v = 1.0
      elsif v == false || v.nil?
        v = 0.0
      end
      v
    end
  end

  def current_preset
    filtered_item_list[@preset_index.modulo(filtered_item_list.size)]
  end

  def filtered_item_list
    @filtered_item_list ||= ItemList.find_all { |e| e[:favorite] }
    # @filtered_item_list ||= ItemList
  end

  def counter_reset
    @counter = 0
  end

  def tixy_color(v)
    rgb = nil
    if GRADATION_MODE
    else
      if v.positive?
        v = 1.0
      else
        v = -1.0
      end
    end
    c = v.abs * COLOR_MAX
    if v.positive?
      rgb = [c, c, c]
    else
      rgb = [c, 0, 0]
    end
    rgb
  end

  # 楕円の半径の割り合いを返す
  def value_to_radius_rate(rv)
    rv.abs * 0.9
  end

  def setup_vars
  end

  def preset_change(sign)
    @preset_index += sign
    counter_reset
  end
end

App.new.run
