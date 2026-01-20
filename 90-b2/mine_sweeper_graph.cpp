/*2052526 信15 白俊豪*/

/*画出伪图形界面*/
/*在相应的格子里面输出相应的数字*//*颜色的设置还是COLOR_WHITER,mine[i][j]*/
/*利用坐标算出相应的格子(行2,列4),即利用x,y算出i,j,输出的位置固定在第2行，第2列的位置*/
#include<iostream>
#include<iomanip>
#include"cmd_console_tools.h"
#include"mine_sweeper.h"
#include<Windows.h>
using namespace std;


void print_cross_start(int line)
{
	cout << endl;
	for (int i = 0; i < line; i++)
	{
		cout << setiosflags(ios::right) << setw(6) << setfill(' ') << i;
		if (i == 9)//补一个空格,后面才能对齐
			cout << " ";
	}
	cout << endl << "  ";
	cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
	if (line == 9)
		cout << "╔══╦══╦══╦══╦══╦══╦══╦══╦══╗" << endl;
	else if (line == 16)
		cout << "╔══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╗" << endl;
	else if (line == 30)
		cout << "╔══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╦══╗" << endl;
	else
		cout << "error" << endl;
}

void print_cross_mid(int line, int row)//列,行
{
	for (int j = 0; j < row; j++)
	{
		cct_setcolor(COLOR_BLACK, COLOR_WHITE);//第一行
		cout << "  ";
		for (int i = 0; i < line; i++)
		{
			cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
			cout << "║";
			cct_setcolor(COLOR_WHITE, COLOR_BLACK);
			cout << "    ";
		}
		cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
		cout << "║" << endl;


		cct_setcolor(COLOR_BLACK, COLOR_WHITE);//第二行
		cout << static_cast<char>(j + 'A') << " ";
		for (int i = 0; i < line; i++)
		{
			cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
			cout << "║";
			cct_setcolor(COLOR_WHITE, COLOR_BLACK);
			cout << "    ";
		}
		cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
		cout << "║" << endl;

		if (j < row - 1)
		{
			cct_setcolor(COLOR_BLACK, COLOR_WHITE);//第三行
			cout << "  ";
			cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
			cout << "╠";
			for (int i = 0; i < line - 1; i++)
			{
				cout << "══";
				cout << "╬";
			}
			cout << "══╣" << endl;
		}
	}
}

void print_cross_mid_color0(int line, int row)
{
	for (int j = 0; j < row; j++)
	{
		cct_setcolor(COLOR_BLACK, COLOR_WHITE);//第一行
		cout << "  ";
		for (int i = 0; i < line; i++)
		{
			cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
			cout << "║";
			cct_setcolor(COLOR_YELLOW, COLOR_BLACK);
			cout << "    ";
		}
		cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
		cout << "║" << endl;


		cct_setcolor(COLOR_BLACK, COLOR_WHITE);//第二行
		cout << static_cast<char>(j + 'A') << " ";
		for (int i = 0; i < line; i++)
		{
			cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
			cout << "║";
			cct_setcolor(COLOR_YELLOW, COLOR_BLACK);
			cout << "    ";
		}
		cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
		cout << "║" << endl;

		if (j < row - 1)
		{
			cct_setcolor(COLOR_BLACK, COLOR_WHITE);//第三行
			cout << "  ";
			cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
			cout << "╠";
			for (int i = 0; i < line - 1; i++)
			{
				cout << "══";
				cout << "╬";
			}
			cout << "══╣" << endl;
		}
	}
}

void print_cross_end(int line)
{
	cct_setcolor(COLOR_BLACK, COLOR_WHITE);
	cout << "  ";
	cct_setcolor(COLOR_HWHITE, COLOR_BLACK);
	if (line == 9)
		cout << "╚══╩══╩══╩══╩══╩══╩══╩══╩══╝" << endl;
	else if (line == 16)
		cout << "╚══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╝" << endl;
	else if (line == 30)
		cout << "╚══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╩══╝" << endl;
	else
		cout << "error" << endl;
}

/*将一个区域的颜色设置为WHITE*/
void resetcolor(const int x, const int y)
{
	cct_showch(x - 2, y - 1, ' ', COLOR_WHITE, COLOR_BLACK, 4);
	cct_showch(x - 2, y, ' ', COLOR_WHITE, COLOR_BLACK, 4);
	cct_gotoxy(x, y);
}

/*将一个区域的颜色设置为YELLOW*/
void resetcolor_yellow(const int x, const int y)
{
	cct_showch(x - 2, y - 1, ' ', COLOR_YELLOW, COLOR_BLACK, 4);
	cct_showch(x - 2, y, ' ', COLOR_YELLOW, COLOR_BLACK, 4);
	cct_gotoxy(x, y);
}

void setcolor_mark(const int x, const int y)
{
	cct_showch(x - 2, y - 1, ' ', COLOR_RED, COLOR_BLACK, 4);
	cct_showch(x - 2, y, ' ', COLOR_RED, COLOR_BLACK, 4);
	cct_showch(x - 1, y, '#', COLOR_RED, COLOR_WHITE, 1);
	cct_gotoxy(x, y);
}



