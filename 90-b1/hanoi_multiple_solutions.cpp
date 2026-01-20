/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <iostream>
#include <iomanip>
#include <windows.h>
#include<conio.h>
#include"cmd_console_tools.h"
#include"hanoi.h"
using namespace std;
/* -----------------------------------------

	 本文件功能：
	1、放被 hanoi_main.cpp/hanoi_menu.cpp 中的各函数调用的菜单各项对应的执行函数

	 本文件要求：
	1、不允许定义外部全局变量（const及#define不在限制范围内）
	2、允许定义静态全局变量（具体需要的数量不要超过文档显示，全局变量的使用准则是：少用、慎用、能不用尽量不用）
	3、静态局部变量的数量不限制，但使用准则也是：少用、慎用、能不用尽量不用
	4、按需加入系统头文件、自定义头文件、命名空间等

   ----------------------------------------- */

int i;//记步数
int a[10];//a柱
int b[10];//b柱
int c[10];//c柱
int top_A, top_B, top_C;//三个栈顶指针
int speed;//速度


/*
移动盘子(x起始位置,y终止位置,栈顶指针定位移出的位置,用b[top_B]传入移动块的层数,层数对应相应的颜色,即第i层的颜色对应的color值为i)
根据from/to，top_from定位到某个柱的中间位置,获得这个位置上的色块颜色
左右分别擦除长度为移动块长度的色块
上移,擦除(直到上限位置)
分情况左移或右移,直到色块中间位置达到A/B/C
下移至top_to的位置
*/

//判断游戏是否结束
int Game_over(int n, char end)
{
	switch (end)
	{
		case 'A':
			if (top_A == n)
				return 1;
			break;
		case 'B':
			if (top_B == n)
				return 1;
			break;

		case 'C':
			if (top_C == n)
				return 1;
			break;
	}
	return 0;
}

void print_cross_stack(int show)
{
	if (show)
	{
		cout << "A:";
		if (a[0] != 10)
			cout << " ";
		for (int i = 0; i < 10; i++)
		{
			if (a[i] != 0)
			{
				if (a[i] != 10)
					cout << a[i] << " ";
				else
					cout << "10 ";
			}
			else
				cout << "  ";
		}
		cout << "B:";
		if (b[0] != 10)
			cout << " ";
		for (int i = 0; i < 10; i++)
		{

			if (b[i] != 0)
			{
				if (b[i] != 10)
					cout << b[i] << " ";
				else
					cout << "10 ";
			}
			else
				cout << "  ";
		}
		cout << "C:";
		if (c[0] != 10)
			cout << " ";
		for (int i = 0; i < 10; i++)
		{

			if (c[i] != 0)
			{
				if (c[i] != 10)
					cout << c[i] << " ";
				else
					cout << "10 ";
			}
			else
				cout << "  ";
		}
	}
}

//向上为1,向下为2,向左为3,向右为4
void clean_print(int color, int x, int y, int WASD)
{
	cct_showch(x - color, y, ' ', COLOR_BLACK, COLOR_BLACK, 2 * color + 1);//覆盖原有位置
	if (WASD == 1)
		y--;
	if (WASD == 2)
		y++;
	if (WASD == 3)
		x--;
	if (WASD == 4)
		x++;
	cct_showch(x - color, y, ' ', color, COLOR_BLACK, 2 * color + 1);//在下一个位置生成
	Sleep(50 - speed * 10);
	cct_setcolor(COLOR_BLACK, COLOR_WHITE);
}

