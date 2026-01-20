// 4-b4: Console cursor movement game with IJKL/arrow keys
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用crossterm跨平台终端库，替代Windows特定的API
// 2. 使用enum表示移动方向和边界模式，类型安全
// 3. 使用struct封装游戏状态，避免全局变量和魔法数字
// 4. 提取边界检查逻辑为独立函数，支持停止和回绕两种模式
// 5. 使用match表达式处理按键，代码更清晰
// 6. 使用Result类型处理IO错误，更健壮
// 7. 消除unsafe代码，使用安全的Rust API

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use rand::Rng;
use std::io::{self, Write};

/// 游戏区域边界
const MAX_X: u16 = 69;
const MAX_Y: u16 = 17;

/// 边界处理模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundaryMode {
    Stop, // 边界停止
    Wrap, // 边界回绕
}

/// 控制模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlMode {
    IJKL,  // 使用IJKL键
    Arrow, // 使用方向键
}

/// 游戏状态
#[derive(Debug)]
struct GameState {
    x: u16,
    y: u16,
    boundary_mode: BoundaryMode,
    control_mode: ControlMode,
}

impl GameState {
    /// 创建新的游戏状态
    fn new(boundary_mode: BoundaryMode, control_mode: ControlMode) -> Self {
        Self {
            x: 34, // 初始位置在中心
            y: 8,
            boundary_mode,
            control_mode,
        }
    }

    /// 向上移动
    fn move_up(&mut self) {
        self.y = match self.boundary_mode {
            BoundaryMode::Stop => self.y.saturating_sub(1).max(1),
            BoundaryMode::Wrap => {
                if self.y > 1 {
                    self.y - 1
                } else {
                    MAX_Y
                }
            }
        };
    }

    /// 向下移动
    fn move_down(&mut self) {
        self.y = match self.boundary_mode {
            BoundaryMode::Stop => (self.y + 1).min(MAX_Y),
            BoundaryMode::Wrap => {
                if self.y < MAX_Y {
                    self.y + 1
                } else {
                    1
                }
            }
        };
    }

    /// 向左移动
    fn move_left(&mut self) {
        self.x = match self.boundary_mode {
            BoundaryMode::Stop => self.x.saturating_sub(1).max(1),
            BoundaryMode::Wrap => {
                if self.x > 1 {
                    self.x - 1
                } else {
                    MAX_X
                }
            }
        };
    }

    /// 向右移动
    fn move_right(&mut self) {
        self.x = match self.boundary_mode {
            BoundaryMode::Stop => (self.x + 1).min(MAX_X),
            BoundaryMode::Wrap => {
                if self.x < MAX_X {
                    self.x + 1
                } else {
                    1
                }
            }
        };
    }
}

/// 初始化边框和随机字符
fn init_border() -> io::Result<()> {
    let mut stdout = io::stdout();

    // 清屏
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // 绘制顶部边框
    queue!(stdout, MoveTo(0, 0), Print("*".repeat(71)))?;

    // 绘制中间行
    for y in 1..=MAX_Y {
        queue!(
            stdout,
            MoveTo(0, y),
            Print("*"),
            MoveTo(MAX_X + 1, y),
            Print("*")
        )?;
    }

    // 绘制底部边框
    queue!(stdout, MoveTo(0, MAX_Y + 1), Print("*".repeat(71)))?;

    // 随机显示20个大写字母
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(1..=MAX_X);
        let y = rng.gen_range(1..=MAX_Y);
        let ch = (b'A' + rng.gen_range(0..26)) as char;
        queue!(stdout, MoveTo(x, y), Print(ch))?;
    }

    stdout.flush()?;
    Ok(())
}

/// 运行游戏主循环
fn run_game(mut state: GameState) -> io::Result<()> {
    let mut stdout = io::stdout();

    loop {
        // 移动光标到当前位置
        execute!(stdout, MoveTo(state.x, state.y))?;

        // 读取按键事件
        if let Event::Key(key_event) = event::read()? {
            match state.control_mode {
                ControlMode::IJKL => {
                    if handle_ijkl_key(&mut state, key_event) {
                        break; // 退出游戏
                    }
                }
                ControlMode::Arrow => {
                    if handle_arrow_key(&mut state, key_event) {
                        break; // 退出游戏
                    }
                }
            }
        }
    }

    Ok(())
}