/*未打开部分为橘色,其余正常输出*/
/*用cct_gotoxy遍历每一格进行输出*/
/*情况7只需要b(是否被打开),mine(雷数)*/
/*情况8加入标记*/
void print_result7(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max], int select)
{
	int i, j;//数组下标
	int x, y;//伪图形坐标
	switch (select)
	{
		case '1':
			for (i = 1; i < W1 + 1; i++)
			{
				for (j = 1; j < L1 + 1; j++)
				{
					y = 1 + 3 * i;
					x = 6 * j;
					if (b[i][j] == OPEN && c[i][j] == no)//打开且未被标记
					{
						resetcolor(x, y);
						if (a[i][j] == 0)//无雷
						{
							if (mine[i][j] != 0)//周围有雷
							{
								cct_setcolor(COLOR_WHITE, mine[i][j]);
								cout << mine[i][j];
							}
							else
								continue;
						}
						else//有雷
						{
							cct_gotoxy(x, y);
							cct_setcolor(COLOR_WHITE, COLOR_BLACK);
							cout << "*";
						}
					}
					else if (c[i][j] == mark)//被标记且未被打开
					{
						setcolor_mark(x, y);
					}
					else//未被标记,未被打开
					{
						resetcolor_yellow(x, y);
					}
					cct_setcolor(COLOR_WHITE, COLOR_BLACK);
				}
			}
			cct_gotoxy(0, 30);
			break;
		case '2':
			for (i = 1; i < W2 + 1; i++)
			{
				for (j = 1; j < L2 + 1; j++)
				{
					y = 1 + 3 * i;
					x = 6 * j;
					if (b[i][j] == OPEN && c[i][j] == no)//打开且未被标记
					{
						resetcolor(x, y);
						if (a[i][j] == 0)//无雷
						{
							if (mine[i][j] != 0)//周围有雷
							{
								cct_setcolor(COLOR_WHITE, mine[i][j]);
								cout << mine[i][j];
							}
							else
								continue;
						}
						else//有雷
						{
							cct_gotoxy(x, y);
							cct_setcolor(COLOR_WHITE, COLOR_BLACK);
							cout << "*";
						}
					}
					else if (c[i][j] == mark)//被标记且未被打开
					{
						setcolor_mark(x, y);
					}
					else//未被标记,未被打开
					{
						resetcolor_yellow(x, y);
					}
					cct_setcolor(COLOR_WHITE, COLOR_BLACK);
				}
			}
			cct_gotoxy(0, 51);
			break;
		case '3':
			for (i = 1; i < W3 + 1; i++)
			{
				for (j = 1; j < L3 + 1; j++)
				{
					y = 1 + 3 * i;
					x = 6 * j;
					if (b[i][j] == OPEN && c[i][j] == no)//打开且未被标记
					{
						resetcolor(x, y);
						if (a[i][j] == 0)//无雷
						{
							if (mine[i][j] != 0)//周围有雷
							{
								cct_setcolor(COLOR_WHITE, mine[i][j]);
								cout << mine[i][j];
							}
							else
								continue;
						}
						else//有雷
						{
							cct_gotoxy(x, y);
							cct_setcolor(COLOR_WHITE, COLOR_BLACK);
							cout << "*";
						}
					}
					else if (c[i][j] == mark)//被标记且未被打开
					{
						setcolor_mark(x, y);
					}
					else//未被标记,未被打开
					{
						resetcolor_yellow(x, y);
					}
					cct_setcolor(COLOR_WHITE, COLOR_BLACK);
				}
			}
			cct_gotoxy(0, 51);
			break;
		default:
			break;
	}
	cct_setcolor();

}

/*输出情况5中的结果*/
void print_result5(int(*mine)[L_max], int(*a)[L_max], int select)
{
	int x, y;//对应输出位置的坐标
	if (select == '1')
	{
		for (int i = 1; i < W1 + 1; i++)
		{
			for (int j = 1; j < L1 + 1; j++)
			{
				y = 1 + 3 * i;
				x = 6 * j;
				cct_gotoxy(x, y);
				if (a[i][j] == 0 && mine[i][j] != 0)//无雷,且mine不为0
				{
					cct_setcolor(COLOR_WHITE, mine[i][j]);
					cout << mine[i][j];
				}
				else if (a[i][j] == 1)//有雷
				{
					cct_setcolor(COLOR_WHITE, COLOR_BLACK);
					cout << "*";
				}

				cct_setcolor(COLOR_WHITE, COLOR_BLACK);
			}
		}
	}
	else if (select == '2')
	{
		for (int i = 1; i < W2 + 1; i++)
		{
			for (int j = 1; j < L2 + 1; j++)
			{
				y = 1 + 3 * i;
				x = 6 * j;
				cct_gotoxy(x, y);
				if (a[i][j] == 0 && mine[i][j] != 0)//无雷,且mine不为0
				{
					cct_setcolor(COLOR_WHITE, mine[i][j]);
					cout << mine[i][j];
				}
				else if (a[i][j] == 1)//有雷
				{
					cct_setcolor(COLOR_WHITE, COLOR_BLACK);
					cout << "*";
				}

				cct_setcolor(COLOR_WHITE, COLOR_BLACK);
			}
		}
	}
	else if (select == '3')
	{
		for (int i = 1; i < W3 + 1; i++)
		{
			for (int j = 1; j < L3 + 1; j++)
			{
				y = 1 + 3 * i;
				x = 6 * j;
				cct_gotoxy(x, y);
				if (a[i][j] == 0 && mine[i][j] != 0)//无雷,且mine不为0
				{
					cct_setcolor(COLOR_WHITE, mine[i][j]);
					cout << mine[i][j];
				}
				else if (a[i][j] == 1)//有雷
				{
					cct_setcolor(COLOR_WHITE, COLOR_BLACK);
					cout << "*";
				}

				cct_setcolor(COLOR_WHITE, COLOR_BLACK);
			}
		}
	}
}

