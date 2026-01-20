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
	2、不允许定义静态全局变量（全局变量的使用准则是：少用、慎用、能不用尽量不用）
	3、静态局部变量的数量不限制，但使用准则也是：少用、慎用、能不用尽量不用
	4、按需加入系统头文件、自定义头文件、命名空间等

   ----------------------------------------- */

//打印长度为 (层数*2+1) 的颜色不同的盘(第i层对应的颜色是i)
void print_plate(int n, int x, int y)
{
	int color = n;
	for (int j = n; j > 0; j--)
	{
		cct_showch(x - j, y--, ' ', color--, COLOR_BLACK, 2 * j + 1);
		Sleep(50);
	}
	cct_setcolor(COLOR_BLACK, COLOR_WHITE);
}

//打印 1-10 个盘
void print_plates(int n, char start)
{
	int X_A = 12;
	int X_B = 44;
	int X_C = 76;
	int Y = 14;
	switch (start)
	{
		case 'A':
			print_plate(n, X_A, Y);
			break;
		case 'B':
			print_plate(n, X_B, Y);
			break;
		case 'C':
			print_plate(n, X_C, Y);
			break;

	}
}

//打印三根柱子（横向）
void print_pillar_cross()
{
	int START_X = 1;
	int START_Y = 15;

	cct_gotoxy(START_X, START_Y);
	cct_showch(START_X, START_Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 23);
	Sleep(50);
	START_X += 23;
	cct_showch(START_X, START_Y, ' ', COLOR_BLACK, COLOR_BLACK, 9);
	Sleep(50);
	START_X += 9;
	cct_showch(START_X, START_Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 23);
	Sleep(50);
	START_X += 23;
	cct_showch(START_X, START_Y, ' ', COLOR_BLACK, COLOR_BLACK, 9);
	Sleep(50);
	START_X += 9;
	cct_showch(START_X, START_Y, ' ', COLOR_HYELLOW, COLOR_BLACK, 23);
}

