/*2052526 信15 白俊豪*/
#include<iostream>
#include<cmath>
#include<iomanip>
#include<Windows.h>
#include"cmd_console_tools.h"
#include"mine_sweeper.h"

using namespace std;

/*雷阵重置*/
void reset(int(*mine)[L_max], int(*a)[L_max])
{
	for (int i = 0; i < W_max; i++)
	{
		for (int j = 0; j < L_max; j++)
		{
			mine[i][j] = a[i][j] = 0;
		}
	}

}

/*输入长度,宽度,雷数生成对应的雷阵*/	/*传入的数组是表示有雷或无雷的数组*/ /*需要保证第一次输入位置周围的八个位置的a均的为0*/
void spawn(int width, int length, int num, int(*a)[L_max], int x, int y)
{
	int count = 0;
	int i, j;//表示行号和列号
	srand((unsigned int)(time(NULL) * time(NULL)));

	/*cout << "无雷位置的坐标为 : " << "(" << static_cast<char>(x + 'A' - 1) << "," << y << ")" << endl;*/

	while (count != num)
	{
		i = rand() % width + 1;
		j = rand() % length + 1;
		if ((i - x) <= 1 && (i - x) >= -1 && (j - y) <= 1 && (j - y) >= -1)//九个位置无雷
			continue;
		else
		{
			if (a[i][j] == no)
			{
				a[i][j] = yes;
				count++;
			}
			else
				continue;
		}


		//cout << "生成雷的位置为 : " << "(" << static_cast<char>(i + 'A' - 1) << "," << j << ")" << endl;
		//cout << "x = " << x << " y = " << y << " i = " << i << " j = " << j << endl;
		//cout << "坐标的差值为 : " << "delta_x = " << i - x << "  " << "delta_y = " << i - y << endl;
		//cout << "结果为 :" << endl;
		//Num_of_mine(mine, a, '1');
		//print_result(mine, a, '1');
		//cout << endl;
		////Sleep(1000);

	}
}

/*用于生成三种情况下的随机雷阵*/
void spawn_mine(int a[][L_max], int select, int x, int y)
{
	switch (select)
	{
		case '1':
			spawn(W1, L1, mine1, a, x, y);
			break;
		case '2':
			spawn(W2, L2, mine2, a, x, y);
			break;
		case '3':
			spawn(W3, L3, mine3, a, x, y);
			break;
		default:
			break;
	}
}

/*计算雷数*/	/*传入数组为存雷数的数组,表示有雷或无雷的数组*/
void Num_of_mine(int(*mine)[L_max], int(*a)[L_max], int select)
{
	switch (select)
	{
		case '1':
			for (int i = 1; i < W1 + 1; i++)
				for (int j = 1; j < L1 + 1; j++)
					mine[i][j] = a[i - 1][j - 1] + a[i][j - 1]
					+ a[i + 1][j - 1] + a[i - 1][j]
					+ a[i + 1][j] + a[i - 1][j + 1]
					+ a[i][j + 1] + a[i + 1][j + 1];
			break;
		case '2':
			for (int i = 1; i < W2 + 1; i++)
				for (int j = 1; j < L2 + 1; j++)
					mine[i][j] = a[i - 1][j - 1] + a[i][j - 1]
					+ a[i + 1][j - 1] + a[i - 1][j]
					+ a[i + 1][j] + a[i - 1][j + 1]
					+ a[i][j + 1] + a[i + 1][j + 1];
			break;
		case'3':
			for (int i = 1; i < W3 + 1; i++)
				for (int j = 1; j < L3 + 1; j++)

					mine[i][j] = a[i - 1][j - 1] + a[i][j - 1]
					+ a[i + 1][j - 1] + a[i - 1][j]
					+ a[i + 1][j] + a[i - 1][j + 1]
					+ a[i][j + 1] + a[i + 1][j + 1];
			break;
	}
}

