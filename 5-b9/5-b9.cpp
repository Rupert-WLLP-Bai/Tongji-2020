/*2052526 信15 白俊豪*/
#include<iostream>
using namespace std;


bool judge(int a[9][9])
{
	int i, j;
	bool LINE[9][9] = { 0 };//前一个元素代表位置,后一个元素代表1-9中的某一个数
	bool COLUMN[9][9] = { 0 };
	bool BLOCK[9][9] = { 0 };
	for (i = 0; i < 9; i++)
	{
		for (j = 0; j < 9; j++)
		{
			int block_position = i / 3 * 3 + j / 3;
			LINE[i][a[i][j]-1] = 1;
			COLUMN[j][a[i][j]-1]= 1;
			BLOCK[block_position][a[i][j]-1] = 1;
		}
	}
	
	//测试
	/*cout << "Line: " << endl;
	for ( i = 0; i < 9; i++)
	{
		for ( j = 0; j < 9; j++)
		{
			cout << LINE[i][j] << " ";
		}
		cout << endl;
	}
	cout << "Column：" << endl;
	for (i = 0; i < 9; i++)
	{
		for (j = 0; j < 9; j++)
		{
			cout << COLUMN[i][j] << " ";
		}
		cout << endl;
	}
	cout << "Block: " << endl;
	for (i = 0; i < 9; i++)
	{
		for (j = 0; j < 9; j++)
		{
			cout << BLOCK[i][j] << " ";
		}
		cout << endl;
	}*/

	//输出
	int flag = 1;

	for ( i = 0; i < 9; i++)
	{
		for ( j = 0; j < 9; j++)
		{
			if (!LINE[i][j])
				return 0;
			if (!COLUMN[i][j])
				return 0;
			if (!BLOCK[i][j])
				return 0;
		}
	}

	return flag;
}

int main()
{
	int i, j, temp;
	int a[9][9] = { 0 };
	bool LINE[9][9] = { 0 };//前一个元素代表位置,后一个元素代表1-9中的某一个数
	bool COLUMN[9][9] = { 0 };
	bool BLOCK[9][9] = { 0 };
	/*1.用三个二维数组记录某个数是否在某一行/列/宫格出现过
	  2.宫格的号数(0-8)用 行数下标/3*3 +  列数下标/3  计算
		例如:(4,5) 处在中间宫格(4)  -->  4/3*3 + 5/3  = 4
	  3.只要这个bool型的二维数组中有0，则不符合数独规则*/
	cout << "请输入9*9的矩阵，值为1-9之间" << endl;

	for (i = 0; i < 9; i++)//输入
	{
		for (j = 0; j < 9; j++)
		{
			while (1)
			{
				cin >> temp;
				if (cin.fail() || temp > 9 || temp < 1)
				{
					cin.clear();
					cin.ignore(1024, '\n');
					cout << "请重新输入第" << i + 1 << "行" << j + 1 << "列(行列均从1开始计数)的值" << endl;
				}
				else
				{
					a[i][j] = temp;
					break;
				}
			}
		}
	}

	bool flag = judge(a);
	if (flag)
		cout << "是数独的解" << endl;
	else
		cout << "不是数独的解" << endl;

	return 0;
}