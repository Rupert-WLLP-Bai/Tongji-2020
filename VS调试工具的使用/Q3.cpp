#define _CRT_SECURE_NO_WARNINGS
#include<iostream>
#include<Windows.h>
#include"VS_Debug.h"
using namespace std;

extern int TEST;

void Q6(int *a)
{
	a[0] = 9;
}

/*问题3的测试函数*/
void f3()
{
	int a = 1;
	char b = 2;
	float c = 3.0f;
	int d[2] = { 0 };
	int e[2][2] = {1,2,3,4};
	int f[][2] = { 1,2,3,4,5,6 };
	int* p1 = &a;
	int* p2 = d;
	(*p1)++;
	Q6(d);


	const char s1[9] = "abcdefg";
	char s2[10];
	const char* S1 = s1;
	const char* S2 = s2;
	
	int& r = a;
	int* q = &d[2];

	TEST = 2;

}