/*输出雷阵,情况1*/
void print_result(int(*mine)[L_max], int(*a)[L_max], int select)
{
	cout << "内部数组：" << endl;
	switch (select)
	{
		case '1':
			cout << "  |1 2 3 4 5 6 7 8 9" << endl;
			cout << "--+--------------------" << endl;
			for (int i = 1; i < W1 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L1 + 1; j++)
				{
					if (a[i][j] == 0)
						cout << mine[i][j] << " ";
					else
						cout << "* ";
				}
				cout << endl;
			}
			break;

		case '2':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g" << endl;
			cout << "--+----------------------------------" << endl;
			for (int i = 1; i < W2 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L2 + 1; j++)
				{
					if (a[i][j] == 0)
						cout << mine[i][j] << " ";
					else
						cout << "* ";
				}
				cout << endl;
			}
			break;

		case '3':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u" << endl;
			cout << "--+--------------------------------------------------------------" << endl;
			for (int i = 1; i < W3 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L3 + 1; j++)
				{
					if (a[i][j] == 0)
						cout << mine[i][j] << " ";
					else
						cout << "* ";
				}
				cout << endl;
			}
			break;
	}
}

/*输出带'X'的雷阵,情况2*/
void print_process2(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int select)
{
	cout << endl << endl << "点开后的数组：" << endl;
	switch (select)
	{
		case '1':
			cout << "  |1 2 3 4 5 6 7 8 9" << endl;
			cout << "--+--------------------" << endl;
			for (int i = 1; i < W1 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L1 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;

		case '2':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g" << endl;
			cout << "--+----------------------------------" << endl;
			for (int i = 1; i < W2 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L2 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;

		case '3':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u" << endl;
			cout << "--+--------------------------------------------------------------" << endl;
			for (int i = 1; i < W3 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L3 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;
	}
}

/*输出带'X'的雷阵,情况3*/
void print_process3(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int select)
{
	cout << endl << endl;
	cout << "当前数组：" << endl;
	switch (select)
	{
		case '1':
			cout << "  |1 2 3 4 5 6 7 8 9" << endl;
			cout << "--+--------------------" << endl;
			for (int i = 1; i < W1 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L1 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;

		case '2':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g" << endl;
			cout << "--+----------------------------------" << endl;
			for (int i = 1; i < W2 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L2 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;

		case '3':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u" << endl;
			cout << "--+--------------------------------------------------------------" << endl;
			for (int i = 1; i < W3 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L3 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;
	}
}

/*输出带'X'的雷阵,情况4*/
void print_process4(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max], int select)
{
	cout << endl << endl;
	cout << "当前数组：" << endl;
	switch (select)
	{
		case '1':
			cout << "  |1 2 3 4 5 6 7 8 9" << endl;
			cout << "--+--------------------" << endl;
			for (int i = 1; i < W1 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L1 + 1; j++)
				{
					if (b[i][j] == OPEN && c[i][j] == no)//打开且未被标记
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						if (c[i][j] == no)//未被标记
							cout << "X ";
						else//被标记
						{
							cct_setcolor(COLOR_RED, COLOR_WHITE);
							cout << "X";
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
					}
				}
				cout << endl;
			}
			break;

		case '2':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g" << endl;
			cout << "--+----------------------------------" << endl;
			for (int i = 1; i < W2 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L2 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							if (c[i][j] == no)//未被标记
								cout << "X ";
							else//被标记
							{
								cct_setcolor(COLOR_RED, COLOR_WHITE);
								cout << "X";
								cct_setcolor(COLOR_BLACK, COLOR_WHITE);
								cout << " ";
							}
					}
					else
					{
						cout << "X ";
					}
				}
				cout << endl;
			}
			break;

		case '3':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u" << endl;
			cout << "--+--------------------------------------------------------------" << endl;
			for (int i = 1; i < W3 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L3 + 1; j++)
				{
					if (b[i][j] == OPEN)
					{
						if (a[i][j] == 0)
						{
							cct_setcolor(COLOR_HYELLOW, mine[i][j]);
							cout << mine[i][j];
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
						else
							cout << "* ";
					}
					else
					{
						if (c[i][j] == no)//未被标记
							cout << "X ";
						else//被标记
						{
							cct_setcolor(COLOR_RED, COLOR_WHITE);
							cout << "X";
							cct_setcolor(COLOR_BLACK, COLOR_WHITE);
							cout << " ";
						}
					}
				}
				cout << endl;
			}
			break;
	}
}

/*初始情况,带'X'的雷阵*/
void print_X(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int select)
{
	cout << "内部数组：" << endl;
	switch (select)
	{
		case '1':
			cout << "  |1 2 3 4 5 6 7 8 9" << endl;
			cout << "--+--------------------" << endl;
			for (int i = 1; i < W1 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L1 + 1; j++)
					cout << "X ";
				cout << endl;
			}
			break;

		case '2':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g" << endl;
			cout << "--+----------------------------------" << endl;
			for (int i = 1; i < W2 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L2 + 1; j++)
					cout << "X ";
				cout << endl;
			}
			break;

		case '3':
			cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u" << endl;
			cout << "--+--------------------------------------------------------------" << endl;
			for (int i = 1; i < W3 + 1; i++)
			{
				cout << static_cast<char>('A' + i - 1) << " |";
				for (int j = 1; j < L3 + 1; j++)
					cout << "X ";
				cout << endl;
			}
			break;
	}
}

