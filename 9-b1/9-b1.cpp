/*2052526 信15 白俊豪*/
#include<iostream>
using namespace std;

int max(int a, int b)
{
	if (a > b)
		return a;
	else
		return b;
}

int max(int a, int b, int c)
{
	int m = a;

	if (m < b)
		m = b;
	if (m < c)
		m = c;

	return m;
}

int max(int a, int b, int c, int d)
{
	int m = a;

	if (m < b)
		m = b;
	if (m < c)
		m = c;
	if (m < d)
		m = d;

	return m;
}

int main()
{
	int num;//输入个数
	int a, b, c, d;//比较的数

	while (1)
	{
		cout << "请输入个数num及num个正整数" << endl;
		while (1)//输入个数
		{
			cin >> num;
			if (cin.fail())//输入错误
			{
				cin.clear();
				cin.ignore(65536, '\n');
				continue;
			}
			else//输入正确
			{
				if (num >= 2 && num <= 4)
					break;
				else
				{
					cout << "个数输入错误" << endl;
					return 0;
				}
			}
		}

		switch (num)
		{
			case 2:
				cin >> a;
				if (cin.fail())//输入错误
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> b;
				if (cin.fail())//输入错误
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				if (a <= 0 || b <= 0)
					continue;
				cout << "max=" << max(a, b) << endl;
				break;
			case 3:
				cin >> a;
				if (cin.fail())//输入错误
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> b;
				if (cin.fail())//输入错误
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> c;
				if (cin.fail())//输入错误
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				if (a <= 0 || b <= 0 || c <= 0)
					continue;
				cout << "max=" << max(a, b, c) << endl;
				break;
			case 4:
				cin >> a;
				if (cin.fail())//输入错误
				{
					cin.ignore(65536, '\n');
					cin.clear();
					continue;
				}
				cin >> b;
				if (cin.fail())//输入错误
				{
					cin.ignore(65536, '\n');
					cin.clear();
					continue;
				}
				cin >> c;
				if (cin.fail())//输入错误
				{
					cin.ignore(65536, '\n');
					cin.clear();
					continue;
				}
				cin >> d;
				if (cin.fail())//输入错误
				{
					cin.ignore(65536, '\n');
					cin.clear();
					continue;
				}
				if (a <= 0 || b <= 0 || c <= 0 || d <= 0)
					continue;
				cout << "max=" << max(a, b, c, d) << endl;
				break;
		}
		break;
	}
}

