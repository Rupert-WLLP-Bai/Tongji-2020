/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include<cstring>
#include<iostream>
#include<iomanip>
#include<conio.h>
using namespace std;

struct KFC
{
	char choice;
	char name[50];
	double price;
};

const struct KFC list[] = {
	{'A', "香辣鸡腿堡",         18},
	{'B', "劲脆鸡腿堡",         18},
	{'C', "新奥尔良烤鸡腿堡",   18.5},
	{'D', "鸡肉火腿帕尼尼",   14.0},
	{'E', "老北京鸡肉卷",       16.5},
	{'F', "川辣嫩牛卷",     19},
	{'G', "吮指原味鸡(1块)",   11.5},
	{'H', "热辣薯片脆皮鸡",   12.5},
	{'I', "新奥尔良烤翅(2块)", 12},
	{'J', "劲爆鸡米花",         10.5},
	{'K', "香辣鸡翅(2块)",     11.0},
	{'L', "热辣香骨鸡(3块)",     11.0},
	{'M', "鲜蔬色拉",           12.5},
	{'N', "薯条(小)",           8},
	{'O', "薯条(中)",           11},
	{'P', "薯条(大)",           13},
	{'Q', "芙蓉蔬荟汤",         8},
	{'R', "原味花筒冰激凌",     6},
	{'S', "醇香土豆泥",         6.5},
	{'T', "香甜粟米棒",         8.0},
	{'U', "葡式蛋挞",           7.5},
	{'V', "百事可乐(小)",       7.0},
	{'W', "百事可乐(中)",       9.5},
	{'X', "百事可乐(大)",       11.5},
	{'Y', "九珍果汁饮料",       12.0},
	{'Z', "纯纯玉米饮",         11.0},
	{'\0', NULL,                0}
};

struct SPECIAL
{
	char combo[20];
	char name[50];
	double price;
};

const struct SPECIAL special[] = {
	{"ANV", "香辣鸡腿堡工作日午餐",    22},
	{"BMV", "劲脆鸡腿堡超值套餐",    24},
	{"ABCGGIIKKOUWWW", "超值全家桶", 100},
	{"KIIRRJUWW", "缤纷小吃桶",  65},
	{"JJ","劲爆鸡米花(2份小)",      9.5},
	{NULL, NULL, 0}
};

/*计数*/
void counter(char input[], int count[])
{
	char uppercase1(char ch);
	char sh[500]{};
	for (unsigned int i = 0; i < strlen(input) + 1; i++)//转为大写
	{
		sh[i] = uppercase1(input[i]);
	}
	for (unsigned int i = 0; i < strlen(input) + 1; i++)//用counter计数
	{
		count[sh[i] - 'A']++;
	}
}

/*计数*/
void counter_const(const char input[], int count[])
{
	char in[500];
	strcpy_s(in, input);
	_strupr(in);
	for (unsigned int i = 0; i < strlen(input) + 1; i++)//用counter计数
	{
		count[in[i] - 'A']++;
	}
}

/*计算优惠*/
bool minus_count(int s1[26], int s2[26])
{
	for (int i = 0; i < 26; i++)
	{
		if (s1[i] >= s2[i])
			continue;
		else
			return 0;
	}

	for (int i = 0; i < 26; i++)
	{
		s1[i] -= s2[i];
	}
	return 1;
}

/*计算优惠*/
double calc_discount(const char input[])
{
	double DISCOUNT[26] = { 0 };//储存每一种优惠的价格差
	for (int i = 0; i < sizeof(special) / sizeof(special[0]) - 1; i++)
	{
		double a = 0;
		for (unsigned int j = 0; j < strlen(special[i].combo); j++)
		{
			a += list[special[i].combo[j] - 'A'].price;
		}
		DISCOUNT[i] = a - special[i].price;
	}

	double discount = 0;
	char in[500];//复制输入到另外一个字符串中
	strcpy_s(in, input);
	void counter(char input[], int count[]);

	for (int i = 0; i < sizeof(special) / sizeof(special[0]) - 1; i++)
	{
		int count_ori[26] = { 0 };
		int count_special[26] = { 0 };
		counter(in, count_ori);
		counter_const(special[i].combo, count_special);
		while (minus_count(count_ori, count_special))
			discount += DISCOUNT[i];
	}

	return discount;
}

