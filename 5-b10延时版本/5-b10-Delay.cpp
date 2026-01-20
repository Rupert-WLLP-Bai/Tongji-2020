/*2052526 信15 白俊豪*/
#include <iostream>
#include <iomanip>
#include<Windows.h>
using namespace std;

int day[366];
int M[12][31];
int k;//日期
int num_of_day_leap[12] = { 31,29,31,30,31,30,31,31,30,31,30,31 };
int num_of_day_plain[12] = { 31,28,31,30,31,30,31,31,30,31,30,31 };
/*得到某年1月1日是星期几*/
int zeller(int y, int m, int d)
{
	int w, c, y1, start;
	if (m >= 3 && m <= 14)
	{
		y1 = y % 100; //得到公式中Y的值
		c = y / 100;  //得到公式中C的值
	}
	else
	{
		m += 12;
		y1 = (y - 1) % 100; //得到公式中Y的值
		c = (y - 1) / 100;  //得到公式中C的值
	}
	w = y1 + y1 / 4 + c / 4 - 2 * c + (13 * (m + 1) / 5) + d - 1;
	while (w <= 0)
		w += 7;
	start = w % 7;
	return start;
}

/*判断闰年*/
bool leap_year(int year)
{
	if (year % 4 == 0 && year % 100 != 0 || year % 400 == 0)
		return 1;
	else
		return 0;
}

/*计算某月某日是星期几*/
void calc_start_day(int year)
{
	int count = 1;
	//计算每一天是星期几
	day[0] = zeller(year, 1, 1);
	for (int i = 0; i < 366; i++)
	{
		if (day[i] != 6)
			day[i + 1] = day[i] + 1;
		else
			day[i + 1] = day[i] - 6;
	}
	//将这些值存入某月某日
	if (leap_year(year))
	{
		int count = 0;
		for (int i = 0; i < 12; i++)
		{
			for (int j = 0; j < num_of_day_leap[i]; j++)
			{
				M[i][j] = day[count];
				count++;
			}
		}
	}
	else
	{
		int count = 0;
		for (int i = 0; i < 12; i++)
		{
			for (int j = 0; j < num_of_day_plain[i]; j++)
			{
				M[i][j] = day[count];
				count++;
			}
		}
	}
}

/*输出固定格式*/
void print_title(int quarter)
{
	switch (quarter)
	{
		case 1:
			cout << "            1月                             2月                             3月" << endl;
			cout << "Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat" << endl;
			break;
		case 2:
			cout << "            4月                             5月                             6月" << endl;
			cout << "Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat" << endl;
			break;
		case 3:
			cout << "            7月                             8月                             9月" << endl;
			cout << "Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat" << endl;
			break;
		case 4:
			cout << "           10月                            11月                            12月" << endl;
			cout << "Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat" << endl;
			break;
		default:
			break;
	}
}

/*输出每月的第一行*/
void print_first_line(int month)
{
	k = 0;
	for (int i = 0; i < M[month - 1][0]; i++)//补空格
	{
		Sleep(50);
		cout << "    ";
	}
	for (int i = 0; i < 7 - M[month - 1][0]; i++)
	{
		Sleep(50);
		cout << i + 1 << "   ";
	}
	Sleep(50);
	cout << "    ";
}

void print_rest_line(int month1, int month2, int month3, bool leap)
{
	int row = 0;
	int count1 = 0;
	int count2 = 0;
	int count3 = 0;
	int x1 = 8 - M[month1 - 1][0];
	int x2 = 8 - M[month2 - 1][0];
	int x3 = 8 - M[month3 - 1][0];
	cout << endl;
	if (leap)
	{
		while (!(x1 == num_of_day_leap[month1 - 1] + 1 && x2 == num_of_day_leap[month2 - 1] + 1 && x3 == num_of_day_leap[month3 - 1] + 1))
		{
			while (row != 7 && count1 != 7)
			{
				Sleep(50);
				if (x1 <= num_of_day_leap[month1 - 1])
					cout << setiosflags(ios::left) << setw(4) << setfill(' ') << x1++;
				else
					cout << "    ";
				count1++;
				row++;
			}
			Sleep(50);
			cout << "    ";
			row = 0;
			while (row != 7 && count2 != 7)
			{
				Sleep(50);
				if (x2 <= num_of_day_leap[month2 - 1])
					cout << setiosflags(ios::left) << setw(4) << setfill(' ') << x2++;
				else
					cout << "    ";
				count2++;
				row++;
			}
			Sleep(50);
			cout << "    ";
			row = 0;
			while (row != 7 && count3 != 7)
			{
				Sleep(50);
				if (x3 <= num_of_day_leap[month3 - 1])
					cout << setiosflags(ios::left) << setw(4) << setfill(' ') << x3++;
				else
					cout << "    ";
				row++;
			}
			row = 0;
			count1 = count2 = count3 = 0;
			cout << endl;
		}
		cout << endl;
	}
	else
	{
		while (!(x1 == num_of_day_plain[month1 - 1] + 1 && x2 == num_of_day_plain[month2 - 1] + 1 && x3 == num_of_day_plain[month3 - 1] + 1))
		{
			while (row != 7 && count1 != 7)
			{
				Sleep(50);
				if (x1 <= num_of_day_plain[month1 - 1])
					cout << setiosflags(ios::left) << setw(4) << setfill(' ') << x1++;
				else
					cout << "    ";
				count1++;
				row++;
			}
			Sleep(50);
			cout << "    ";
			row = 0;
			while (row != 7 && count2 != 7)
			{
				Sleep(50);
				if (x2 <= num_of_day_plain[month2 - 1])
					cout << setiosflags(ios::left) << setw(4) << setfill(' ') << x2++;
				else
					cout << "    ";
				count2++;
				row++;
			}
			Sleep(50);
			cout << "    ";
			row = 0;
			while (row != 7 && count3 != 7)
			{
				Sleep(50);
				if (x3 <= num_of_day_plain[month3 - 1])
					cout << setiosflags(ios::left) << setw(4) << setfill(' ') << x3++;
				else
					cout << "    ";
				count3++;
				row++;
			}
			row = 0;
			count1 = count2 = count3 = 0;
			cout << endl;
		}
		cout << endl;
	}
	
}

int main()
{
	for (int i = 0; i < 366; i++)
		day[i] = -1;
	int year;
	while (1)
	{
		cout << "请输入年份[1900-2100]" << endl;
		cin >> year;
		if (!cin.fail() && year >= 1900 && year <= 2100)
			break;
		else
		{
			cin.clear();
			cin.ignore(65536, '\n');
		}
	}
	cout << year << "年的日历:" << endl;
	calc_start_day(year);

	print_title(1);
	for (int i = 1; i <= 3; i++)
		print_first_line(i);
	print_rest_line(1, 2, 3, leap_year(year));
	print_title(2);
	for (int i = 4; i < 7; i++)
		print_first_line(i);
	print_rest_line(4, 5, 6, leap_year(year));
	print_title(3);
	for (int i = 7; i <= 9; i++)
		print_first_line(i);
	print_rest_line(7, 8, 9, leap_year(year));
	print_title(4);
	for (int i = 10; i <= 12; i++)
		print_first_line(i);
	print_rest_line(10, 11, 12, leap_year(year));
	return 0;
}