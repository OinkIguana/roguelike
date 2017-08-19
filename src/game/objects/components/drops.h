#ifndef __GAME_DROPS_COMPONENT_H__
#define __GAME_DROPS_COMPONENT_H__

#include "../component.h"

namespace Game {
    class Drops : public ObjectComponent {
        std::shared_ptr<Object> _drop;
    public:
        Drops(std::shared_ptr<Object>);
        virtual void on_destroy(Object&, Cell&) override;
    };
}

#endif
