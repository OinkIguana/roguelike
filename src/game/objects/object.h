#ifndef __GAME_OBJECT_H__
#define __GAME_OBJECT_H__

#include <memory>
#include "stats.h"

namespace Game {
    class Cell;
    struct Command;

    class Object {
        std::weak_ptr<Cell> _cell;
    public:
        std::shared_ptr<Cell> cell() const;
        void cell(std::shared_ptr<Cell> ncell);
        virtual char symbol() const = 0;
        virtual bool collectable(std::shared_ptr<Object>) const;
        virtual void collect(std::shared_ptr<Object>);
        virtual bool interactable(std::shared_ptr<Object>) const;
        virtual void interact(std::shared_ptr<Object>);
        virtual bool attackable(std::shared_ptr<Object>) const;
        virtual void attack(std::shared_ptr<Object>);
        virtual Command update(Command);

        Stats stats;
        int x, y;
    };
}

#endif
