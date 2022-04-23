# -*- coding: utf-8; compile-command: "bundle exec rsdl main.rb" -*-
require "bundler/setup"
Bundler.require(:default)

require "./item_list"

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

class App
  include Math

  CIRCLE_MODE    = false
  BLOCK_N        = 16

  class << self
    def run(*args)
      new(*args).run
    end
  end

  def run
    SDL.init(SDL::INIT_EVERYTHING)

    color_depth = 16
    flags = 0
    flags |= SDL::HWSURFACE
    flags |= SDL::DOUBLEBUF
    flags |= SDL::HWACCEL
    # flags |= SDL::FULLSCREEN
    @screen = SDL.set_video_mode(640, 480, color_depth, flags)
    @window_rect = Vector2d(@screen.w, @screen.h)

    @item_index = 0
    @frame_counter = 0

    @fps = 1
    @fps_counter = 0
    @old_time = SDL.get_ticks

    @cell_wh        = @window_rect * 1.0 / BLOCK_N
    @inner_top_left = @window_rect * 0.5 - @cell_wh * BLOCK_N * 0.5

    loop do
      while event = SDL::Event2.poll
        case event
        when SDL::Event2::Quit
          exit
        when SDL::Event2::KeyDown
          if event.sym == SDL::Key::ESCAPE || event.sym == SDL::Key::Q
            exit
          end
          if event.sym == SDL::Key::LEFT
            @item_index -= 1
          end
          if event.sym == SDL::Key::RIGHT
            @item_index += 1
          end
        end
      end

      @fps_counter += 1
      v = SDL.get_ticks
      t = v - @old_time
      if t >= 1000
        @fps = @fps_counter
        @old_time = v
        @fps_counter = 0
        p @fps
      end

      @screen.fill_rect(0, 0, *@window_rect, [0, 0, 0])

      time = SDL.get_ticks.fdiv(1000)
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
              rgb = tixy_color(r)
              if CIRCLE_MODE
                @screen.drawAAFilledEllipse(*center, *radius, rgb)
              else
                @screen.fill_rect(*top_left, *(radius * 2), rgb)
              end
            end
          end
          index += 1
        end
      end

      @frame_counter += 1
      @screen.flip
    end
  end

  def func_call(t, i, x, y)
    if e = current_item
      v = instance_exec(t, Float(i), Float(x), Float(y), &e[:func])
      if v == true
        v = 1.0
      elsif v == false || v.nil?
        v = 0.0
      end
      v
    end
  end

  def current_item
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
    c = v.abs * 255
    if v.positive?
      [c, c, c]
    else
      [c, 0, 0]
    end
  end

  run
end