//打印三根柱子(竖向)
void print_pillar_vertical()
{
	const int Y_MAX = 3;//上限位置
	const int X_A = 12;
	const int X_B = 44;
	const int X_C = 76;
	for (int y = 15; y >= Y_MAX; y--)
	{
		cct_showch(X_A, y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
		Sleep(50);
		cct_showch(X_B, y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
		Sleep(50);
		cct_showch(X_C, y, ' ', COLOR_HYELLOW, COLOR_BLACK, 1);
		Sleep(50);
	}
}

//打印三根柱子
void print_pillar()
{
	cct_setcursor(CURSOR_INVISIBLE);
	print_pillar_cross();
	print_pillar_vertical();
	cct_setcursor(CURSOR_VISIBLE_NORMAL);
	cct_setcolor(COLOR_BLACK, COLOR_WHITE);
	cct_gotoxy(0, 30);
}

//打印汉诺塔的设置参数
void print_start(char start, char end, int level, bool show)
{
	cout << "从 " << start << " 移动到 " << end << " , 共 " << level << " 层";
	if (show)
		cout << ", 延时设置为 " << speed;
}

//等待输入回车,放在主函数循环的最后，回车返回菜单开头
void wait_for_enter()
{
	cout << endl << "按回车键继续";
	while (_getch() != '\r')
		;
	cout << endl << endl;
	cct_cls();
}

//打印初始菜单
void print_menu()
{
	cout << "---------------------------------" << endl;
	cout << "1.基本解" << endl;
	cout << "2.基本解(步数记录)" << endl;
	cout << "3.内部数组显示(横向)" << endl;
	cout << "4.内部数组显示(纵向 + 横向)" << endl;
	cout << "5.图形解 - 预备 - 画三个圆柱" << endl;
	cout << "6.图形解 - 预备 - 在起始柱上画n个盘子" << endl;
	cout << "7.图形解 - 预备 - 第一次移动" << endl;
	cout << "8.图形解 - 自动移动版本" << endl;
	cout << "9.图形解 - 游戏版" << endl;
	cout << "0.退出" << endl;
	cout << "-------------------------------- -" << endl;
	cout << "[请选择:]  ";
}

//得到汉诺塔层数
int get_level()
{
	int level;
	while (1)
	{
		cout << "请输入汉诺塔的层数(1-10)" << endl;
		cin >> level;
		while (cin.fail())
		{
			cin.clear();
			cin.ignore(1024, '\n');
			cout << "请输入汉诺塔的层数(1-10)" << endl;
			cin >> level;
		}
		if (level >= 1 && level <= 10)
			break;
	}
	cin.clear();
	cin.ignore(1024, '\n');
	return level;
}

//得到起始柱
char get_start()
{
	char start;
	while (1)
	{
		cout << "请输入起始柱(A-C)" << endl;
		cin >> start;
		while (cin.fail())
		{
			cin.clear();
			cin.ignore(1024, '\n');
			cout << "请输入起始柱(A-C)" << endl;
			cin >> start;
		}
		if (start == 'A' || start == 'B' || start == 'C')
			break;
		else if (start == 'a' || start == 'b' || start == 'c')
		{
			start -= 32;
			break;
		}
		cin.clear();
		cin.ignore(1024, '\n');
	}
	cin.clear();
	cin.ignore(1024, '\n');
	return start;
}

//得到目标柱
char get_end(char start)
{
	char end;
	while (1)
	{
		cout << "请输入目标柱(A-C)" << endl;
		cin >> end;
		while (cin.fail())
		{
			cin.clear();
			cin.ignore(1024, '\n');
			cout << "请输入目标柱(A-C)" << endl;
			cin >> end;
		}
		if (end == start || end == start + 32 || end == start - 32)
		{
			cin.clear();
			cin.ignore();
			cout << "起始柱(" << start << ")不能与目标柱(" << end << ")相同" << endl;
			continue;
		}
		if (end == 'A' || end == 'B' || end == 'C')
			break;
		else if (end == 'a' || end == 'b' || end == 'c')
		{
			end -= 32;
			break;
		}
		cin.clear();
		cin.ignore(1024, '\n');
	}
	cin.clear();
	cin.ignore(1024, '\n');
	return end;
}

//输入并改变select,level,start,mid,end的值
void get_choice(int* select, char* start, char* mid, char* end, int* level)
{
	*select = _getch();
	while (*select < '0' || *select>'9')
		*select = _getch();
	*select -= '0';
	cout << *select << endl << endl;
	if (*select == 0)
		return;
	if (*select == 5)
		return;
	*level = get_level();
	*start = get_start();
	*end = get_end(*start);
	*mid = 'A' + 'B' + 'C' - *start - *end;
}

//得到延时值
int get_speed()
{
	while (1)
	{
		cout << "请输入移动速度(0-5: 0-按回车单步演示 1-延时最长 5-延时最短)" << endl;
		cin >> speed;
		if (!cin.fail() && speed >= 0 && speed <= 5)
			break;
		else
		{
			cin.clear();
			cin.ignore(1024, '\n');
		}
	}
	return speed;
}

//输入命令并做相应的操作(分情况返回不同的值AB,AC,BA,BC,CA,CB)
int fetch_instruction()
{
	char ch[20];//输入的字符超过19个自动清空
	char ch1;
	int i;
	while (1)
	{
		cct_showch(62, 34, ' ', COLOR_BLACK, COLOR_WHITE, 20);
		cct_gotoxy(62, 34);
		for (i = 0; i < 20; i++)
			ch[i] = '\0';

		for (i = 0; i < 20; i++)
		{
			char temp = _getch();
			if (!temp||temp==-32)//读到了方向键之类的按键
			{
				ch1 = _getch();//读掉下一个值
				i--;
				continue;
			}
			else if (temp == '\b')
			{
				i--;
				continue;
			}
			else
			{
				ch[i] = temp;
				if (ch[i] == '\r')
					break;
				cout << ch[i];
			}
		}
		if (i == 20)//输入 满20个字符清空
			continue;
		if (ch[0] == 'Q' || ch[0] == 'q')//输入Q返回0 退出
			if (ch[1] == '\r')
				return 0;

		if (strlen(ch) == 2 + 1)//输入两个判断有效性后跳出
			if (ch[0] == 'A' || ch[0] == 'a' || ch[0] == 'B' || ch[0] == 'b' || ch[0] == 'C' || ch[0] == 'c')
				if (ch[1] == 'A' || ch[1] == 'a' || ch[1] == 'B' || ch[1] == 'b' || ch[1] == 'C' || ch[1] == 'c')
					if (ch[0] != ch[1] && ch[0] != ch[1] + 32 && ch[0] != ch[1] - 32)
						break;
	}


	switch (ch[0])
	{
		case 'a':
		case 'A':
			if (ch[1] == 'b' || ch[1] == 'B')
				return 1;
			else
				return 2;
			break;

		case 'b':
		case 'B':
			if (ch[1] == 'a' || ch[1] == 'A')
				return 3;
			else
				return 4;

		case'c':
		case 'C':
			if (ch[1] == 'a' || ch[1] == 'A')
				return 5;
			else
				return 6;
			break;
	}
	return -1;
}

//根据select的值不同选择不同的出口，调用对应的hanoi函数
void menu(int n, char start, char mid, char end, int select)
{
	int input = 0;//检查9的输入是否有效


	switch (select)
	{
		case 1:
			hanoi(n, start, mid, end, 1);
			break;
		case 2:
			hanoi(n, start, mid, end, 2);
			break;
		case 3:
			spawn(start, n);
			hanoi(n, start, mid, end, 3);
			break;
		case 4:
			spawn(start, n);
			speed = get_speed();
			cct_cls();
			cct_setcursor(CURSOR_INVISIBLE);
			print_start(start, end, n, 1);
			print_vertical(4);
			print_vertical_menu(4);
			print_start_group(speed, 4);
			hanoi(n, start, mid, end, 4);
			cct_setcursor(CURSOR_VISIBLE_NORMAL);
			break;
		case 5:
			cct_cls();
			print_pillar();
			break;
		case 6:
			cct_cls();
			print_start(start, end, n, 0);
			print_pillar();
			print_plates(n, start);
			cct_gotoxy(0, 30);
			break;
		case 7:
			cct_cls();
			spawn(start, n);
			print_start(start, end, n, 0);
			print_pillar();
			cct_setcursor(CURSOR_INVISIBLE);
			print_plates(n, start);
			//目标柱位置(奇数层从start到end,偶数层从start到mid)
			if (select % 2 == 0)//偶数
				move(1, start, mid, 7);
			else//奇数
				move(1, start, end, 7);


			cct_setcursor(CURSOR_VISIBLE_NORMAL);
			cct_gotoxy(0, 35);
			break;

		case 8:
			spawn(start, n);
			speed = get_speed();
			cct_cls();
			print_start(start, end, n, 1);
			print_pillar();
			print_plates(n, start);
			print_vertical(8);
			cct_setcursor(CURSOR_INVISIBLE);
			print_vertical_menu(8);
			print_start_group(speed, 8);
			hanoi(n, start, mid, end, 8);
			cct_setcursor(CURSOR_VISIBLE_NORMAL);

			cct_gotoxy(0, 35);

			break;

		case 9://游戏版
			speed = 5;
			spawn(start, n);
			cct_cls();
			print_start(start, end, n, 0);
			print_pillar();
			print_plates(n, start);
			print_vertical(8);
			cct_setcursor(CURSOR_INVISIBLE);
			print_vertical_menu(8);
			print_start_group(5, 8);
			cct_gotoxy(0, 34);
			cct_setcursor(CURSOR_VISIBLE_NORMAL);
			cout << "请输入移动的柱号(命令形式：AC=A顶端的盘子移动到C，Q=退出) ： ";
			while (1)//进入游戏
			{
				while (1)//输入部分
				{
					input = fetch_instruction();//获取输入

					//输入为Q则退出游戏
					if (input == 0)
					{
						cct_gotoxy(0, 35);
						cout << "游戏中止!!!!!";
						return;
					}


					//源柱为空
					if (!column_empty(input))
					{
						cct_gotoxy(0, 35);
						cout << "源柱为空!";
						Sleep(1000);
						cct_showch(0, 35, ' ', COLOR_BLACK, COLOR_WHITE, 50);

						continue;
					}

					//大盘压小盘
					if (!large_over_small(input))
					{
						cct_gotoxy(0, 35);
						cout << "大盘压小盘，非法移动!";
						Sleep(1000);
						cct_showch(0, 35, ' ', COLOR_BLACK, COLOR_WHITE, 50);

						continue;
					}

					//有效则跳出输入部分
					break;
				}
				//移动
				switch (input)
				{
					case 1:
						print_cross(a[top_A - 1], 'A', 'B', 9);
						break;
					case 2:
						print_cross(a[top_A - 1], 'A', 'C', 9);
						break;
					case 3:
						print_cross(b[top_B - 1], 'B', 'A', 9);
						break;
					case 4:
						print_cross(b[top_B - 1], 'B', 'C', 9);
						break;
					case 5:
						print_cross(c[top_C - 1], 'C', 'A', 9);
						break;
					case 6:
						print_cross(c[top_C - 1], 'C', 'B', 9);
						break;


				}
				//判断是否游戏结束(判断end柱的栈顶指针是否为n)
				if (Game_over(n, end))
				{
					cct_gotoxy(0, 35);
					cout << "游戏结束!!!!!";
					break;
				}
			}
			cct_setcursor(CURSOR_VISIBLE_NORMAL);
			cct_gotoxy(0, 37);
			break;//对应case 9 的break
	}
}