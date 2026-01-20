/*2052521 信13 张耀尹*/
#include<iostream>
#include<iomanip>
#include<Windows.h>
#include <conio.h>
#include "mine_sweeper.h"
#include "cmd_console_tools.h"
using namespace std;
int main()
{
	/* demo中首先执行此句，将cmd窗口设置为40行x120列（缓冲区宽度120列，行数9000行，即cmd窗口右侧带有垂直滚动杆）*/
	cct_setconsoleborder(100, 30, 100, 9000);
	while (1)
	{
		char ch;
		menu();
		while (1)
		{
			ch = _getch();
			if (ch > '0' && ch <= '9')
				choose(ch);
			else if (ch == '0')
				break;
			else
				;
		}
	}
	return 0;
}