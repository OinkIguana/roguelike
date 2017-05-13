#ifndef __GAME_NAMED_COMPONENT_H__
#define __GAME_NAMED_COMPONENT_H__

#include "../component.h"

namespace Game {
    class Named : public ObjectComponent {
        std::string _name;
    public:
        Named(std::string);
        virtual std::string name() const override;
    };
}

#endif
