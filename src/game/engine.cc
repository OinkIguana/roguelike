#include "engine.h"
#include "update.h"
#include "command.h"
#include "view.h"
#include "info.h"
#include "map.h"

#include "objects/characters/player.h"
#include "objects/exit.h"

#include <iostream>

namespace Game {
    Engine::Engine(std::shared_ptr<View> view) : view{ view } {}

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
                map->create_avoiding<Exit>(player, [&level_complete] () { level_complete = true; });
            } else {
                // update the map
                switch(cmd.type) {
                case CommandType::Move: {
                    int x = player->x, y = player->y;
                    switch(cmd.data.direction) {
                        case Direction::Up:     y -= 1; break;
                        case Direction::Down:   y += 1; break;
                        case Direction::Left:   x -= 1; break;
                        case Direction::Right:  x += 1; break;
                    }
                    auto cell = map->cell_at(x, y);
                    if(cell->available()) {
                        auto thing = cell->contents;
                        if(thing) thing->collect(player);
                        map->cell_at(player->x, player->y)->clear();
                        cell->set_contents(player);
                    }
                }   break;
                default: ;
                }
                if(level_complete) continue;
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
