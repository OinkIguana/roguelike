#include <ncurses.h>
#include <locale.h>
#include <iostream>
#include "default.h"
#include "../game/info.h"

namespace View {
    Default::Default() {
        setlocale(LC_ALL, "");
        initscr();
        noecho();
        keypad(stdscr, TRUE);
        start_color();
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        curs_set(0);
        ESCDELAY = 0;
    }

    Default::~Default() {
        endwin();
    }

    void Default::update(Game::Update update) {
        switch(update.type) {
        case Game::UpdateType::MapChange: {
                auto _map = static_cast<Game::Map*>(update.map);
                map = _map->to_strings();
                objects = _map->object_strings();
                level = _map->floor();
            }
            break;
        case Game::UpdateType::PlayerChange: {
                auto _obj = static_cast<Game::Object*>(update.object);
                names[0] = _obj->name();
                stats[0] = _obj->stats;
                inventory = _obj->inventory;
                viewport.x = std::max(std::min(static_cast<int>(map[0].size()) - viewport.w, _obj->x - viewport.w / 2), 0);
                viewport.y = std::max(std::min(static_cast<int>(map.size()) - viewport.h, _obj->y - viewport.h / 2), 0);
            }
            break;
        case Game::UpdateType::TargetChange: {
                auto _obj = static_cast<Game::Object*>(update.object);
                names[1] = _obj->name();
                stats[1] = _obj->stats;
            }
            break;
        case Game::UpdateType::MessageChange:
            message = update.message;
            break;
        }
    }

    Game::Command Default::character_select() {
        // TODO: add some character classes
        getch();
        state = [this] () { return world_view(); };
        return { Game::CommandType::CharacterSelect, Game::CharacterType::Human };
    }

    Game::Command Default::world_view() {
        switch(getch()) {
        case KEY_UP:
        case 'W':
        case 'w':
            last_direction = Game::Direction::Up;
            return Game::Command{ Game::CommandType::Move, Game::Direction::Up };
        case KEY_DOWN:
        case 'S':
        case 's':
            last_direction = Game::Direction::Down;
            return Game::Command{ Game::CommandType::Move, Game::Direction::Down };
        case KEY_RIGHT:
        case 'D':
        case 'd':
            last_direction = Game::Direction::Right;
            return Game::Command{ Game::CommandType::Move, Game::Direction::Right };
        case KEY_LEFT:
        case 'A':
        case 'a':
            last_direction = Game::Direction::Left;
            return Game::Command{ Game::CommandType::Move, Game::Direction::Left };
        case 'X':
        case 'x':
            return Game::Command{ Game::CommandType::Attack, last_direction };
        case 'Z':
        case 'z':
            return Game::Command{ Game::CommandType::Interact, last_direction };
        case 27:
            return Game::Command{ Game::CommandType::Quit };
        default:
            return command();
        }
    }

    Game::Command Default::command() {
        return state();
    }
    void Default::redraw() {
        // border
        mvaddch(0, 0, '+');
        mvaddch(viewport.h + 1, 0, '+');
        mvaddch(0, viewport.w + 1, '+');
        mvaddch(viewport.h + 1, viewport.w + 1, '+');
        for(int i = 1; i < viewport.h + 1; ++i) {
            mvaddch(i, viewport.w + 1, '|');
            mvaddch(i, 0, '|');
        }
        for(int i = 1; i < viewport.w + 1; ++i) {
            mvaddch(0, i, '-');
            mvaddch(viewport.h + 1, i, '-');
        }
        // map
        int x = 1, y = 1;
        for(int j = viewport.y; j < viewport.h + viewport.y; ++j) {
            for(int i = viewport.x; i < viewport.w + viewport.x; ++i) {
                // TODO: evaluate usage of c-style casts
                if(j >= 0 && i >= 0 && j < static_cast<int>(map.size()) && i < static_cast<int>(map[j].size())) {
                    mvaddch(y, x, objects[j][i] == ' ' ? map[j][i] : objects[j][i]);
                } else {
                    mvaddch(y, x, ' ');
                }
                ++x;
            }
            ++y;
            x = 1;
        }
        // HUD
        mvprintw(viewport.h + 2, 1, message.c_str());

        mvprintw(viewport.h + 3, 60, ("Floor " + std::to_string(level)).c_str());
        mvprintw(viewport.h + 3, 1, ("- " + names[0] + " -").c_str());
        mvprintw(viewport.h + 4, 1, ("HP: " + std::to_string(stats[0].hp) + "/" + std::to_string(stats[0].max_hp)).c_str());
        mvprintw(viewport.h + 5, 1, ("ATK: " + std::to_string(stats[0].atk) + "\tDEF: " + std::to_string(stats[0].def)).c_str());

        if(stats[1].hp > 0) {
            mvprintw(viewport.h + 3, 30, ("- " + names[1] + " -").c_str());
            mvprintw(viewport.h + 4, 30, ("HP: " + std::to_string(stats[1].hp) + "/" + std::to_string(stats[1].max_hp)).c_str());
            mvprintw(viewport.h + 5, 30, ("ATK: " + std::to_string(stats[1].atk) + "\tDEF: " + std::to_string(stats[1].def)).c_str());
        }

        mvprintw(viewport.h + 6, 1, ("Coins: " + std::to_string(inventory.coins)).c_str());
        refresh();
    }
}
