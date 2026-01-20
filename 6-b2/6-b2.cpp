 /*2052526 信15 白俊豪*/
#include <iostream>
#include <cstring>
#define SIZE 81
using namespace std;

bool f(char* a)
{
	char* p1 = a;
	char* p2 = &a[strlen(a) - 1];
	int mid = strlen(a) / 2;
	while(p1<p2)
	{
		if (*p1++ != *p2--)
			return 0;
	}
	return 1;
}

int main()
{
	char a[SIZE];
	cout << "请输入一个长度小于80的字符串（回文串）" << endl;
	fgets(a, SIZE,stdin);
	a[strlen(a) - 1] = '\0';
	if (f(a))
		cout << "yes" << endl;
	else
		cout << "no" << endl;
	return 0;
}
