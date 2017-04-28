#include "engine.h"
#include "update.h"
#include "command.h"
#include "view.h"
#include "info.h"
#include "map.h"

#include "objects/characters/player.h"
#include "objects/characters/enemies/goblin.h"
#include "objects/exit.h"

#include <iostream>

namespace Game {
    Engine::Engine(std::shared_ptr<View> view) : view{ view } {}

    bool Engine::proc(Command cmd, std::shared_ptr<Object> obj, bool player) {
        switch(cmd.type) {
        case CommandType::Move: {
                int x = obj->x, y = obj->y;
                switch(cmd.data.direction) {
                    case Direction::Up:     y -= 1; break;
                    case Direction::Down:   y += 1; break;
                    case Direction::Left:   x -= 1; break;
                    case Direction::Right:  x += 1; break;
                }
                auto cell = map->cell_at(x, y);
                // TODO: find a nicer way of determining these flags than passing "player"
                if(cell->available(obj, true, player, player, true)) {
                    auto thing = cell->contents;
                    if(thing) thing->collect(obj);
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
                auto cell = map->cell_at(x, y);
                if(cell->contents && cell->contents->attackable(obj)) {
                    cell->contents->attack(obj);
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
                auto cell = map->cell_at(x, y);
                if(cell->contents && cell->contents->interactable(obj)) {
                    cell->contents->interact(obj);
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
                player = map->create<Player>();
                map->create<Goblin>();
                map->create<Goblin>();
                map->create<Goblin>();
                map->create_avoiding<Exit>(player, [&level_complete] () { level_complete = true; });
            } else {
                // update the map
                if(proc(cmd, player, true)) {
                    if(level_complete) continue;
                    std::vector<std::pair<std::shared_ptr<Object>, Command>> actions;
                    for(auto& cell : *map) {
                        if(cell.contents) {
                            actions.emplace_back(cell.contents, cell.contents->update(cmd));
                        }
                    }
                    for(auto& action : actions) {
                        proc(action.second, action.first);
                    }
                }
            }
            // update the display
            view->update(Game::Update{ UpdateType::MapChange, map.get() });
            view->redraw();
            // process player input
            cmd = view->command();
        } while(cmd);

        // end the game

        return 0;
    }
}