/*检查是否踩雷,如果踩雷输出游戏结束,并标记出雷的位置*/
/*返回值为1表示踩雷,否则返回0*//*情况3*/
int judge(const int x, const int y, int(*a)[L_max])
{
	if (a[x][y] == yes)
		return 1;
	return 0;
}


/*检查是否踩雷,如果踩雷输出游戏结束,并标记出雷的位置*/
/*返回值为1表示踩雷,否则返回0*//*情况3*/
int judge4(const int x, const int y, int(*a)[L_max], int(*b)[L_max], int(*c)[L_max])
{
	if (a[x][y] == yes && c[x][y] == no && b[x][y] == OPEN)//未被标记且被点开
		return 1;
	return 0;
}

/*检查是否找出所有雷*/
int game_over(int(*b)[L_max], int select)
{
	int cnt = 0;
	switch (select)
	{
		case '1':
			for (int i = 1; i <= W1; i++)
				for (int j = 1; j <= L1; j++)
				{
					if (b[i][j] == OPEN)
						cnt++;
				}
			if (cnt == L1 * W1 - mine1)
				return 1;
			else
				return 0;
			break;

		case'2':
			for (int i = 1; i <= W2; i++)
				for (int j = 1; j <= L2; j++)
				{
					if (b[i][j] == OPEN)
						cnt++;
				}
			if (cnt == L2 * W2 - mine2)
				return 1;
			else
				return 0;
			break;
		case'3':
			for (int i = 1; i <= W3; i++)
				for (int j = 1; j <= L3; j++)
				{
					if (b[i][j] == OPEN)
						cnt++;
				}
			if (cnt == L3 * W3 - mine3)
				return 1;
			else
				return 0;
			break;
	}

	return -1;
}

/*情况1*/
void case1(int(*mine)[L_max], int(*a)[L_max])
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
		print_result(mine, a, '1');
	}
	else if (difficulty == '2')
	{
		x = rand() % W2 + 1;
		y = rand() % L2 + 1;
		spawn(W2, L2, mine2, a, x, y);
		Num_of_mine(mine, a, '2');
		print_result(mine, a, '2');
	}
	else
	{
		x = rand() % W3 + 1;
		y = rand() % L3 + 1;
		spawn(W3, L3, mine3, a, x, y);
		Num_of_mine(mine, a, '3');
		print_result(mine, a, '3');
	}
	reset(mine, a);
}

/*情况2*//*如果数组open的值为1则输出数字否则输出X,输出数字的颜色,背景色另外定义*/
void case2(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max])
{
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();

	int x, y;
	int quit = 1;


	if (difficulty == '1')
	{
		print_X(mine, a, b, '1');
		quit = fetch_instruction(&x, &y, '1', 0);
		if (!quit)
			return;
		spawn(W1, L1, mine1, a, x, y);
		Num_of_mine(mine, a, '1');
		open_matrix1(a, b, x, y);
		print_process2(mine, a, b, '1');
	}
	else if (difficulty == '2')
	{
		print_X(mine, a, b, '2');
		quit = fetch_instruction(&x, &y, '2', 0);
		if (!quit)
			return;
		spawn(W2, L2, mine2, a, x, y);
		Num_of_mine(mine, a, '2');
		open_matrix2(a, b, x, y);
		print_process2(mine, a, b, '2');
	}
	else
	{
		print_X(mine, a, b, '3');
		quit = fetch_instruction(&x, &y, '3', 0);
		if (!quit)
			return;
		spawn(W3, L3, mine3, a, x, y);
		Num_of_mine(mine, a, '3');
		open_matrix3(a, b, x, y);
		print_process2(mine, a, b, '3');
	}
	reset(mine, a);
	reset(mine, b);
}

