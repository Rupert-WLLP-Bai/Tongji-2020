/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
int pointer[3];//栈顶指针
int line[3][10];//三个栈
int i = 0;
void spawn(char start, int level)
{
	for (int i = 0; i < 10; i++)//初始化
	{
		for (int j = 0; j < 3; j++)
		{
			line[j][i] = 0;
		}
	}
	for (int i = 0; i < 3; i++)
	{
		pointer[i] = 0;
	}
	switch (start)//开始的数组状态
	{
		case 'A':
		{
			while (pointer[0] <= level)
			{
				line[0][pointer[0]++] = level - pointer[0];
			}
			pointer[0]--;
			break;
		}
		case 'B':
		{
			while (pointer[1] <= level)
			{
				line[1][pointer[1]++] = level - pointer[1];
			}
			pointer[1]--;
			break;
		}
		case 'C':
		{
			while (pointer[2] <= level)
			{
				line[2][pointer[2]++] = level - pointer[2];
			}
			pointer[2]--;
			break;
		}
	}

	printf("初始:             ");
	printf("A:");
	if (line[0][0] != 10)
		printf(" ");
	for (int i = 0; i < 10; i++)
	{
		if (line[0][i] != 0)
		{
			if (line[0][i] != 10)
				printf("%d ", line[0][i]);
			else
				printf("10 ");
		}
		else
			printf("  ");
	}
	printf("B:");
	if (line[1][0] != 10)
		printf(" ");
	for (int i = 0; i < 10; i++)
	{
		if (line[1][i] != 0)
		{
			if (line[1][i] != 10)
				printf("%d ", line[1][i]);
			else
				printf("10 ");
		}
		else
			printf("  ");
	}
	printf("C:");
	if (line[2][0] != 10)
		printf(" ");
	for (int i = 0; i < 10; i++)
	{
		if (line[2][i] != 0)
		{
			if (line[2][i] != 10)
				printf("%d ", line[2][i]);
			else
				printf("10 ");
		}
		else
			printf("  ");
	}
	printf("\n");
}
void haoni(int n, char src, char tmp, char dst)
{

	void move(int, char, char);
	if (n == 1)
	{
		i++;
		printf("第%4d步", i);
		move(n, src, dst);
	}
	else
	{
		haoni(n - 1, src, dst, tmp);
		i++;
		printf("第%4d步", i);
		move(n, src, dst);
		haoni(n - 1, tmp, src, dst);
	}
}

void move(int n, char x, char y)
{
	printf("(%2d)%c-->%c ", n, x, y);
	int temp;
	switch (x)
	{
		case 'A':
			if (y == 'B')
			{
				temp = line[0][--pointer[0]];
				line[0][pointer[0]] = 0;
				line[1][pointer[1]] = temp;
				pointer[1] ++;
			}
			else if (y == 'C')
			{
				temp = line[0][--pointer[0]];
				line[0][pointer[0]] = 0;
				line[2][pointer[2]] = temp;
				pointer[2] ++;
			}
			break;
		case 'B':
			if (y == 'A')
			{
				temp = line[1][--pointer[1]];
				line[1][pointer[1]] = 0;
				line[0][pointer[0]] = temp;
				pointer[0] ++;
			}
			else if (y == 'C')
			{
				temp = line[1][--pointer[1]];
				line[1][pointer[1]] = 0;
				line[2][pointer[2]] = temp;
				pointer[2] ++;
			}
			break;
		case 'C':
			if (y == 'A')
			{
				temp = line[2][--pointer[2]];
				line[2][pointer[2]] = 0;
				line[0][pointer[0]] = temp;
				pointer[0] ++;
			}
			else if (y == 'B')
			{
				temp = line[2][--pointer[2]];
				line[2][pointer[2]] = 0;
				line[1][pointer[1]] = temp;
				pointer[1] ++;
			}
			break;
	}
	printf("A:");
	if (line[0][0] != 10)
		printf(" ");
	for (int i = 0; i < 10; i++)
	{
		if (line[0][i] != 0)
		{
			if (line[0][i] != 10)
				printf("%d ", line[0][i]);
			else
				printf("10 ");
		}
		else
			printf("  ");
	}
	printf("B:");
	if (line[1][0] != 10)
		printf(" ");
	for (int i = 0; i < 10; i++)
	{
		if (line[1][i] != 0)
		{
			if (line[1][i] != 10)
				printf("%d ", line[1][i]);
			else
				printf("10 ");
		}
		else
			printf("  ");
	}
	printf("C:");
	if (line[2][0] != 10)
		printf(" ");
	for (int i = 0; i < 10; i++)
	{
		if (line[2][i] != 0)
		{
			if (line[2][i] != 10)
				printf("%d ", line[2][i]);
			else
				printf("10 ");
		}
		else
			printf("  ");
	}
	printf("\n");
}

int main()
{
	int level;
	int re;
	char start, mid, end;
	while (1)
	{
		printf("请输入汉诺塔的层数(1-10)\n");
		re = scanf("%d", &level);
		while (re != 1)
		{
			rewind(stdin);
			printf("请输入汉诺塔的层数(1-10)\n");
			re = scanf("%d", &level);
		}
		if (level >= 1 && level <= 10)
			break;
	}
	rewind(stdin);
	while (1)
	{
		printf("请输入起始柱(A-C)\n");
		re = scanf("%c", &start);
		while (re != 1)
		{
			rewind(stdin);
			printf("请输入起始柱(A-C)\n");
			re = scanf("%c", &start);
		}
		if (start == 'A' || start == 'B' || start == 'C')
			break;
		else if (start == 'a' || start == 'b' || start == 'c')
		{
			start -= 32;
			break;
		}
		rewind(stdin);
	}
	rewind(stdin);
	while (1)
	{
		printf("请输入目标柱(A-C)\n");
		re = scanf("%c", &end);
		while (re != 1)
		{
			rewind(stdin);
			printf("请输入目标柱(A-C)\n");
			re = scanf("%c", &end);
		}
		if (end == start || end == start + 32 || end == start - 32)
		{
			rewind(stdin);
			printf("目标柱不能与起始柱相同\n");
			continue;
		}
		if (end == 'A' || end == 'B' || end == 'C')
			break;
		else if (end == 'a' || end == 'b' || end == 'c')
		{
			end -= 32;
			break;
		}
		rewind(stdin);
	}
	mid = 'A' + 'B' + 'C' - start - end;
	spawn(start, level);
	haoni(level, start, mid, end);
	return 0;
}