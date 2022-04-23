# -*- coding: utf-8; compile-command: "bundle exec rsdl benchmark.rb" -*-
require "bundler/setup"
Bundler.require(:default)

BLOCK_N = 16

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

SDL.init(SDL::INIT_EVERYTHING)

flags = 0
flags |= SDL::HWSURFACE
flags |= SDL::DOUBLEBUF
flags |= SDL::HWACCEL
flags |= SDL::NOFRAME
flags |= SDL::FULLSCREEN
@screen = SDL.set_video_mode(640, 480, 16, flags)
@window_rect = Vector2d(@screen.w, @screen.h)

@item_index = 0
@frame_counter = 0

@fps = 1
@fps_counter = 0
@old_time = SDL.get_ticks

@cell_wh        = @window_rect * 0.9 / BLOCK_N
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

  BLOCK_N.times do |y|
    BLOCK_N.times do |x|
      center = @inner_top_left + @cell_wh * Vector2d(x, y) + @cell_wh * 0.5
      radius = @cell_wh * 0.5 * 0.95
      top_left = center - radius
      rgb = rand(2).zero? ? [255, 0, 0] : [255, 255, 255]
      @screen.fill_rect(*top_left, *(radius * 2), rgb)
    end
  end

  @frame_counter += 1
  @screen.flip
end