/*情况3*//*需要判断是否点到雷*/
void case3(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max])
{
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();
	int x, y;
	int quit = 1;

	if (difficulty == '1')
	{
		print_X(mine, a, b, '1');
		quit = fetch_instruction(&x, &y, '1', 0);//获取输入
		if (!quit)
			return;

		spawn(W1, L1, mine1, a, x, y);
		Num_of_mine(mine, a, '1');
		open_matrix1(a, b, x, y);
		print_process3(mine, a, b, '1');

		while (1)
		{
			if (game_over(b, '1'))
			{
				cout << endl << endl << "你赢了,游戏结束";
				return;
			}
			quit = fetch_instruction(&x, &y, '1', 0);//获取输入
			if (!quit)
				return;
			open_matrix(a, b, x, y, 1);

			if (judge(x, y, a))
			{
				print_process3(mine, a, b, '1');
				cout << endl << endl << "你输了,游戏结束";
				return;
			}
			else
			{
				print_process3(mine, a, b, '1');
			}
		}
	}

	else if (difficulty == '2')
	{
		print_X(mine, a, b, '2');
		quit = fetch_instruction(&x, &y, '2', 0);//获取输入
		if (!quit)
			return;

		spawn(W2, L2, mine2, a, x, y);
		Num_of_mine(mine, a, '2');
		open_matrix2(a, b, x, y);
		print_process3(mine, a, b, '2');

		while (1)
		{
			if (game_over(b, '2'))
			{
				cout << endl << endl << "你赢了,游戏结束";
				return;
			}
			quit = fetch_instruction(&x, &y, '2', 0);//获取输入
			if (!quit)
				return;
			open_matrix(a, b, x, y, 2);

			if (judge(x, y, a))
			{
				print_process3(mine, a, b, '2');
				cout << endl << endl << "你输了,游戏结束";
				return;
			}
			else
			{
				print_process3(mine, a, b, '2');
			}
		}
	}

	else
	{
		print_X(mine, a, b, '3');
		quit = fetch_instruction(&x, &y, '3', 0);//获取输入
		if (!quit)
			return;

		spawn(W3, L3, mine3, a, x, y);
		Num_of_mine(mine, a, '3');
		open_matrix3(a, b, x, y);
		print_process3(mine, a, b, '3');

		while (1)
		{
			if (game_over(b, '3'))
			{
				cout << endl << endl << "你赢了,游戏结束";
				return;
			}
			quit = fetch_instruction(&x, &y, '3', 0);//获取输入
			if (!quit)
				return;
			open_matrix(a, b, x, y, 3);

			if (judge(x, y, a))
			{
				print_process3(mine, a, b, '3');
				cout << endl << endl << "你输了,游戏结束";
				return;
			}
			else
			{
				print_process3(mine, a, b, '3');
			}
		}
	}

}

/*情况4*//*在情况3的基础上加入 标记,取消标记,显示时间的功能*/
	   /*判断游戏结束的条件需要改变*/
