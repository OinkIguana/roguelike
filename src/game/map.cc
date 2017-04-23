#include "map.h"
#include "cell.h"
#include "room.h"

#include <functional>
#include <cmath>
#include <algorithm>
#include <random>
#include <numeric>
#include <iostream>

std::default_random_engine rng;

// flood fills a continuous section of a grid[y][x]-like data structure with v
//  given a predicate
template<typename T>
void flood_fill(std::vector<std::vector<T>>& grid, unsigned int x, unsigned int y, T v, std::function<bool(int, int)> pred) {
    if(pred(x, y)) {
        grid[y][x] = v;
        if(y > 0)
            flood_fill(grid, x, y - 1, v, pred);
        if(y + 1 < grid.size())
            flood_fill(grid, x, y + 1, v, pred);
        if(x > 0)
            flood_fill(grid, x - 1, y, v, pred);
        if(x + 1 < grid[y].size())
            flood_fill(grid, x + 1, y, v, pred);
    }
}

const int MIN_ROOMS = 5;
const int MAX_ROOMS = 30;

namespace Game {
    Map::Map(int level) {
        rng.seed(level);
        struct Rectangle { int x, y, w, h; };
        struct Point { int x, y; };

        const int rm_count = std::min(MIN_ROOMS + level / 2, MAX_ROOMS);
        h = 20 + 2 * level;
        w = std::round(1.618 * h * 2);

        // create all the cells
        cells = std::vector<std::vector<std::shared_ptr<Cell>>>(h, std::vector<std::shared_ptr<Cell>>(w));
        for(unsigned int y = 0; y < cells.size(); ++y) {
            for(unsigned int x = 0; x < cells[y].size(); ++x) {
                cells[y][x] = std::make_shared<Cell>(x, y);
            }
        }

        // distributions for room generation
        std::uniform_int_distribution<int> rx(0, w - 1);
        std::uniform_int_distribution<int> ry(0, w - 1);
        std::poisson_distribution<int> rw(10);
        std::poisson_distribution<int> rh(5);
        std::bernoulli_distribution rcoll(0.95);

        // generate rooms
        auto rm_boxes = std::vector<Rectangle>(rm_count);
        for(int i = 0; i < rm_count; ++i) {
            int xx, yy, ww, hh;
            bool collides = false;
            do {
                xx = rx(rng);
                yy = ry(rng);
                ww = rw(rng) + 2;
                hh = rh(rng) + 2;
                // only allow collisions sometimes
                collides = rcoll(rng) && any_of(rm_boxes.begin(), rm_boxes.begin() + i, [xx, yy, ww, hh] (Rectangle rm) {
                    return
                        (rm.x + rm.w >= xx && rm.x < xx + ww) &&
                        (rm.y + rm.h >= yy && rm.y < yy + hh);
                });
                // never allow going out of the dungeon area
                if(!collides && (xx + ww > w || yy + hh > h)) { collides = true; }
            } while(collides);
            rm_boxes.emplace_back(Rectangle{ xx, yy, ww, hh });
        }

        // set cell types
        for(auto box : rm_boxes) {
            for(int y = box.y; y < box.y + box.h; ++y) {
                for(int x = box.x; x < box.x + box.w; ++x) {
                    if(cells[y][x]->type != Cell::Type::Room) {
                        cells[y][x]->type = x == box.x || x == box.x + box.w - 1
                            ? Cell::Type::WallV
                            : y == box.y || y == box.y + box.h - 1
                                ? Cell::Type::WallH
                                : Cell::Type::Room;
                    }
                }
            }
        }
        // build cell graph
        std::vector<std::vector<int>> cl_graph(h, std::vector<int>(w, 0));
        int r = 1;
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                if(cl_graph[y][x] == 0 && cells[y][x]->type == Cell::Type::Room) {
                    flood_fill(cl_graph, x, y, r++, [&cl_graph, this] (int x, int y) { return cl_graph[y][x] == 0 && cells[y][x]->type == Cell::Type::Room; });
                }
            }
        }
        // figure out which cells are in each room
        std::vector<std::vector<Point>> rm_cells(r);
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                if(cl_graph[y][x] != 0) {
                    rm_cells[cl_graph[y][x] - 1].emplace_back(Point{ x, y });
                }
            }
        }
        // assign Cells to Rooms
        rooms = std::vector<std::shared_ptr<Room>>(r);
        std::transform(rm_cells.begin(), rm_cells.end(), rooms.begin(), [this] (std::vector<Point> points) {
            std::vector<std::shared_ptr<Cell>> cls(points.size());
            std::transform(points.begin(), points.end(), cls.begin(), [this] (Point point) { return cells[point.y][point.x]; });
            return std::make_shared<Room>(cls);
        });
    }

    std::string Map::to_string() const {
        std::string x = std::accumulate(cells.begin(), cells.end(), std::string(), [](std::string a, auto row) {
            return a + std::accumulate(row.begin(), row.end(), std::string(), [](std::string a, auto cell) {
                return a + static_cast<char>(cell->type);
            }) + "\n";
        });
        std::cerr << x;
        return x;
    }
}