/*输出内容及其数量*/
void print_result(int count[], double discount)
{
	double sum = 0;
	int m = 0;//用于控制输出"+"
	for (int i = 0; i < 26; i++)
	{
		if (count[i] != 0)
			m++;
	}
	cout << "您的点餐 = ";
	for (int k = 0; k < 26; k++)
	{
		sum += list[k].price * count[k];
		if (count[k] == 0)
			continue;
		else if (count[k] == 1)
		{
			cout << list[k].name;
			if (--m >= 1)
				cout << "+";
		}
		else
		{
			cout << list[k].name << "*" << count[k];
			if (--m >= 1)
				cout << "+";
		}
	}
	cout << endl << "总计:" << sum - discount << "元" << endl;

	cout << endl;
}

/*错误提示*/
static void to_be_continued(const char* prompt, const int X = 0, const int Y = 22)
{
	if (prompt)
		cout << prompt << "，按任意键继续";
	else
		cout << "按任意键继续";

	int ch = _getch();

	return;
}

/*检查输入*/
/*正确*/
int check_input(char input[])
{
	if (input[0] == '0' && input[1] == '\0')//输入0退出
		return 0;
	else if (input[0] == '\0')//只输入回车则直接跳过后面的步骤，重新打印菜单
	{
		return -1;
	}
	else
	{
		for (unsigned int i = 0; i < strlen(input); i++)
		{
			if ((input[i] >= 'a' && input[i] <= 'z') || (input[i] >= 'A' && input[i] <= 'Z'))
				continue;
			else
			{
				return -2;//输入错误
			}
		}
	}
	return 1;//输入正确
}

/*字符转大写*/
char uppercase1(char ch)
{
	if (ch >= 'a' && ch <= 'z')
		return ch - 32;
	else
		return ch;
}



/*打印优惠*/
void print_discount()
{
	cout << endl;
	cout << "【优惠信息】：" << endl;
	for (int i = 0; i < sizeof(special) / sizeof(special[0]) - 1; i++)
	{
		cout << special[i].name << "=";
		for (unsigned int j = 0, count = 1; j < strlen(special[i].combo); j++)
		{

			if (special[i].combo[j + 1] == special[i].combo[j])
			{
				count++;
				continue;
			}
			else
			{
				if (count == 1)
					cout << list[special[i].combo[j] - 'A'].name;
				else
					cout << list[special[i].combo[j] - 'A'].name << "*" << count;
			}
			if (j + 1 < strlen(special[i].combo))
				cout << "+";
		}
		cout << "=" << special[i].price;
		cout << endl << endl;
	}
}

/*打印菜单*/
void print_menu()
{
	cout << "=============================================================" << endl;
	cout << "                      KFC 2020冬季菜单                       " << endl;
	cout << "=============================================================" << endl;
	for (int i = 0; list[i].choice != '\0'; i++)
	{
		cout << list[i].choice << " ";
		cout << setiosflags(ios::left) << setw(20) << list[i].name;
		cout << resetiosflags(ios::left);
		cout << setiosflags(ios::right) << setw(7) << list[i].price;
		cout << resetiosflags(ios::right);
		if (i % 2 == 0)
		{
			cout << " | ";
			continue;
		}
		else
			cout << endl;
	}
	print_discount();

	cout << "【输入规则说明】：" << endl;
	cout << "ANV = 香辣鸡腿堡 + 薯条(小) + 百事可乐(小) / akaak = 香辣鸡腿堡 * 3 + 香辣鸡翅 * 2" << endl;
	cout << "字母不分大小写，不限顺序，单独输入0则退出程序" << endl;
	cout << endl << "请点单:";
}

/*主函数*/
int main()
{
	int check;
	double discount = 0;
	char input[1000];//点单输入
	while (1)
	{
		int count[26] = { 0 };
		system("cls");
		print_menu();
		cin.getline(input, 1000);
		check = check_input(input);
		if (check == 0)
			break;
		else if (check == -1)
			continue;
		else if (check == -2)
		{
			to_be_continued("输入错误");
			continue;
		}
		else//输入正确
		{
			counter(input, count);
			discount = calc_discount(_strupr(input));
			print_result(count, discount);
			to_be_continued("点餐完成");
		}
	}
	return 0;
}
