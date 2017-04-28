#ifndef __GAME_EXIT_H__
#define __GAME_EXIT_H__

#include "object.h"
#include <functional>

namespace Game {
    class Exit : public Object {
        std::function<void()> callback;
    public:
        Exit(std::function<void()> callback);
        virtual char symbol() const override;
        virtual bool collectable(std::shared_ptr<Object>) const;
        virtual void collect(std::shared_ptr<Object>);
    };
}

#endif
