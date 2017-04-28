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
        int w, h;
        std::vector<std::vector<std::shared_ptr<Cell>>> cells;
        std::vector<std::shared_ptr<Room>> rooms;
    public:
        Map(int);
        std::string to_string() const;
        std::string object_string() const;

        std::shared_ptr<Room> choose_room() const;

        template<typename T, typename ... Args>
        std::shared_ptr<T> create(Args ... args);
        template<typename T, typename ... Args>
        std::shared_ptr<T> create_avoiding(std::shared_ptr<Object>, Args ... args);
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
    };

    template<typename T, typename ... Args>
    std::shared_ptr<T> Map::create(Args ... args) {
        auto obj = std::make_shared<T>(args...);

        std::shared_ptr<Room> rm = choose_room();
        auto cell = rm->choose_cell();
        cell->set_contents(obj);

        return obj;
    }

    template<typename T, typename ... Args>
    std::shared_ptr<T> Map::create_avoiding(std::shared_ptr<Object> avoid, Args ... args) {
        auto obj = std::make_shared<T>(args...);

        std::shared_ptr<Room> rm;
        do {
            rm = choose_room();
        } while(rm->contains(avoid));

        auto cell = rm->choose_cell();
        cell->set_contents(obj);

        return obj;
    }
}

#endif
