/*2052526 信15 白俊豪*/
#include<iostream>
using namespace std;
int main()
{
	int a[21];
	int b;
	int i = 0;
	bool empty = 1;
	cout << "请输入任意个正整数(升序，最多20个)，以-1结束" << endl;

	for (i = 0; i < 20; i++)//初始化
	{
		a[i] = 0;
	}

	i = 0;
	while (i < 20)//读入数据(最多20个)
	{
		cin >> b;
		if (b > 0)//正整数存入数组
		{
			a[i] = b;
			empty = 0;
			i++;
		}
		if (b < 0)//负数退出
			break;
	}

	if (empty)
		cout << "无有效输入" << endl;
	else
	{
		cout << "原数组为:" << endl;
		for (int i = 0; i < 20; i++)
		{
			if (a[i] != 0)
				cout << a[i] << " ";
		}
		cin.clear();
		cin.ignore(1024, '\n');
		//输入插入数组的数
		cout << endl << "请输入要插入的正整数:" << endl;
		cin >> a[20];
		//冒泡排序
		for (int i = 0; i < 21; i++)
		{
			for (int j = 0; j < 21 - 1 - i; j++)
			{
				if (a[j] > a[j + 1])
				{
					int temp = a[j];
					a[j] = a[j + 1];
					a[j + 1] = temp;
				}
			}
		}

		cout << "插入后的数组为:" << endl;

		for (i = 0; i < 21; i++)
		{
			if (a[i] != 0)
				cout << a[i] << " ";
		}
	}
	

	return 0;
}