void case4(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max])
{
	int spawned = 0;//判断是否已经生成雷阵
	print_choose_difficulty();
	int difficulty;
	get_difficulty(&difficulty);
	cct_cls();
	int x, y;
	int quit = 1;
	LARGE_INTEGER tick, begin, end;
	QueryPerformanceFrequency(&tick); //获得时钟频率
	QueryPerformanceCounter(&begin);  //获得初始硬件定时器计数


	if (difficulty == '1')
	{
		print_X(mine, a, b, '1');
		quit = fetch_instruction(&x, &y, '1', 1);//获取输入,功能开启
		if (!quit)
			return;
		if (quit == 2)//标记功能
		{
			if (b[x][y] == HIDE)
				c[x][y] = mark;
			print_process4(mine, a, b, c, '1');

		}
		if (quit == -2)//显示时间
		{
			QueryPerformanceCounter(&end); //获得终止硬件定时器计数
			cout << endl << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(3)
				<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;

		}
		if (quit == -1)//取消标记
		{
			c[x][y] = no;
			print_process4(mine, a, b, c, '1');

		}
		else if (quit == 1)
		{
			spawn(W1, L1, mine1, a, x, y);
			spawned = 1;
			Num_of_mine(mine, a, '1');
			open_matrix(a, b, x, y, 1);
			print_process4(mine, a, b, c, '1');
		}
		while (1)
		{
			if (game_over(b, '1'))
			{
				cout << endl << endl << "你赢了,游戏结束";
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "耗时 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				return;
			}
			quit = fetch_instruction(&x, &y, '1', 1);//获取输入,功能开启
			if (!quit)
				return;
			if (quit == 2)//标记功能
			{
				if (b[x][y] == HIDE)
					c[x][y] = mark;
				print_process4(mine, a, b, c, '1');
				continue;
			}
			if (quit == -2)//显示时间
			{
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				continue;
			}
			if (quit == -1)//取消标记
			{
				c[x][y] = no;
				print_process4(mine, a, b, c, '1');
				continue;
			}
			else if (quit == 1)
			{
				if (!spawned)
				{
					spawn(W1, L1, mine1, a, x, y);
					spawned = 1;
					Num_of_mine(mine, a, '1');
				}
				if (c[x][y] == no)
					open_matrix(a, b, x, y, 1);
			}

			if (judge(x, y, a))
			{
				print_process4(mine, a, b, c, '1');
				cout << endl << endl << "你输了,游戏结束";
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "耗时 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				return;
			}
			else
			{
				print_process4(mine, a, b, c, '1');
			}
		}
	}

	else if (difficulty == '2')
	{
		print_X(mine, a, b, '1');
		quit = fetch_instruction(&x, &y, '2', 1);//获取输入,功能开启
		if (!quit)
			return;
		if (quit == 2)//标记功能
		{
			if (b[x][y] == HIDE)
				c[x][y] = mark;
			print_process4(mine, a, b, c, '2');

		}
		if (quit == -2)//显示时间
		{
			QueryPerformanceCounter(&end); //获得终止硬件定时器计数
			cout << endl << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(3)
				<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;

		}
		if (quit == -1)//取消标记
		{
			c[x][y] = no;
			print_process4(mine, a, b, c, '2');

		}
		else if (quit == 1)
		{
			spawn(W2, L2, mine2, a, x, y);
			spawned = 1;
			Num_of_mine(mine, a, '2');
			open_matrix(a, b, x, y, 2);
			print_process4(mine, a, b, c, '2');
		}
		while (1)
		{
			if (game_over(b, '2'))
			{
				cout << endl << endl << "你赢了,游戏结束";
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "耗时 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				return;
			}
			quit = fetch_instruction(&x, &y, '2', 1);//获取输入,功能开启
			if (!quit)
				return;
			if (quit == 2)//标记功能
			{
				if (b[x][y] == HIDE)
					c[x][y] = mark;
				print_process4(mine, a, b, c, '2');
				continue;
			}
			if (quit == -2)//显示时间
			{
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				continue;
			}
			if (quit == -1)//取消标记
			{
				c[x][y] = no;
				print_process4(mine, a, b, c, '2');
				continue;
			}
			else if (quit == 1)
			{
				if (!spawned)
				{
					spawn(W2, L2, mine2, a, x, y);
					spawned = 1;
					Num_of_mine(mine, a, '2');
				}
				if (c[x][y] == no)
					open_matrix(a, b, x, y, 2);
			}

			if (judge(x, y, a))
			{
				print_process4(mine, a, b, c, '2');
				cout << endl << endl << "你输了,游戏结束";
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "耗时 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				return;
			}
			else
			{
				print_process4(mine, a, b, c, '2');
			}
		}
	}

	else
	{
		print_X(mine, a, b, '3');
		quit = fetch_instruction(&x, &y, '3', 1);//获取输入,功能开启
		if (!quit)
			return;
		if (quit == 2)//标记功能
		{
			if (b[x][y] == HIDE)
				c[x][y] = mark;
			print_process4(mine, a, b, c, '3');

		}
		if (quit == -2)//显示时间
		{
			QueryPerformanceCounter(&end); //获得终止硬件定时器计数
			cout << endl << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(3)
				<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;

		}
		if (quit == -1)//取消标记
		{
			c[x][y] = no;
			print_process4(mine, a, b, c, '3');

		}
		else if (quit == 1)
		{
			spawn(W3, L3, mine3, a, x, y);
			spawned = 1;
			Num_of_mine(mine, a, '3');
			open_matrix(a, b, x, y, 3);
			print_process4(mine, a, b, c, '3');
		}
		while (1)
		{
			if (game_over(b, '3'))
			{
				cout << endl << endl << "你赢了,游戏结束";
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "耗时 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				return;
			}
			quit = fetch_instruction(&x, &y, '3', 1);//获取输入,功能开启
			if (!quit)
				return;
			if (quit == 2)//标记功能
			{
				if (b[x][y] == HIDE)
					c[x][y] = mark;
				print_process4(mine, a, b, c, '3');
				continue;
			}
			if (quit == -2)//显示时间
			{
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "已运行时间 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				continue;
			}
			if (quit == -1)//取消标记
			{
				c[x][y] = no;
				print_process4(mine, a, b, c, '3');
				continue;
			}
			else if (quit == 1)
			{
				if (!spawned)
				{
					spawn(W1, L1, mine1, a, x, y);
					spawned = 1;
					Num_of_mine(mine, a, '3');
				}
				if (c[x][y] == no)
					open_matrix(a, b, x, y, 3);
			}

			if (judge(x, y, a))
			{
				print_process4(mine, a, b, c, '3');
				cout << endl << endl << "你输了,游戏结束";
				QueryPerformanceCounter(&end); //获得终止硬件定时器计数
				cout << endl << "耗时 :" << setiosflags(ios::fixed) << setprecision(3)
					<< double(end.QuadPart - begin.QuadPart) / tick.QuadPart << "秒" << endl;
				return;
			}
			else
			{
				print_process4(mine, a, b, c, '3');
			}
		}
	}
}

