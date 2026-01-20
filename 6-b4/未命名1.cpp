/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<stdlib.h>
#define SIZE 80
inline void input(char *name)
{
	printf("文件名以下形式均可以：\na.txt：不带路径形式\n..\\data\\b.dat：相对路径形式\nC : \\Windows\\System32\\c.dat：绝对相对路径形式\n请输入文件名 : ");
#if __GNUC__
		scanf("%s", name);
#elif _MSC_VER
	scanf_s("%s", name,SIZE);
#endif
}

void print_line(char* sh,int count)
{
	for (int i = 0; i < count; i++)
	{
		printf("%c", sh[i]);
	}
}

void output(char* name)
{
	FILE* fp1;
	int count = 0;
	char ch;
	char sh[16];
	fp1 = fopen(name, "r");
	if (fp1 == NULL)
	{
		printf("文件%s打开失败!\n",name);
		exit(EXIT_FAILURE);
	}
	while ((ch = fgetc(fp1)) != EOF)
	{
		sh[count % 16] = ch;
		if (count % 16 == 0)
		{
			if(count >= 16)
				printf("\n");
			printf("%08x ", count);
		}
		if (count % 16 == 8)
			printf("- ");
		count++;
		printf("%02x ",(int)ch);
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