/*输出初始的格子*/
void print_graph(int select)
{
	cct_setfontsize("点阵字体", 16, 8);
	switch (select)
	{
		case '1':
			cct_setconsoleborder(59, 35);
			print_cross_start(9);
			print_cross_mid(9, 9);
			print_cross_end(9);
			cct_setcolor();
			break;
		case '2':
			cct_setconsoleborder(101, 56);
			print_cross_start(16);
			print_cross_mid(16, 16);
			print_cross_end(16);
			cct_setcolor();
			break;
		case '3':
			cct_setconsoleborder(185, 56);
			print_cross_start(30);
			print_cross_mid(30, 16);
			print_cross_end(30);
			cct_setcolor();
			break;
	}
}

/*输出初始的带颜色的格子*/
void print_graph_color_0(int select)
{
	cct_setfontsize("点阵字体", 16, 8);
	switch (select)
	{
		case '1':
			cct_setconsoleborder(59, 37);
			print_cross_start(9);
			print_cross_mid_color0(9, 9);
			print_cross_end(9);
			cct_setcolor();
			break;
		case '2':
			cct_setconsoleborder(101, 56);
			print_cross_start(16);
			print_cross_mid_color0(16, 16);
			print_cross_end(16);
			cct_setcolor();
			break;
		case '3':
			cct_setconsoleborder(185, 56);
			print_cross_start(30);
			print_cross_mid_color0(30, 16);
			print_cross_end(30);
			cct_setcolor();
			break;
	}
}

/*情况5*/
/*生成雷阵并输出,0不输出,雷输出黑色的*,其他的数字带颜色,输出的位置固定在第2行，第2列的位置*/
void case5(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max])
{
	int x, y;
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();


	srand((unsigned int)time(NULL));
	if (difficulty == '1')
	{
		x = rand() % W1 + 1;
		y = rand() % L1 + 1;
		spawn(W1, L1, mine1, a, x, y);
		Num_of_mine(mine, a, '1');
		print_graph('1');
		print_result5(mine, a, '1');
	}
	else if (difficulty == '2')
	{
		x = rand() % W2 + 1;
		y = rand() % L2 + 1;
		spawn(W2, L2, mine2, a, x, y);
		Num_of_mine(mine, a, '2');
		print_graph('2');
		print_result5(mine, a, '2');
	}
	else
	{
		x = rand() % W3 + 1;
		y = rand() % L3 + 1;
		spawn(W3, L3, mine3, a, x, y);
		Num_of_mine(mine, a, '3');
		print_graph('3');
		print_result5(mine, a, '3');
	}
	reset(mine, a);
	cct_setcolor();
	cout << endl << endl;
}


/*检查坐标*/
bool check(int difficulty, const int x, const int y)
{
	if (x == 0 || x == 1)//检查列
		return 0;
	if (x % 6 == 2 || x % 6 == 3)//检查列
		return 0;
	if (y % 3 == 2 || y == 0 || y == 1)//检查行
		return 0;
	//检查边界
	switch (difficulty)
	{
		case '1':
			if (y > 29)
				return 0;
			break;
		case '2':
		case '3':
			if (y > 50)
				return 0;
			break;
	}

	return 1;
}

/*检查鼠标的位置是否合法并输出*/
/*按左键退出*/
void check_mouse1(int difficulty)
{
	cct_enable_mouse();//添加鼠标支持
	int X = 0, Y = 0, loop = 1;
	int ret, maction;
	int keycode1, keycode2;
	int row, line;//行列
	int start_X, start_Y;//初始的输出位置
	cct_getxy(start_X, start_Y);
	cct_setcursor(CURSOR_INVISIBLE);

	while (1)
	{

		ret = cct_read_keyboard_and_mouse(X, Y, maction, keycode1, keycode2);
		if (ret == CCT_MOUSE_EVENT)
		{
			cct_showch(start_X, start_Y, ' ', COLOR_BLACK, COLOR_WHITE, 185);
			if (check(difficulty, X, Y))//位置合法
			{
				if (maction == MOUSE_LEFT_BUTTON_CLICK)//按下左键
					break;
				else//输出对应的坐标
				{
					cct_gotoxy(start_X, start_Y);
					line = (X + 2) / 6 - 1;
					row = Y / 3;
					cout << "[当前光标] " << static_cast<char>(row - 1 + 'A') << "行" << line << "列";
				}
			}
			else//输出位置不合法
			{
				cct_gotoxy(start_X, start_Y);
				cout << "[当前光标] 位置非法";
			}
		}
	}
}

