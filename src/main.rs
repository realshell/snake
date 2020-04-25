use libc::usleep;
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;
use ncurses::*;
use rand::Rng;

struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

#[derive(PartialEq)]
enum SnakeDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    dir: SnakeDir,
    snake: Vec<Point>,
}

struct Food {
    x: i32,
    y: i32,
}

fn move_snake(snake: &mut Snake, food: &mut Food) -> Result<(), String> {
    match hit_the_wall(&snake) {
        true => return Err(String::from("hit the walll")),
        false => {}
    }
    match crash_body(snake) {
        true => return Err(String::from("you crash body")),
        false => {}
    }
    match get_food(snake, food) {
        true => {
            snake.snake.insert(0, next_head(snake));
            *food = create_food(&snake);
        }
        false => {
            let tail = snake.snake.pop().unwrap();
            mvaddch(tail.y, tail.x, ' ' as u32);
            snake.snake.insert(0, next_head(snake));
        }
    }
    Ok(())
}

fn hit_the_wall(snake: &Snake) -> bool {
    let head = snake.snake.get(0).unwrap();
    match snake.dir {
        SnakeDir::DOWN => {
            if head.y == 1 {
                return true;
            }
        }
        SnakeDir::UP => {
            if head.y == LINES() - 2 {
                return true;
            }
        }
        SnakeDir::LEFT => {
            if head.x == 1 {
                return true;
            }
        }
        SnakeDir::RIGHT => {
            if head.x == COLS() - 2 {
                return true;
            }
        }
    }
    false
}

fn next_head(snake: &Snake) -> Point {
    let head = &snake.snake[0];
    return match snake.dir {
        SnakeDir::UP => Point {
            x: head.x,
            y: head.y + 1,
        },
        SnakeDir::DOWN => Point {
            x: head.x,
            y: head.y - 1,
        },
        SnakeDir::LEFT => Point {
            x: head.x - 1,
            y: head.y,
        },
        SnakeDir::RIGHT => Point {
            x: head.x + 1,
            y: head.y,
        },
    };
}

fn add_snake(snake: &Snake) {
    for e in &snake.snake {
        mvaddch(e.y, e.x, 'O' as u32);
    }
}

fn crash_body(snake: &Snake) -> bool {
    return snake.snake.contains(&next_head(snake));
}

fn create_food(snake: &Snake) -> Food {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1, COLS() - 1);
    let y = rng.gen_range(1, LINES() - 1);
    for e in &snake.snake {
        if e.x == x && e.y == y {
            return create_food(&snake);
        }
    }
    return Food { x: x, y: y };
}

fn get_food(snake: &Snake, food: &Food) -> bool {
    let head = &snake.snake[0];
    return head.x == food.x && head.y == food.y;
}

fn add_food(food: &Food) {
    mvaddch(food.y, food.x, 'O' as u32);
}

fn add_wall() {
    for i in 1..(COLS() - 1) {
        mvaddch(0, i, ACS_HLINE());
    }
    for i in 1..(LINES() - 1) {
        mvaddch(i, COLS() - 1, ACS_VLINE());
    }
    for i in 1..(COLS() - 1) {
        mvaddch(LINES() - 1, i, ACS_HLINE());
    }
    for i in 1..(LINES() - 1) {
        mvaddch(i, 0, ACS_VLINE());
    }
    mvaddch(0, 0, ACS_ULCORNER());
    mvaddch(0, COLS() - 1, ACS_URCORNER());
    mvaddch(LINES() - 1, COLS() - 1, ACS_LRCORNER());
    mvaddch(LINES() - 1, 0, ACS_LLCORNER());
}

fn hit_key(snake: &mut Snake) {
    match getch() {
        KEY_UP => {
            if snake.dir == SnakeDir::LEFT || snake.dir == SnakeDir::RIGHT {
                snake.dir = SnakeDir::DOWN
            }
        }
        KEY_DOWN => {
            if snake.dir == SnakeDir::LEFT || snake.dir == SnakeDir::RIGHT {
                snake.dir = SnakeDir::UP
            }
        }
        KEY_LEFT => {
            if snake.dir == SnakeDir::UP || snake.dir == SnakeDir::DOWN {
                snake.dir = SnakeDir::LEFT
            }
        }
        KEY_RIGHT => {
            if snake.dir == SnakeDir::UP || snake.dir == SnakeDir::DOWN {
                snake.dir = SnakeDir::RIGHT
            }
        }
        _ => {}
    };
}

fn win_size_change(snake: &Snake, food: &mut Food) -> Result<(), String> {
    unsafe {
        OLD_WIN_W = WIN_W;
        OLD_WIN_H = WIN_H;
        WIN_W = COLS();
        WIN_H = LINES();
        if WIN_W != OLD_WIN_W || OLD_WIN_H != WIN_H {
            clear();
        }
        if food.x < 1 || food.x > COLS() - 2 || food.y < 1 || food.y > LINES() - 2 {
            *food = create_food(&snake);
            clear();
        }
        for p in &snake.snake {
            if p.x < 0 || p.x > COLS() - 1 || p.y <01 || p.y > LINES() - 1 {
                endwin();
                return Err(String::from("snake is lost"));
            }
        }
    }
    Ok(())
}

static mut WIN_W: i32 = 0;
static mut WIN_H: i32 = 0;
static mut OLD_WIN_W: i32 = 0;
static mut OLD_WIN_H: i32 = 0;

fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_INVISIBLE);
    nodelay(stdscr(), true);
    leaveok(stdscr(), true);
    scrollok(stdscr(), false);
    let mut snake = Snake {
        dir: SnakeDir::RIGHT,
        snake: vec![Point { x: 3, y: 3 }, Point { x: 2, y: 3 }],
    };
    let mut food = create_food(&snake);
    loop {
        hit_key(&mut snake);
        match move_snake(&mut snake, &mut food) {
            Ok(()) => {}
            Err(info) => {
                endwin();
                println!("{}\nGame Over", info);
                return;
            }
        }
        match win_size_change(&snake, &mut food) {
            Ok(()) => {}
            Err(info) => {
                endwin();
                println!("{}\nGame Over", info);
                return;
            }
        }
        add_wall();
        add_snake(&snake);
        add_food(&food);
        refresh();
        match snake.dir {
            SnakeDir::UP => unsafe { usleep(400000) },
            SnakeDir::DOWN => unsafe { usleep(400000) },
            SnakeDir::LEFT => unsafe { usleep(200000) },
            SnakeDir::RIGHT => unsafe { usleep(200000) },
        };
    }
}