//移动盘子
void move_plate(char start, char end, int top_from, int top_to, int change)
{
	int DEBUG = 0;
	const int Y_MAX = 2;//上移的上限位置
	const int X_A = 12;//A柱正中间
	const int X_B = 44;//B柱正中间
	const int X_C = 76;//C柱正中间
	int X, Y;//记录块的位置
	switch (start)
	{
		case 'A':
			if (end == 'B')
			{
				X = X_A;
				Y = 15 - top_from;
				while (Y > Y_MAX)//上移
				{
					clean_print(change, X_A, Y, 1);
					cct_showch(X_A, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y--;
				}
				while (X < X_B)//右移
				{
					clean_print(change, X, Y_MAX, 4);
					//cct_showch(0, Y_MAX, ' ', COLOR_BLACK, COLOR_BLACK, 80);
					X++;
				}
				while (Y < 14 - top_to)//下移
				{
					clean_print(change, X_B, Y, 2);
					if (DEBUG > 0)
						cct_showch(X_B, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y++;
					DEBUG++;
				}
			}
			if (end == 'C')
			{
				X = X_A;
				Y = 15 - top_from;
				while (Y > Y_MAX)//上移
				{
					clean_print(change, X_A, Y, 1);
					cct_showch(X_A, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y--;
				}
				while (X < X_C)//右移
				{
					clean_print(change, X, Y_MAX, 4);
					//cct_showch(0, Y_MAX, ' ', COLOR_BLACK, COLOR_BLACK, 80);
					X++;
				}
				while (Y < 14 - top_to)//下移
				{
					clean_print(change, X_C, Y, 2);
					if (DEBUG > 0)
						cct_showch(X_C, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y++;
					DEBUG++;
				}
			}
			break;
		case 'B':
			if (end == 'A')
			{
				X = X_B;
				Y = 15 - top_from;
				while (Y > Y_MAX)//上移
				{
					clean_print(change, X_B, Y, 1);
					cct_showch(X_B, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y--;
				}
				while (X > X_A)//左移
				{
					clean_print(change, X, Y_MAX, 3);
					//cct_showch(0, Y_MAX, ' ', COLOR_BLACK, COLOR_BLACK, 80);
					X--;
				}
				while (Y < 14 - top_to)//下移
				{
					clean_print(change, X_A, Y, 2);
					if (DEBUG > 0)
						cct_showch(X_A, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y++;
					DEBUG++;
				}
			}
			if (end == 'C')
			{
				X = X_B;
				Y = 15 - top_from;
				while (Y > Y_MAX)//上移
				{
					clean_print(change, X_B, Y, 1);
					cct_showch(X_B, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y--;
				}
				while (X < X_C)//右移
				{
					clean_print(change, X, Y_MAX, 4);
					//cct_showch(0, Y_MAX, ' ', COLOR_BLACK, COLOR_BLACK, 80);
					X++;
				}
				while (Y < 14 - top_to)//下移
				{
					clean_print(change, X_C, Y, 2);
					if (DEBUG > 0)
						cct_showch(X_C, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y++;
					DEBUG++;
				}
			}
			break;

		case 'C':
			if (end == 'A')
			{
				X = X_C;
				Y = 15 - top_from;
				while (Y > Y_MAX)//上移
				{
					clean_print(change, X_C, Y, 1);
					cct_showch(X_C, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y--;
				}
				while (X > X_A)//左移
				{
					clean_print(change, X, Y_MAX, 3);
					//cct_showch(0, Y_MAX, ' ', COLOR_BLACK, COLOR_BLACK, 80);
					X--;
				}
				while (Y < 14 - top_to)//下移
				{
					clean_print(change, X_A, Y, 2);
					if (DEBUG > 0)
						cct_showch(X_A, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y++;
					DEBUG++;
				}
			}
			if (end == 'B')
			{
				X = X_C;
				Y = 15 - top_from;
				while (Y > Y_MAX)//上移
				{
					clean_print(change, X_C, Y, 1);
					cct_showch(X_C, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y--;
				}
				while (X > X_B)//左移
				{
					clean_print(change, X, Y_MAX, 3);
					//cct_showch(0, Y_MAX, ' ', COLOR_BLACK, COLOR_BLACK, 80);
					X--;
				}
				while (Y < 14 - top_to)//下移
				{
					clean_print(change, X_B, Y, 2);
					if (DEBUG > 0)
						cct_showch(X_B, Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
					Y++;
					DEBUG++;
				}
			}
			break;
	}
	cct_setcolor(COLOR_BLACK, COLOR_WHITE);
	cct_gotoxy(0, 32);
}

//延时设置相关(Speed为0时,等待回车)
void delay(int SPEED)
{
	if (SPEED)
		Sleep(200 - SPEED * 40);
	else
	{
		while (_getch() != '\r')
			;
	}
}

//输出初始的数组内容
void print_start_group(int speed, int select)
{
	if (select == 4)
		cct_gotoxy(0, 15);
	if (select == 8)
		cct_gotoxy(0, 32);

	cout << "初始:                  ";
	cout << "A:";
	if (a[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{
		if (a[i] != 0)
		{
			if (a[i] != 10)
				cout << a[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << "B:";
	if (b[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{

		if (b[i] != 0)
		{
			if (b[i] != 10)
				cout << b[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << "C:";
	if (c[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{

		if (c[i] != 0)
		{
			if (c[i] != 10)
				cout << c[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	delay(speed);
}

//改变数组内容,输出内容
void stack(char x, char y, int show, bool moveplate, int n, int seven)
{
	void print_with_step(int, char, char);
	if (moveplate)
		cct_gotoxy(0, 32);

	//进栈,出栈
	int temp;
	switch (x)
	{
		case 'A':
			if (y == 'B')
			{
				if (moveplate)
					cct_gotoxy(0, 32);

				temp = a[--top_A];
				b[top_B] = temp;
				top_B++;
				a[top_A] = 0;
				if (!seven)
				{
					print_with_step(n, x, y);
					print_cross_stack(show);
				}
				if (moveplate && !seven)
					print_vertical(8);
				if (moveplate)
					move_plate(x, y, top_A + 1, top_B - 1, temp);

			}
			else if (y == 'C')
			{
				if (moveplate)
					cct_gotoxy(0, 32);

				temp = a[--top_A];
				c[top_C] = temp;
				top_C++;
				a[top_A] = 0;
				if (!seven)
				{
					print_with_step(n, x, y);
					print_cross_stack(show);
				}
				if (moveplate && !seven)
					print_vertical(8);
				if (moveplate)
					move_plate(x, y, top_A + 1, top_C - 1, temp);
			}
			break;
		case 'B':
			if (y == 'A')
			{
				if (moveplate)
					cct_gotoxy(0, 32);

				temp = b[--top_B];
				a[top_A] = temp;
				top_A++;
				b[top_B] = 0;
				if (!seven)
				{
					print_with_step(n, x, y);
					print_cross_stack(show);
				}
				if (moveplate && !seven)
					print_vertical(8);
				if (moveplate)
					move_plate(x, y, top_B + 1, top_A - 1, temp);
			}
			else if (y == 'C')
			{
				if (moveplate)
					cct_gotoxy(0, 32);

				temp = b[--top_B];
				c[top_C] = temp;
				top_C++;
				b[top_B] = 0;
				if (!seven)
				{
					print_with_step(n, x, y);
					print_cross_stack(show);
				}
				if (moveplate && !seven)
					print_vertical(8);
				if (moveplate)
					move_plate(x, y, top_B + 1, top_C - 1, temp);
			}
			break;
		case 'C':
			if (y == 'A')
			{
				if (moveplate)
					cct_gotoxy(0, 32);

				temp = c[--top_C];
				a[top_A] = temp;
				top_A++;
				c[top_C] = 0;
				if (!seven)
				{
					print_with_step(n, x, y);
					print_cross_stack(show);
				}
				if (moveplate && !seven)
					print_vertical(8);
				if (moveplate)
					move_plate(x, y, top_C + 1, top_A - 1, temp);
			}
			else if (y == 'B')
			{
				if (moveplate)
					cct_gotoxy(0, 32);

				temp = c[--top_C];
				b[top_B] = temp;
				top_B++;
				c[top_C] = 0;
				if (!seven)
				{
					print_with_step(n, x, y);
					print_cross_stack(show);
				}
				if (moveplate && !seven)
					print_vertical(8);
				if (moveplate)
					move_plate(x, y, top_C + 1, top_B - 1, temp);
			}
			break;
	}
}


//判断源柱是否为空
int column_empty(int input)
{
	switch (input)
	{
		case 1:
		case 2:
			if (top_A == 0)
				return 0;
			break;
		case 3:
		case 4:
			if (top_B == 0)
				return 0;
			break;
		case 5:
		case 6:
			if (top_C == 0)
				return 0;
			break;
	}
	return 1;
}

//判断是否大盘压小盘
int large_over_small(int input)
{
	switch (input)
	{
		case 1:
			if (top_B == 0)
				return 1;
			if (a[top_A - 1] > b[top_B - 1])
				return 0;
			else
				return 1;
			break;
		case 2:
			if (top_C == 0)
				return 1;
			if (a[top_A - 1] > c[top_C - 1])
				return 0;
			else
				return 1;
			break;
		case 3:
			if (top_A == 0)
				return 1;
			if (b[top_B - 1] > a[top_A - 1])
				return 0;
			else
				return 1;
			break;
		case 4:
			if (top_C == 0)
				return 1;
			if (b[top_B - 1] > c[top_C - 1])
				return 0;
			else
				return 1;
			break;
		case 5:
			if (top_A == 0)
				return 1;
			if (c[top_C - 1] > a[top_A - 1])
				return 0;
			else
				return 1;
			break;
		case 6:
			if (top_B == 0)
				return 1;
			if (c[top_C - 1] > b[top_B - 1])
				return 0;
			else
				return 1;
			break;
	}
	return -1;
}

//初始化数组
void spawn(char start, int level)
{
	for (int i = 0; i < 10; i++)
	{
		a[i] = b[i] = c[i] = 0;//初始化
	}

	top_A = top_B = top_C = 0;
	switch (start)//开始的数组状态
	{
		case 'A':
		{
			while (top_A < level)
			{
				a[top_A] = level - top_A;
				top_A++;
			}
			break;
		}
		case 'B':
		{
			while (top_B < level)
			{
				b[top_B] = level - top_B;
				top_B++;
			}
			break;
		}
		case 'C':
		{
			while (top_C < level)
			{
				c[top_C] = level - top_C;
				top_C++;
			}
			break;
		}
	}

}

//情况1(基本解)的输出
void print_1(int n, char x, char y)
{
	cout << n << "# ";
	cout << x << "--->" << y << " " << endl;
}

//带步数的输出(情况2)
void print_with_step(int n, char x, char y)
{
	i++;
	cout << "第" << setw(4) << i << " 步";
	cout << "(" << setw(2) << n << "#: ";
	cout << x << "-->" << y << ")   ";
}

//情况3,4,8的输出
void print_cross(int n, char x, char y, int select)
{
	if (select == 3)
	{
		stack(x, y, 1, 0, n, 0);
		cout << endl;
	}
	if (select == 4)
	{
		cct_gotoxy(0, 15);

		stack(x, y, 1, 0, n, 0);

		print_vertical(4);
		delay(speed);
		cct_gotoxy(0, 20);
	}
	if (select == 7)
	{
		cct_gotoxy(0, 32);
		stack(x, y, 0, 1, n, 1);

	}
	if (select == 8)
	{
		cct_gotoxy(0, 32);
		stack(x, y, 1, 1, n, 0);
		delay(speed);
	}
	if (select == 9)
	{
		cct_gotoxy(0, 32);
		stack(x, y, 1, 1, n, 0);
	}
}

//竖向打印(4,8共用)
void print_vertical(int select)
{
	int x = 10;
	int y = 11;
	if (select == 8)
		y += 15;
	cct_gotoxy(x, y);//A柱
	for (int j = 0; j < 10; j++)
	{
		if (a[j] == 10)
		{
			cout << "10";
			cct_gotoxy(x, --y);
		}
		else if (a[j] != 0)
		{
			cout << " " << a[j];
			cct_gotoxy(x, --y);
		}
		else
		{
			cout << "  ";
			cct_gotoxy(x, --y);
		}
	}

	x = 20, y = 11;//B柱
	if (select == 8)
		y += 15;

	cct_gotoxy(x, y);
	for (int j = 0; j < 10; j++)
	{
		if (b[j] == 10)
		{
			cout << "10";
			cct_gotoxy(x, --y);
		}
		else if (b[j] != 0)
		{
			cout << " " << b[j];
			cct_gotoxy(x, --y);
		}
		else
		{
			cout << "  ";
			cct_gotoxy(x, --y);
		}
	}

	x = 30, y = 11;//C柱
	if (select == 8)
		y += 15;
	cct_gotoxy(x, y);
	for (int j = 0; j < 10; j++)
	{
		if (c[j] == 10)
		{
			cout << "10";
			cct_gotoxy(x, --y);
		}
		else if (c[j] != 0)
		{
			cout << " " << c[j];
			cct_gotoxy(x, --y);
		}
		else
		{
			cout << "  ";
			cct_gotoxy(x, --y);
		}
	}

	cct_gotoxy(90, 15);
}

//竖向打印背景(4,8共用)
void print_vertical_menu(int select)
{
	if (select == 4)
	{
		cct_gotoxy(9, 12);
		cout << "=========================";
		cct_gotoxy(9, 13);
		cout << "  A         B         C  ";
	}
	if (select == 8)
	{
		{
			cct_gotoxy(9, 27);
			cout << "=========================";
			cct_gotoxy(9, 28);
			cout << "  A         B         C  ";
		}
	}
}

//移动(根据select进行分类)
void move(int n, char x, char y, int select)
{
	switch (select)
	{
		case 1:
			print_1(n, x, y);
			break;
		case 2:
			print_with_step(n, x, y);
			cout << endl;
			break;
		case 3://3,4共用
		case 4:
			print_cross(n, x, y, select);
			break;
		case 5://5,6,7都不输出内部数组
		case 6:
			break;
		case 7:
			print_cross(n, x, y, select);
			break;
		case 8:
			print_cross(n, x, y, select);
			break;

		default:
			break;
	}
}

//汉诺塔递归函数
void hanoi(int n, char src, char tmp, char dst, int select)
{
	if (n == 1)
		move(n, src, dst, select);
	else
	{
		hanoi(n - 1, src, dst, tmp, select);
		move(n, src, dst, select);
		hanoi(n - 1, tmp, src, dst, select);
	}
}