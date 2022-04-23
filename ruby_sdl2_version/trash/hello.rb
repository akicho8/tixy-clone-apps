require "bundler/setup"
Bundler.require(:default)

SDL2.init(SDL2::INIT_EVERYTHING)
window = SDL2::Window.create("(title)", SDL2::Window::POS_CENTERED, SDL2::Window::POS_CENTERED, 640, 480, 0)
window_flags = 0
window_flags |= SDL2::Renderer::Flags::ACCELERATED
window_flags |= SDL2::Renderer::Flags::PRESENTVSYNC
renderer = window.create_renderer(-1, window_flags)

120.times do
  SDL2::Event.poll
  renderer.present
end
