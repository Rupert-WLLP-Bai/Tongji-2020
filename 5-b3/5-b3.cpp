/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;

int leap_year(int year)
{
	if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0)
		return 1;
	else
		return 0;
}

int f(int year, int month, int day)
{
	int a[12] = { 31,28,31,30,31,30,31,31,30,31,30,31 };//平年
	int b[12] = { 31,29,31,30,31,30,31,31,30,31,30,31 };//闰年
	int sum = 0;
	if (leap_year(year))
	{
		for (int i = 0; i < month - 1; i++)
		{
			sum += b[i];
		}
		sum += day;
	}
	else
	{
		for (int i = 0; i < month - 1; i++)
		{
			sum += a[i];
		}
		sum += day;
	}

	return sum;
}

int main()
{
	int year, month, day;
	cout << "请输入年,月,日 : " << endl;
	cin >> year >> month >> day; //输入部分
		//判断正确性	cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year,month,day) << "天" << endl;
	if (year >= 2000 && year <= 2030)
	{
		if (month > 12 || month < 0)
			cout << "月份不正确" << endl;
		else
		{
			if (leap_year(year))
			{
				if (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12)
				{ 
					if (!(day >= 1 && day <= 31))
						cout << "输入错误-日与月的关系非法" << endl;
					else
						cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year, month, day) << "天" << endl;
				}
				else if(month == 4 || month == 6 || month == 9 || month == 11)
				{
					if (!(day >= 1 && day <= 30))
						cout << "输入错误-日与月的关系非法" << endl;
					else
						cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year, month, day) << "天" << endl;
				}
				else//闰年2月
				{
					if (!(day >= 1 && day <= 29))
						cout << "输入错误-日与月的关系非法" << endl;
					else
						cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year, month, day) << "天" << endl;
				}
			}
			else
			{
				if (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12)
				{
					if (!(day >= 1 && day <= 31))
						cout << "输入错误-日与月的关系非法" << endl;
					else
						cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year, month, day) << "天" << endl;
				}
				else if (month == 4 || month == 6 || month == 9 || month == 11)
				{
					if (!(day >= 1 && day <= 30))
						cout << "输入错误-日与月的关系非法" << endl;
					else
						cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year, month, day) << "天" << endl;
				}
				else//平年2月
				{
					if (!(day >= 1 && day <= 28))
						cout << "输入错误-日与月的关系非法" << endl;
					else
						cout << year << "-" << month << "-" << day << "是" << year << "年的第" << f(year, month, day) << "天" << endl;
				}
			}
		}
	}
	else
	{
		cout << "输入错误-年份输入错误" << endl;
	}

	return 0;
}