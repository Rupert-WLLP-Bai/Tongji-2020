/*为8-b1的测试生成随机的文本*/
#define _CRT_SECURE_NO_WARNINGS
#include<iostream>
#include<time.h>
#include<stdlib.h>
#define SIZE 145320
char ch[SIZE];//全局数组不用考虑栈

using namespace std;

void datagen(char* ch)
{
	srand((unsigned int)time(0));
	int count = 0;
	for (int i = 0; i < SIZE; i++)
	{
		ch[i] = rand();
		////任意ASCII码为正的字符
		//ch[i] = rand() % 127 + 1;
		////数字
		//ch[i] = rand() % 10 + '0';
		////可见
		//ch[i] = rand() % 94 + 33;
	}
}

int main()
{
	FILE* fp1;
	FILE* fp2;
	FILE* fp3;
	datagen(ch);
	fp1 = fopen("E:\\Homework\\高程\\Debug\\hello.docx", "wb");
	if (fp1 == NULL)
	{
		printf("文件打开失败!\n");
		exit(EXIT_FAILURE);
	}
	fp2 = fopen("E:\\Homework\\高程\\Debug\\hello.txt", "wb");
	if (fp2 == NULL)
	{
		printf("文件打开失败!\n");
		exit(EXIT_FAILURE);
	}
	fp3 = fopen("E:\\Homework\\高程\\Debug\\hello.bin", "wb");
	if (fp3 == NULL)
	{
		printf("文件打开失败!\n");
		exit(EXIT_FAILURE);
	}
	else
	{
		printf("文件打开成功!\n");
		for (int i = 0; i < SIZE; i++)
		{
			fputc(ch[i], fp1);
			fputc(ch[i], fp2);
			fputc(ch[i], fp3);
		}
		printf("文件写入完成!\n");
	}
	return 0;
}