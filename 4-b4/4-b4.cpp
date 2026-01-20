/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <iostream>
#include <cstdio>
#include <conio.h>
#include <time.h>
#include <windows.h>
using namespace std;

const int MAX_X = 69; //定义*组成的边框的宽度
const int MAX_Y = 17; //定义*组成的边框的高度

/***************************************************************************
  函数名称：
  功    能：完成与system("cls")一样的功能，但效率高
  输入参数：
  返 回 值：
  说    明：清除整个屏幕缓冲区，不仅仅是可见窗口区域(使用当前颜色)
***************************************************************************/
void cls(const HANDLE hout)
{
	COORD coord = { 0, 0 };
	CONSOLE_SCREEN_BUFFER_INFO binfo; /* to get buffer info */
	DWORD num;

	/* 取当前缓冲区信息 */
	GetConsoleScreenBufferInfo(hout, &binfo);
	/* 填充字符 */
	FillConsoleOutputCharacter(hout, (TCHAR)' ', binfo.dwSize.X * binfo.dwSize.Y, coord, &num);
	/* 填充属性 */
	FillConsoleOutputAttribute(hout, binfo.wAttributes, binfo.dwSize.X * binfo.dwSize.Y, coord, &num);

	/* 光标回到(0,0) */
	SetConsoleCursorPosition(hout, coord);
	return;
}

/***************************************************************************
  函数名称：gotoxy
  功    能：将光标移动到指定位置
  输入参数：HANDLE hout ：输出设备句柄
			int X       ：指定位置的x坐标
			int Y       ：指定位置的y坐标
  返 回 值：无
  说    明：此函数不准修改
***************************************************************************/
void gotoxy(const HANDLE hout, const int X, const int Y)
{
	COORD coord;
	coord.X = X;
	coord.Y = Y;
	SetConsoleCursorPosition(hout, coord);
}

/***************************************************************************
  函数名称：showch
  功    能：在指定位置处打印一个指定的字符
  输入参数：HANDLE hout ：输出设备句柄
			int X       ：指定位置的x坐标
			int Y       ：指定位置的y坐标
			char ch     ：要打印的字符
  返 回 值：无
  说    明：此函数不准修改
***************************************************************************/
void showch(const HANDLE hout, const int X, const int Y, const char ch)
{
	gotoxy(hout, X, Y);
	putchar(ch);
}

/***************************************************************************
  函数名称：init_border
  功    能：显示初始的边框及随机字符
  输入参数：HANDLE hout：输出设备句柄
  返 回 值：无
  说    明：此函数不准修改
***************************************************************************/
void init_border(const HANDLE hout)
{
	gotoxy(hout, 0, 0); //光标移回左上角(0,0)
	cout << "***********************************************************************" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "*                                                                     *" << endl;
	cout << "***********************************************************************" << endl;

	/* 随机显示20个大写字母，字母的值、XY坐标都随机显示
	   rand()函数的功能：随机生成一个在 0-32767 之间的整数
	   思考：在什么情况下，下面这个循环执行生成后，你看到的实际字母个数不足20个？ */
	int i;
	for (i = 0; i < 20; i++)
		showch(hout, rand() % MAX_X + 1, rand() % MAX_Y + 1, 'A' + rand() % 26);

	return;
}

/* -- 按需增加的若干函数可以放在此处 --*/
/***************************************************************************
  函数名称：move_by_ijkl
  功    能：
  输入参数：
  返 回 值：
  说    明：
***************************************************************************/
int move_by_arrow(int huirao)
{
	const HANDLE hout = GetStdHandle(STD_OUTPUT_HANDLE); //取标准输出设备对应的句柄
	int x = 34, y = 8;
	gotoxy(hout, x, y);
	int move = 0;
	int ch = 224;
	while (1)
	{
		int ch = _getch();
		if (ch == 224)
		{
			move = _getch();

			if (move == 72) //向上移动
			{
				if (huirao)
					y > 1 ? y-- : y += 16;
				else if (y > 1)
					y--;
			}
			if (move == 75) //向左移动
			{
				if (huirao)
					x > 1 ? x-- : x += 68;
				else if (x > 1)
					x--;
			}
			if (move == 80) //向下移动
			{
				if (huirao)
					y < 17 ? y++ : y -= 16;
				else if (y < 17)
					y++;
			}
			if (move == 77) //向右移动
			{
				if (huirao)
					x < 69 ? x++ : x -= 68;
				else if (x < 69)
					x++;
			}
			gotoxy(hout, x, y);
		}

		else
		{
			if (ch == 32)
			{
				showch(hout, x, y, ' ');
				gotoxy(hout, x, y);
			}
			if (ch == 113 || ch == 81) //Q或q退出move_by_ijkl
				break;
		}
		
	}
	return 0;
}

