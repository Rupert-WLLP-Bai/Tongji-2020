/*2052526 信15 白俊豪*/
#include<iostream>
#include<time.h>
using namespace std;

int get_length_of_password()
{
	int i;
	cin >> i;
	if (cin.fail())
	{
		cout << "输入含有非法字符" << endl;
		cin.clear();
		cin.ignore(65536, '\n');
		return 0;//返回0就退出
	}
	else if (i <= 11 || i >= 17)
	{
		cout << "密码长度[" << i << "]不正确" << endl;
		return 0;
	}
	return i;
}

int get_num_of_capital_letter()
{
	int i;
	cin >> i;
	if (cin.fail())
	{
		cout << "输入含有非法字符" << endl;
		cin.clear();
		cin.ignore(65536, '\n');
		return 0;//返回0就退出
	}
	else if (i <= 1)
	{
		cout << "大写字母个数[" << i << "]不正确" << endl;
		return 0;
	}
	return i;
}

int get_num_of_lowercase_letter()
{
	int i;
	cin >> i;
	if (cin.fail())
	{
		cout << "输入含有非法字符" << endl;
		cin.clear();
		cin.ignore(65536, '\n');
		return 0;//返回0就退出
	}
	else if (i <= 1)
	{
		cout << "小写字母个数[" << i << "]不正确" << endl;
		return 0;
	}
	return i;
}

int get_num_of_num()
{
	int i;
	cin >> i;
	if (cin.fail())
	{
		cout << "输入含有非法字符" << endl;
		cin.clear();
		cin.ignore(65536, '\n');
		return 0;//返回0就退出
	}
	else if (i <= 1)
	{
		cout << "数字个数[" << i << "]不正确" << endl;
		return 0;
	}
	return i;
}

int get_num_of_other()
{
	int i;
	cin >> i;
	if (cin.fail())
	{
		cout << "输入含有非法字符" << endl;
		cin.clear();
		cin.ignore(65536, '\n');
		return 0;//返回0就退出
	}
	else if (i <= 1)
	{
		cout << "其他符号个数[" << i << "]不正确" << endl;
		return 0;
	}
	return i;
}

/*生成并输出密码*/
void keygen(int a, int b, int c, int d, int length, long long cnt)//cnt用于每次生成不同的随机数
{
	srand((unsigned int)(time(0)));
	char a1;
	char Password[16] = { 0 };
	char ch[5][10];//四种类型的字符: 1.大写字母(65-90) 2.小写字母(97-122) 3.数字(48-57) 4.其他字符 ch[4]用于补齐密码的位数
	int miss = length - a - b - c - d;
	for (int i = 0; i < a; i++)
		ch[0][i] = rand() % 26 + 65;
	for (int i = 0; i < b; i++)
		ch[1][i] = rand() % 26 + 97;
	for (int i = 0; i < c; i++)
		ch[2][i] = rand() % 10 + 48;
	for (int i = 0; i < d; i++)
	{
		while (1)
		{
			a1 = rand() % 94 + 33;
			if (!((a1 >= 'A' && a1 <= 'Z') ||
				(a1 >= 'a' && a1 <= 'z') ||
				(a1 >= '0' && a1 <= '9')))//其他可见字符
			{
				ch[3][i] = a1;
				break;
			}
		}
	}
	for (int i = 0; i < miss; i++)
		ch[4][i] = rand() % 94 + 33;

	//将这些字符放入Password数组
	int j = 0;
	for (int i = 0; i < a; i++, j++)
		Password[j] = ch[0][i];
	for (int i = 0; i < b; i++, j++)
		Password[j] = ch[1][i];
	for (int i = 0; i < c; i++, j++)
		Password[j] = ch[2][i];
	for (int i = 0; i < d; i++, j++)
		Password[j] = ch[3][i];
	for (int i = 0; i < miss; i++, j++)
		Password[j] = ch[4][i];
	//再随机数一次打乱顺序

	srand((unsigned int)(2 * time(NULL) * (cnt + 1)));
	for (int i = 0; i < 100; i++)
	{
		while (1)
		{
			int t1, t2;
			t1 = rand() % length;
			t2 = rand() % length;
			if (t1 != t2)
			{
				char temp = Password[t1];
				Password[t1] = Password[t2];
				Password[t2] = temp;
				break;
			}
		}
	}

	//输出结果
	for (int i = 0; i < length; i++)
		cout << Password[i];

	//cout << endl << a << " " << b << " " << c << " " << d << " " << miss << endl;
	cout << endl;
}

int main()
{
	int length;//密码的长度为12-16
	int num_of_capital_letter;//至少为2
	int num_of_lowercase_letter;//至少为2
	int num_of_num;//至少为2
	int num_of_other;//至少为2
	cout << "输入请输入密码长度(12-16)，大写字母个数(≥2)，小写字母个数(≥2)，数字个数(≥2)，其它符号个数(≥2)" << endl;
	while (1)
	{
		length = get_length_of_password();
		if (!length)
			break;

		num_of_capital_letter = get_num_of_capital_letter();
		if (!num_of_capital_letter)

			break;
		num_of_lowercase_letter = get_num_of_lowercase_letter();
		if (!num_of_lowercase_letter)
			break;

		num_of_num = get_num_of_num();
		if (!num_of_num)
			break;

		num_of_other = get_num_of_other();
		if (!num_of_other)
			break;

		if (num_of_capital_letter + num_of_lowercase_letter + num_of_num + num_of_other > length)
		{
			cout << "所有字符类型之和[" << num_of_capital_letter << "+"
				<< num_of_lowercase_letter << "+" << num_of_num
				<< "+" << num_of_other << "]大于总密码长度[" << length << "]" << endl;
			break;
		}

		//运行到该处说明输入正确,以下是密码生成部分(参数为： 4种字符的最少个数,密码的长度length)
		//生成十串密码
		cout << length << " " << num_of_capital_letter << " " << num_of_lowercase_letter << " " << num_of_num << " " << num_of_other << endl;
		for (int i = 0; i < 10; i++)
			keygen(num_of_capital_letter, num_of_lowercase_letter, num_of_num, num_of_other, length, i);


		break;
	}

	return 0;
}
