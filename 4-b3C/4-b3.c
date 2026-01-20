/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>

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

int calendar(int year, int month, int row)
{
	int spaces;
	int day = 1;
	printf("%d年%d月\n", year, month);
	printf("======================================================\n");
	printf("星期日  星期一  星期二  星期三  星期四  星期五  星期六\n");
	printf("======================================================\n");

	if (row != 0) //补充前置空格
	{
		for (spaces = row; spaces > 1; spaces--)
		{
			printf("        ");
		}
		printf("    ");
	}

	if ((year % 100 != 0 && year % 4 == 0) || year % 400 == 0) //闰年
	{
		switch (month)
		{
			case 1:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);

				break;
			case 2:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 29);
				break;
			case 3:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 4:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 5:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
			case 6:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 7:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 8:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 9:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 10:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 11:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 12:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
		}
	}

	else //平年
	{
		switch (month)
		{
			case 1:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);

				break;
			case 2:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 28);
				break;
			case 3:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 4:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 5:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
			case 6:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 7:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 8:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 9:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 10:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
			case 11:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 30);
				break;
			case 12:
				do
				{
					if (row == 0)
					{
						printf("%4d", day);
						row++;
					}
					else if (row != 0)
					{
						printf("%8d", day);
						row++;
					}
					if (row > 6) //换行重置row的值
					{
						row = 0;
						printf("\n");
					}
					day++;
				} while (day <= 31);
				break;
		}
	}
	return 0;
}

int main()
{
	int y, m;
	int row;
	int _return;
	//输入以及判断合法性(1.平年,闰年 2.非法字符)
	while (1)
	{
		printf("输入年月,范围是(1900.1-2099.12) : \n");
		_return = scanf("%d%d", &y, &m);
		while (_return != 2)
		{
			rewind(stdin);
			printf("输入非法,请重新输入\n");
			printf("输入年月,范围是(1900.1-2099.12) : \n");
			_return = scanf("%d%d", &y, &m);
		}

		if (y >= 1900 && y <= 2099)
		{
			if (m >= 1 && m <= 12)
				break;
			else
			{
				printf("月份错误,请重新输入\n");
				rewind(stdin);
			}
		}
		else
		{
			printf("年份错误,请重新输入\n");
			rewind(stdin);
		}
	}
	printf("\n");
	row = zeller(y, m, 1);
	calendar(y, m, row);
	return 0;
}