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
        case Game::UpdateType::MapChange:
            map = update.map->to_string();
            objects = update.map->object_string();
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
            return Game::Command{ Game::CommandType::Move, Game::Direction::Up };
        case KEY_DOWN:
        case 'S':
        case 's':
            return Game::Command{ Game::CommandType::Move, Game::Direction::Down };
        case KEY_RIGHT:
        case 'D':
        case 'd':
            return Game::Command{ Game::CommandType::Move, Game::Direction::Right };
        case KEY_LEFT:
        case 'A':
        case 'a':
            return Game::Command{ Game::CommandType::Move, Game::Direction::Left };
        default:
            return { Game::CommandType::Quit };
        }
    }

    Game::Command Default::command() {
        return state();
    }
    void Default::redraw() {
        // TODO: draw a nice border, HUD, and restrict the view port
        int x = 0, y = 0;
        for(unsigned int c = 0; c < map.length(); ++c) {
            if(map[c] == '\n') {
                x = 0;
                ++y;
            } else {
                mvaddch(y, x, objects[c] == ' ' ? map[c] : objects[c]);
                ++x;
            }
        }
        refresh();
    }
}
