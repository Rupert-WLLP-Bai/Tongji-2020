/*2052526 信15 白俊豪*/
#include <iostream>
using namespace std;
int main()
{
	int year, month, day;
	int order;
	int is_leap_year = 0;
	cout << "请输入年,月,日 : " << endl;
	cin >> year >> month >> day; //输入部分

	//判断闰年
	if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0)
		is_leap_year = 1;
	//判断月份,月份与日期的关系是否合法
	if (is_leap_year == 1) // leap year
	{
		switch (month)
		{
			case 1:
				if (day >= 1 && day <= 31)
				{
					order = day;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;

			case 2:
				if (day >= 1 && day <= 29)
				{
					order = day + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 3:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 29;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 4:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 29 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 5:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 29 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 6:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 29 + 31 + 30 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 7:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 29 + 31 + 30 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 8:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 9:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 10:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 11:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 12:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			default:
				cout << "月份不正确" << endl;
				break;
		}
	}
	else //not leap year
	{
		switch (month)
		{
			case 1:
				if (day >= 1 && day <= 31)
				{
					order = day;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;

			case 2:
				if (day >= 1 && day <= 28)
				{
					order = day + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 3:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 28;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 4:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 28 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 5:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 28 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 6:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 28 + 31 + 30 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 7:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 28 + 31 + 30 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 8:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 9:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 10:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 11:
				if (day >= 1 && day <= 30)
				{
					order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			case 12:
				if (day >= 1 && day <= 31)
				{
					order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30;
					cout << year << "-" << month << "-" << day << "是" << year << "年的第" << order << "天" << endl;
				}
				else
				{
					cout << "输入错误-日与月的关系非法" << endl;
				}
				break;
			default:
				cout << "月份不正确" << endl;
				break;
		}
	}
	return 0;
}

//Done.