#!/bin/sh
cd ~/src
rm -fr ruby-sdl2-playground
mkdir -p ruby-sdl2-playground
cd ruby-sdl2-playground
bundle init
bundle add ruby-sdl2 --require sdl2 --optimistic
bundle exec ruby -r sdl2 -e "p SDL2"

cat <<'EOF' > main.rb
require "bundler/setup"
Bundler.require(:default)

SDL2.init(SDL2::INIT_EVERYTHING)
pos = SDL2::Window::POS_CENTERED
window = SDL2::Window.create("(title)", pos, pos, 640, 480, 0)
flags = 0
flags |= SDL2::Renderer::Flags::PRESENTVSYNC
renderer = window.create_renderer(-1, flags)

120.times do
  SDL2::Event.poll
  renderer.present
end
EOF

bundle exec ruby main.rb

