/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
#include <cstdio>
using namespace std;

/* -----------------------------------------------------------------------------------
		允许   ：按需增加一个或多个函数，但是所有增加的函数中不允许任何形式的循环
				 定义符号常量
				 定义const型变量
		不允许 ：定义全局变量
   ----------------------------------------------------------------------------------- */
void Print_Sequential(char a, char b)
{
	if (b >= a)
	{
		cout << a;
		Print_Sequential(a + 1, b);
	}
}

void Print_Reverse(char a, char b)
{
	if (b >= a)
	{
		cout << b;
		Print_Reverse(a, b - 1);
	}
}


/***************************************************************************
  函数名称：
  功    能：正向或反向打印三角塔
  输入参数：order - 0 ：正三角形塔
			order - 1 ：倒三角形塔
  返 回 值：
  说    明：
***************************************************************************/
void print_tower(char start_ch, char end_ch, int order)
{
	if (order % 3 == 0)
	{
		order += 3;
		if (start_ch > end_ch)
			return;
		else
		{
			print_tower(start_ch, end_ch - 1, order);
			cout << setw(order / 3) << setfill(' ');
			Print_Reverse(start_ch + 1, end_ch);
			Print_Sequential(start_ch, end_ch);
			cout << endl;
		}
	}
	if (order == 1)
	{
		if (start_ch > end_ch)
			return;
		else
		{
			cout << setw((long long)start_ch + 1 - 'A') << setfill(' ');
			Print_Sequential(start_ch, end_ch - 1);
			Print_Reverse(start_ch, end_ch);
			cout << endl;
			print_tower(start_ch + 1, end_ch, 1);
		}
	}
}

/***************************************************************************
  函数名称：
  功    能：
  输入参数
  返 回 值：
  说    明：main函数不准修改
***************************************************************************/
int main()
{
	char end_ch;

	/* 键盘输入结束字符(仅大写有效) */
	while (1)
	{
		cout << "请输入结束字符(A~Z)" << endl;
		end_ch = getchar();       //读缓冲区第一个字符
		while (getchar() != '\n') //清空缓冲区剩余字符
			;
		if (end_ch >= 'A' && end_ch <= 'Z')
			break;
	}

	/* 正三角字母塔(中间为A) */
	cout << setfill('=') << setw(((long long)end_ch - 'A') * 2 + 1) << '=' << endl; /* 按字母塔最大长度输出= */
	cout << "正三角字母塔" << endl;
	cout << setfill('=') << setw(((long long)end_ch - 'A') * 2 + 1) << '=' << endl; /* 按字母塔最大长度输出= */
	print_tower('A', end_ch, 0);
	cout << endl;

	/* 倒三角字母塔(两边为A) */
	cout << setfill('=') << setw(((long long)end_ch - 'A') * 2 + 1) << '=' << endl; /* 按字母塔最大长度输出= */
	cout << "倒三角字母塔" << endl;
	cout << setfill('=') << setw(((long long)end_ch - 'A') * 2 + 1) << '=' << endl; /* 按字母塔最大长度输出= */
	print_tower('A', end_ch, 1);
	cout << endl;

	return 0;
}
