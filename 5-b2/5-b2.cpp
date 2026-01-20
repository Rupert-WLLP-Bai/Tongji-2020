/*2052526 信15 白俊豪*/
#include<iostream>
using namespace std;

int main()
{
	int a[100];
	int i, j;
	for ( i = 0; i < 100; i++)
	{
		a[i] = -1;//初始状态为灭
	}
	
	for ( j = 1; j <= 100; j++)
	{
		for ( i = 0; i < 100; i++)
		{
			if ((i+1)%j == 0)
				a[i] = -a[i];//改变状态
		}
	}

	for (i = 0; i < 100; i++)
	{
		if (a[i] == 1)//亮
		{
			if (i != 99)
				cout << i + 1 << " ";
			else
				cout << i + 1;
		}
	}
	cout << endl;
	return 0;
}