/*整合打开雷阵的函数*/
void open_matrix(int(*a)[L_max], int(*b)[L_max], int x, int y, int select)
{
	if (select == 1)
		open_matrix1(a, b, x, y);
	else if (select == 2)
		open_matrix2(a, b, x, y);
	else
		open_matrix3(a, b, x, y);

}
/*递归处理打开的位置*//*情况1*/
void open_matrix1(int(*a)[L_max], int(*b)[L_max], int x, int y)
{
	b[x][y] = OPEN;
	/*cout << "x = " << x << " y = " << y << endl;
	for (int i = 1; i <= W1; i++)
	{
		for (int j = 1; j <= L1; j++)
		{
			cout << b[i][j];
		}
		cout << endl;
	}*/
	if (!stop1(a, b, x, y))//递归终止条件
		return;
	else//打开周围8个格子
	{
		if (b[x - 1][y - 1] == HIDE)
			open_matrix(a, b, x - 1, y - 1, 1);
		if (b[x - 1][y] == HIDE)
			open_matrix(a, b, x - 1, y, 1);
		if (b[x - 1][y + 1] == HIDE)
			open_matrix(a, b, x - 1, y + 1, 1);
		if (b[x][y - 1] == HIDE)
			open_matrix(a, b, x, y - 1, 1);
		if (b[x][y + 1] == HIDE)
			open_matrix(a, b, x, y + 1, 1);
		if (b[x + 1][y - 1] == HIDE)
			open_matrix(a, b, x + 1, y - 1, 1);
		if (b[x + 1][y] == HIDE)
			open_matrix(a, b, x + 1, y, 1);
		if (b[x + 1][y + 1] == HIDE)
			open_matrix(a, b, x + 1, y + 1, 1);
	}
}

/*周围有雷返回0,遇到边界返回0,否则返回1*//*情况1*/
bool stop1(int(*a)[L_max], int(*b)[L_max], int x, int y)
{
	if (x < 1 || x > W1 || y < 1 || y > L1)//边界条件
		return 0;
	for (int i = x - 1; i <= x + 1; i++)
	{
		for (int j = y - 1; j <= y + 1; j++)
		{
			if (i == x && j == y)//不检查自己
				continue;
			else
			{
				if (a[i][j] == 1)
					return 0;
			}
		}
	}
	return 1;
}

