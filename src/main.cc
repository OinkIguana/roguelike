#include "./game/engine.h"
#include "./view/default.h"
#include <memory>

int main(int argc, char* argv[]) {
    auto view = std::make_shared<View::Default>();
    Game::Engine engine(view);
    return engine.start();
}
