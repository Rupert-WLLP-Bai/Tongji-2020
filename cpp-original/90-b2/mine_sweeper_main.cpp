/*2052526 信15 白俊豪*/

#include<iostream>
#include"cmd_console_tools.h"
#include"mine_sweeper.h"
using namespace std;

int main()
{
	int mine[W_max][L_max] = { 0 };//计算雷数
	int a[W_max][L_max] = { no };//0,1雷阵
	int b[W_max][L_max] = { HIDE };//用于记录该位置是否已经被处理过,初始状态是全部隐藏
	int c[W_max][L_max] = { no };//是否被标记
	int select;
	while (1)
	{
		cct_setcolor(COLOR_BLACK, COLOR_WHITE);
		cct_setconsoleborder(100, 30);
		cct_setfontsize("新宋体", 24);
		print_menu();
		get_choice(&select);
		if (select == '0')
			break;
		cct_cls();
		menu(select, mine, a, b, c);
		wait_for_enter();
	}

	return 0;
}