/*检查鼠标的位置是否合法并输出,需要左键退出的功能*/
/*按左键打开雷阵,即按左键返回所指位置坐标对应的数组下标，进行相应的打开操作*/
void mouse_open_1(int difficulty, int* i, int* j)
{
	cct_enable_mouse();//添加鼠标支持
	int X = 0, Y = 0, loop = 1;
	int ret, maction;
	int keycode1, keycode2;
	int row, line;//行列
	int start_X, start_Y;//初始的输出位置
	cct_getxy(start_X, start_Y);
	cct_setcursor(CURSOR_INVISIBLE);

	while (1)
	{
		ret = cct_read_keyboard_and_mouse(X, Y, maction, keycode1, keycode2);
		line = (X + 2) / 6 - 1;
		row = Y / 3;
		if (ret == CCT_MOUSE_EVENT)
		{
			cct_showch(start_X, start_Y, ' ', COLOR_BLACK, COLOR_WHITE, 185);
			if (check(difficulty, X, Y))//位置合法
			{
				if (maction == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					*i = row;
					*j = line + 1;
					break;
				}
				else//输出对应的坐标
				{
					cct_gotoxy(start_X, start_Y);
					line = (X + 2) / 6 - 1;
					row = Y / 3;
					cout << "[当前光标] " << static_cast<char>(row - 1 + 'A') << "行" << line << "列";
				}
			}
			else//输出位置不合法
			{
				cct_gotoxy(start_X, start_Y);
				cout << "[当前光标] 位置非法";
			}
		}
	}

}


/*检查鼠标的位置是否合法并输出,需要左键退出的功能*/
/*按左键打开雷阵,即按左键返回所指位置坐标对应的数组下标，进行相应的打开操作*/
/*右键标记或取消标记*/
/*按ESC退出*/
int mouse_open_2(int difficulty, int* i, int* j)
{

	cct_enable_mouse();//添加鼠标支持
	int X = 0, Y = 0, loop = 1;
	int ret, maction;
	int keycode1, keycode2;
	int row, line;//行列
	int start_X, start_Y;//初始的输出位置
	cct_getxy(start_X, start_Y);
	cct_setcursor(CURSOR_INVISIBLE);

	while (1)
	{
		ret = cct_read_keyboard_and_mouse(X, Y, maction, keycode1, keycode2);
		line = (X + 2) / 6 - 1;
		row = Y / 3;
		if (ret == CCT_MOUSE_EVENT)
		{
			cct_showch(start_X, start_Y, ' ', COLOR_BLACK, COLOR_WHITE, 20);
			cct_gotoxy(start_X, start_Y);
			if (check(difficulty, X, Y))//位置合法
			{
				if (maction == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					*i = row;
					*j = line + 1;
					return MOUSE_LEFT_BUTTON_CLICK;
				}
				else if (maction == MOUSE_RIGHT_BUTTON_CLICK)//按下右键
				{
					*i = row;
					*j = line + 1;
					return MOUSE_RIGHT_BUTTON_CLICK;
				}
				else//输出对应的坐标
				{
					cct_gotoxy(start_X, start_Y);
					line = (X + 2) / 6 - 1;
					row = Y / 3;
					cout << "[当前光标] " << static_cast<char>(row - 1 + 'A') << "行" << line << "列";
					cct_gotoxy(start_X, start_Y);
				}
			}
			else//输出位置不合法
			{
				cct_gotoxy(start_X, start_Y);
				cout << "[当前光标] 位置非法";
				cct_gotoxy(start_X, start_Y);
			}
		}
		else if (ret == CCT_KEYBOARD_EVENT)
		{
			if (keycode1 == 27)
				return 27;
			if (keycode1 == ' ')
				return ' ';
		}

	}
	return 0;
}
/*情况6*/
/*检查鼠标位置的合法性,输出对应的行列位置,在合法位置按左键退出*/
/*用case5生成雷阵*/
/*获取鼠标的位置与事件*/
/*输出相应的位置,左键break;*/
void case6(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max])
{
	int x, y;
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();


	srand((unsigned int)time(NULL));
	if (difficulty == '1')
	{
		x = rand() % W1 + 1;
		y = rand() % L1 + 1;
		spawn(W1, L1, mine1, a, x, y);
		Num_of_mine(mine, a, '1');
		print_graph('1');
		print_result5(mine, a, '1');
	}
	else if (difficulty == '2')
	{
		x = rand() % W2 + 1;
		y = rand() % L2 + 1;
		spawn(W2, L2, mine2, a, x, y);
		Num_of_mine(mine, a, '2');
		print_graph('2');
		print_result5(mine, a, '2');
	}
	else
	{
		x = rand() % W3 + 1;
		y = rand() % L3 + 1;
		spawn(W3, L3, mine3, a, x, y);
		Num_of_mine(mine, a, '3');
		print_graph('3');
		print_result5(mine, a, '3');
	}
	cct_setcolor();
	cout << endl << endl;
	check_mouse1(difficulty);

}

