/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
using namespace std;
int i = 0;
int a[10];
int b[10];
int c[10];
int top_A, top_B, top_C;
void spawn(char start,int level)
{
	for (int i = 0; i < 10; i++)
	{
		a[i] = b[i] = c[i] = 0;//初始化
	}

	top_A = top_B = top_C = 0;
	switch (start)//开始的数组状态
	{
		case 'A':
		{
			while (top_A <= level)
			{
				a[top_A++] = level - top_A;
			}
			top_A--;
			break;
		}
		case 'B':
		{
			while (top_B <= level)
			{
				b[top_B++] = level - top_B;
			}
			top_B--;
			break;
		}
		case 'C':
		{
			while (top_C <= level)
			{
				c[top_C++] = level - top_C;
			}
			top_C--;
			break;
		}
	}

	cout << "初始:                ";
	cout << "A:";
	if (a[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{
		if (a[i] != 0)
		{
			if (a[i] != 10)
				cout << a[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << "B:";
	if (b[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{

		if (b[i] != 0)
		{
			if (b[i] != 10)
				cout << b[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << "C:";
	if (c[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{

		if (c[i] != 0)
		{
			if (c[i] != 10)
				cout << c[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << endl;
}

void haoni(int n, char src, char tmp, char dst)
{

	void move(int, char, char);
	if (n == 1)
	{
		i++;
		cout << "第" << setw(4) << i << " 步";
		move(n, src, dst);
	}
	else
	{
		haoni(n - 1, src, dst, tmp);
		i++;
		cout << "第" << setw(4) << i << " 步";
		move(n, src, dst);
		haoni(n - 1, tmp, src, dst);

	}
}

void move(int n, char x, char y)
{
	cout << "(" << setw(2) << n << "): ";
	cout << x << "-->" << y << " ";
	//进栈,出栈
	int temp;
	switch (x)
	{
		case 'A':
			if (y == 'B')
			{
				temp = a[--top_A];
				a[top_A] = 0;
				b[top_B] = temp;
				top_B++;
			}
			else if (y == 'C')
			{
				temp = a[--top_A];
				a[top_A] = 0;
				c[top_C] = temp;
				top_C++;
			}
			break;
		case 'B':
			if (y == 'A')
			{
				temp = b[--top_B];
				b[top_B] = 0;
				a[top_A] = temp;
				top_A++;
			}
			else if (y == 'C')
			{
				temp = b[--top_B];
				b[top_B] = 0;
				c[top_C] = temp;
				top_C++;
			}
			break;
		case 'C':
			if (y == 'A')
			{
				temp = c[--top_C];
				c[top_C] = 0;
				a[top_A] = temp;
				top_A++;
			}
			else if (y == 'B')
			{
				temp = c[--top_C];
				c[top_C] = 0;
				b[top_B] = temp;
				top_B++;
			}
			break;
	}

	//输出
	cout << "A:";
	if (a[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{
		if (a[i] != 0)
		{
			if (a[i] != 10)
				cout << a[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << "B:";
	if (b[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{

		if (b[i] != 0)
		{
			if (b[i] != 10)
				cout << b[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << "C:";
	if (c[0] != 10)
		cout << " ";
	for (int i = 0; i < 10; i++)
	{

		if (c[i] != 0)
		{
			if (c[i] != 10)
				cout << c[i] << " ";
			else
				cout << "10 ";
		}
		else
			cout << "  ";
	}
	cout << endl;
}

/***************************************************************************
  函数名称：
  功    能：
  输入参数：
  返 回 值：
  说    明：完成输入(或调用输入函数)、调用递归函数
***************************************************************************/
int main()
{
	int level;
	char start, mid, end;
	while (1)
	{
		cout << "请输入汉诺塔的层数(1-10)" << endl;
		cin >> level;
		while (cin.fail())
		{
			cin.clear();
			cin.ignore(1024, '\n');
			cout << "请输入汉诺塔的层数(1-10)" << endl;
			cin >> level;
		}
		if (level >= 1 && level <= 10)
			break;
	}
	cin.clear();
	cin.ignore(1024, '\n');
	while (1)
	{
		cout << "请输入起始柱(A-C)" << endl;
		cin >> start;
		while (cin.fail())
		{
			cin.clear();
			cin.ignore(1024, '\n');
			cout << "请输入起始柱(A-C)" << endl;
			cin >> start;
		}
		if (start == 'A' || start == 'B' || start == 'C')
			break;
		else if (start == 'a' || start == 'b' || start == 'c')
		{
			start -= 32;
			break;
		}
		cin.clear();
		cin.ignore(1024, '\n');
	}
	cin.clear();
	cin.ignore(1024, '\n');
	while (1)
	{
		cout << "请输入目标柱(A-C)" << endl;
		cin >> end;
		while (cin.fail())
		{
			cin.clear();
			cin.ignore(1024, '\n');
			cout << "请输入目标柱(A-C)" << endl;
			cin >> end;
		}
		if (end == start || end == start + 32 || end == start - 32)
		{
			cin.clear();
			cin.ignore();
			cout << "起始柱不能与目标柱相同" << endl;
			continue;
		}
		if (end == 'A' || end == 'B' || end == 'C')
			break;
		else if (end == 'a' || end == 'b' || end == 'c')
		{
			end -= 32;
			break;
		}
		cin.clear();
		cin.ignore(1024, '\n');
	}
	mid = 'A' + 'B' + 'C' - start - end;
	spawn(start, level);
	haoni(level, start, mid, end);
	return 0;
}