/// 处理IJKL按键
///
/// # Returns
/// * `true` - 退出游戏
/// * `false` - 继续游戏
fn handle_ijkl_key(state: &mut GameState, key_event: KeyEvent) -> bool {
    let mut stdout = io::stdout();

    match key_event.code {
        KeyCode::Char('i') | KeyCode::Char('I') => state.move_up(),
        KeyCode::Char('k') | KeyCode::Char('K') => state.move_down(),
        KeyCode::Char('j') | KeyCode::Char('J') => state.move_left(),
        KeyCode::Char('l') | KeyCode::Char('L') => state.move_right(),
        KeyCode::Char(' ') => {
            // 空格键清除当前位置的字符
            let _ = execute!(stdout, Print(' '), MoveTo(state.x, state.y));
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return true,
        _ => {}
    }

    false
}

/// 处理方向键
///
/// # Returns
/// * `true` - 退出游戏
/// * `false` - 继续游戏
fn handle_arrow_key(state: &mut GameState, key_event: KeyEvent) -> bool {
    let mut stdout = io::stdout();

    match key_event.code {
        KeyCode::Up => state.move_up(),
        KeyCode::Down => state.move_down(),
        KeyCode::Left => state.move_left(),
        KeyCode::Right => state.move_right(),
        KeyCode::Char(' ') => {
            // 空格键清除当前位置的字符
            let _ = execute!(stdout, Print(' '), MoveTo(state.x, state.y));
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return true,
        _ => {}
    }

    false
}

/// 显示菜单并获取用户选择
fn show_menu() -> io::Result<Option<u8>> {
    let mut stdout = io::stdout();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    println!("1.用I、J、K、L键控制上下左右(大小写均可，边界停止)");
    println!("2.用I、J、K、L键控制上下左右(大小写均可，边界回绕)");
    println!("3.用箭头键控制上下左右，边界停止");
    println!("4.用箭头键控制上下左右，边界回绕");
    println!("0.退出");
    print!("[请选择0-4]");
    stdout.flush()?;

    // 读取按键
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('0') => return Ok(Some(0)),
                KeyCode::Char('1') => return Ok(Some(1)),
                KeyCode::Char('2') => return Ok(Some(2)),
                KeyCode::Char('3') => return Ok(Some(3)),
                KeyCode::Char('4') => return Ok(Some(4)),
                _ => continue,
            }
        }
    }
}

/// 等待回车键返回菜单
fn wait_for_enter() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, MoveTo(0, 23))?;
    print!("游戏结束,请按回车键返回菜单");
    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // 启用原始模式
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;

    let result = run_main_loop();

    // 恢复终端状态
    execute!(stdout, Show)?;
    terminal::disable_raw_mode()?;

    result
}