/*情况7*/
void case7(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max])
{
	int x, y;
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();

	if (difficulty == '1')
	{
		print_graph_color_0('1');
		cct_setcolor();
		mouse_open_1(difficulty, &x, &y);
		spawn(W1, L1, mine1, a, x, y);
		Num_of_mine(mine, a, '1');
		open_matrix1(a, b, x, y);
		print_result7(mine, a, b, c, '1');

	}
	else if (difficulty == '2')
	{
		print_graph_color_0('2');
		cct_setcolor();
		mouse_open_1(difficulty, &x, &y);
		spawn(W2, L2, mine2, a, x, y);
		Num_of_mine(mine, a, '2');
		open_matrix2(a, b, x, y);
		print_result7(mine, a, b, c, '2');
	}
	else
	{
		print_graph_color_0('3');
		cct_setcolor();
		mouse_open_1(difficulty, &x, &y);
		spawn(W3, L3, mine3, a, x, y);
		Num_of_mine(mine, a, '3');
		open_matrix3(a, b, x, y);
		print_result7(mine, a, b, c, '3');
	}



	cout << endl << endl;
}

/*情况8*/
void case8(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max])
{
	int spawned = 0;//判断是否已经生成雷阵
	int x, y;
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();
	int ret;
	if (difficulty == '1')
	{
		print_graph_color_0('1');
		cct_setcolor();
		while (1)
		{
			ret = mouse_open_2(difficulty, &x, &y);
			if (ret == MOUSE_RIGHT_BUTTON_CLICK)
			{
				if (c[x][y] == mark)
					c[x][y] = no;
				else if (c[x][y] == no && b[x][y] == HIDE)
					c[x][y] = mark;
				print_result7(mine, a, b, c, '1');

			}
			else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
			{
				if (c[x][y] != mark)
				{
					break;
				}
			}
			else if (ret == 27)//按下ESC
				return;
		}
		spawn(W1, L1, mine1, a, x, y);
		spawned = 1;
		Num_of_mine(mine, a, '1');
		open_matrix(a, b, x, y, 1);
		print_result7(mine, a, b, c, '1');
		while (1)
		{
			if (game_over(b, '1'))
			{
				cct_gotoxy(0, 30);
				cout << "你赢了，游戏结束";
				return;
			}
			if (judge4(x, y, a, b, c))
			{
				print_result7(mine, a, b, c, '1');
				cct_gotoxy(0, 30);
				cout << "你输了，游戏结束";
				return;
			}
			else
			{
				ret = mouse_open_2(difficulty, &x, &y);
				if (ret == MOUSE_RIGHT_BUTTON_CLICK)
				{
					if (c[x][y] == mark)
						c[x][y] = no;
					else if (c[x][y] == no && b[x][y] == HIDE)
						c[x][y] = mark;
				}
				else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					if (c[x][y] != mark)
					{
						open_matrix(a, b, x, y, 1);
					}
				}
				else if (ret == 27)//按下ESC
					return;

				print_result7(mine, a, b, c, '1');
				cct_gotoxy(0, 30);
			}
		}
	}
	else if (difficulty == '2')
	{
		print_graph_color_0('2');
		cct_setcolor();
		while (1)
		{
			ret = mouse_open_2(difficulty, &x, &y);
			if (ret == MOUSE_RIGHT_BUTTON_CLICK)
			{
				if (c[x][y] == mark)
					c[x][y] = no;
				else if (c[x][y] == no && b[x][y] == HIDE)
					c[x][y] = mark;
				print_result7(mine, a, b, c, '2');

			}
			else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
			{
				if (c[x][y] != mark)
				{
					break;
				}
			}
			else if (ret == 27)//按下ESC
				return;
		}
		spawn(W2, L2, mine2, a, x, y);
		spawned = 1;
		Num_of_mine(mine, a, '2');
		open_matrix(a, b, x, y, 2);
		print_result7(mine, a, b, c, '2');
		while (1)
		{
			if (game_over(b, '2'))
			{
				cct_gotoxy(0, 51);
				cout << "你赢了，游戏结束";
				return;
			}
			if (judge4(x, y, a, b, c))
			{
				print_result7(mine, a, b, c, '2');
				cct_gotoxy(0, 51);
				cout << "你输了，游戏结束";
				return;
			}
			else
			{
				cct_gotoxy(0, 51);
				ret = mouse_open_2(difficulty, &x, &y);
				if (ret == MOUSE_RIGHT_BUTTON_CLICK)
				{
					if (c[x][y] == mark)
						c[x][y] = no;
					else if (c[x][y] == no && b[x][y] == HIDE)
						c[x][y] = mark;
				}
				else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					if (c[x][y] != mark)
					{
						open_matrix(a, b, x, y, 2);
					}
				}
				else if (ret == 27)//按下ESC
					return;

				print_result7(mine, a, b, c, '2');
				cct_gotoxy(0, 51);
			}
		}
	}
	else
	{
		print_graph_color_0('3');
		cct_setcolor();
		while (1)
		{
			ret = mouse_open_2(difficulty, &x, &y);
			if (ret == MOUSE_RIGHT_BUTTON_CLICK)
			{
				if (c[x][y] == mark)
					c[x][y] = no;
				else if (c[x][y] == no && b[x][y] == HIDE)
					c[x][y] = mark;
				print_result7(mine, a, b, c, '3');

			}
			else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
			{
				if (c[x][y] != mark)
				{
					break;
				}
			}
			else if (ret == 27)//按下ESC
				return;
		}
		spawn(W3, L3, mine3, a, x, y);
		spawned = 1;
		Num_of_mine(mine, a, '3');
		open_matrix(a, b, x, y, 3);
		print_result7(mine, a, b, c, '3');
		while (1)
		{
			if (game_over(b, '3'))
			{
				cct_gotoxy(0, 51);
				cout << "你赢了，游戏结束";
				return;
			}
			if (judge4(x, y, a, b, c))
			{
				print_result7(mine, a, b, c, '3');
				cct_gotoxy(0, 51);
				cout << "你输了，游戏结束";
				return;
			}
			else
			{
				cct_gotoxy(0, 51);
				ret = mouse_open_2(difficulty, &x, &y);
				if (ret == MOUSE_RIGHT_BUTTON_CLICK)
				{
					if (c[x][y] == mark)
						c[x][y] = no;
					else if (c[x][y] == no && b[x][y] == HIDE)
						c[x][y] = mark;
				}
				else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					if (c[x][y] != mark)
					{
						open_matrix(a, b, x, y, 3);
					}
				}
				else if (ret == 27)//按下ESC
					return;

				print_result7(mine, a, b, c, '3');
				cct_gotoxy(0, 51);
			}
		}
	}

}

