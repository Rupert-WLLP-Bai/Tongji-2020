/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
#include <windows.h>
#include<conio.h>
#include"5-b7.h"
using namespace std;
int i = 0;//记步数
int a[10]; 
int b[10];
int c[10];
int top_A, top_B, top_C;
static int speed, show;

void print_start(char start, char end, int level,int speed, int show)
{
	cout << "从" << start << "移动到" << end << " ,共" << level << "层" << " ,延时设置为 " << speed << ", ";
	if (!show)
		cout << "不";
	cout << "显示内部数组值" << endl;
}

void print_vertical()
{
	int x = 10;
	int y = 11;
	
	cct_gotoxy(x,y);//A柱
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
			cout << "  ";
	}

	x = 20, y = 11;//B柱
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
			cout << "  ";
	}

	x = 30, y = 11;//C柱
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
			cout << "  ";
	}
}

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
			while (top_A <= level)
			{
				a[top_A++] = level - top_A;
			}
			top_A--;
			break;
		}
		case 'B':
		{
			while (top_B <= level)
			{
				b[top_B++] = level - top_B;
			}
			top_B--;
			break;
		}
		case 'C':
		{
			while (top_C <= level)
			{
				c[top_C++] = level - top_C;
			}
			top_C--;
			break;
		}
	}
	if (!speed)
		while (1)
		{
			int ch;
			ch = _getch();
			if (ch == 13)
				break;
		}
	if (show)
	{
		cout << "初始:                ";
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
		cout << endl;
	}
	print_vertical();
}

void hanoi(int n, char src, char tmp, char dst)
{
	void print_menu();
	void move(int, char, char);
	if (n == 1)
	{
		i++;
		print_menu();
		cct_gotoxy(0, 17);
		cout << "第" << setw(4) << i << " 步";
		move(n, src, dst);
		if (speed)
			Sleep(800 - speed * 160);
		if (!speed)
			while (1)
			{
				int ch;
				ch = _getch();
				if (ch == 13)
					break;
			}
	}
	else
	{
		hanoi(n - 1, src, dst, tmp);
		i++;
		print_menu();
		cct_gotoxy(0, 17);
		cout << "第" << setw(4) << i << " 步";
		move(n, src, dst);
		if (!speed)
			while (1)
			{
				int ch;
				ch = _getch();
				if (ch == 13)
					break;
			}
		if (speed)
			Sleep(800 - speed * 160);
		hanoi(n - 1, tmp, src, dst);
	}
}

void move(int n, char x, char y)
{
	cout << "(" << setw(2) << n << "): ";
	cout << x << "-->" << y << " ";
	//进栈,出栈
	int temp;
	switch (x)
	{
		case 'A':
			if (y == 'B')
			{
				temp = a[--top_A];
				a[top_A] = 0;
				b[top_B] = temp;
				top_B++;
			}
			else if (y == 'C')
			{
				temp = a[--top_A];
				a[top_A] = 0;
				c[top_C] = temp;
				top_C++;
			}
			break;
		case 'B':
			if (y == 'A')
			{
				temp = b[--top_B];
				b[top_B] = 0;
				a[top_A] = temp;
				top_A++;
			}
			else if (y == 'C')
			{
				temp = b[--top_B];
				b[top_B] = 0;
				c[top_C] = temp;
				top_C++;
			}
			break;
		case 'C':
			if (y == 'A')
			{
				temp = c[--top_C];
				c[top_C] = 0;
				a[top_A] = temp;
				top_A++;
			}
			else if (y == 'B')
			{
				temp = c[--top_C];
				c[top_C] = 0;
				b[top_B] = temp;
				top_B++;
			}
			break;
	}

	//输出
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
		cout << endl;
	}
	print_vertical();
}

void print_menu()
{
	cct_gotoxy(9, 12);
	cout << "=========================";
	cct_gotoxy(9, 13);
	cout << "  A         B         C  ";
}

int main()
{
	int level;
	char start, mid, end;
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
	while (1)
	{
		cout << "请输入是否显示内部数组值(0-不显示 1-显示)" << endl;
		cin >> show;
		if (!cin.fail() && show >= 0 && show <= 1)
			break;
		else
		{
			cin.clear();
			cin.ignore(1024, '\n');
		}
	}

	mid = 'A' + 'B' + 'C' - start - end;

	cct_cls();
	print_start(start, end,level, speed, show);
	print_menu();
	cct_gotoxy(0, 17);
	spawn(start, level);
	if (speed)
		Sleep(800 - speed * 160);
	if (!speed)
		while (1)
		{
			int ch;
			ch = _getch();
			if (ch == 13)
				break;
		}
	hanoi(level, start, mid, end);
	cct_gotoxy(0, 25);
	return 0;
}