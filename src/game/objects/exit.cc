#include "exit.h"
#include "characters/player.h"

namespace Game {
    Exit::Exit(std::function<void()> callback) : callback{ callback } {};
    char Exit::symbol() const { return '/'; }
    bool Exit::collectable(std::shared_ptr<Object> obj) const {
        return !!std::dynamic_pointer_cast<Player>(obj);
    }
    void Exit::collect(std::shared_ptr<Object>) { callback(); }
}