/*情况9*/
/*按空格输出时间，根据标记个数显示所剩的雷数*/
void case9(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max])
{
	LARGE_INTEGER tick, begin, end;
	QueryPerformanceFrequency(&tick); //获得时钟频率
	QueryPerformanceCounter(&begin);  //获得初始硬件定时器计数
	int rest = 0;//剩余雷数
	int M = 0;//已被标记
	int spawned = 0;//判断是否已经生成雷阵
	int x, y;
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();
	int ret;

	if (difficulty == '1')
	{
		rest = mine1;
		print_graph_color_0('1');
		cct_gotoxy(0, 0);
		cout << "按ESC退出,按空格显示时间";
		cct_setcolor();
		cct_gotoxy(0, 30);

		while (1)
		{
			ret = mouse_open_2(difficulty, &x, &y);
			if (ret == MOUSE_RIGHT_BUTTON_CLICK)
			{
				if (c[x][y] == mark)
				{
					c[x][y] = no;
					M--;
				}
				else if (c[x][y] == no && b[x][y] == HIDE)
				{
					c[x][y] = mark;
					M++;
				}
				rest = mine1 - M;
				if (rest >= 0)
				{
					cct_gotoxy(0, 0);
					cout << "                                                   ";
					cct_gotoxy(0, 0);
					cout << "剩余雷数: " << rest << ",按ESC退出,按空格显示时间";
					cct_gotoxy(0, 30);
				}
				else
				{
					cct_gotoxy(0, 0);
					cout << "                                                   ";
					cct_gotoxy(0, 0);
					cout << "剩余雷数: 0,按ESC退出,按空格显示时间";
					cct_gotoxy(0, 30);
				}
				print_result7(mine, a, b, c, '1');

			}
			else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
			{
				if (c[x][y] != mark)
				{
					break;
				}
			}
			else if (ret == 27)//按下ESC
				return;
			else if (ret == ' ')//按下SPACE
			{
				cct_gotoxy(0, 0);
				cout << "                                                     ";
				cct_gotoxy(0, 0);
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				cout << ",按ESC退出,按空格显示时间";
				cct_gotoxy(0, 30);
			}
		}
		spawn(W1, L1, mine1, a, x, y);
		spawned = 1;
		Num_of_mine(mine, a, '1');
		open_matrix(a, b, x, y, 1);
		print_result7(mine, a, b, c, '1');
		while (1)
		{
			if (game_over(b, '1'))
			{
				cct_gotoxy(0, 30);
				cout << "你赢了，游戏结束"<<endl;
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				return;
			}
			if (judge4(x, y, a, b, c))
			{
				print_result7(mine, a, b, c, '1');
				cct_gotoxy(0, 30);
				cout << "你输了，游戏结束"<<endl;
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				return;
			}
			else
			{
				ret = mouse_open_2(difficulty, &x, &y);
				if (ret == MOUSE_RIGHT_BUTTON_CLICK)
				{
					if (c[x][y] == mark)
					{
						c[x][y] = no;
						M--;
					}
					else if (c[x][y] == no && b[x][y] == HIDE)
					{
						c[x][y] = mark;
						M++;
					}
					rest = mine1 - M;
					if (rest >= 0)
					{
						cct_gotoxy(0, 0);
						cout << "                                                   ";
						cct_gotoxy(0, 0);
						cout << "剩余雷数: " << rest << ",按ESC退出,按空格显示时间";
						cct_gotoxy(0, 30);
					}
					else
					{
						cct_gotoxy(0, 0);
						cout << "                                                   ";
						cct_gotoxy(0, 0);
						cout << "剩余雷数: 0,按ESC退出,按空格显示时间";
						cct_gotoxy(0, 30);
					}
					print_result7(mine, a, b, c, '1');

				}
				else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					if (c[x][y] != mark)
					{
						open_matrix(a, b, x, y, 1);
					}
					print_result7(mine, a, b, c, '1');
				}
				else if (ret == 27)//按下ESC
					return;
				else if (ret == ' ')//按下SPACE
				{
					cct_gotoxy(0, 0);
					cout << "                                                     ";
					cct_gotoxy(0, 0);
					QueryPerformanceCounter(&end); //获得终止硬件定时器计数
					cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
					cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
						<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
					cct_setcolor();
					cout << ",按ESC退出,按空格显示时间";
					cct_gotoxy(0, 30);
				}
				print_result7(mine, a, b, c, '1');
				cct_gotoxy(0, 30);
			}
		}
	}
	else if (difficulty == '2')
	{
		rest = mine2;
		print_graph_color_0('2');
		cct_gotoxy(0, 0);
		cout << "按ESC退出,按空格显示时间";
		cct_setcolor();
		cct_gotoxy(0, 51);

		while (1)
		{
			ret = mouse_open_2(difficulty, &x, &y);
			if (ret == MOUSE_RIGHT_BUTTON_CLICK)
			{
				if (c[x][y] == mark)
				{
					c[x][y] = no;
					M--;
				}
				else if (c[x][y] == no && b[x][y] == HIDE)
				{
					c[x][y] = mark;
					M++;
				}
				rest = mine2 - M;
				if (rest >= 0)
				{
					cct_gotoxy(0, 0);
					cout << "                                                   ";
					cct_gotoxy(0, 0);
					cout << "剩余雷数: " << rest << ",按ESC退出,按空格显示时间";
					cct_gotoxy(0, 51);
				}
				else
				{
					cct_gotoxy(0, 0);
					cout << "                                                   ";
					cct_gotoxy(0, 0);
					cout << "剩余雷数: 0,按ESC退出,按空格显示时间";
					cct_gotoxy(0, 51);
				}
				print_result7(mine, a, b, c, '1');
				cct_gotoxy(0, 51);
			}
			else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
			{
				if (c[x][y] != mark)
				{
					break;
				}
			}
			else if (ret == 27)//按下ESC
				return;
			else if (ret == ' ')//按下SPACE
			{
				cct_gotoxy(0, 0);
				cout << "                                                     ";
				cct_gotoxy(0, 0);
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				cout << ",按ESC退出,按空格显示时间";
				cct_gotoxy(0, 51);
			}
		}
		spawn(W2, L2, mine2, a, x, y);
		spawned = 1;
		Num_of_mine(mine, a, '2');
		open_matrix(a, b, x, y, 2);
		print_result7(mine, a, b, c, '2');
		while (1)
		{
			if (game_over(b, '2'))
			{
				cct_gotoxy(0, 51);
				cout << "你赢了，游戏结束"<<endl;
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				return;
			}
			if (judge4(x, y, a, b, c))
			{
				print_result7(mine, a, b, c, '2');
				cct_gotoxy(0, 51);
				cout << "你输了，游戏结束"<<endl;
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				return;
			}
			else
			{
				ret = mouse_open_2(difficulty, &x, &y);
				if (ret == MOUSE_RIGHT_BUTTON_CLICK)
				{
					if (c[x][y] == mark)
					{
						c[x][y] = no;
						M--;
					}
					else if (c[x][y] == no && b[x][y] == HIDE)
					{
						c[x][y] = mark;
						M++;
					}
					rest = mine2 - M;
					if (rest >= 0)
					{
						cct_gotoxy(0, 0);
						cout << "                                                   ";
						cct_gotoxy(0, 0);
						cout << "剩余雷数: " << rest << ",按ESC退出,按空格显示时间";
						cct_gotoxy(0, 51);
					}
					else
					{
						cct_gotoxy(0, 0);
						cout << "                                                   ";
						cct_gotoxy(0, 0);
						cout << "剩余雷数: 0,按ESC退出,按空格显示时间";
						cct_gotoxy(0, 51);
					}
					print_result7(mine, a, b, c, '2');

				}
				else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					if (c[x][y] != mark)
					{
						open_matrix(a, b, x, y, 2);
					}
					print_result7(mine, a, b, c, '2');
				}
				else if (ret == 27)//按下ESC
					return;
				else if (ret == ' ')//按下SPACE
				{
					cct_gotoxy(0, 0);
					cout << "                                                     ";
					cct_gotoxy(0, 0);
					QueryPerformanceCounter(&end); //获得终止硬件定时器计数
					cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
					cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
						<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
					cct_setcolor();
					cout << ",按ESC退出,按空格显示时间";
					cct_gotoxy(0, 51);
				}
				print_result7(mine, a, b, c, '2');
				cct_gotoxy(0, 51);
			}
		}
	}
	else
	{
		rest = mine3;
		print_graph_color_0('3');
		cct_gotoxy(0, 0);
		cout << "按ESC退出,按空格显示时间";
		cct_setcolor();
		cct_gotoxy(0, 51);

		while (1)
		{
			ret = mouse_open_2(difficulty, &x, &y);
			if (ret == MOUSE_RIGHT_BUTTON_CLICK)
			{
				if (c[x][y] == mark)
				{
					c[x][y] = no;
					M--;
				}
				else if (c[x][y] == no && b[x][y] == HIDE)
				{
					c[x][y] = mark;
					M++;
				}
				rest = mine3 - M;
				if (rest >= 0)
				{
					cct_gotoxy(0, 0);
					cout << "                                                   ";
					cct_gotoxy(0, 0);
					cout << "剩余雷数: " << rest << ",按ESC退出,按空格显示时间";
					cct_gotoxy(0, 51);
				}
				else
				{
					cct_gotoxy(0, 0);
					cout << "                                                   ";
					cct_gotoxy(0, 0);
					cout << "剩余雷数: 0,按ESC退出,按空格显示时间";
					cct_gotoxy(0, 51);
				}
				print_result7(mine, a, b, c, '3');

			}
			else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
			{
				if (c[x][y] != mark)
				{
					break;
				}
			}
			else if (ret == 27)//按下ESC
				return;
			else if (ret == ' ')//按下SPACE
			{
				cct_gotoxy(0, 0);
				cout << "                                                     ";
				cct_gotoxy(0, 0);
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				cout << ",按ESC退出,按空格显示时间";
				cct_gotoxy(0, 51);
			}
		}
		spawn(W3, L3, mine3, a, x, y);
		spawned = 1;
		Num_of_mine(mine, a, '3');
		open_matrix(a, b, x, y, 3);
		print_result7(mine, a, b, c, '3');
		while (1)
		{
			if (game_over(b, '3'))
			{
				cct_gotoxy(0, 51);
				cout << "你赢了，游戏结束"<<endl;
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				return;
			}
			if (judge4(x, y, a, b, c))
			{
				print_result7(mine, a, b, c, '3');
				cct_gotoxy(0, 51);
				cout << "你输了，游戏结束"<<endl;
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
				cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
				cct_setcolor();
				return;
			}
			else
			{
				ret = mouse_open_2(difficulty, &x, &y);
				if (ret == MOUSE_RIGHT_BUTTON_CLICK)
				{
					if (c[x][y] == mark)
					{
						c[x][y] = no;
						M--;
					}
					else if (c[x][y] == no && b[x][y] == HIDE)
					{
						c[x][y] = mark;
						M++;
					}
					rest = mine3 - M;
					if (rest >= 0)
					{
						cct_gotoxy(0, 0);
						cout << "                                                   ";
						cct_gotoxy(0, 0);
						cout << "剩余雷数: " << rest << ",按ESC退出,按空格显示时间";
						cct_gotoxy(0, 51);
					}
					else
					{
						cct_gotoxy(0, 0);
						cout << "                                                   ";
						cct_gotoxy(0, 0);
						cout << "剩余雷数: 0,按ESC退出,按空格显示时间";
						cct_gotoxy(0, 51);
					}
					print_result7(mine, a, b, c, '3');

				}
				else if (ret == MOUSE_LEFT_BUTTON_CLICK)//按下左键
				{
					if (c[x][y] != mark)
					{
						open_matrix(a, b, x, y, 3);
					}
					print_result7(mine, a, b, c, '3');
				}
				else if (ret == 27)//按下ESC
					return;
				else if (ret == ' ')//按下SPACE
				{
					cct_gotoxy(0, 0);
					cout << "                                                     ";
					cct_gotoxy(0, 0);
					QueryPerformanceCounter(&end); //获得终止硬件定时器计数
					cct_setcolor(COLOR_BLACK, COLOR_YELLOW);
					cout << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(5)
						<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒";
					cct_setcolor();
					cout << ",按ESC退出,按空格显示时间";
					cct_gotoxy(0, 51);
				}
				print_result7(mine, a, b, c, '3');
				cct_gotoxy(0, 51);
			}
		}

	}
}