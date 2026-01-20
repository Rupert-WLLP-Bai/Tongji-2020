/*2052526 信15 白俊豪*/
#include<iostream>
#include<time.h>
#include<Windows.h>
using namespace std;

int Num_of_mine(int i, int j, bool mine[12][28])
{
	int num;
	num = mine[i - 1][j - 1] + mine[i][j - 1] + mine[i + 1][j - 1] + mine[i - 1][j]
		+ mine[i + 1][j] + mine[i - 1][j + 1] + mine[i][j + 1] + mine[i + 1][j + 1];
	return num;
}

void print_result(bool mine[][28], int a[][28])
{
	for (int i = 1; i < 11; i++)
	{
		for (int j = 1; j < 27; j++)
		{
			if (mine[i][j] == 0)
				cout << a[i][j] << " ";
			else
				cout << "* ";
		}
		cout << endl;
	}
}

int main()
{
	int count = 0;
	int i, j;//定位雷的行号和列号
	int a[12][28] = { 0 };//最外一圈是0
	bool mine[12][28] = { 0 };//无雷为0，有雷为1
	srand((unsigned int)(time(NULL)));
	while (count != 50)
	{
		i = rand() % 10 + 1;
		j = rand() % 26 + 1;
		if (mine[i][j] == 0)
		{
			mine[i][j] = 1;
			count++;
		}
	}

	for (int i = 1; i < 11; i++)
	{
		for (int j = 1; j < 27; j++)
		{
			a[i][j] = Num_of_mine(i, j, mine);
		}
	}

	int change_i, change_j, change;
re : 
	change = rand() % 5 - 2;
	if (change == 0)
		goto re;
re2:
	change_i = rand() % 10 + 1;
	change_j = rand() % 26 + 1;
	if (mine[change_i][change_j] == 1)
		goto re2;
	a[change_i][change_j] += change;
	print_result(mine, a);
	return 0;
}