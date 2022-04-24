require "./mini_framework"
require "./item_list"

class App < Base
  BLOCK_N      = 16      # 辺の長さ
  VIEW_SIZE_RATE = 1.0     # 画面に対する表示領域の大きさ
  COLOR_MAX      = 255     # 色の要素の最大

  def setup
    super

    @item_index = 0

    @window_rect    = Vector2d(*window.size)
    @cell_wh        = @window_rect * 1.0 / BLOCK_N
    @inner_top_left = @window_rect * 0.5 - @cell_wh * BLOCK_N * 0.5
  end

  def event_handle(ev)
    super

    case ev
    when SDL2::Event::KeyDown
      case ev.scancode
      when SDL2::Key::Scan::LEFT
        preset_change(-1)
      when SDL2::Key::Scan::RIGHT
        preset_change(1)
      end
    end
  end

  def view
    super

    renderer.draw_color = [0, 0, 0]
    renderer.clear

    time = SDL2.get_ticks.fdiv(1000)
    index = 0
    BLOCK_N.times do |y|
      BLOCK_N.times do |x|
        r = func_call(time, index, x, y)
        if r.kind_of?(Numeric)
          if r.nonzero?
            r = r.clamp(-1.0, 1.0)
            center = @inner_top_left + @cell_wh * Vector2d(x, y) + @cell_wh * 0.5
            radius = @cell_wh * 0.5 * r.abs * 0.95
            top_left = center - radius
            renderer.draw_color = tixy_color(r)
            renderer.fill_rect(SDL2::Rect.new(*top_left, *(radius * 2)))
          end
        end
        index += 1
      end
    end
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
    filtered_item_list[@item_index.modulo(filtered_item_list.size)]
  end

  def filtered_item_list
    @filtered_item_list ||= ItemList.find_all { |e| e[:favorite] }
    # @filtered_item_list ||= ItemList
  end

  def tixy_color(v)
    if v.positive?
      v = 1.0
    else
      v = -1.0
    end
    c = v.abs * COLOR_MAX
    if v.positive?
      [c, c, c]
    else
      [c, 0, 0]
    end
  end

  # 楕円の半径の割り合いを返す
  def value_to_radius_rate(rv)
    rv.abs * 0.9
  end

  def setup_vars
  end

  def preset_change(sign)
    @item_index += sign
  end
end

App.new.run
