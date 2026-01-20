/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdbool.h>

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

int main()
{
	int y, m, d, result;
	bool is_leap_year = 0;
	int _return;
	//输入以及判断合法性(1.平年,闰年 2.非法字符)
	while (1)
	{
		printf("输入年月日,范围是(1900.1.1-2099.12.31) : \n");
		_return = scanf("%d%d%d", &y, &m, &d);
		while (_return != 3)
		{
			rewind(stdin);
			printf("输入错误,请重新输入\n");
			printf("输入年月日,范围是(1900.1.1-2099.12.31) : \n");
			_return = scanf("%d%d%d", &y, &m, &d);
		}

		if (y >= 1900 && y <= 2099)
		{
			if ((y % 4 == 0 && y % 100 != 0) || y % 400 == 0)
				is_leap_year = true;
			if (m >= 1 && m <= 12)
			{
				if (m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12)
				{
					if (d < 1 || d > 31)
					{
						printf("输入错误,请重新输入\n");
					}
					else
						break;
				}
				else if (m == 4 || m == 6 || m == 9 || m == 11)
				{
					if (d < 1 || d > 30)
					{
						printf("输入错误,请重新输入\n");
					}
					else
						break;
				}

				else
				{
					if (is_leap_year)
					{
						if (d < 1 || d > 29)
						{
							printf("输入错误,请重新输入\n");
						}
						else
							break;
					}
					else
					{
						if (d < 1 || d > 28)
						{
							printf("输入错误,请重新输入\n");
						}
						else
							break;
					}
				}
			}
			else
			{
				printf("输入错误,请重新输入\n");
			}
		}
		else
		{
			printf("输入错误,请重新输入\n");
		}
	}

	//结果处理
	result = zeller(y, m, d);
	switch (result)
	{
		case 0:
			printf("星期日\n");
			break;
		case 1:
			printf("星期一\n");
			break;
		case 2:
			printf("星期二\n");
			break;
		case 3:
			printf("星期三\n");
			break;
		case 4:
			printf("星期四\n");
			break;
		case 5:
			printf("星期五\n");
			break;
		case 6:
			printf("星期六\n");
			break;
	}
	printf("\n");
	return 0;
}