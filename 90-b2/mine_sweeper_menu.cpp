/*2052526 信15 白俊豪*/

#include<iostream>
#include<conio.h>
#include<iomanip>
#include"cmd_console_tools.h"
#include"mine_sweeper.h"
using namespace std;


/*打印主菜单*/
void print_menu()
{
	cout << "------------------------------------------" << endl;
	cout << "1.选择难度并显示内部数组" << endl;
	cout << "2.输入初始位置并显示被打开的初始区域" << endl;
	cout << "3.内部数组基础版" << endl;
	cout << "4.内部数组完整版（标记、运行时间）" << endl;
	cout << "5.画出伪图形化框架并显示内部数据" << endl;
	cout << "6.检测鼠标位置和合法性（左键单击退出）" << endl;
	cout << "7.鼠标选择初始位置并显示被打开的初始区域" << endl;
	cout << "8.伪图形界面基础版" << endl;
	cout << "9.伪图形界面完整版" << endl;
	cout << "0.退出游戏" << endl;
	cout << "------------------------------------------" << endl;
	cout << "[请选择] : ";
}

/*打印难度选择界面*/
void print_choose_difficulty()
{
	cout << "请选择难度：" << endl;
	cout << "1.初级(9 * 9 - 10颗雷)" << endl;
	cout << "2.中级(16 * 16 - 40颗雷)" << endl;
	cout << "3.高级(16 * 30 - 99颗雷)" << endl;
	cout << "请输入[1..3]：";
}

/*按回车继续,并且清屏*/
void wait_for_enter()
{
	cout << endl << endl;
	cout << "按回车键继续...";
	while (_getch() != '\r')
		;
	cct_cls();
}

/*菜单函数*//*传入所有需要用的参数*/
void menu(int select, int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max],int(*c)[L_max])
{

	switch (select)
	{
		case '1':
			case1(mine, a);
			reset(mine, a);
			break;
		case '2':
			case2(mine, a, b);
			reset(mine, a);
			reset(mine, b);
			break;
		case'3':
			case3(mine, a, b);
			reset(mine, a);
			reset(mine, b);
			break;
		case '4':
			case4(mine, a, b, c);
			reset(a, b);
			reset(mine, c);
			break;
		case '5':
			case5(mine, a, b);
			break;
		case '6':
			case6(mine, a, b);
			break;
		case '7':
			case7(mine, a, b, c);
			reset(a, b);
			reset(mine, c);
			break;
		case '8':
			case8(mine, a, b, c);
			reset(mine, a);
			reset(b, c);
			break;
		case '9':
			case9(mine, a, b, c);
			reset(mine, a);
			reset(b, c);
			break;
	}
}