/*递归处理打开的位置*//*情况2*/
void open_matrix2(int(*a)[L_max], int(*b)[L_max], int x, int y)
{
	b[x][y] = OPEN;
	if (!stop2(a, b, x, y))//递归终止条件
	{
		return;
	}
	else//打开周围8个格子
	{
		if (b[x - 1][y - 1] == HIDE)
			open_matrix(a, b, x - 1, y - 1, 2);
		if (b[x - 1][y] == HIDE)
			open_matrix(a, b, x - 1, y, 2);
		if (b[x - 1][y + 1] == HIDE)
			open_matrix(a, b, x - 1, y + 1, 2);
		if (b[x][y - 1] == HIDE)
			open_matrix(a, b, x, y - 1, 2);
		if (b[x][y + 1] == HIDE)
			open_matrix(a, b, x, y + 1, 2);
		if (b[x + 1][y - 1] == HIDE)
			open_matrix(a, b, x + 1, y - 1, 2);
		if (b[x + 1][y] == HIDE)
			open_matrix(a, b, x + 1, y, 2);
		if (b[x + 1][y + 1] == HIDE)
			open_matrix(a, b, x + 1, y + 1, 2);
	}
}

/*周围有雷返回0,遇到边界返回0,否则返回1*//*情况2*/
bool stop2(int(*a)[L_max], int(*b)[L_max], int x, int y)
{
	if (x < 1 || x > W2 || y < 1 || y > L2)//边界条件
		return 0;
	for (int i = x - 1; i <= x + 1; i++)
	{
		for (int j = y - 1; j <= y + 1; j++)
		{
			if (i == x && j == y)//不检查自己
				continue;
			else
			{
				if (a[i][j] == 1)
					return 0;
			}
		}
	}
	return 1;
}

/*递归处理打开的位置*//*情况3*/
void open_matrix3(int(*a)[L_max], int(*b)[L_max], int x, int y)
{
	b[x][y] = OPEN;

	/*测试代码*/
	/*cout << endl << "x = " << x << " y = " << y << endl;
	for (int i = 1; i <= W3; i++)
	{
		for (int j = 1; j <= L3; j++)
		{
			cout << b[i][j]<<" ";
		}
		cout << endl;
	}
	cout << endl;
	for (int i = 1; i <= W3; i++)
	{
		for (int j = 1; j <= L3; j++)
		{
			if (a[i][j] == 0)
				cout << a[i][j] << " ";
			else
			{
				cct_setcolor(COLOR_RED, COLOR_BLUE);
				cout << "*";
				cct_setcolor(COLOR_BLACK, COLOR_WHITE);
				cout << " ";
			}
		}
		cout << endl;
	}*/

	if (!stop3(a, b, x, y))//递归终止条件
		return;

	else//打开周围8个格子
	{
		if (b[x - 1][y - 1] == HIDE)
			open_matrix(a, b, x - 1, y - 1, 3);
		if (b[x - 1][y] == HIDE)
			open_matrix(a, b, x - 1, y, 3);
		if (b[x - 1][y + 1] == HIDE)
			open_matrix(a, b, x - 1, y + 1, 3);
		if (b[x][y - 1] == HIDE)
			open_matrix(a, b, x, y - 1, 3);
		if (b[x][y + 1] == HIDE)
			open_matrix(a, b, x, y + 1, 3);
		if (b[x + 1][y - 1] == HIDE)
			open_matrix(a, b, x + 1, y - 1, 3);
		if (b[x + 1][y] == HIDE)
			open_matrix(a, b, x + 1, y, 3);
		if (b[x + 1][y + 1] == HIDE)
			open_matrix(a, b, x + 1, y + 1, 3);
	}
}

/*周围有雷返回0,遇到边界返回0,否则返回1*//*情况3*/
bool stop3(int(*a)[L_max], int(*b)[L_max], int x, int y)
{
	if (x < 1 || x > W3 || y < 1 || y > L3)//边界条件
		return 0;
	for (int i = x - 1; i <= x + 1; i++)
	{
		for (int j = y - 1; j <= y + 1; j++)
		{
			if (i == x && j == y)//不检查自己
				continue;
			else
			{
				if (a[i][j] == 1)
					return 0;
			}
		}
	}
	return 1;
}

