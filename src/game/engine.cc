#include "engine.h"
#include "update.h"
#include "command.h"
#include "view.h"
#include "info.h"
#include "map.h"

#include <iostream>
#include <algorithm>

namespace Game {
    Engine::Engine(std::shared_ptr<View> view) : view{ view } {}

    bool Engine::proc(Command cmd, std::shared_ptr<Object> obj) {
        if(obj->dead()) { return true; }
        switch(cmd.type) {
        case CommandType::Move: {
                int x = obj->x, y = obj->y;
                switch(cmd.data.direction) {
                    case Direction::Up:     y -= 1; break;
                    case Direction::Down:   y += 1; break;
                    case Direction::Left:   x -= 1; break;
                    case Direction::Right:  x += 1; break;
                }
                if(y < 0 || y >= map->height() || x < 0 || x >= map->width()) { return false; }
                auto cell = map->cell_at(x, y);
                if(cell->available(*obj, true, obj->type == Object::Type::Player, obj->type == Object::Type::Player, true)) {
                    auto thing = cell->contents;
                    if(thing) {
                        thing->collect(*obj);
                        thing->destroy();
                    }
                    map->cell_at(obj->x, obj->y)->clear();
                    cell->set_contents(obj);
                } else {
                    return false;
                }
            }
            break;
        case CommandType::Attack: {
                int x = obj->x, y = obj->y;
                switch(cmd.data.direction) {
                    case Direction::Up:     y -= 1; break;
                    case Direction::Down:   y += 1; break;
                    case Direction::Left:   x -= 1; break;
                    case Direction::Right:  x += 1; break;
                }
                if(y < 0 || y >= map->height() || x < 0 || x >= map->width()) { return false; }
                auto cell = map->cell_at(x, y);
                if(cell->contents && cell->contents->attackable(*obj)) {
                    cell->contents->attack(*obj);
                } else {
                    return false;
                }
            }
            break;
        case CommandType::Interact: {
                int x = obj->x, y = obj->y;
                switch(cmd.data.direction) {
                    case Direction::Up:     y -= 1; break;
                    case Direction::Down:   y += 1; break;
                    case Direction::Left:   x -= 1; break;
                    case Direction::Right:  x += 1; break;
                }
                if(y < 0 || y >= map->height() || x < 0 || x >= map->width()) { return false; }
                auto cell = map->cell_at(x, y);
                if(cell->contents && cell->contents->interactable(*obj)) {
                    cell->contents->interact(*obj);
                } else {
                    return false;
                }
            }
            break;
        default:
            return false;
        }
        return true;
    }

    int Engine::start() {
        // intialize new game
        info = std::make_shared<Info>();
        info->level = 0;

        // ignore everything until character class is chosen
        Command cmd;
        do {
            cmd = view->command();
            if(cmd.type == CommandType::CharacterSelect) {
                info->race = cmd.data.character;
            } else if(cmd.type == CommandType::Quit) { return 0; }
        } while(cmd.type != CommandType::CharacterSelect);

        // play the game
        bool level_complete = true;
        do {
            if(level_complete) {
                level_complete = false;
                // create new level
                map = std::make_shared<Map>(++info->level);
                // populate
                player = map->add(Object::Player());
                player->cell()->set_visible(true);
                map->add(Object::Goblin());
                map->add(Object::Goblin());
                map->add(Object::Goblin());
                map->add_avoiding(player, Object::Exit([&level_complete] () { level_complete = true; }));
            } else {
                // update the map
                std::vector<std::pair<std::shared_ptr<Object>, Command>> actions;
                for(auto& cell : *map) {
                    if(cell.contents) {
                        auto events = cell.contents->update(cmd);
                        std::for_each(events.begin(), events.end(), [&actions, &cell] (Command event) {
                            actions.emplace_back(cell.contents, event);
                        });
                    }
                }
                // ensure objects move in the right type-order
                std::sort(actions.begin(), actions.end(), [] (std::pair<std::shared_ptr<Object>, Command> a, std::pair<std::shared_ptr<Object>, Command> b) {
                    return a.first->type < b.first->type;
                });
                for(auto& action : actions) { proc(action.second, action.first); }
                if(level_complete) continue;
            }
            // update the display
            view->update(Game::Update{ UpdateType::MapChange, map.get() });
            view->update(Game::Update{ UpdateType::PlayerChange, player.get() });
            view->redraw();
            // process player input
            cmd = view->command();
        } while(cmd);

        // end the game

        return 0;
    }
}
