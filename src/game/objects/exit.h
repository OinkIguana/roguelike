#ifndef __GAME_EXIT_H__
#define __GAME_EXIT_H__

#include "object.h"
#include <functional>

namespace Game {
    class Exit : public Object {
        std::function<void()> callback;
    public:
        Exit(std::function<void()> callback) : callback{ callback } {};
        virtual char symbol() const override { return '/'; }
        virtual bool collectable() const override { return true; }
        virtual void collect(std::shared_ptr<Object>) override { callback(); }
    };
}

#endif
