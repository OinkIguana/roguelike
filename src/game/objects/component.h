#ifndef __GAME_OBJECT_COMPONENT_H__
#define __GAME_OBJECT_COMPONENT_H__

#include <memory>
#include <string>
#include "../command.h"
#include "../cell.h"
#include "object.h"

namespace Game {
    class ObjectComponent {
    public:
        virtual ~ObjectComponent() = 0;
        virtual std::string name() const;
        virtual bool collectable(Object&, const Object&) const;
        virtual void collect(Object&, Object&);
        virtual bool interactable(Object&, const Object&) const;
        virtual void interact(Object&, Object&);
        virtual bool attackable(Object&, const Object&) const;
        virtual void attack(Object&, Object&);

        virtual void on_destroy(Object&, Cell&);

        virtual Command update(Command, Object&);
    };
}

#endif
