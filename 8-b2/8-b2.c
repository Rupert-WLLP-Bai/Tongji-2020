/*2052526 信15 白俊豪*/
/*已验证 2052312 许志康、2052538 陈诺、2052539 彭俊翔、2051500 杜奇蔚、2052521 张耀尹、2051837 覃政的hex文件生成*/
#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define SIZE 200

void input_file(char* name)
{
	fprintf(stderr, "文件名以下形式均可以：\na.txt：不带路径形式\n..\\data\\b.dat：相对路径形式\nC : \\Windows\\System32\\c.dat：绝对相对路径形式\n请输入要转换的hex格式文件名 : ");
	fgets(name, SIZE, stdin);
	if (name[strlen(name) - 1] == '\n')//删去末尾的\n
		name[strlen(name) - 1] = '\0';
}

void output_file(char* name)
{
	fprintf(stderr, "请输入转换后的文件名 : ");
	fgets(name, SIZE, stdin);
	if (name[strlen(name) - 1] == '\n')//删去末尾的\n
		name[strlen(name) - 1] = '\0';
}

/*将2位十六进制字符数组转为十六进制的数*/
int convert_hex(char ch1, char ch2)
{
	int a;
	int i1, i2;
	if (ch1 >= '0' && ch1 <= '9')
	{
		i1 = (ch1 - '0') * 16;
	}
	else
	{
		i1 = (ch1 - 'a' + 10) * 16;
	}

	if (ch2 >= '0' && ch2 <= '9')
	{
		i2 = (ch2 - '0') * 1;
	}
	else
	{
		i2 = (ch2 - 'a' + 10) * 1;
	}

	a = i1 + i2;
	return a;
}

/*读入一行，处理一行*/
/*前十个字符为序号，读完丢弃*/
/*再读中间的内容*/
/*最后的*/
void read_input_file(char* input, char* output)
{
	char line[82];//读一行
	int ch;//中间的字符
	char sh;
	int count = 0;

	FILE* fp1, * fp2;
	fp1 = fopen(input, "r");
	fp2 = fopen(output, "wb");
	if (fp1 == NULL)
	{
		fprintf(stderr, "文件%s打开失败!\n", input);
		exit(EXIT_FAILURE);
	}
	if (fp2 == NULL)
	{
		fclose(fp1);
		fprintf(stderr, "文件%s打开失败!\n", input);
		exit(EXIT_FAILURE);
	}


	while (1)
	{
		static int i = 1;
		if (getc(fp1) == EOF)//判断是否到文件末尾
			break;
		fgets(line, 81, fp1);//strlen(line)最大是80，最小是65，读完前8个是72
		//fprintf(stderr, "第%d行长为%d\n",i++, strlen(line));
		/*第一个十六进制数的位置是9,10*/
		count = strlen(line) - 64;

		if (count <= 8)
		{
			for (int i = 0; i < count; i++)
			{
				ch = convert_hex(line[9 + i * 3], line[10 + i * 3]);
				sh = (char)ch;
				fputc(sh, fp2);
			}
		}
		else
		{
			for (int i = 0; i < 8; i++)
			{
				ch = convert_hex(line[9 + i * 3], line[10 + i * 3]);
				sh = (char)ch;
				fputc(sh, fp2);
			}
			for (int i = 8; i < count; i++)
			{
				ch = convert_hex(line[11 + i * 3], line[12 + i * 3]);
				sh = (char)ch;
				fputc(sh, fp2);
			}
		}
	}
	fclose(fp1);
	fclose(fp2);
}



int main()
{
	char input[SIZE];//输入文件的文件名
	char output[SIZE];//输出文件的文件名
	input_file(input);//获取输入文件的文件名
	output_file(output);//获取输出文件的文件名
	read_input_file(input, output);//读入内容并做转换
	return 0;
}
