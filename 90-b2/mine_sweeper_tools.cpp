/*2052526 信15 白俊豪*/

#include<iostream>
#include<Windows.h>
#include<iomanip>
#include<conio.h>
#include"mine_sweeper.h"
using namespace std;

/*获取主菜单中的选项*/
void get_choice(int* select)
{
	while (1)
	{
		*select = _getch();
		if (*select >= '0' && *select <= '9')
			break;
	}
}

/*获取难度*/
void get_difficulty(int* d)
{
	while (1)
	{
		*d = _getch();
		if (*d >= '1' && *d <= '3')
			break;
	}
}

/*获取命令,对应相应的数组下标*//*分三种情况分别设定范围*/
/*MARK表示是否开启!,&,#对应的功能*//*传入两个坐标值*/
/*返回值: 0退出游戏,1打开雷阵,2标记雷阵,-1取消标记,-2显示时间*/
int fetch_instruction(int* x, int* y, int select, bool MARK)
{

	char temp1;
	char temp2;

	/*最多的输入是三个字符*/
	if (MARK)/*标记,显示时间功能开启*/
	{
		cout << endl << endl
			<< "特殊输入说明：& - 游戏已运行时间(单字符即可，不需要加坐标)" << endl
			<< "              ! - 标记该坐标为雷(例：!E3)" << endl
			<< "              # - 取消标记      (例：#E3)" << endl
			<< "请输入（坐标必须先行后列，严格区分大小写，例：G1 / Af，按Q / q退出）： ";
		int j = 0;
		char ch[3];
		while (j < 3)
		{
			temp1 = _getch();
			if (temp1 == 0 || temp1 == -32)//方向键
			{
				temp1 = _getch();
				continue;
			}
			if (temp1 == 'q' || temp1 == 'Q')
			{
				cout << temp1;
				return 0;//退出游戏
			}
			else if (temp1 == '&')
			{
				cout << temp1;
				return -2;//显示时间
			}
			else if (temp1 == '!')//标记
			{
				ch[j++] = temp1;
				cout << ch[0];
				while (j < 3)
				{
					temp1 = _getch();
					//输入部分
					switch (select)
					{
						case '1'://行的坐标为A-I,列的坐标为1-9
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'I')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							*y = ch[2] - '0';
							break;
						case '2'://行的坐标为A-P,列的坐标为1-9,a-g
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'P')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[1] - '0';
								}
								else if (temp2 >= 'a' && temp2 <= 'g')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - 'a' + 10;
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							break;
						case '3'://行的坐标为A-P,列的坐标为1-9,a-u
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'P')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - '0';
								}
								else if (temp2 >= 'a' && temp2 <= 'u')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - 'a' + 10;
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							break;

					}
				}
				return 2;//标记雷阵
			}
			else if (temp1 == '#')//取消标记
			{
				ch[j++] = temp1;
				cout << ch[0];
				while (j < 3)
				{
					temp1 = _getch();
					//输入部分
					switch (select)
					{
						case '1'://行的坐标为A-I,列的坐标为1-9
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'I')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							*y = ch[2] - '0';
							break;
						case '2'://行的坐标为A-P,列的坐标为1-9,a-g
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'P')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[1] - '0';
								}
								else if (temp2 >= 'a' && temp2 <= 'g')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - 'a' + 10;
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							break;
						case '3'://行的坐标为A-P,列的坐标为1-9,a-u
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'P')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - '0';
								}
								else if (temp2 >= 'a' && temp2 <= 'u')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - 'a' + 10;
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							break;

					}
				}
				return -1;//取消标记
			}
			//打开雷阵
			else
			{
				j = 1;
				while (j < 3)
				{
					//输入部分
					switch (select)
					{
						case '1'://行的坐标为A-I,列的坐标为1-9
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'I')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							*y = ch[2] - '0';
							break;
						case '2'://行的坐标为A-P,列的坐标为1-9,a-g
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'P')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[1] - '0';
								}
								else if (temp2 >= 'a' && temp2 <= 'g')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - 'a' + 10;
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							break;
						case '3'://行的坐标为A-P,列的坐标为1-9,a-u
							if (j == 1)//输入行
							{
								if (temp1 >= 'A' && temp1 <= 'P')
								{
									ch[j++] = temp1;
									cout << ch[1];
								}
								else
									continue;
							}
							if (j == 2)//输入列
							{
								temp2 = _getch();
								if (temp2 >= '1' && temp2 <= '9')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - '0';
								}
								else if (temp2 >= 'a' && temp2 <= 'u')
								{
									ch[j++] = temp2;
									cout << ch[2];
									*y = ch[2] - 'a' + 10;
								}
								else
									continue;
							}
							//得到坐标
							*x = ch[1] - 'A' + 1;
							break;

					}
				}
				return 1;
			}
		}

	}

	else/*基础功能,只需要获取两个字符,返回其对应的*/
	{
		cout << endl << endl
			<< "输入非雷位置的行列坐标（先行后列，严格区分大小写，例：G1/Af，按Q/q退出）：";
		char sh[2];
		int i = 0;
		while (i < 2)
		{
			temp1 = _getch();
			if (temp1 == 0 || temp1 == -32)//方向键
			{
				temp1 = _getch();
				continue;
			}
			if (temp1 == 'q' || temp1 == 'Q')
			{
				cout << temp1;
				return 0;//退出游戏
			}
			//输入部分
			switch (select)
			{
				case '1'://行的坐标为A-I,列的坐标为1-9
					if (i == 0)//输入行
					{
						if (temp1 >= 'A' && temp1 <= 'I')
						{
							sh[i++] = temp1;
							cout << sh[0];
						}
						else
							continue;
					}
					if (i == 1)//输入列
					{
						temp2 = _getch();
						if (temp2 >= '1' && temp2 <= '9')
						{
							sh[i++] = temp2;
							cout << sh[1];
						}
						else
							continue;
					}
					//得到坐标
					*x = sh[0] - 'A' + 1;
					*y = sh[1] - '0';
					break;
				case '2'://行的坐标为A-P,列的坐标为1-9,a-g
					if (i == 0)//输入行
					{
						if (temp1 >= 'A' && temp1 <= 'P')
						{
							sh[i++] = temp1;
							cout << sh[0];
						}
						else
							continue;
					}
					if (i == 1)//输入列
					{
						temp2 = _getch();
						if (temp2 >= '1' && temp2 <= '9')
						{
							sh[i++] = temp2;
							cout << sh[1];
							*y = sh[1] - '0';
						}
						else if (temp2 >= 'a' && temp2 <= 'g')
						{
							sh[i++] = temp2;
							cout << sh[1];
							*y = sh[1] - 'a' + 10;
						}
						else
							continue;
					}
					//得到坐标
					*x = sh[0] - 'A' + 1;
					break;
				case '3'://行的坐标为A-P,列的坐标为1-9,a-u
					if (i == 0)//输入行
					{
						if (temp1 >= 'A' && temp1 <= 'P')
						{
							sh[i++] = temp1;
							cout << sh[0];
						}
						else
							continue;
					}
					if (i == 1)//输入列
					{
						temp2 = _getch();
						if (temp2 >= '1' && temp2 <= '9')
						{
							sh[i++] = temp2;
							cout << sh[1];
							*y = sh[1] - '0';
						}
						else if (temp2 >= 'a' && temp2 <= 'u')
						{
							sh[i++] = temp2;
							cout << sh[1];
							*y = sh[1] - 'a' + 10;
						}
						else
							continue;
					}
					//得到坐标
					*x = sh[0] - 'A' + 1;
					break;

			}

			//打开雷阵
			return 1;
		}
	}

	return -3;
}