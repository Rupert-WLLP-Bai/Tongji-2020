/*2052526 信15 白俊豪*/
#include<iostream>
#include<cstring>
using namespace std;

int judge(char a[20], int num[5])/*输入的值*/
{
	bool flag = 1;
	int n[5] = { 0 };//实际上的字符的个数,以及密码的长度
	n[0] = strlen(a);
	for (unsigned int i = 0; i < strlen(a); i++)//1.大写字母(65-90) 2.小写字母(97-122) 3.数字(48-57) 4.其他字符 ch[4]用于补齐密码的位数
	{
		if (a[i] >= 'A' && a[i] <= 'Z')//大写
		{
			n[1]++;
			continue;
		}
		if (a[i] >= 'a' && a[i] <= 'z')//小写
		{
			n[2]++;
			continue;
		}
		if (a[i] >= '0' && a[i] <= '9')//数字
		{
			n[3]++;
			continue;
		}

		if (a[i] >= 33 && a[i] <= 126 &&
			(!((a[i] >= 'A' && a[i] <= 'Z') ||
				(a[i] >= 'a' && a[i] <= 'z') ||
				(a[i] >= '0' && a[i] <= '9'))))//其他可见字符
		{
			n[4]++;
			continue;
		}
		else//33-126以外的字符
			return 0;
	}

	if (n[0] != num[0])
		return 0;

	for (int i = 1; i < 5; i++)
	{
		if (num[i] <= n[i])
		{
			flag = 1;
			continue;
		}
		else
		{
			flag = 0;
			break;
		}
	}
	return flag;
}

int check()//正确返回1,错误返回0
{
	bool flag = 1;
	int a[5] = { 0 };//长度，大写，小写，数字，其他字符的最小个数
	char ch[10][20];//十行密码

	while (getchar() != '\n');//读取首行并丢弃

	for (int i = 0; i < 5; i++)
	{
		cin >> a[i];
		if (cin.fail())
		{
			cin.clear();
			cin.ignore(65536, '\n');
			return 0;
		}
	}
	if (a[0] < 12 || a[0]>16 || a[1] < 2 || a[2] < 2 || a[3] < 2 || a[4] < 2)//输入参数不匹配
		return 0;

	while (getchar() != '\n');

	//读取后十行
	for (int i = 0; i < 10; i++)
	{
		cin.getline(ch[i], 20);
	}

	for (int i = 0; i < 10; i++)
	{
		flag = judge(ch[i], a);
		if (!flag)
			break;
	}
	return flag;
}

int main()
{
	if (check())
		cout << "正确" << endl;
	else
		cout << "错误" << endl;

	return 0;
}