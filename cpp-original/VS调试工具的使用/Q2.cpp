#define _CRT_SECURE_NO_WARNINGS
#include<iostream>
#include<Windows.h>
#include"VS_Debug.h"
using namespace std;

int g(unsigned int a, unsigned int b)
{
	while (a > b)
	{
		a = 2 * a + 1;
		b = 3 * b + 2;
	}
	return a;
}
/*问题2的测试函数*/
void f2()
{
	static int a = 5;
	
	a = 10;
	return;
}