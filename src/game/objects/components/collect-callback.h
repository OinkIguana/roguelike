#ifndef __GAME_COLLECT_CALLBACK_COMPONENT_H__
#define __GAME_COLLECT_CALLBACK_COMPONENT_H__

#include "../component.h"
#include <functional>

namespace Game {
    class CollectCallback : public ObjectComponent {
        std::function<void()> _cb;
    public:
        CollectCallback(std::function<void()>);
        virtual bool collectable(Object&, const Object&) const override;
        virtual void collect(Object&, Object&) override;
    };
}

#endif
