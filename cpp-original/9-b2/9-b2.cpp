/*2052526 信15 白俊豪*/
#include<iostream>
using namespace std;

int min(int a, int b, int c = -1, int d = -1)
{
	int m = a;

	if (m > b)
		m = b;
	if (m > c && c != -1)
		m = c;
	if (m > d && d != -1)
		m = d;

	return m;
}

int main()
{
	int num;
	int a, b, c, d;
	while (1)
	{
		cout << "请输入个数num及num个正整数" << endl;
		/*输入个数*/
		while (1)
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
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> b;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				if (a <= 0 || b <= 0)
					continue;
				cout << "min=" << min(a, b);
				break;
			case 3:
				cin >> a;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> b;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> c;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				if (a <= 0 || b <= 0 || c <= 0)
					continue;
				cout << "min=" << min(a, b, c);
				break;
			case 4:
				cin >> a;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> b;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> c;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
					continue;
				}
				cin >> d;
				if (cin.fail())
				{
					cin.clear();
					cin.ignore(65536, '\n');
				}
				if (a <= 0 || b <= 0 || c <= 0 || d <= 0)
					continue;
				cout << "min=" << min(a, b, c, d);
				break;
		}
		break;
	}
}