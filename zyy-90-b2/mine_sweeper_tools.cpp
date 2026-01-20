/*2052521 信13 张耀尹*/
#include<iostream>
#include<iomanip>
#include<Windows.h>
#include <conio.h>
#include "mine_sweeper.h"
#include "cmd_console_tools.h"
using namespace std;
void title(char ch, int& xx, int& yy, int& total)
{
	if (ch == '1')
	{
		xx = 9;
		yy = 9;
		total = 10;
		cout << "  |1 2 3 4 5 6 7 8 9" << endl;
		cout << "--+--------------------" << endl;
	}
	else if (ch == '2')
	{
		xx = 16;
		yy = 16;
		total = 40;
		cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g" << endl;
		cout << "--+----------------------------------" << endl;
	}
	else
	{
		xx = 16;
		yy = 30;
		total = 99;
		cout << "  |1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u" << endl;
		cout << "--+--------------------------------------------------------------" << endl;
	}
}
void mine_keep(char choice, char ch)//base1[9][9] = { 0 }, base2[16][16] = { 0 }, base3[16][30] = { 0 };
{
	int i = 0, ran_num, xx, yy, total;
	int a[100], base[18][32] = { 0 };
	title(ch, xx, yy, total);
	srand((unsigned)time(0));
	while (i < total)
	{
		ran_num = rand() % (xx * yy);
		a[i] = ran_num;
		int hang = ran_num / yy;
		int lie = ran_num % yy;
		base[hang + 1][lie + 1] = 500;
		i++;
	}
	int nine[8] = { 0 };
	for (i = 1; i <= xx; i++)
	{
		cout << static_cast<char>(i + 64) << " |";
		for (int j = 1; j <= yy; j++)
		{
			if (base[i][j] == 500)
			{
				cout << "*" << " ";
				continue;
			}
			else
			{
				int sum = 0;
				nine[0] = base[i - 1][j - 1];
				nine[1] = base[i - 1][j];
				nine[2] = base[i - 1][j + 1];
				nine[3] = base[i][j - 1];
				nine[4] = base[i][j + 1];
				nine[5] = base[i + 1][j - 1];
				nine[6] = base[i + 1][j];
				nine[7] = base[i + 1][j + 1];
				for (int t = 0; t < 8; t++)
					sum = sum + nine[t];
				base[i][j] = sum / 500;
				cout << base[i][j] << " ";
				for (int k = 0; k < 8; k++)
					nine[k] = 0;
			}
		}
		cout << endl;
	}
}
int stop_check(int open_or_not[18][32], int base[18][32], int m1, int m2,int xx,int yy)
{
	if (m1<1 || m2<1 || m1>xx || m2>yy)
		return 0;
	else
	{
		for (int i = m1 - 1; i <= m1 + 1; i++)
		{
			for (int j = m2 - 1; j < m2 + 1; j++)
			{
				if (i != m1 || j != m2)
				{
					if (base[i][j] == 500)
						return 0;
				}
				else
					continue;
			}
		}
	}
	return 1;
}
void open_check(int open_or_not[18][32], int base[18][32], int flag[18][32], int m1, int m2, int xx, int yy, int y_base1, int y_base2)
{
	open_or_not[m1][m2] = 1;
	if (stop_check(open_or_not,base,m1,m2,xx,yy)==1)
		return;
	else
	{
		if (base[m1 - 1][m2 - 1] == 0 )//左上角为计数是0吗 是的话 周围打开 
			open_check(open_or_not, base, flag, m1 - 1, m2 - 1, xx, yy, y_base1, y_base2);
		if (base[m1 - 1][m2] == 0 )//如果不是就不用管左上角 看正上方
			open_check(open_or_not, base, flag, m1 - 1, m2, xx, yy, y_base1, y_base2);
		if (base[m1 - 1][m2 + 1] == 0 )
			open_check(open_or_not, base, flag, m1 - 1, m2 + 1, xx, yy, y_base1, y_base2);
		if (base[m1][m2 - 1] == 0)
			open_check(open_or_not, base, flag, m1, m2 - 1, xx, yy, y_base1, y_base2);
		if (base[m1][m2 + 1] == 0 )
			open_check(open_or_not, base, flag, m1, m2 + 1, xx, yy, y_base1, y_base2);
		if (base[m1 + 1][m2 - 1] == 0 )
			open_check(open_or_not, base, flag, m1 + 1, m2 - 1, xx, yy, y_base1, y_base2);
		if (base[m1 + 1][m2] == 0)
			open_check(open_or_not, base, flag, m1 + 1, m2, xx, yy, y_base1, y_base2);
		if (base[m1 + 1][m2 + 1] == 0)
			open_check(open_or_not, base, flag, m1 + 1, m2 + 1, xx, yy, y_base1, y_base2);
	}
}
/*void open_check(int open_or_not[18][32], int base[18][32], int flag[18][32], int m1, int m2, int xx, int yy, int y_base1, int y_base2)
{
	if (m1 >= 1 && m2 >= 1 && m1 <= y_base1 && m2 <= y_base2)//先把以检查目标为中心的九宫格全记为1
	{
		for (int t = m1 - 1; t <= m1 + 1; t++)
			for (int j = m2 - 1; j <= m2 + 1; j++)
				open_or_not[t][j] = 1;
		flag[m1][m2] = 1;
		if (base[m1 - 1][m2 - 1] == 0 && flag[m1 - 1][m2 - 1] == 0)//左上角为计数是0吗 是的话 周围打开 
			open_check(open_or_not, base, flag, m1 - 1, m2 - 1, xx, yy, y_base1, y_base2);
		if (base[m1 - 1][m2] == 0 && flag[m1 - 1][m2] == 0)//如果不是就不用管左上角 看正上方
			open_check(open_or_not, base, flag, m1 - 1, m2, xx, yy, y_base1, y_base2);
		if (base[m1 - 1][m2 + 1] == 0 && flag[m1 - 1][m2 + 1] == 0)
			open_check(open_or_not, base, flag, m1 - 1, m2 + 1, xx, yy, y_base1, y_base2);
		if (base[m1][m2 - 1] == 0 && flag[m1][m2 - 1] == 0)
			open_check(open_or_not, base, flag, m1, m2 - 1, xx, yy, y_base1, y_base2);
		if (base[m1][m2 + 1] == 0 && flag[m1][m2 + 1] == 0)
			open_check(open_or_not, base, flag, m1, m2 + 1, xx, yy, y_base1, y_base2);
		if (base[m1 + 1][m2 - 1] == 0 && flag[m1 + 1][m2 - 1] == 0)
			open_check(open_or_not, base, flag, m1 + 1, m2 - 1, xx, yy, y_base1, y_base2);
		if (base[m1 + 1][m2] == 0 && flag[m1 + 1][m2] == 0)
			open_check(open_or_not, base, flag, m1 + 1, m2, xx, yy, y_base1, y_base2);
		if (base[m1 + 1][m2 + 1] == 0 && flag[m1 + 1][m2 + 1] == 0)
			open_check(open_or_not, base, flag, m1 + 1, m2 + 1, xx, yy, y_base1, y_base2);
	}
}*/
void mine_open(char choice, char ch, char fstx, char secx)
{
	int i = 0, figure = 500, ran_num, xx, yy, total;
	int a[100], base[18][32] = { 0 }, nine[8] = { 0 }, open_or_not[18][32] = { 0 };
	int flag[18][32] = { 0 };
	title(ch, xx, yy, total);
	srand((unsigned)time(0));
	while (1)
	{
		i = 0, figure = 500;
		for (int t = 0; t < 18; t++)
		{
			for (int j = 0; j < 32; j++)
			{
				base[t][j] = 0;
				open_or_not[t][j] = 0;
			}
		}
		for (int t = 0; t < 8; t++)
			nine[t] = 0;
		for (int t = 0; t < 100; t++)
			a[t] = 0;
		while (i < total)
		{
			int hang = rand() % xx;
			int lie = rand() % yy;
			if (base[hang + 1][lie + 1] == 0)
			{
				base[hang + 1][lie + 1] = 500;
				i++;
			}
		}
		for (int t = 1; t <= xx; t++)
		{
			for (int j = 1; j <= yy; j++)
			{
				if (base[t][j] == 500)
					continue;
				else
				{
					int sum = 0;
					nine[0] = base[t - 1][j - 1];
					nine[1] = base[t - 1][j];
					nine[2] = base[t - 1][j + 1];
					nine[3] = base[t][j - 1];
					nine[4] = base[t][j + 1];
					nine[5] = base[t + 1][j - 1];
					nine[6] = base[t + 1][j];
					nine[7] = base[t + 1][j + 1];
					for (int t = 0; t < 8; t++)
						sum = sum + nine[t];
					base[t][j] = sum / 500;
					for (int k = 0; k < 8; k++)
						nine[k] = 0;
				}
			}
		}
		int chu = 0, moo, y_base1, y_base2 = 0;
		moo = xx + fstx - 48 - 6, y_base1 = fstx - 64;
		if (secx >= '1' && secx <= '9')
		{
			chu = 2 * (secx - 48) + 1;
			y_base2 = secx - 48;
		}
		else if (secx >= 'a' && secx <= 'u')
		{
			chu = 2 * (secx - 87) + 1;
			y_base2 = secx - 87;
		}
		figure = base[y_base1][y_base2] + 48;
		if (figure == 48)
		{
			open_check(open_or_not, base, flag,y_base1, y_base2, xx, yy, y_base1, y_base2);
			for (int t = 1; t <= xx; t++)
			{
				cout << static_cast<char>(t + 64) << " |";
				for (int j = 1; j <= yy; j++)
				{
					if (base[t][j] == 500)
						cout << "* ";
					else
					{
						if (open_or_not[t][j] == 1)
						{
							cct_showch(2 * j + 1, 10 + t + xx, char(base[t][j] + 48), 14, 1, 1);
							cct_setcolor();
							cout << " ";
						}
						else
							cout << base[t][j] << " ";
					}
				}
				cout << endl;
			}
			//break;
			cout << endl;
			for (int t = 1; t <= xx; t++)
			{
				cout << static_cast<char>(t + 64) << " |";
				for (int j = 1; j <= yy; j++)
				{
					cout << open_or_not[t][j] << " ";
				}
				cout << endl;
			}
			break;
		}
	}

}


