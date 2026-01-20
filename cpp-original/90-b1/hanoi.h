/*2052526 信15 白俊豪*/
#pragma once

/* -----------------------------------------

	 本文件功能：
	1、为了保证 hanoi_main.cpp/hanoi_menu.cpp/hanoi_multiple_solutions.cpp 能相互访问函数的函数声明
	2、一个以上的cpp中用到的宏定义（#define）或全局只读（const）变量，个数不限
	3、可以参考 cmd_console_tools.h 的写法（认真阅读并体会）
   ----------------------------------------- */

extern int speed;//速度
extern int i;//步数
extern int top_A;//A柱的栈顶指针
extern int top_B;//B柱的栈顶指针
extern int top_C;//C柱的栈顶指针
extern int a[10];//A柱
extern int b[10];//B柱
extern int c[10];//C柱



/*菜单相关*/
void wait_for_enter();
void print_menu();
void menu(int n, char start, char mid, char end, int select);

/*汉诺塔的主体*/
void hanoi(int n, char src, char tmp, char dst, int select);
void move(int n, char x, char y, int select);
void spawn(char start, int level);

/*汉诺塔的输出相关*/
void print_vertical_menu(int select);
void print_vertical(int select);
void print_cross_stack(int show);
void print_start_group(int, int select);
void print_cross(int n, char x, char y, int select);

/*判断游戏是否结束，操作是否违规*/
int column_empty(int input);
int large_over_small(int input);
int Game_over(int n, char end);

/*获取输入的参数*/
void get_choice(int* select, char* start, char* mid, char* end, int* level);
int get_speed();
int get_level();
char get_start();
char get_end(char start);