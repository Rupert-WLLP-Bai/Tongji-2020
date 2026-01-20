#define _CRT_SECURE_NO_WARNINGS
#include<iostream>
#include<Windows.h>
#include"VS_Debug.h"

using namespace std;

void h()
{
	static int j = 0;
}

int main()
{
	static int k = 0;
	if (1)/*问题1*/
	{
		f1();
		k++;
	}

	if (1)/*问题2*/
	{
		f2();
		h();
		k++;
	}

	if (1)/*问题3*/
	{
		f3();
		k++;
	}
	return 0;
}