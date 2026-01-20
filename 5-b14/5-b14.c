/*2052526 信15 白俊豪*/
/*已验证 2052312 许志康、2052521 张耀尹、2052849 滕明鑫、2052081 郑雄、2051500 杜奇蔚 的扫雷内部数组*/
#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<string.h>
#include<stdbool.h>

int check_1(char ch[12][100])
{
	int count = 0;
	for (int i = 0; i < 12; i++)
	{
		for (int j = 0; j < 100; j++)
		{
			if (ch[i][j] == '*')
				count++;
		}
	}
	
	if (count != 50)
		return -1;
	else
		return 0;
}

int Num_of_mine(int i, int j, bool mine[12][28])
{
	int num;
	num = mine[i - 1][j - 1] + mine[i][j - 1] + mine[i + 1][j - 1] + mine[i - 1][j]
		+ mine[i + 1][j] + mine[i - 1][j + 1] + mine[i][j + 1] + mine[i + 1][j + 1];
	return num;
}

int check_2(char ch[12][100],int a[12][28],bool mine[12][28])//正确返回0，否则返回-2
{
	for (int i = 1; i < 11; i++)
	{
		for (int j = 1; j < 27; j++)
		{
			if (!mine[i][j])
			{
				if ((int)(ch[i][j-1])-'0' != a[i][j])
					return -2;
			}
		}
	}
	return 0;
}

int main()
{
	int k = 0;
	int a[12][28] = { 0 };
	char sh[12][100] = { 0 };
	char ch[12][100] = { 0 };
	bool mine[12][28] = { 0 };

	for (int i = 1; i < 11; i++)
	{
		fgets(sh[i], 100, stdin);
	}

	for (int i = 1; i < 11; i++)//重新写入ch中
	{
		k = 0;
		for(int j = 0; j < 60; j++)
		{
			if (sh[i][j]==' '|| sh[i][j]=='\n')
				continue;
			else
				ch[i][k++] = sh[i][j];
		}
	}

	for (int i = 1; i < 11; i++)//构建bool型雷阵
	{
		for (int j = 1; j < 27; j++)
		{
			if (ch[i][j-1] == '*')
				mine[i][j] = 1;
		}
	}

	for (int i = 1; i < 11; i++)//重新计算雷数
	{
		for (int j = 1; j < 27; j++)
		{
			a[i][j] = Num_of_mine(i, j, mine);
		}
	}
	
	//雷阵0/1测试
	/*for (int i = 0; i < 12; i++)
	{
		for (int j = 0; j < 28; j++)
		{
			printf("%d",mine[i][j]);
		}
		printf("\n");
	}*/

	//5-b13结果
	/*for (int i = 0; i < 12; i++)
	{
		puts(ch[i]);
	}*/


	//雷阵0/1测试
	/*printf("bool型雷阵 : \n");
	for (int i = 1; i < 11; i++)
	{
		for (int j = 1; j < 27; j++)
		{
			printf("%d",mine[i][j]);
		}
		printf("\n");
	}*/

	//ch测试
	/*printf("ch(输入的值) : \n");
	for (int i = 1; i < 11; i++)
	{
		for (int j = 1; j < 27; j++)
		{
			printf("%c",ch[i][j-1]);
		}
		printf("\n");
	}*/

	//a测试
	/*printf("重新计算的值 : \n");
	for (int i = 1; i < 11; i++)
	{
		for (int j = 1; j < 27; j++)
		{
			if (!mine[i][j])
				printf("%d", a[i][j]);
			else
				printf("%%");
		}
		printf("\n");
	}*/

	if (check_1(ch) == -1)
		printf("错误\n");
	else if (check_2(ch,a,mine) == -2)
		printf("错误\n");
	else
		printf("正确\n");

	return 0;
}
