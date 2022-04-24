# -*- coding: utf-8; compile-command: "be rsdl main.rb --fps 60" -*-
# -*- coding: utf-8; compile-command: "be rsdl main.rb --fps 60 --full-screen" -*-
# tixy.land

require "bundler/setup"
Bundler.require(:default)

require "active_support/isolated_execution_state"

# Stylet.config.fps = 30

class App < Stylet::Base
  include Math

  include Stylet::Input::Base
  include Stylet::Input::ExtensionButton
  include Stylet::Input::StandardKeybordBind
  include Stylet::Input::JoystickBindMethod
  include Stylet::Input::MouseButtonBind

  CIRCLE_MODE    = false
  GRADATION_MODE = false
  BLOCK_N      = 16
  VIEW_SIZE_RATE = 0.8
  COLOR_MAX      = 255

  ItemList = [
    { favorite: false, name: "default",                                              func: -> (t, i, x, y) { sin(y/8+t)                                                     }},
    { favorite: false, name: "for every dot return 0 or 1 to change the visibility", func: -> (t, i, x, y) { rand < 0.1                                                     }},
    { favorite: false, name: "use a float between 0 and 1 to define the size",       func: -> (t, i, x, y) { rand                                                           }},
    { favorite: false, name: "parameter `t` is the time in seconds",                 func: -> (t, i, x, y) { sin(t)                                                         }},
    { favorite: false, name: "parameter `i` is the index of the dot (0..255)",       func: -> (t, i, x, y) { i / 256                                                        }},
    { favorite: false, name: "`x` is the column index from 0 to 15",                 func: -> (t, i, x, y) { x / 16                                                         }},
    { favorite: false, name: "`y` is the row also from 0 to 15",                     func: -> (t, i, x, y) { y / 16                                                         }},
    { favorite: false, name: "positive numbers are white, negatives are red",        func: -> (t, i, x, y) { y - 7.5                                                        }},
    { favorite: false, name: "use the time to animate values",                       func: -> (t, i, x, y) { y - t                                                          }},
    { favorite: false, name: "multiply the time to change the speed",                func: -> (t, i, x, y) { y - t*4                                                        }},
    { favorite: false, name: "create PresetInfo using different color",              func: -> (t, i, x, y) { [1, 0, -1][i%3]                                                }},
    { favorite: true,  name: "skip `Math` to use methods",                           func: -> (t, i, x, y) { sin(t-sqrt((x-7.5)**2+(y-6)**2))                               }},
    { favorite: false, name: "more examples",                                        func: -> (t, i, x, y) { sin(y/8 + t)                                                   }},
    { favorite: false, name: "simple triangle",                                      func: -> (t, i, x, y) { y - x                                                          }},
    { favorite: false, name: "quarter triangle",                                     func: -> (t, i, x, y) { (y > x) && (14-x < y)                                          }},
    { favorite: false, name: "pattern",                                              func: -> (t, i, x, y) { i%4 - y%4                                                      }},
    { favorite: false, name: "grid",                                                 func: -> (t, i, x, y) { (i%4)>0 && (y%4)>0                                             }},
    { favorite: false, name: "square",                                               func: -> (t, i, x, y) { x>3 && y>3 && x<12 && y<12                                     }},
    { favorite: false, name: "animated square",                                      func: -> (t, i, x, y) { (x>t && y>t && x<15-t && y<15-t) ? -1 : 0                      }},
    { favorite: false, name: "mondrian squares",                                     func: -> (t, i, x, y) { (y-6) * (x-6)                                                  }},
    { favorite: true,  name: "moving cross",                                         func: -> (t, i, x, y) { (y-4*t) * (x-2-t)                                              }},
    { favorite: false, name: "sierpinski",                                           func: -> (t, i, x, y) { (4*t).to_i & i.to_i & x.to_i & y.to_i                          }},
    { favorite: false, name: "binary clock",                                         func: -> (t, i, x, y) { y==8 && (t*10).to_i & (1<<x)                                   }},
    { favorite: false, name: "random noise",                                         func: -> (t, i, x, y) { rand(-1.0..1.0)                                                }},
    { favorite: false, name: "static smooth noise",                                  func: -> (t, i, x, y) { sin(i**2)                                                      }},
    { favorite: true,  name: "animated smooth noise",                                func: -> (t, i, x, y) { cos(t + i + x * y)                                             }},
    { favorite: true,  name: "waves",                                                func: -> (t, i, x, y) { sin(x/2) - sin(x-t) - y+6                                      }},
    { favorite: true,  name: "bloop bloop bloop by @v21",                            func: -> (t, i, x, y) { (x-8)*(y-8) - sin(t)*64                                        }},
    { favorite: true,  name: "fireworks by @p_malin and @aemkei",                    func: -> (t, i, x, y) { -0.4/(hypot(x-t%10,y-t%8)-t%2*9)                               }},
    { favorite: true,  name: "ripples by @thespite",                                 func: -> (t, i, x, y) { sin(t-sqrt(x*x+y*y))                                           }},
    { favorite: true,  name: "scrolling TIXY font by @atesgoral",                    func: -> (t, i, x, y) { [5463,2194,2386][y.to_i + (t*9).to_i & 7] & (1 << x-1)         }},
    { favorite: true,  name: "3d checker board by @p_malin",                         func: -> (t, i, x, y) { y>0 && (((x-8) / y + t*5).to_i & 1 ^ (1/y*8).to_i & 1) * y / 5 }},
    { favorite: false, name: "sticky blood by @joeytwiddle",                         func: -> (t, i, x, y) { y-t*3+9+3*cos(x*3-t)-5*sin(x*7)                                }},
    { favorite: true,  name: "3d starfield by @p_malin",                             func: -> (t, i, x, y) { d=y*y%5.9+1;(((x+t*50/d).to_i&15).zero? ? 1/d : 0)             }},
    { favorite: false, name: "dialogue with an alien by @chiptune",                  func: -> (t, i, x, y) { 1.0/32.0*tan(t/64.0*x*tan(i-x))                                }},
    { favorite: true,  name: "space invader by @keithclarkcouk + @zozuar",           func: -> (t, i, x, y) { 'p}¶¼<¼¶}p'.codepoints[x] & 2**y.to_i                        }},
    { favorite: true,  name: "hungry pac man by @p_malin and @aemkei",               func: -> (t, i, x, y) { hypot(x-=t%4*5,y-=8)<6 && (x<y || y<-x)                        }},
    { favorite: false, name: "spectrum analyser by @joeytwiddle",                    func: -> (t, i, x, y) { x.to_i.even? && y < 9 && y > (4 + sin(8*t+x*x) + x / 4)        }},
    { favorite: false, name: "diagonals",                                            func: -> (t, i, x, y) { y == x || ((15-x == y) ? -1 : 0 )                              }},
    { favorite: false, name: "frame",                                                func: -> (t, i, x, y) { x==0 || x==15 || y==0 || y==15                                 }},
    { favorite: true,  name: "drop",                                                 func: -> (t, i, x, y) { 8*t%13 - hypot(x-7.5, y-7.5)                                   }},
    { favorite: true,  name: "rotation",                                             func: -> (t, i, x, y) { sin(2*atan((y-7.5)/(x-7.5))+5*t)                               }},
    { favorite: true,  name: "wipe",                                                 func: -> (t, i, x, y) { (x-y) - sin(t) * 16                                            }},
    { favorite: false, name: "soft wipe",                                            func: -> (t, i, x, y) { (x-y)/24 - sin(t)                                              }},
    { favorite: false, name: "disco",                                                func: -> (t, i, x, y) { sin(t*5) * tan(t*7)                                            }},
    { favorite: false, name: "input is limited to 32 characters!",                   func: -> (t, i, x, y) { (x-5)**2 + (y-5)**2 - 99*sin(t)                                }},
  ]

  def initialize(*)
    super

    @item_index = 0
    @counter      = 0
  end

  setup do
  end

  update do
    joys.each {|joy| bit_update_by_joy(joy) }
    key_bit_update_all
    key_counter_update_all

    vputs "#{@item_index}: #{current_preset[:name]}"
    preset_change

    time = SDL.get_ticks.fdiv(1000)
    index = 0
    BLOCK_N.times do |y|
      BLOCK_N.times do |x|
        r = func_call(time, index, x, y)
        if r.kind_of?(Numeric)
          if r.nonzero?
            r = r.clamp(-1.0, 1.0)
            rgb = tixy_color(r)
            center = @inner_top_left + @cell_wh.map2(vec2[x, y]) { |a, b| a * b } + @cell_wh.scale(0.5)
            radius = @cell_wh.scale(0.5).scale(r.abs).scale(0.95)
            top_left = center - radius
            if CIRCLE_MODE
              screen.drawAAFilledEllipse(*center, *radius, rgb)
            else
              screen.fill_rect(*top_left, *(radius * 2), rgb)
            end
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
    filtered_item_list[@item_index.modulo(filtered_item_list.size)]
  end

  def filtered_item_list
    # @filtered_item_list ||= ItemList.find_all { |e| e[:favorite] }
    @filtered_item_list ||= ItemList
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

  def setup_vars
    @cell_wh        = vec2[srect.w, srect.h].scale(1.0 / BLOCK_N).scale(VIEW_SIZE_RATE) # 画面の大きさから1つのセルのサイズを求める
    @inner_top_left = srect.center - @cell_wh.scale(BLOCK_N * 0.5)                      # 左上
  end

  def screen_open
    super
    setup_vars
  end

  def preset_change
    if axis.right.press? || axis.left.press? || button.btA.trigger? || button.btB.trigger?
      @item_index += axis.right.repeat + button.btA.repeat
      @item_index -= axis.left.repeat + button.btB.repeat
      counter_reset
    end
    if button.btC.repeat == 1
      counter_reset
    end
  end

  run
end
