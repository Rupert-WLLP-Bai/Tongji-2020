/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define SIZE 100

void input(char* name)
{
	fprintf(stderr,"文件名以下形式均可以：\na.txt：不带路径形式\n..\\data\\b.dat：相对路径形式\nC : \\Windows\\System32\\c.dat：绝对相对路径形式\n请输入文件名 : ");
	fgets(name, SIZE, stdin);
	if (name[strlen(name) - 1] == '\n')//删去末尾的\n
		name[strlen(name) - 1] = '\0';
}

/*输出一行的内容*/
void print_line(char* sh, int count)
{
	printf("    ");
	for (int i = 0; i <= count % 16; i++)
	{
		if (sh[i] < 33 || sh[i]>126)//空格仍然输出 "."
			printf(".");
		else
			printf("%c", sh[i]);
	}
}

void output(char* name)
{
	FILE* fp1;
	fp1 = fopen(name, "rb");
	if (fp1 == NULL)
	{
		fprintf(stderr,"文件%s打开失败!\n", name);
		exit(EXIT_FAILURE);
	}

	int count = 0;
	int ch;
	char sh[16];
	while ((ch = fgetc(fp1)) != EOF)//用int型的原因是区分 十六进制的FF 和 EOF
	{
		sh[count % 16] = ch;
		if (count % 16 == 0)
		{
			if (count >= 16)
				printf("\n");
			printf("%08x  ", count);//打印序号
		}
		if (count % 16 == 8)
			printf("- ");
		printf("%02x ", (unsigned char)ch);//强制类型转换,用0补齐
		if ((count + 1) % 16 == 0)
		{
			print_line(sh, count);
		}
		count++;
	}
	//跳出循环后判断是否需要再输出一行
	if (count % 16 != 0)//补齐空格
	{
		if (count % 16 <= 8)//再补两个空格
			printf("  ");
		for (int i = count % 16; i < 16; i++)
			printf("   ");
		print_line(sh, count-1);
	}
	printf("\n");
	fclose(fp1);
}


int main()
{
	char name[SIZE];//储存文件名
	input(name);
	output(name);
	return 0;
}