/***************************************************************************
  函数名称：move_by_ijkl
  功    能：
  输入参数：
  返 回 值：
  说    明：左键对应先后得到两个值224 75
***************************************************************************/
int move_by_ijkl(int huirao)
{
	const HANDLE hout = GetStdHandle(STD_OUTPUT_HANDLE); //取标准输出设备对应的句柄
	int x = 34, y = 8;
	gotoxy(hout, x, y);
	int move = 0;
	while (1)
	{
		move = _getch();

		if (move == 113 || move == 81) //Q或q退出move_by_ijkl
			break;
		if (move == 73 || move == 105) //向上移动
		{
			if (huirao)
				y > 1 ? y-- : y += 16;
			else if (y > 1)
				y--;
		}
		if (move == 74 || move == 106) //向左移动
		{
			if (huirao)
				x > 1 ? x-- : x += 68;
			else if (x > 1)
				x--;
		}
		if (move == 75 || move == 107) //向下移动
		{
			if (huirao)
				y < 17 ? y++ : y -= 16;
			else if (y < 17)
				y++;
		}
		if (move == 76 || move == 108) //向右移动
		{
			if (huirao)
				x < 69 ? x++ : x -= 68;
			else if (x < 69)
				x++;
		}
		gotoxy(hout, x, y);
		if (move == 32)
		{
			showch(hout, x, y, ' ');
			gotoxy(hout, x, y);
		}
	}
	return 0;
}
/***************************************************************************
  函数名称：menu
  功    能：菜单选择
  输入参数：选择菜单的号数
  返 回 值：输入值的ASCII码
  说    明：
***************************************************************************/
int menu()
{
	//cout<<"5.用箭头键控制上下左右(边界停止，演示HPKM可移动的错误，此项不需要实现)"<<endl;
	//cout<<"6.用箭头键控制上下左右(边界回绕，演示HPKM可移动的错误，此项不需要实现)"<<endl;
	int a;
	cout << "1.用I、J、K、L键控制上下左右(大小写均可，边界停止)" << endl;
	cout << "2.用I、J、K、L键控制上下左右(大小写均可，边界回绕)" << endl;
	cout << "3.用箭头键控制上下左右，边界停止" << endl;
	cout << "4.用箭头键控制上下左右，边界回绕" << endl;
	cout << "0.退出" << endl;
	cout << "[请选择0-4]";
	while (1)
	{
		a = _getch();
		if (a == 48)
			break;
		else if (a == 49)
			break;
		else if (a == 50)
			break;
		else if (a == 51)
			break;
		else if (a == 52)
			break;
	}
	return a;
}

/***************************************************************************
  函数名称：
  功    能：
  输入参数：
  返 回 值：
  说    明：main函数仅用于初始演示，可以按题目要求全部推翻重写
***************************************************************************/
int main()
{
	const HANDLE hout = GetStdHandle(STD_OUTPUT_HANDLE);
	int select;
	while (1)
	{
		select = menu();
		if (select == 48) //输入0，退出程序
			break;

		if (select == 49) //对应情况1
		{
			cls(hout);           //清屏
			init_border(hout);   //初始化
			move_by_ijkl(0);     //移动(不回绕)
			gotoxy(hout, 0, 23); //光标回位
			cout << "游戏结束,请按回车键返回菜单";
			while (1)
			{
				int ch;
				ch = _getch();
				if (ch == 13)
				{
					cls(hout); //清屏
					break;
				}
			}
		}

		if (select == 50) //对应情况2
		{
			cls(hout);           //清屏
			init_border(hout);   //初始化
			move_by_ijkl(1);     //移动(回绕)
			gotoxy(hout, 0, 23); //光标回位
			cout << "游戏结束,请按回车键返回菜单";
			while (1)
			{
				int ch;
				ch = _getch();
				if (ch == 13)
				{
					cls(hout); //清屏
					break;
				}
			}
		}

		if (select == 51) //对应情况3
		{
			cls(hout);           //清屏
			init_border(hout);   //初始化
			move_by_arrow(0);     //移动(不回绕)
			gotoxy(hout, 0, 23); //光标回位
			cout << "游戏结束,请按回车键返回菜单";
			while (1)
			{
				int ch;
				ch = _getch();
				if (ch == 13)
				{
					cls(hout); //清屏
					break;
				}
			}
		}

		if (select == 52) //对应情况4
		{
			cls(hout);           //清屏
			init_border(hout);   //初始化
			move_by_arrow(1);   //移动(回绕)
			gotoxy(hout, 0, 23); //光标回位
			cout << "游戏结束,请按回车键返回菜单";
			while (1)
			{
				int ch;
				ch = _getch();
				if (ch == 13)
				{
					cls(hout); //清屏
					break;
				}
			}
		}

			cls(hout);
	}

	return 0;
}
