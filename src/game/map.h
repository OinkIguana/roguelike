#ifndef __GAME_MAP_H__
#define __GAME_MAP_H__

#include <vector>
#include <memory>
#include <string>
#include "room.h"
#include "cell.h"

namespace Game {
    class Object;

    class Map {
        int w, h, _floor;
        std::vector<std::vector<std::shared_ptr<Cell>>> cells;
        std::vector<std::shared_ptr<Room>> rooms;
    public:
        Map(int);
        std::vector<std::string> to_strings() const;
        std::vector<std::string> object_strings() const;

        std::shared_ptr<Room> choose_room() const;

        std::shared_ptr<Object> add(std::shared_ptr<Object>);
        std::shared_ptr<Object> add_avoiding(std::shared_ptr<Object>, std::shared_ptr<Object>);
        std::shared_ptr<Cell> cell_at(int, int) const;

        class Iterator {
            friend class Map;
            int x, y;
            Map & map;
            Iterator(int x, int y, Map &);
        public:
            Iterator operator++();
            Cell& operator*();
            bool operator!=(Iterator&);
        };

        // cell iteration
        Iterator begin();
        Iterator end();

        inline int width() const { return w; }
        inline int height() const { return h; }
        inline int floor() const { return _floor; }
    };
}

#endif