fn run_main_loop() -> io::Result<()> {
    loop {
        match show_menu()? {
            Some(0) => break, // 退出
            Some(1) => {
                // IJKL键，边界停止
                init_border()?;
                let state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
                run_game(state)?;
                wait_for_enter()?;
            }
            Some(2) => {
                // IJKL键，边界回绕
                init_border()?;
                let state = GameState::new(BoundaryMode::Wrap, ControlMode::IJKL);
                run_game(state)?;
                wait_for_enter()?;
            }
            Some(3) => {
                // 方向键，边界停止
                init_border()?;
                let state = GameState::new(BoundaryMode::Stop, ControlMode::Arrow);
                run_game(state)?;
                wait_for_enter()?;
            }
            Some(4) => {
                // 方向键，边界回绕
                init_border()?;
                let state = GameState::new(BoundaryMode::Wrap, ControlMode::Arrow);
                run_game(state)?;
                wait_for_enter()?;
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_new() {
        let state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        assert_eq!(state.x, 34);
        assert_eq!(state.y, 8);
        assert_eq!(state.boundary_mode, BoundaryMode::Stop);
        assert_eq!(state.control_mode, ControlMode::IJKL);
    }

    #[test]
    fn test_move_up_stop_mode() {
        let mut state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        state.y = 5;

        state.move_up();
        assert_eq!(state.y, 4);

        // 测试边界停止
        state.y = 1;
        state.move_up();
        assert_eq!(state.y, 1); // 不应该小于1
    }

    #[test]
    fn test_move_up_wrap_mode() {
        let mut state = GameState::new(BoundaryMode::Wrap, ControlMode::IJKL);
        state.y = 5;

        state.move_up();
        assert_eq!(state.y, 4);

        // 测试边界回绕
        state.y = 1;
        state.move_up();
        assert_eq!(state.y, MAX_Y); // 应该回绕到底部
    }

    #[test]
    fn test_move_down_stop_mode() {
        let mut state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        state.y = 5;

        state.move_down();
        assert_eq!(state.y, 6);

        // 测试边界停止
        state.y = MAX_Y;
        state.move_down();
        assert_eq!(state.y, MAX_Y); // 不应该超过MAX_Y
    }

    #[test]
    fn test_move_down_wrap_mode() {
        let mut state = GameState::new(BoundaryMode::Wrap, ControlMode::IJKL);
        state.y = 5;

        state.move_down();
        assert_eq!(state.y, 6);

        // 测试边界回绕
        state.y = MAX_Y;
        state.move_down();
        assert_eq!(state.y, 1); // 应该回绕到顶部
    }

    #[test]
    fn test_move_left_stop_mode() {
        let mut state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        state.x = 5;

        state.move_left();
        assert_eq!(state.x, 4);

        // 测试边界停止
        state.x = 1;
        state.move_left();
        assert_eq!(state.x, 1); // 不应该小于1
    }

    #[test]
    fn test_move_left_wrap_mode() {
        let mut state = GameState::new(BoundaryMode::Wrap, ControlMode::IJKL);
        state.x = 5;

        state.move_left();
        assert_eq!(state.x, 4);

        // 测试边界回绕
        state.x = 1;
        state.move_left();
        assert_eq!(state.x, MAX_X); // 应该回绕到右边
    }

    #[test]
    fn test_move_right_stop_mode() {
        let mut state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        state.x = 5;

        state.move_right();
        assert_eq!(state.x, 6);

        // 测试边界停止
        state.x = MAX_X;
        state.move_right();
        assert_eq!(state.x, MAX_X); // 不应该超过MAX_X
    }

    #[test]
    fn test_move_right_wrap_mode() {
        let mut state = GameState::new(BoundaryMode::Wrap, ControlMode::IJKL);
        state.x = 5;

        state.move_right();
        assert_eq!(state.x, 6);

        // 测试边界回绕
        state.x = MAX_X;
        state.move_right();
        assert_eq!(state.x, 1); // 应该回绕到左边
    }

    #[test]
    fn test_boundary_modes() {
        // 测试两种边界模式的区别
        let mut stop_state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        let mut wrap_state = GameState::new(BoundaryMode::Wrap, ControlMode::IJKL);

        stop_state.x = 1;
        wrap_state.x = 1;

        stop_state.move_left();
        wrap_state.move_left();

        assert_eq!(stop_state.x, 1); // 停止模式保持在边界
        assert_eq!(wrap_state.x, MAX_X); // 回绕模式跳到另一边
    }

    #[test]
    fn test_control_modes() {
        // 测试两种控制模式
        let ijkl_state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        let arrow_state = GameState::new(BoundaryMode::Stop, ControlMode::Arrow);

        assert_eq!(ijkl_state.control_mode, ControlMode::IJKL);
        assert_eq!(arrow_state.control_mode, ControlMode::Arrow);
    }

    #[test]
    fn test_multiple_moves() {
        let mut state = GameState::new(BoundaryMode::Stop, ControlMode::IJKL);
        state.x = 10;
        state.y = 10;

        // 测试连续移动
        state.move_right();
        state.move_right();
        state.move_down();
        state.move_down();

        assert_eq!(state.x, 12);
        assert_eq!(state.y, 12);
    }
}
