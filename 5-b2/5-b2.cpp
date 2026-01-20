/*2052526 ÐÅ15 °×¿¡ºÀ*/
#include<iostream>
using namespace std;

int main()
{
	int a[100];
	int i, j;
	for ( i = 0; i < 100; i++)
	{
		a[i] = -1;//³õÊ¼×´Ì¬ÎªÃð
	}
	
	for ( j = 1; j <= 100; j++)
	{
		for ( i = 0; i < 100; i++)
		{
			if ((i+1)%j == 0)
				a[i] = -a[i];//¸Ä±ä×´Ì¬
		}
	}

	for (i = 0; i < 100; i++)
	{
		if (a[i] == 1)//ÁÁ
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