void mine_hide(char choice, char ch)
{
	int i = 0, ran_num, xx, yy, total;
	int a[100], base[18][32] = { 0 };
	title(ch, xx, yy, total);
	for (i = 1; i <= xx; i++)
	{
		cout << static_cast<char>(i + 64) << " |";
		for (int j = 1; j <= yy; j++)
			cout << "X ";
		cout << endl;
	}
}
void first_input(char choice, char ch, char& fstx, char& secx)
{
	char end_letter;
	if (ch == '1')
		end_letter = 'I';
	else
		end_letter = 'P';
	int flag = 0;
	while (1)
	{
		if ((fstx = _getch()) >= 'A' && fstx <= end_letter)
		{
			cout << fstx;
			break;
		}
		if (fstx == 'Q' || fstx == 'q')
		{
			cout << fstx;
			flag = 1;
			break;
		}
	}
	if (flag == 0)
	{
		while (1)
		{
			if ((ch == '1' && (secx = _getch()) >= '1' && secx <= '9')
				|| (ch == '2' && ((secx = _getch()) >= '1' && secx <= '9') || (secx >= 'a' && secx <= 'g'))
				|| (ch == '3' && ((secx = _getch()) >= '1' && secx <= '9') || (secx >= 'a' && secx <= 'u')))
			{
				cout << secx;
				break;
			}
		}
	}
}
void fun1(char choice)
{
	char ch;
	sub_menu(ch);
	mine_keep(choice, ch);
	cout << endl << endl << endl;
	cout << "按回车键继续!";
}
void fun2(char choice)
{
	char ch, fstx, secx;
	sub_menu(ch);
	mine_hide(choice, ch);
	cout << endl << endl << endl;
	cout << "输入非雷位置的行列坐标（先行后列，严格区分大小写，例：G1/Af，按Q/q退出）：";
	first_input(choice, ch, fstx, secx);
	if (fstx != 'Q' && fstx != 'q')
	{
		cout << endl << endl << "点开后的数组：" << endl;
		mine_open(choice, ch, fstx, secx);
		getchar();
	}
	cout << endl;
	cout << "按回车键继续!";
}
void fun3(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}
void fun4(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}
void fun5(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}
void fun6(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}
void fun7(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}
void fun8(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}
void fun9(char choice)
{
	cout << "ABCDE" << endl;
	cout << endl;
	cout << "按回车键继续!";
}