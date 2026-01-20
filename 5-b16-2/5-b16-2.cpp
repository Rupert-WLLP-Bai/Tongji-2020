/*2052526 信15 白俊豪*/
#include<iostream>
#include<cmath>
#include<string>
#define SIZE 10
using namespace std;

/*输入信息*/
string input(int i)
{
	string a;
	cout << "请输入第" << i + 1 << "个人的学号、姓名、成绩" << endl;
	getline(cin, a);
	return a;
}

/*冒泡排序*/
void bubble(string a[SIZE])
{
	for (int i = 0; i < SIZE - 1; i++)//冒泡排序
	{
		for (int j = 0; j < SIZE - i - 1; j++)
			if (a[j] < a[j + 1])
				swap(a[j], a[j + 1]);
	}
}

/*得到成绩的字符串*/
string get_score(string a)
{
	a.erase(0, a.find_last_of(' ', a.size()) + 1);//最后一个空格出现的位置之前的部分全部删去
	return a;
}

/*获取不及格的名单*/
bool result(string score)
{
	//将score转为整型
	int a = 0;
	for (unsigned int i = 0; i < score.length(); i++)
	{
		a += (score[i] - '0') * int(pow(10, score.length() - i - 1));
	}
	int pass = 60;;//用成绩和"60"比较

	if (a >= 60)
		return 1;
	else
		return 0;
}

/*输出结果*/
void print_result(bool pass[SIZE], string a[SIZE])
{
	cout << endl << "不及格名单:" << endl;
	for (int i = 0; i < SIZE; i++)
	{
		if (!pass[i])
			cout << a[i] << endl;
	}
}

int main()
{
	string a[SIZE];//十个学生
	string score[SIZE];//十个成绩
	bool pass[10] = { 0 };//记录是否及格
	for (int i = 0; i < 10; i++)//输入
		a[i] = input(i);

	bubble(a);//冒泡排序

	//得到成绩
	for (int i = 0; i < SIZE; i++)
	{
		score[i] = a[i];
		score[i] = get_score(score[i]);
		pass[i] = result(score[i]);
	}
	//输出结果
	print_result(pass, a);

	return 0;
}
